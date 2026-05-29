use icu_calendar::{Date, cal::Hebrew, types::Weekday};

use crate::limudim::{
    BAVLI_DAF_COUNT_MODERN, HebrewDateExt, LimudCalculator,
    cycle::Cycle,
    interval::Interval,
    limud_calculator::{CycleFinder, InternalLimudCalculator},
    units::{BAVLI_TRACTATES, Daf, Tractate},
};

fn initial_cycle_date() -> Date<Hebrew> {
    #[allow(clippy::expect_used)]
    let date = Date::try_new_gregorian(2005, 3, 6)
        .expect("hard-coded Gregorian date should be valid")
        .to_calendar(Hebrew);
    date
}

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

const fn start_daf(tractate: Tractate) -> u16 {
    match tractate {
        Tractate::Kinnim => 23,
        Tractate::Tamid => 26,
        Tractate::Midos => 34,
        _ => 2,
    }
}

const fn end_daf(tractate: Tractate) -> Option<u16> {
    let daf = match tractate {
        Tractate::Berachos => 64,
        Tractate::Shabbos => 157,
        Tractate::Eruvin => 105,
        Tractate::Pesachim => 121,
        Tractate::Shekalim => 22,
        Tractate::Yoma => 88,
        Tractate::Sukkah => 56,
        Tractate::Beitzah => 40,
        Tractate::RoshHashanah => 35,
        Tractate::Taanis => 31,
        Tractate::Megillah => 32,
        Tractate::MoedKatan => 29,
        Tractate::Chagigah => 27,
        Tractate::Yevamos => 122,
        Tractate::Kesubos => 112,
        Tractate::Nedarim => 91,
        Tractate::Nazir => 66,
        Tractate::Sotah => 49,
        Tractate::Gitin => 90,
        Tractate::Kiddushin => 82,
        Tractate::BavaKamma => 119,
        Tractate::BavaMetzia => 119,
        Tractate::BavaBasra => 176,
        Tractate::Sanhedrin => 113,
        Tractate::Makkos => 24,
        Tractate::Shevuos => 49,
        Tractate::AvodahZarah => 76,
        Tractate::Horiyos => 14,
        Tractate::Zevachim => 120,
        Tractate::Menachos => 110,
        Tractate::Chullin => 142,
        Tractate::Bechoros => 61,
        Tractate::Arachin => 34,
        Tractate::Temurah => 34,
        Tractate::Kerisos => 28,
        Tractate::Meilah => 22,
        Tractate::Kinnim => 25,
        Tractate::Tamid => 33,
        Tractate::Midos => 37,
        Tractate::Niddah => 73,
        _ => 0,
    };
    if daf == 0 { None } else { Some(daf) }
}

#[derive(Default)]
/// Calculates the Daf Hashavua Bavli schedule.
pub struct DafHashavuaBavli {}

impl InternalLimudCalculator<Daf> for DafHashavuaBavli {
    fn interval_end_calculation(_cycle: Cycle, hebrew_date: Date<Hebrew>) -> Option<Date<Hebrew>> {
        let day_number = day_of_week_number(hebrew_date);
        hebrew_date.add_days(7 - day_number)
    }
    fn cycle_finder(&self) -> CycleFinder {
        CycleFinder::Initial(initial_cycle_date())
    }
    fn cycle_end_calculation(hebrew_date: Date<Hebrew>, _iteration: Option<i32>) -> Option<Date<Hebrew>> {
        let day_number = day_of_week_number(hebrew_date);
        hebrew_date.add_days((BAVLI_DAF_COUNT_MODERN * 7) - day_number)
    }

    fn unit_for_interval(&self, interval: &Interval, _limud_date: &Date<Hebrew>) -> Option<Daf> {
        interval.cycle.iteration?;
        daf_at_offset((interval.iteration - 1) as usize)
    }
}

impl LimudCalculator<Daf> for DafHashavuaBavli {}

fn daf_at_offset(offset: usize) -> Option<Daf> {
    let mut remaining = offset;
    for tractate in BAVLI_TRACTATES {
        let start = start_daf(tractate);
        let end = end_daf(tractate)?;
        let count = usize::from(end - start + 1);
        if remaining < count {
            return Some(Daf {
                tractate,
                page: start + remaining as u16,
            });
        }
        remaining -= count;
    }
    None
}
#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use crate::limudim::from_gregorian_date;

    use super::*;

    #[test]
    fn daf_hashavua_bavli_simple_date() {
        let test_date = from_gregorian_date(2018, 10, 10);
        let limud = DafHashavuaBavli::default().limud(test_date).expect("limud exists");
        assert_eq!(limud.tractate, Tractate::Megillah);
        assert_eq!(limud.page, 2);
    }

    #[test]
    fn daf_hashavua_bavli_before_cycle_began() {
        let test_date = from_gregorian_date(2005, 3, 5);
        let limud = DafHashavuaBavli::default().limud(test_date);
        assert!(limud.is_none());
    }

    #[test]
    fn daf_hashavua_bavli_first_day_of_cycle() {
        let test_date = from_gregorian_date(2057, 2, 18);
        let limud = DafHashavuaBavli::default().limud(test_date).expect("limud exists");
        assert_eq!(limud.page, 2);
        assert_eq!(limud.tractate, Tractate::Berachos);
    }

    #[test]
    fn daf_hashavua_bavli_last_day_of_cycle() {
        let test_date = from_gregorian_date(2057, 2, 17);
        let limud = DafHashavuaBavli::default().limud(test_date).expect("limud exists");
        assert_eq!(limud.page, 73);
        assert_eq!(limud.tractate, Tractate::Niddah);
    }
}
