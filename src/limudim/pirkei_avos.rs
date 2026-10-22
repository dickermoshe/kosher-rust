use core::fmt;

use icu_calendar::{Date, cal::Hebrew, types::Weekday};

use crate::{
    calendar::{
        Region,
        month::{AV, ELUL, NISAN, SIVAN},
    },
    limudim::engine::{Cycle, CycleFinder, HebrewDateExt, InternalLimud, Interval},
};

fn day_of_week_number(date: Date<Hebrew>) -> i32 {
    match date.weekday() {
        Weekday::Sunday => 1,
        Weekday::Monday => 2,
        Weekday::Tuesday => 3,
        Weekday::Wednesday => 4,
        Weekday::Thursday => 5,
        Weekday::Friday => 6,
        Weekday::Saturday => 7,
    }
}

/// The Pirkei Avos read on a Shabbos afternoon. Prints as "Pirkei Avos 3" or "Pirkei Avos 5-6".
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PirkeiAvosUnit {
    /// One chapter (*perek*), 1 to 6.
    Single(u8),
    /// Two chapters read together, at the end of the summer.
    Combined(u8, u8),
}

impl fmt::Display for PirkeiAvosUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PirkeiAvosUnit::Single(chapter) => write!(f, "Pirkei Avos {chapter}"),
            PirkeiAvosUnit::Combined(first, second) => write!(f, "Pirkei Avos {first}-{second}"),
        }
    }
}

/// Pirkei Avos: a chapter each Shabbos afternoon from Pesach to Rosh Hashana, doubled up at the
/// end of the summer. The [`Region`] matters because the second day of Yom Tov in the diaspora
/// can fall on Shabbos.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PirkeiAvos(pub Region);

impl InternalLimud<PirkeiAvosUnit> for PirkeiAvos {
    fn cycle_finder(&self) -> CycleFinder {
        if self.0.in_israel() {
            CycleFinder::Perpetual(Self::find_yearly_cycle_israel)
        } else {
            CycleFinder::Perpetual(Self::find_yearly_cycle_diaspora)
        }
    }

    fn unit_for_interval(&self, interval: &Interval, _limud_date: &Date<Hebrew>) -> Option<PirkeiAvosUnit> {
        let iteration = interval.iteration;

        // First 18 weeks: standard 1-6 cycle repeated 3 times
        if iteration < 19 {
            let chapter = ((iteration - 1) % 6) + 1;
            return Some(PirkeiAvosUnit::Single(chapter as u8));
        }

        // Fourth round: use weeks remaining logic (like hebcal)
        // Calculate weeks remaining from this interval's Shabbat to the end of the cycle
        let days_until_end = interval.end_date.days_until(&interval.cycle.end_date)?;
        let weeks_remain = days_until_end.div_ceil(7);

        match weeks_remain {
            0 => Some(PirkeiAvosUnit::Combined(5, 6)),
            1 => Some(PirkeiAvosUnit::Combined(3, 4)),
            2 => {
                // If iteration % 6 == 1, return [2], else [1,2]
                if (iteration - 1) % 6 == 0 {
                    Some(PirkeiAvosUnit::Combined(1, 2))
                } else {
                    Some(PirkeiAvosUnit::Single(((iteration - 1) % 6 + 1) as u8))
                }
            }
            3 => Some(PirkeiAvosUnit::Single(1)),
            _ => {
                // Continue normal cycle for weeks > 3 remaining
                let chapter = ((iteration - 1) % 6) + 1;
                Some(PirkeiAvosUnit::Single(chapter as u8))
            }
        }
    }
    fn interval_end_calculation(_cycle: Cycle, hebrew_date: Date<Hebrew>) -> Option<Date<Hebrew>> {
        // Each interval is a week, ending on Shabbos
        let day_number = day_of_week_number(hebrew_date);
        hebrew_date.add_days(7 - day_number)
    }

    fn is_skip_interval(&self, interval: &Interval) -> bool {
        let end_month = interval.end_date.month().to_input();
        let end_day = interval.end_date.day_of_month().0;

        // Skip erev Tisha B'Av (8th of Av) - applies to both Israel and diaspora
        if end_month == AV && end_day == 8 {
            return true;
        }

        // Skip Tisha B'Av (9th of Av) - applies to both Israel and diaspora
        if end_month == AV && end_day == 9 {
            return true;
        }

        // Skip 7th of Sivan (2nd day Shavuot) - only outside Israel
        if !self.0.in_israel() && end_month == SIVAN && end_day == 7 {
            return true;
        }

        false
    }
}
super::schedule!(PirkeiAvos => PirkeiAvosUnit);

impl PirkeiAvos {
    fn find_yearly_cycle_israel(date: Date<Hebrew>) -> Option<(Date<Hebrew>, Date<Hebrew>)> {
        Self::find_yearly_cycle(true, date)
    }

    fn find_yearly_cycle_diaspora(date: Date<Hebrew>) -> Option<(Date<Hebrew>, Date<Hebrew>)> {
        Self::find_yearly_cycle(false, date)
    }

    /// Find the Pirkei Avos cycle for a given date.
    /// Cycle starts the day after Pesach (Nissan 22 in Israel, Nissan 23 outside)
    /// and ends on the last Shabbos before Rosh Hashanah.
    ///
    /// `None` from the year 10000 on: the calendar builds dates by year only up to 9999.
    fn find_yearly_cycle(in_israel: bool, date: Date<Hebrew>) -> Option<(Date<Hebrew>, Date<Hebrew>)> {
        let year = date.year().extended_year();

        // Day after Pesach: Nissan 22 in Israel, Nissan 23 outside
        let anchor_day = if in_israel { 22 } else { 23 };
        let cycle_start_this_year = Date::try_new_hebrew_v2(year, NISAN, anchor_day).ok()?;

        // Determine which year's cycle we're in
        let (start_date, cycle_year) = if date >= cycle_start_this_year {
            (cycle_start_this_year, year)
        } else {
            // We're before this year's cycle starts, use previous year's cycle
            let prev_year_start = Date::try_new_hebrew_v2(year - 1, NISAN, anchor_day).ok()?;
            (prev_year_start, year - 1)
        };

        // End date: last Shabbos before Rosh Hashanah of the following year, the day after 29 Elul.
        let rosh_hashana = Date::try_new_hebrew_v2(cycle_year, ELUL, 29).ok()?.add_days(1)?;
        let day_number = day_of_week_number(rosh_hashana);
        // Subtract days to get to the previous Shabbos
        let end_date = rosh_hashana.add_days(-day_number).unwrap_or(rosh_hashana);

        Some((start_date, end_date))
    }
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use icu_calendar::types::Month;

    use crate::calendar::month::TAMMUZ;

    use super::*;

    fn from_hebrew_date(year: i32, month: Month, day: u8) -> Date<Hebrew> {
        Date::try_new_hebrew_v2(year, month, day).expect("hard-coded Hebrew date should be valid")
    }

    // Test cases based on Python test_pirkei_avos_calculator.py

    /// The summer of 9999 has its portions; from 10000 on, which the calendar can't build, there
    /// are none.
    #[test]
    fn test_past_year_9999() {
        let last_summer = from_hebrew_date(9999, TAMMUZ, 2);
        let year_10000 = from_hebrew_date(9999, ELUL, 29).add_days(1).expect("1 Tishrei 10000");
        let far_future = crate::calendar::HebrewDate::from(jiff::civil::date(9999, 12, 31)).icu();
        for region in [Region::Israel, Region::Diaspora] {
            assert!(PirkeiAvos(region).limud(last_summer).is_some(), "{region:?}");
            assert_eq!(PirkeiAvos(region).limud(year_10000), None, "{region:?}");
            assert_eq!(PirkeiAvos(region).limud(far_future), None, "{region:?}");
        }
    }

    #[test]
    fn test_simple_date() {
        // JewishDate(5778, 3, 1) - 1st of Sivan 5778
        let test_date = from_hebrew_date(5778, SIVAN, 1);
        let calculator = PirkeiAvos(Region::Diaspora);
        let limud = calculator.limud(test_date).expect("limud exists");
        // Python test expects description '6'
        assert_eq!(limud, PirkeiAvosUnit::Single(6));
    }

    #[test]
    fn test_near_end_of_cycle() {
        // JewishDate(5778, 6, 20) - 20th of Elul 5778
        let test_date = from_hebrew_date(5778, ELUL, 20);
        let calculator = PirkeiAvos(Region::Diaspora);
        let limud = calculator.limud(test_date).expect("limud exists");
        // Python test expects description '3 - 4'
        assert_eq!(limud, PirkeiAvosUnit::Combined(3, 4));
    }

    #[test]
    fn test_after_cycle_completes() {
        // JewishDate(5778, 6, 29) - 29th of Elul 5778
        let test_date = from_hebrew_date(5778, ELUL, 29);
        let calculator = PirkeiAvos(Region::Diaspora);
        let limud = calculator.limud(test_date);
        assert!(limud.is_none());
    }

    #[test]
    fn test_before_cycle_starts() {
        // JewishDate(5778, 1, 20) - 20th of Nissan 5778
        let test_date = from_hebrew_date(5778, NISAN, 20);
        let calculator = PirkeiAvos(Region::Diaspora);
        let limud = calculator.limud(test_date);
        assert!(limud.is_none());
    }

    #[test]
    fn test_8th_day_pesach_outside_israel() {
        // JewishDate(5778, 1, 22) - 22nd of Nissan 5778
        let test_date = from_hebrew_date(5778, NISAN, 22);
        let calculator = PirkeiAvos(Region::Diaspora);
        let limud = calculator.limud(test_date);
        assert!(limud.is_none());
    }

    #[test]
    fn test_day_after_pesach_outside_israel() {
        // JewishDate(5778, 1, 23) - 23rd of Nissan 5778
        let test_date = from_hebrew_date(5778, NISAN, 23);
        let calculator = PirkeiAvos(Region::Diaspora);
        let limud = calculator.limud(test_date).expect("limud exists");
        // Python test expects description '1'
        assert_eq!(limud, PirkeiAvosUnit::Single(1));
    }

    #[test]
    fn test_compounding_before_cycle_end_outside_israel() {
        // JewishDate(5778, 6, 14) - 14th of Elul 5778
        let test_date = from_hebrew_date(5778, ELUL, 14);
        let calculator = PirkeiAvos(Region::Diaspora);
        let limud = calculator.limud(test_date).expect("limud exists");
        assert_eq!(limud, PirkeiAvosUnit::Combined(1, 2));

        // JewishDate(5778, 6, 15) - 15th of Elul 5778
        let test_date2 = from_hebrew_date(5778, ELUL, 15);
        let limud2 = calculator.limud(test_date2).expect("limud exists");
        assert_eq!(limud2, PirkeiAvosUnit::Combined(3, 4));
    }

    #[test]
    fn test_8th_day_pesach_in_israel() {
        // JewishDate(5778, 1, 22) - 22nd of Nissan 5778 (Shabbos)
        let test_date = from_hebrew_date(5778, NISAN, 22);
        let calculator = PirkeiAvos(Region::Israel);
        let limud = calculator.limud(test_date).expect("limud exists");
        // In Israel, cycle starts on 22nd, and if it's Shabbos, that's the first interval
        assert_eq!(limud, PirkeiAvosUnit::Single(1));
    }

    #[test]
    fn test_day_after_pesach_in_israel() {
        // JewishDate(5778, 1, 23) - 23rd of Nissan 5778
        let test_date = from_hebrew_date(5778, NISAN, 23);
        let calculator = PirkeiAvos(Region::Israel);
        let limud = calculator.limud(test_date).expect("limud exists");
        // Python test expects description '2'
        assert_eq!(limud, PirkeiAvosUnit::Single(2));
    }

    #[test]
    fn test_compounding_before_cycle_end_in_israel() {
        // JewishDate(5778, 6, 21) - 21st of Elul 5778
        let test_date = from_hebrew_date(5778, ELUL, 21);
        let calculator = PirkeiAvos(Region::Israel);
        let limud = calculator.limud(test_date).expect("limud exists");
        assert_eq!(limud, PirkeiAvosUnit::Combined(3, 4));
    }

    #[test]
    fn test_7_sivan_on_shabbos_outside_israel() {
        // 5769 - Sivan 7 falls on Shabbos outside Israel
        // JewishDate(5769, 3, 3) - 3rd of Sivan 5769
        let test_date = from_hebrew_date(5769, SIVAN, 3);
        let calculator = PirkeiAvos(Region::Diaspora);
        let limud = calculator.limud(test_date);
        // This interval should be skipped, returning None for the unit
        // The Python test expects limud to exist but with None unit
        // In our implementation, we just don't return a unit for skipped intervals
        assert!(limud.is_none());
    }

    #[test]
    fn test_iteration_following_7_sivan_on_shabbos_outside_israel() {
        // JewishDate(5769, 3, 8) - 8th of Sivan 5769
        let test_date = from_hebrew_date(5769, SIVAN, 8);
        let calculator = PirkeiAvos(Region::Diaspora);
        let limud = calculator.limud(test_date).expect("limud exists");
        // Python test expects description '1' (starts new sub-cycle)
        assert_eq!(limud, PirkeiAvosUnit::Single(1));
    }

    #[test]
    fn test_7_sivan_on_shabbos_in_israel() {
        // JewishDate(5769, 3, 3) - 3rd of Sivan 5769
        let test_date = from_hebrew_date(5769, SIVAN, 3);
        let calculator = PirkeiAvos(Region::Israel);
        let limud = calculator.limud(test_date).expect("limud exists");
        // In Israel, no skip - Python test expects description '1'
        assert_eq!(limud, PirkeiAvosUnit::Single(1));
    }

    #[test]
    fn test_iteration_following_7_sivan_on_shabbos_in_israel() {
        // JewishDate(5769, 3, 8) - 8th of Sivan 5769
        let test_date = from_hebrew_date(5769, SIVAN, 8);
        let calculator = PirkeiAvos(Region::Israel);
        let limud = calculator.limud(test_date).expect("limud exists");
        // Python test expects description '2'
        assert_eq!(limud, PirkeiAvosUnit::Single(2));
    }
}
