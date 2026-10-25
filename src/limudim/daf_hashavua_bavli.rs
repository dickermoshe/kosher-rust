use icu_calendar::{Date, cal::Hebrew, types::Weekday};

use crate::limudim::{
    engine::{Cycle, CycleFinder, HebrewDateExt, InternalLimud, Interval},
    units::{BavliTractate, Daf},
};

use super::daf_yomi_bavli::BAVLI_DAF_COUNT_MODERN;

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

const fn start_daf(tractate: BavliTractate) -> u16 {
    match tractate {
        BavliTractate::Kinnim => 23,
        BavliTractate::Tamid => 26,
        BavliTractate::Midos => 34,
        _ => 2,
    }
}

const fn end_daf(tractate: BavliTractate) -> u16 {
    match tractate {
        BavliTractate::Berachos => 64,
        BavliTractate::Shabbos => 157,
        BavliTractate::Eruvin => 105,
        BavliTractate::Pesachim => 121,
        BavliTractate::Shekalim => 22,
        BavliTractate::Yoma => 88,
        BavliTractate::Sukkah => 56,
        BavliTractate::Beitzah => 40,
        BavliTractate::RoshHashanah => 35,
        BavliTractate::Taanis => 31,
        BavliTractate::Megillah => 32,
        BavliTractate::MoedKatan => 29,
        BavliTractate::Chagigah => 27,
        BavliTractate::Yevamos => 122,
        BavliTractate::Kesubos => 112,
        BavliTractate::Nedarim => 91,
        BavliTractate::Nazir => 66,
        BavliTractate::Sotah => 49,
        BavliTractate::Gitin => 90,
        BavliTractate::Kiddushin => 82,
        BavliTractate::BavaKamma => 119,
        BavliTractate::BavaMetzia => 119,
        BavliTractate::BavaBasra => 176,
        BavliTractate::Sanhedrin => 113,
        BavliTractate::Makkos => 24,
        BavliTractate::Shevuos => 49,
        BavliTractate::AvodahZarah => 76,
        BavliTractate::Horiyos => 14,
        BavliTractate::Zevachim => 120,
        BavliTractate::Menachos => 110,
        BavliTractate::Chullin => 142,
        BavliTractate::Bechoros => 61,
        BavliTractate::Arachin => 34,
        BavliTractate::Temurah => 34,
        BavliTractate::Kerisos => 28,
        BavliTractate::Meilah => 22,
        BavliTractate::Kinnim => 25,
        BavliTractate::Tamid => 33,
        BavliTractate::Midos => 37,
        BavliTractate::Niddah => 73,
    }
}

/// Daf HaShavua, one page of the Bavli a week, Sunday to Shabbos, from 6 March 2005.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct DafHashavuaBavli;

impl InternalLimud<Daf<BavliTractate>> for DafHashavuaBavli {
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

    fn unit_for_interval(&self, interval: &Interval, _limud_date: &Date<Hebrew>) -> Option<Daf<BavliTractate>> {
        interval.cycle.iteration?;
        daf_at_offset((interval.iteration - 1) as usize)
    }
}

super::schedule!(DafHashavuaBavli => Daf<BavliTractate>);

fn daf_at_offset(offset: usize) -> Option<Daf<BavliTractate>> {
    let mut remaining = offset;
    for tractate in BavliTractate::ALL {
        let start = start_daf(tractate);
        let end = end_daf(tractate);
        let count = usize::from(end - start + 1);
        if remaining < count {
            return Some(Daf {
                tractate,
                daf: start + remaining as u16,
            });
        }
        remaining -= count;
    }
    None
}
#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use crate::limudim::engine::from_gregorian_date;

    use super::*;

    #[test]
    fn daf_hashavua_bavli_simple_date() {
        let test_date = from_gregorian_date(2018, 10, 10);
        let limud = DafHashavuaBavli.limud(test_date).expect("limud exists");
        assert_eq!(limud.tractate, BavliTractate::Megillah);
        assert_eq!(limud.daf, 2);
    }

    #[test]
    fn daf_hashavua_bavli_before_cycle_began() {
        let test_date = from_gregorian_date(2005, 3, 5);
        let limud = DafHashavuaBavli.limud(test_date);
        assert!(limud.is_none());
    }

    #[test]
    fn daf_hashavua_bavli_first_day_of_cycle() {
        let test_date = from_gregorian_date(2057, 2, 18);
        let limud = DafHashavuaBavli.limud(test_date).expect("limud exists");
        assert_eq!(limud.daf, 2);
        assert_eq!(limud.tractate, BavliTractate::Berachos);
    }

    #[test]
    fn daf_hashavua_bavli_last_day_of_cycle() {
        let test_date = from_gregorian_date(2057, 2, 17);
        let limud = DafHashavuaBavli.limud(test_date).expect("limud exists");
        assert_eq!(limud.daf, 73);
        assert_eq!(limud.tractate, BavliTractate::Niddah);
    }
}
