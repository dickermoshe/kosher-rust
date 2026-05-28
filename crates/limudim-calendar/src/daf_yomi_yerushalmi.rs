use hebrew_holiday_calendar::{HebrewHolidayCalendar, HebrewMonth, Holiday};

use crate::{
    constants::YERUSHALMI_DAF_COUNT,
    date::{days_between, from_gregorian_date, from_hebrew_date, DateExt, HebrewDate},
    interval::Interval,
    limud_calculator::{CycleFinder, InternalLimudCalculator},
    units::{Daf, YERUSHALMI_TRACTATES},
    LimudCalculator,
};

const YERUSHALMI_CYCLE_START: (i32, u8, u8) = (1980, 2, 2);

/// Pages per masechta in the Vilna Yerushalmi, matching KosherJava's `BLATT_PER_MASECHTA`.
const YERUSHALMI_BLATT_PER_MASECHTA: [u16; 39] = [
    68, 37, 34, 44, 31, 59, 26, 33, 28, 20, 13, 92, 65, 71, 22, 22, 42, 26, 26, 33, 34, 22, 19, 85, 72, 47, 40, 47, 54,
    48, 44, 37, 34, 44, 9, 57, 37, 19, 13,
];

#[derive(Default)]
/// Calculates the Daf Yomi Yerushalmi schedule using the Vilna Edition of the Jerusalem Talmud.
pub struct DafYomiYerushalmiVilna {}

impl DafYomiYerushalmiVilna {
    fn cycle_start_date() -> HebrewDate {
        from_gregorian_date(
            YERUSHALMI_CYCLE_START.0,
            YERUSHALMI_CYCLE_START.1,
            YERUSHALMI_CYCLE_START.2,
        )
    }
}

impl InternalLimudCalculator<Daf> for DafYomiYerushalmiVilna {
    fn limud(&self, limud_date: HebrewDate) -> Option<Daf> {
        let cycle_start = Self::cycle_start_date();
        if limud_date < cycle_start {
            return None;
        }
        if is_skip_day(&limud_date) {
            return None;
        }

        let mut prev_cycle = cycle_start;
        let mut next_cycle = next_cycle_start(prev_cycle)?;
        while limud_date >= next_cycle {
            prev_cycle = next_cycle;
            next_cycle = next_cycle_start(prev_cycle)?;
        }

        let daf_no = days_between(prev_cycle, limud_date)?;
        let special_days = count_special_days(prev_cycle, limud_date)?;
        let mut total = daf_no - special_days;
        if total < 0 {
            return None;
        }

        for (&tractate, &blatt) in YERUSHALMI_TRACTATES.iter().zip(YERUSHALMI_BLATT_PER_MASECHTA.iter()) {
            let blatt = i32::from(blatt);
            if total < blatt {
                return Some(Daf {
                    tractate,
                    page: (total + 1) as u16,
                });
            }
            total -= blatt;
        }

        None
    }

    fn cycle_finder(&self) -> CycleFinder {
        CycleFinder::Initial(Self::cycle_start_date())
    }

    fn cycle_end_calculation(hebrew_date: HebrewDate, _iteration: Option<i32>) -> Option<HebrewDate> {
        cycle_end_date(hebrew_date)
    }

    fn unit_for_interval(&self, interval: &Interval, limud_date: &HebrewDate) -> Option<Daf> {
        let _ = interval;
        self.limud(*limud_date)
    }

    fn is_skip_interval(&self, interval: &Interval) -> bool {
        is_skip_day(&interval.start_date)
    }
}

impl LimudCalculator<Daf> for DafYomiYerushalmiVilna {}

fn cycle_end_date(cycle_start: HebrewDate) -> Option<HebrewDate> {
    let mut end_date = cycle_start.add_days(YERUSHALMI_DAF_COUNT - 1)?;
    let mut found_days = count_special_days(cycle_start, end_date)?;
    while found_days > 0 {
        let new_start_date = end_date.add_days(1)?;
        end_date = end_date.add_days(found_days)?;
        found_days = count_special_days(new_start_date, end_date)?;
    }
    Some(end_date)
}

fn next_cycle_start(cycle_start: HebrewDate) -> Option<HebrewDate> {
    cycle_end_date(cycle_start)?.add_days(1)
}

fn is_between(start: HebrewDate, date: HebrewDate, end: HebrewDate) -> bool {
    start < date && date <= end
}

fn tisha_bav_date(year: i32) -> Option<HebrewDate> {
    let date = from_hebrew_date(year, HebrewMonth::Av, 9);
    if date.day_of_week_number() == 7 {
        date.add_days(1)
    } else {
        Some(date)
    }
}

fn count_special_days(start: HebrewDate, end: HebrewDate) -> Option<i32> {
    let start_year = start.year().extended_year();
    let end_year = end.year().extended_year();
    let mut special_days = 0;

    for year in start_year..=end_year {
        let yom_kippur = from_hebrew_date(year, HebrewMonth::Tishrei, 10);
        if is_between(start, yom_kippur, end) {
            special_days += 1;
        }

        let tisha_bav = tisha_bav_date(year);
        if is_between(start, tisha_bav?, end) {
            special_days += 1;
        }
    }

    Some(special_days)
}

fn is_skip_day(date: &HebrewDate) -> bool {
    date.holidays(false, false)
        .any(|h| h == &Holiday::TishahBav || h == &Holiday::YomKippur)
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use icu_calendar::{cal::Hebrew, Date};

    use crate::date::from_gregorian_date;

    use super::*;
    use crate::units::Tractate;

    #[test]
    fn daf_yomi_yerushalmi_simple_date() {
        let test_date = from_gregorian_date(2017, 12, 28);
        let limud = DafYomiYerushalmiVilna::default()
            .limud(test_date)
            .expect("limud exists");
        assert_eq!(limud.page, 33);
        assert_eq!(limud.tractate, Tractate::BavaMetzia);
    }

    #[test]
    fn daf_yomi_yerushalmi_before_cycle_began() {
        let test_date = from_gregorian_date(1980, 1, 1);
        let limud = DafYomiYerushalmiVilna::default().limud(test_date);
        assert!(limud.is_none());
    }

    #[test]
    fn daf_yomi_yerushalmi_first_day_of_cycle() {
        let test_date = from_gregorian_date(2005, 10, 3);
        let limud = DafYomiYerushalmiVilna::default()
            .limud(test_date)
            .expect("limud exists");
        assert_eq!(limud.page, 1);
        assert_eq!(limud.tractate, Tractate::Berachos);
    }

    #[test]
    fn daf_yomi_yerushalmi_last_day_of_cycle() {
        let test_date = from_gregorian_date(2010, 1, 12);
        let limud = DafYomiYerushalmiVilna::default()
            .limud(test_date)
            .expect("limud exists");
        assert_eq!(limud.page, 13);
        assert_eq!(limud.tractate, Tractate::Niddah);
    }

    #[test]
    fn daf_yomi_yerushalmi_last_skip_day() {
        // JewishDate(5778, 7, 10) is Tishrei 10 (Yom Kippur) - a skip day
        let test_date = Date::<Hebrew>::from_hebrew_date(5778, hebrew_holiday_calendar::HebrewMonth::Tishrei, 10)
            .expect("valid hebrew date");
        let limud = DafYomiYerushalmiVilna::default().limud(test_date);

        assert!(limud.is_none());
    }

    #[test]
    fn daf_yomi_yerushalmi_1980_02_02() {
        let test_date = from_gregorian_date(1980, 2, 2);
        let limud = DafYomiYerushalmiVilna::default()
            .limud(test_date)
            .expect("limud exists");
        assert_eq!(limud.page, 1);
        assert_eq!(limud.tractate, Tractate::Berachos);
    }

    #[test]
    fn daf_yomi_yerushalmi_1982_05_15() {
        let test_date = from_gregorian_date(1982, 5, 15);
        let limud = DafYomiYerushalmiVilna::default()
            .limud(test_date)
            .expect("limud exists");
        assert_eq!(limud.page, 4);
        assert_eq!(limud.tractate, Tractate::Chagigah);
    }

    #[test]
    fn daf_yomi_yerushalmi_1984_05_12() {
        let test_date = from_gregorian_date(1984, 5, 12);
        let limud = DafYomiYerushalmiVilna::default()
            .limud(test_date)
            .expect("limud exists");
        assert_eq!(limud.page, 13);
        assert_eq!(limud.tractate, Tractate::Niddah);
    }

    #[test]
    fn daf_yomi_yerushalmi_1984_05_13() {
        let test_date = from_gregorian_date(1984, 5, 13);
        let limud = DafYomiYerushalmiVilna::default()
            .limud(test_date)
            .expect("limud exists");
        assert_eq!(limud.page, 1);
        assert_eq!(limud.tractate, Tractate::Berachos);
    }

    #[test]
    fn daf_yomi_yerushalmi_1990_08_01() {
        let test_date = from_gregorian_date(1990, 8, 1);
        let limud = DafYomiYerushalmiVilna::default()
            .limud(test_date)
            .expect("limud exists");
        assert_eq!(limud.page, 40);
        assert_eq!(limud.tractate, Tractate::Yoma);
    }

    #[test]
    fn daf_yomi_yerushalmi_2000_01_01() {
        let test_date = from_gregorian_date(2000, 1, 1);
        let limud = DafYomiYerushalmiVilna::default()
            .limud(test_date)
            .expect("limud exists");
        assert_eq!(limud.page, 66);
        assert_eq!(limud.tractate, Tractate::Kesubos);
    }

    #[test]
    fn daf_yomi_yerushalmi_2005_10_02() {
        let test_date = from_gregorian_date(2005, 10, 2);
        let limud = DafYomiYerushalmiVilna::default()
            .limud(test_date)
            .expect("limud exists");
        assert_eq!(limud.page, 13);
        assert_eq!(limud.tractate, Tractate::Niddah);
    }

    #[test]
    fn daf_yomi_yerushalmi_2007_06_15() {
        let test_date = from_gregorian_date(2007, 6, 15);
        let limud = DafYomiYerushalmiVilna::default()
            .limud(test_date)
            .expect("limud exists");
        assert_eq!(limud.page, 68);
        assert_eq!(limud.tractate, Tractate::Pesachim);
    }

    #[test]
    fn daf_yomi_yerushalmi_2010_01_11() {
        let test_date = from_gregorian_date(2010, 1, 11);
        let limud = DafYomiYerushalmiVilna::default()
            .limud(test_date)
            .expect("limud exists");
        assert_eq!(limud.page, 12);
        assert_eq!(limud.tractate, Tractate::Niddah);
    }

    #[test]
    fn daf_yomi_yerushalmi_2015_04_23() {
        let test_date = from_gregorian_date(2015, 4, 23);
        let limud = DafYomiYerushalmiVilna::default()
            .limud(test_date)
            .expect("limud exists");
        assert_eq!(limud.page, 3);
        assert_eq!(limud.tractate, Tractate::Orlah);
    }

    #[test]
    fn daf_yomi_yerushalmi_2020_01_01() {
        let test_date = from_gregorian_date(2020, 1, 1);
        let limud = DafYomiYerushalmiVilna::default()
            .limud(test_date)
            .expect("limud exists");
        assert_eq!(limud.page, 28);
        assert_eq!(limud.tractate, Tractate::Eruvin);
    }

    #[test]
    fn daf_yomi_yerushalmi_2025_10_02() {
        let test_date = from_gregorian_date(2025, 10, 2);
        let limud = DafYomiYerushalmiVilna::default().limud(test_date);
        assert!(limud.is_none());
    }

    #[test]
    fn daf_yomi_yerushalmi_1980_09_20() {
        let test_date = from_gregorian_date(1980, 9, 20);
        let limud = DafYomiYerushalmiVilna::default().limud(test_date);
        assert!(limud.is_none());
    }

    #[test]
    fn daf_yomi_yerushalmi_1990_09_29() {
        let test_date = from_gregorian_date(1990, 9, 29);
        let limud = DafYomiYerushalmiVilna::default().limud(test_date);
        assert!(limud.is_none());
    }

    #[test]
    fn daf_yomi_yerushalmi_2000_10_09() {
        let test_date = from_gregorian_date(2000, 10, 9);
        let limud = DafYomiYerushalmiVilna::default().limud(test_date);
        assert!(limud.is_none());
    }

    #[test]
    fn daf_yomi_yerushalmi_2010_09_18() {
        let test_date = from_gregorian_date(2010, 9, 18);
        let limud = DafYomiYerushalmiVilna::default().limud(test_date);
        assert!(limud.is_none());
    }

    #[test]
    fn daf_yomi_yerushalmi_1980_07_21() {
        let test_date = from_gregorian_date(1980, 7, 21);
        let limud = DafYomiYerushalmiVilna::default()
            .limud(test_date)
            .expect("limud exists");
        assert_eq!(limud.page, 32);
        assert_eq!(limud.tractate, Tractate::Kilayim);
    }

    #[test]
    fn daf_yomi_yerushalmi_1990_07_31() {
        let test_date = from_gregorian_date(1990, 7, 31);
        let limud = DafYomiYerushalmiVilna::default().limud(test_date);
        assert!(limud.is_none());
    }

    #[test]
    fn daf_yomi_yerushalmi_2000_08_10() {
        let test_date = from_gregorian_date(2000, 8, 10);
        let limud = DafYomiYerushalmiVilna::default().limud(test_date);
        assert!(limud.is_none());
    }

    #[test]
    fn daf_yomi_yerushalmi_2010_07_20() {
        let test_date = from_gregorian_date(2010, 7, 20);
        let limud = DafYomiYerushalmiVilna::default().limud(test_date);
        assert!(limud.is_none());
    }

    #[test]
    fn daf_yomi_yerushalmi_1980_03_01() {
        let test_date = from_gregorian_date(1980, 3, 1);
        let limud = DafYomiYerushalmiVilna::default()
            .limud(test_date)
            .expect("limud exists");
        assert_eq!(limud.page, 29);
        assert_eq!(limud.tractate, Tractate::Berachos);
    }

    #[test]
    fn daf_yomi_yerushalmi_1982_01_01() {
        let test_date = from_gregorian_date(1982, 1, 1);
        let limud = DafYomiYerushalmiVilna::default()
            .limud(test_date)
            .expect("limud exists");
        assert_eq!(limud.page, 31);
        assert_eq!(limud.tractate, Tractate::Yoma);
    }

    #[test]
    fn daf_yomi_yerushalmi_1984_04_01() {
        let test_date = from_gregorian_date(1984, 4, 1);
        let limud = DafYomiYerushalmiVilna::default()
            .limud(test_date)
            .expect("limud exists");
        assert_eq!(limud.page, 28);
        assert_eq!(limud.tractate, Tractate::AvodahZarah);
    }
}
