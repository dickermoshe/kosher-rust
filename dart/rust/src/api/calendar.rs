//! Methods Dart calls on kosher-rust's calendar types. The `frb_override_` methods replace the ones
//! that take `self`, which would use up the Dart object.

use extend::ext;
use flutter_rust_bridge::frb;
use jiff::{
    civil::{Date, Weekday},
    Timestamp,
};
use kosher_rust::calendar::{HebrewDate, HebrewMonth, HebrewYear, Molad, Parsha, Region, SpecialDay, YearLength};

#[ext]
pub impl HebrewDate {
    /// The date `day` `month` `year`, for example 15 Nisan 5785.
    ///
    /// Returns null when there is no such day. Day 0, day 30 of a 29-day month, Adar I in a common
    /// year and dates outside the range a `HebrewDate` covers have none.
    fn of(year: i32, month: HebrewMonth, day: u8) -> Option<HebrewDate> {
        HebrewDate::new(year, month, day)
    }

    /// The Hebrew date of the Gregorian `date`.
    ///
    /// Returns null when `date` is outside the range a `HebrewDate` covers, 1 January of year 1
    /// through 29 Elul 9999.
    fn from_gregorian(date: Date) -> Option<HebrewDate> {
        HebrewDate::try_from(date).ok()
    }

    /// The year.
    #[frb(getter)]
    fn frb_override_year(&self) -> HebrewYear {
        self.year()
    }

    /// The month.
    #[frb(getter)]
    fn frb_override_month(&self) -> HebrewMonth {
        self.month()
    }

    /// The day of the month, from 1.
    #[frb(getter)]
    fn frb_override_day(&self) -> u8 {
        self.day()
    }

    /// The day of the year, from 1 on Rosh Hashana.
    #[frb(getter)]
    fn frb_override_day_of_year(&self) -> u16 {
        self.day_of_year()
    }

    /// The day of the week, 1 for Monday through 7 for Sunday.
    #[frb(getter)]
    fn frb_override_weekday(&self) -> Weekday {
        self.weekday()
    }

    /// The same day on the Gregorian calendar.
    fn frb_override_to_gregorian(&self) -> Date {
        self.to_gregorian()
    }

    /// The date `days` days later, or earlier when `days` is negative.
    ///
    /// Returns null past either end of the range a `HebrewDate` covers.
    fn frb_override_add_days(&self, days: i32) -> Option<HebrewDate> {
        self.add_days(days)
    }

    /// The holidays, fasts and other notable days that fall on this date.
    ///
    /// Includes the modern Israeli days (Yom HaShoah, Yom HaZikaron, Yom HaAtzmaut, Yom
    /// Yerushalayim). Filter them out with `SpecialDay.isModern` if you don't want them.
    fn frb_override_special_days(&self, region: Region) -> Vec<SpecialDay> {
        self.special_days(region).collect()
    }

    /// Whether work is forbidden, because it is Shabbos or a day of Yom Tov.
    fn frb_override_is_assur_bemelacha(&self, region: Region) -> bool {
        self.is_assur_bemelacha(region)
    }

    /// Whether candles are lit this evening, because tomorrow is Shabbos or Yom Tov.
    fn frb_override_has_candle_lighting(&self, region: Region) -> bool {
        self.has_candle_lighting(region)
    }

    /// Whether this is one of the Ten Days of Repentance, 1 to 10 Tishrei.
    #[frb(getter)]
    fn frb_override_is_aseres_yemei_teshuva(&self) -> bool {
        self.is_aseres_yemei_teshuva()
    }

    /// The weekly *parsha* read on this Shabbos.
    ///
    /// Returns null on a weekday, and on a Shabbos when the Yom Tov reading replaces it.
    fn frb_override_parsha(&self, region: Region) -> Option<Parsha> {
        self.parsha(region)
    }

    /// The special Shabbos this is, if any. That is Shekalim, Zachor, Parah, Hachodesh, Hagadol,
    /// Chazon, Nachamu, Shuva or Shira.
    ///
    /// Returns null on a weekday or an ordinary Shabbos.
    fn frb_override_special_parsha(&self, region: Region) -> Option<Parsha> {
        self.special_parsha(region)
    }

    /// The weekly *parsha* of the next Shabbos that has one, skipping Shabbosos when the Yom Tov
    /// reading replaces it.
    ///
    /// On Shabbos this looks at next Shabbos. Use `parsha` for today's. Returns null only near the
    /// limits of the supported date range.
    fn frb_override_next_parsha(&self, region: Region) -> Option<Parsha> {
        self.next_parsha(region)
    }

    /// The molad of this date's month.
    #[frb(getter)]
    fn frb_override_molad(&self) -> Molad {
        self.molad()
    }

    /// Whether `other` is the same day.
    fn is_same_day(&self, other: &HebrewDate) -> bool {
        self == other
    }

    /// The date written out, for example "15 Nisan 5784".
    #[frb(name = "toString")]
    fn display(&self) -> String {
        self.to_string()
    }
}

#[ext]
pub impl HebrewYear {
    /// The Hebrew year `year`, for example 5785.
    ///
    /// Returns null outside the years 3761 to 9999.
    fn of(year: i32) -> Option<HebrewYear> {
        HebrewYear::new(year)
    }

    /// The year as a number.
    #[frb(getter)]
    fn value(&self) -> i32 {
        self.get()
    }

    /// Whether this year has thirteen months, as years 3, 6, 8, 11, 14, 17 and 19 of the 19-year
    /// cycle do.
    #[frb(getter)]
    fn frb_override_is_leap(&self) -> bool {
        self.is_leap()
    }

    /// The number of days in the year. That is 353, 354 or 355, or 383, 384 or 385 in a leap year.
    #[frb(getter)]
    fn frb_override_days(&self) -> u16 {
        self.days()
    }

    /// Whether Cheshvan and Kislev are long or short this year.
    #[frb(getter)]
    fn frb_override_length(&self) -> YearLength {
        self.length()
    }

    /// Whether Cheshvan has 30 days this year.
    #[frb(getter)]
    fn frb_override_is_cheshvan_long(&self) -> bool {
        self.is_cheshvan_long()
    }

    /// Whether Kislev has 29 days this year.
    #[frb(getter)]
    fn frb_override_is_kislev_short(&self) -> bool {
        self.is_kislev_short()
    }

    /// The months of this year in order from Tishrei. There are 12, or 13 in a leap year.
    #[frb(getter)]
    fn frb_override_months(&self) -> Vec<HebrewMonth> {
        self.months().to_vec()
    }

    /// The number of days in `month` this year.
    ///
    /// Returns null for Adar I in a common year.
    fn frb_override_days_in(&self, month: HebrewMonth) -> Option<u8> {
        self.days_in(month)
    }

    /// The first day of the year, 1 Tishrei.
    ///
    /// Returns null for 3761, whose Rosh Hashana is before the first day a `HebrewDate` covers.
    #[frb(getter)]
    fn frb_override_rosh_hashana(&self) -> Option<HebrewDate> {
        self.rosh_hashana()
    }
}

#[ext]
pub impl Molad {
    /// The moment of the molad, converted from Jerusalem mean time, in seconds since the Unix
    /// epoch.
    #[frb(getter)]
    fn frb_override_timestamp(&self) -> Timestamp {
        self.timestamp()
    }

    /// The day of the week, counting from midnight, 1 for Monday through 7 for Sunday.
    #[frb(getter)]
    fn frb_override_weekday(&self) -> Weekday {
        self.weekday()
    }

    /// The hour, from 0 to 23.
    #[frb(getter)]
    fn frb_override_hours(&self) -> u8 {
        self.hours()
    }

    /// The minutes past the hour, from 0 to 59.
    #[frb(getter)]
    fn frb_override_minutes(&self) -> u8 {
        self.minutes()
    }

    /// The *chalakim* past the minute, from 0 to 17. A *chelek* is 1/1080 of an hour, or 3⅓
    /// seconds.
    #[frb(getter)]
    fn frb_override_chalakim(&self) -> u8 {
        self.chalakim()
    }
}

#[ext]
pub impl HebrewMonth {
    /// The name in English, for example "Nisan" or "Adar II".
    #[frb(getter)]
    fn display_name(self) -> String {
        self.name().into()
    }

    /// The name in Hebrew, for example "ניסן".
    #[frb(getter)]
    fn hebrew_name(self) -> String {
        self.hebrew().into()
    }
}

#[ext]
pub impl Parsha {
    /// The name in English, for example "Bereshis" or "Vayakhel-Pekudei".
    #[frb(getter)]
    fn display_name(self) -> String {
        self.name().into()
    }

    /// The name in Hebrew, for example "בראשית".
    #[frb(getter)]
    fn hebrew_name(self) -> String {
        self.hebrew().into()
    }
}

#[ext]
pub impl SpecialDay {
    /// The name in English, for example "Pesach".
    #[frb(getter)]
    fn display_name(self) -> String {
        self.name().into()
    }

    /// The name in Hebrew, for example "פסח".
    #[frb(getter)]
    fn hebrew_name(self) -> String {
        self.hebrew().into()
    }
}
