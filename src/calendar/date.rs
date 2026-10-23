use core::fmt;
use core::iter::FusedIterator;

use icu_calendar::options::DateAddOptions;
use icu_calendar::types::{DateDuration, Weekday as IcuWeekday};
use icu_calendar::{AsCalendar, Date, Gregorian, Iso, cal::Hebrew};
use jiff::civil::{self, Weekday};
use jiff_icu::{ConvertFrom, ConvertTryFrom};
use thiserror::Error;

use super::molad::{Molad, molad_of_month};
use super::month::{ADAR, ADARI, AV, HebrewMonth, NISAN, SHEVAT, TISHREI};
use super::parsha::Parsha;
use super::parsha_tables::get_parsha_list;
use super::special_day::{SpecialDay, SpecialDayIterator};
use super::year::{HebrewYear, LAST_YEAR, elapsed_days};

/// Where the calendar is kept: Israel or the diaspora.
///
/// Outside Israel, Yom Tov has a second day, which also shifts the weekly *parsha* for some weeks
/// of the year.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Region {
    /// Eretz Yisrael, with one day of Yom Tov.
    Israel,
    /// Everywhere else, with two days of Yom Tov.
    Diaspora,
}

impl Region {
    pub(crate) const fn in_israel(self) -> bool {
        matches!(self, Region::Israel)
    }
}

/// A day on the Hebrew calendar, and everything the calendar says about it.
///
/// Convert from a [`jiff::civil::Date`] or any ICU date with `HebrewDate::try_from`, or build one
/// with [`HebrewDate::new`]. A Hebrew day begins at nightfall, but a `HebrewDate` is the civil day
/// it mostly overlaps. So `HebrewDate::try_from(date(2024, 4, 23))` is 15 Nisan, the first day of
/// Pesach, not the evening before.
///
/// A `HebrewDate` covers 1 January of year 1, which is 18 Teves 3761, through 29 Elul 9999, which
/// is in Gregorian 6239. Converting a date outside that range fails with [`DateOutOfRange`].
///
/// ```
/// use jiff::civil::date;
/// use kosher_rust::calendar::{SpecialDay, HebrewDate, HebrewMonth, Region};
///
/// let day = HebrewDate::try_from(date(2024, 4, 23))?;
/// assert_eq!(day.to_string(), "15 Nisan 5784");
/// assert_eq!(day.month(), HebrewMonth::Nisan);
/// assert!(day.special_days(Region::Diaspora).any(|h| h == SpecialDay::Pesach));
/// assert_eq!(day.to_gregorian(), Some(date(2024, 4, 23)));
/// # Ok::<(), kosher_rust::calendar::DateOutOfRange>(())
/// ```
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct HebrewDate(Date<Hebrew>);

/// A date outside the range a [`HebrewDate`] covers, which is 1 January of year 1 (18 Teves 3761)
/// through 29 Elul 9999.
#[derive(Debug, Error, PartialEq, Eq, Clone, Copy, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[error("the date is outside the range a HebrewDate covers, 1 January of year 1 through 29 Elul 9999")]
pub struct DateOutOfRange;

impl HebrewDate {
    /// The date `day` `month` `year`, for example `HebrewDate::new(5785, HebrewMonth::Nisan, 15)`.
    ///
    /// Returns `None` when there is no such day. Day 0, day 30 of a 29-day month, Adar I in a
    /// common year and dates outside the range a `HebrewDate` covers have none.
    pub fn new(year: i32, month: HebrewMonth, day: u8) -> Option<Self> {
        Self::checked(Date::try_new_hebrew_v2(year, month.to_icu(), day).ok()?)
    }

    /// `date`, if it is in the range a `HebrewDate` covers.
    pub(crate) fn checked(date: Date<Hebrew>) -> Option<Self> {
        let first_day = Date::try_new_iso(1, 1, 1).ok()?.to_calendar(Hebrew);
        (date >= first_day && date.year().extended_year() <= LAST_YEAR).then_some(Self(date))
    }

    pub(crate) const fn from_icu(date: Date<Hebrew>) -> Self {
        Self(date)
    }

    pub(crate) const fn icu(self) -> Date<Hebrew> {
        self.0
    }

    /// The year.
    pub fn year(self) -> HebrewYear {
        HebrewYear::unchecked(self.0.year().extended_year())
    }

    /// The month.
    pub fn month(self) -> HebrewMonth {
        // ICU only produces Hebrew months for a Hebrew date, so this always matches.
        HebrewMonth::from_icu(self.0.month().to_input()).unwrap_or(HebrewMonth::Tishrei)
    }

    /// The day of the month, from 1.
    pub fn day(self) -> u8 {
        self.0.day_of_month().0
    }

    /// The day of the year, from 1 on Rosh Hashana.
    pub fn day_of_year(self) -> u16 {
        self.0.day_of_year().0
    }

    /// The day of the week.
    pub fn weekday(self) -> Weekday {
        Weekday::convert_from(self.0.weekday())
    }

    /// The same day on the Gregorian calendar.
    ///
    /// Returns `None` outside the years -9999 to 9999, which `jiff` cannot represent.
    pub fn to_gregorian(self) -> Option<civil::Date> {
        civil::Date::convert_try_from(self.0.to_calendar(Gregorian)).ok()
    }

    /// The date `days` days later, or earlier when `days` is negative.
    ///
    /// Returns `None` past either end of the range a `HebrewDate` covers.
    pub fn add_days(self, days: i32) -> Option<Self> {
        Self::checked(
            self.0
                .try_added_with_options(DateDuration::for_days(days), DateAddOptions::default())
                .ok()?,
        )
    }

    /// The holidays, fasts and other notable days that fall on this date, in calendar order.
    ///
    /// Includes the modern Israeli days (Yom HaShoah, Yom HaZikaron, Yom HaAtzmaut, Yom
    /// Yerushalayim); filter them out with [`SpecialDay::is_modern`] if you don't want them.
    ///
    /// ```
    /// use jiff::civil::date;
    /// use kosher_rust::calendar::{SpecialDay, HebrewDate, Region};
    ///
    /// // 16 Nisan is the second day of Pesach in the diaspora and Chol Hamoed in Israel.
    /// let day = HebrewDate::try_from(date(2024, 4, 24))?;
    /// assert!(day.special_days(Region::Diaspora).any(|h| h == SpecialDay::Pesach));
    /// assert!(day.special_days(Region::Israel).any(|h| h == SpecialDay::CholHamoedPesach));
    /// # Ok::<(), kosher_rust::calendar::DateOutOfRange>(())
    /// ```
    pub fn special_days(self, region: Region) -> impl FusedIterator<Item = SpecialDay> {
        SpecialDayIterator::new(self.0, region.in_israel())
    }

    /// Whether work is forbidden: Shabbos, or a day of Yom Tov.
    pub fn is_assur_bemelacha(self, region: Region) -> bool {
        self.0.weekday() == IcuWeekday::Saturday || self.special_days(region).any(|h| h.is_assur_bemelacha())
    }

    /// Whether candles are lit this evening, because tomorrow is Shabbos or Yom Tov.
    pub fn has_candle_lighting(self, region: Region) -> bool {
        self.add_days(1)
            .is_some_and(|tomorrow| tomorrow.is_assur_bemelacha(region))
    }

    /// Whether this is one of the Ten Days of Repentance, 1 to 10 Tishrei.
    pub fn is_aseres_yemei_teshuva(self) -> bool {
        self.0.month().to_input() == TISHREI && self.day() <= 10
    }

    /// The weekly *parsha* read on this Shabbos.
    ///
    /// Returns `None` on a weekday, and on a Shabbos when the Yom Tov reading replaces it.
    pub fn parsha(self, region: Region) -> Option<Parsha> {
        let date = self.0;
        if date.weekday() != IcuWeekday::Saturday {
            return None;
        }

        let parsha_list = get_parsha_list(&date, region.in_israel())?;

        let rosh_hashana_day_of_week = elapsed_days(date.year().extended_year() as i64) % 7;
        let day = rosh_hashana_day_of_week + date.day_of_year().0 as i64;
        let week_index = usize::try_from(day / 7).ok()?;
        parsha_list.get(week_index).copied().flatten()
    }

    /// The special Shabbos this is, if any: Shekalim, Zachor, Parah, Hachodesh, Hagadol, Chazon,
    /// Nachamu, Shuva or Shira.
    ///
    /// Returns `None` on a weekday or an ordinary Shabbos.
    pub fn special_parsha(self, region: Region) -> Option<Parsha> {
        let date = self.0;
        if date.weekday() != IcuWeekday::Saturday {
            return None;
        }

        let month = date.month().to_input();
        let day = self.day();
        let is_leap = self.year().is_leap();

        // Shekalim: the Shabbos on or before Rosh Chodesh Adar (Adar II in a leap year).
        if ((month == SHEVAT && !is_leap) || (month == ADARI && is_leap)) && (day == 25 || day == 27 || day == 29) {
            return Some(Parsha::Shekalim);
        }

        if month == ADAR {
            if day == 1 {
                return Some(Parsha::Shekalim);
            }
            // Zachor: the Shabbos before Purim.
            if day == 8 || day == 9 || day == 11 || day == 13 {
                return Some(Parsha::Zachor);
            }
            // Parah: the Shabbos before Hachodesh.
            if day == 18 || day == 20 || day == 22 || day == 23 {
                return Some(Parsha::Parah);
            }
            // Hachodesh: the Shabbos on or before Rosh Chodesh Nisan.
            if day == 25 || day == 27 || day == 29 {
                return Some(Parsha::Hachodesh);
            }
        }

        if month == NISAN {
            if day == 1 {
                return Some(Parsha::Hachodesh);
            }
            if (8..=14).contains(&day) {
                return Some(Parsha::Hagadol);
            }
        }

        if month == AV {
            if (4..=9).contains(&day) {
                return Some(Parsha::Chazon);
            }
            if (10..=16).contains(&day) {
                return Some(Parsha::Nachamu);
            }
        }

        if month == TISHREI && (3..=8).contains(&day) {
            return Some(Parsha::Shuva);
        }

        if self.parsha(region) == Some(Parsha::Beshalach) {
            return Some(Parsha::Shira);
        }

        None
    }

    /// The weekly *parsha* of the next Shabbos that has one, skipping Shabbosos when the Yom Tov
    /// reading replaces it.
    ///
    /// On Shabbos this looks at next Shabbos; use [`parsha`](Self::parsha) for today's. Returns
    /// `None` only near the limits of the supported date range.
    pub fn next_parsha(self, region: Region) -> Option<Parsha> {
        let days_to_shabbos = match self.0.weekday() {
            IcuWeekday::Monday => 5,
            IcuWeekday::Tuesday => 4,
            IcuWeekday::Wednesday => 3,
            IcuWeekday::Thursday => 2,
            IcuWeekday::Friday => 1,
            IcuWeekday::Saturday => 7,
            IcuWeekday::Sunday => 6,
        };

        let mut date = self.add_days(days_to_shabbos)?;
        // Bounded: Yom Tov never displaces more than a few weeks in a row.
        for _ in 0..60 {
            if let Some(parsha) = date.parsha(region) {
                return Some(parsha);
            }
            date = date.add_days(7)?;
        }
        None
    }

    /// The molad of this date's month.
    ///
    /// Returns `None` near the ends of the range a `HebrewDate` covers.
    pub fn molad(self) -> Option<Molad> {
        molad_of_month(&self.0)
    }
}

impl TryFrom<civil::Date> for HebrewDate {
    type Error = DateOutOfRange;

    fn try_from(date: civil::Date) -> Result<Self, DateOutOfRange> {
        Self::checked(Date::<Iso>::convert_from(date).to_calendar(Hebrew)).ok_or(DateOutOfRange)
    }
}

impl<C: AsCalendar> TryFrom<Date<C>> for HebrewDate {
    type Error = DateOutOfRange;

    fn try_from(date: Date<C>) -> Result<Self, DateOutOfRange> {
        Self::checked(date.to_calendar(Hebrew)).ok_or(DateOutOfRange)
    }
}

/// Formats as "15 Nisan 5784", with "Adar I" and "Adar II" in a leap year.
impl fmt::Display for HebrewDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let year = self.year();
        write!(f, "{} {} {}", self.day(), self.month().english_in(year), year)
    }
}

impl fmt::Debug for HebrewDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("HebrewDate")
            .field("year", &self.year().get())
            .field("month", &self.month())
            .field("day", &self.day())
            .finish()
    }
}

impl core::hash::Hash for HebrewDate {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.year().hash(state);
        self.month().hash(state);
        self.day().hash(state);
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for HebrewDate {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "{=u8} {} {=i32}", self.day(), self.month(), self.year().get())
    }
}
