use icu_calendar::{Date, cal::Hebrew};

use crate::limudim::{
    BAVLI_DAF_COUNT_EARLY, BAVLI_DAF_COUNT_MODERN, HebrewDateExt, Limud, SHEKALIM_EXPANSION_CYCLE,
    cycle::Cycle,
    interval::Interval,
    limud::{CycleFinder, InternalLimud},
    units::{BAVLI_TRACTATES, Daf, Tractate},
};

fn initial_cycle_date() -> Date<Hebrew> {
    #[allow(clippy::expect_used)]
    let date = Date::try_new_gregorian(1923, 9, 11)
        .expect("hard-coded Gregorian date should be valid")
        .to_calendar(Hebrew);
    date
}

pub const fn start_daf(tractate: Tractate) -> u16 {
    match tractate {
        Tractate::Kinnim => 23,
        Tractate::Tamid => 26,
        Tractate::Midos => 34,
        _ => 2,
    }
}

/// Last daf number (Berachos ends at 64)
pub const fn end_daf(tractate: Tractate, iteration: i32) -> Option<u16> {
    let daf = match tractate {
        Tractate::Berachos => 64,
        Tractate::Shabbos => 157,
        Tractate::Eruvin => 105,
        Tractate::Pesachim => 121,
        Tractate::Shekalim => {
            if iteration < SHEKALIM_EXPANSION_CYCLE {
                13
            } else {
                22
            }
        }
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
/// Calculates the Daf Yomi Bavli schedule.
pub struct DafYomiBavli {}

impl InternalLimud<Daf> for DafYomiBavli {
    fn limud(&self, limud_date: Date<Hebrew>) -> Option<Daf> {
        let cycle = Cycle::from_cycle_initiation(initial_cycle_date(), Self::cycle_end_calculation, limud_date)?;
        if limud_date > cycle.end_date {
            return None;
        }
        let offset = cycle.start_date.days_until(&limud_date)? as usize;
        daf_at_offset(cycle.iteration?, offset as usize)
    }

    fn cycle_finder(&self) -> CycleFinder {
        CycleFinder::Initial(initial_cycle_date())
    }
    fn cycle_end_calculation(hebrew_date: Date<Hebrew>, _iteration: Option<i32>) -> Option<Date<Hebrew>> {
        let days = _iteration
            .map(|i| {
                if i < SHEKALIM_EXPANSION_CYCLE {
                    BAVLI_DAF_COUNT_EARLY
                } else {
                    BAVLI_DAF_COUNT_MODERN
                }
            })
            .unwrap_or(BAVLI_DAF_COUNT_EARLY);
        hebrew_date.add_days(days - 1)
    }

    fn unit_for_interval(&self, _interval: &Interval, limud_date: &Date<Hebrew>) -> Option<Daf> {
        self.limud(*limud_date)
    }
}

impl Limud<Daf> for DafYomiBavli {}

fn daf_at_offset(cycle_iteration: i32, offset: usize) -> Option<Daf> {
    let mut remaining = offset;
    for tractate in BAVLI_TRACTATES {
        let start = start_daf(tractate);
        let end = end_daf(tractate, cycle_iteration)?;
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
    fn daf_yomi_bavli_simple_date() {
        let test_date = from_gregorian_date(2017, 12, 28);
        let limud = DafYomiBavli::default().limud(test_date).expect("limud exists");
        assert_eq!(limud.page, 30);
        assert_eq!(limud.tractate, Tractate::Shevuos);
    }

    #[test]
    fn daf_yomi_bavli_before_cycle_began() {
        let test_date = from_gregorian_date(1920, 1, 1);
        let limud = DafYomiBavli::default().limud(test_date);
        assert!(limud.is_none());
    }

    #[test]
    fn daf_yomi_bavli_first_day_of_cycle() {
        let test_date = from_gregorian_date(2012, 8, 3);
        let limud = DafYomiBavli::default().limud(test_date).expect("limud exists");
        assert_eq!(limud.page, 2);
        assert_eq!(limud.tractate, Tractate::Berachos);
    }

    #[test]
    fn daf_yomi_bavli_last_day_of_cycle() {
        let test_date = from_gregorian_date(2020, 1, 4);
        let limud = DafYomiBavli::default().limud(test_date).expect("limud exists");
        assert_eq!(limud.page, 73);
        assert_eq!(limud.tractate, Tractate::Niddah);
    }

    #[test]
    fn daf_yomi_bavli_before_shekalim_transition_end_of_shekalim() {
        let test_date = from_gregorian_date(1969, 4, 28);
        let limud = DafYomiBavli::default().limud(test_date).expect("limud exists");
        assert_eq!(limud.page, 13);
        assert_eq!(limud.tractate, Tractate::Shekalim);
    }

    #[test]
    fn daf_yomi_bavli_before_shekalim_transition_beginning_of_yoma() {
        let test_date = from_gregorian_date(1969, 4, 29);
        let limud = DafYomiBavli::default().limud(test_date).expect("limud exists");
        assert_eq!(limud.page, 2);
        assert_eq!(limud.tractate, Tractate::Yoma);
    }

    #[test]
    fn daf_yomi_bavli_end_of_meilah() {
        let test_date = from_gregorian_date(2019, 10, 9);
        let limud = DafYomiBavli::default().limud(test_date).expect("limud exists");
        assert_eq!(limud.page, 22);
        assert_eq!(limud.tractate, Tractate::Meilah);
    }

    #[test]
    fn daf_yomi_bavli_beginning_of_kinnim() {
        let test_date = from_gregorian_date(2019, 10, 10);
        let limud = DafYomiBavli::default().limud(test_date).expect("limud exists");
        assert_eq!(limud.page, 23);
        assert_eq!(limud.tractate, Tractate::Kinnim);
    }

    #[test]
    fn daf_yomi_bavli_beginning_of_tamid() {
        let test_date = from_gregorian_date(2019, 10, 13);
        let limud = DafYomiBavli::default().limud(test_date).expect("limud exists");
        assert_eq!(limud.page, 26);
        assert_eq!(limud.tractate, Tractate::Tamid);
    }

    #[test]
    fn daf_yomi_bavli_second_day_of_midos() {
        // Note: Midos starts at page 34 on 2019-10-21. This tests the second day.
        let test_date = from_gregorian_date(2019, 10, 22);
        let limud = DafYomiBavli::default().limud(test_date).expect("limud exists");
        assert_eq!(limud.page, 35);
        assert_eq!(limud.tractate, Tractate::Midos);
    }

    #[test]
    fn daf_yomi_bavli_after_midos() {
        let test_date = from_gregorian_date(2019, 10, 25);
        let limud = DafYomiBavli::default().limud(test_date).expect("limud exists");
        assert_eq!(limud.page, 2);
        assert_eq!(limud.tractate, Tractate::Niddah);
    }
}
