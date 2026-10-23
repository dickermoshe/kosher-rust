use icu_calendar::{Date, cal::Hebrew};

use crate::limudim::{
    engine::{Cycle, CycleFinder, HebrewDateExt, InternalLimud, Interval},
    units::{BavliTractate, Daf},
};

/// Dafim in Daf Yomi Bavli cycles 1-7, before Shekalim was expanded.
const BAVLI_DAF_COUNT_EARLY: i32 = 2702;

/// Dafim in Daf Yomi Bavli cycles 8 onward, with the Vilna Shekalim.
pub(super) const BAVLI_DAF_COUNT_MODERN: i32 = 2711;

/// The cycle in which Shekalim grew from 13 to 22 dafim (Vilna pagination).
const SHEKALIM_EXPANSION_CYCLE: i32 = 8;

fn initial_cycle_date() -> Date<Hebrew> {
    #[allow(clippy::expect_used)]
    let date = Date::try_new_gregorian(1923, 9, 11)
        .expect("hard-coded Gregorian date should be valid")
        .to_calendar(Hebrew);
    date
}

pub(crate) const fn start_daf(tractate: BavliTractate) -> u16 {
    match tractate {
        BavliTractate::Kinnim => 23,
        BavliTractate::Tamid => 26,
        BavliTractate::Midos => 34,
        _ => 2,
    }
}

/// Last daf number (Berachos ends at 64)
pub(crate) const fn end_daf(tractate: BavliTractate, iteration: i32) -> u16 {
    match tractate {
        BavliTractate::Berachos => 64,
        BavliTractate::Shabbos => 157,
        BavliTractate::Eruvin => 105,
        BavliTractate::Pesachim => 121,
        BavliTractate::Shekalim => {
            if iteration < SHEKALIM_EXPANSION_CYCLE {
                13
            } else {
                22
            }
        }
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

/// Daf Yomi, one page of the Bavli a day, from 11 September 1923.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct DafYomiBavli;

fn daf_yomi_bavli_for_date(limud_date: Date<Hebrew>) -> Option<Daf<BavliTractate>> {
    let cycle = Cycle::from_cycle_initiation(initial_cycle_date(), DafYomiBavli::cycle_end_calculation, limud_date)?;
    if limud_date > cycle.end_date {
        return None;
    }
    let offset = cycle.start_date.days_until(&limud_date)? as usize;
    daf_at_offset(cycle.iteration?, offset)
}

impl InternalLimud<Daf<BavliTractate>> for DafYomiBavli {
    fn limud(&self, limud_date: Date<Hebrew>) -> Option<Daf<BavliTractate>> {
        daf_yomi_bavli_for_date(limud_date)
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

    fn unit_for_interval(&self, _interval: &Interval, limud_date: &Date<Hebrew>) -> Option<Daf<BavliTractate>> {
        daf_yomi_bavli_for_date(*limud_date)
    }
}

super::schedule!(DafYomiBavli => Daf<BavliTractate>);

fn daf_at_offset(cycle_iteration: i32, offset: usize) -> Option<Daf<BavliTractate>> {
    let mut remaining = offset;
    for tractate in BavliTractate::ALL {
        let start = start_daf(tractate);
        let end = end_daf(tractate, cycle_iteration);
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
    fn daf_yomi_bavli_simple_date() {
        let test_date = from_gregorian_date(2017, 12, 28);
        let limud = DafYomiBavli.limud(test_date).expect("limud exists");
        assert_eq!(limud.daf, 30);
        assert_eq!(limud.tractate, BavliTractate::Shevuos);
    }

    #[test]
    fn daf_yomi_bavli_before_cycle_began() {
        let test_date = from_gregorian_date(1920, 1, 1);
        let limud = DafYomiBavli.limud(test_date);
        assert!(limud.is_none());
    }

    #[test]
    fn daf_yomi_bavli_first_day_of_cycle() {
        let test_date = from_gregorian_date(2012, 8, 3);
        let limud = DafYomiBavli.limud(test_date).expect("limud exists");
        assert_eq!(limud.daf, 2);
        assert_eq!(limud.tractate, BavliTractate::Berachos);
    }

    #[test]
    fn daf_yomi_bavli_last_day_of_cycle() {
        let test_date = from_gregorian_date(2020, 1, 4);
        let limud = DafYomiBavli.limud(test_date).expect("limud exists");
        assert_eq!(limud.daf, 73);
        assert_eq!(limud.tractate, BavliTractate::Niddah);
    }

    #[test]
    fn daf_yomi_bavli_before_shekalim_transition_end_of_shekalim() {
        let test_date = from_gregorian_date(1969, 4, 28);
        let limud = DafYomiBavli.limud(test_date).expect("limud exists");
        assert_eq!(limud.daf, 13);
        assert_eq!(limud.tractate, BavliTractate::Shekalim);
    }

    #[test]
    fn daf_yomi_bavli_before_shekalim_transition_beginning_of_yoma() {
        let test_date = from_gregorian_date(1969, 4, 29);
        let limud = DafYomiBavli.limud(test_date).expect("limud exists");
        assert_eq!(limud.daf, 2);
        assert_eq!(limud.tractate, BavliTractate::Yoma);
    }

    #[test]
    fn daf_yomi_bavli_end_of_meilah() {
        let test_date = from_gregorian_date(2019, 10, 9);
        let limud = DafYomiBavli.limud(test_date).expect("limud exists");
        assert_eq!(limud.daf, 22);
        assert_eq!(limud.tractate, BavliTractate::Meilah);
    }

    #[test]
    fn daf_yomi_bavli_beginning_of_kinnim() {
        let test_date = from_gregorian_date(2019, 10, 10);
        let limud = DafYomiBavli.limud(test_date).expect("limud exists");
        assert_eq!(limud.daf, 23);
        assert_eq!(limud.tractate, BavliTractate::Kinnim);
    }

    #[test]
    fn daf_yomi_bavli_beginning_of_tamid() {
        let test_date = from_gregorian_date(2019, 10, 13);
        let limud = DafYomiBavli.limud(test_date).expect("limud exists");
        assert_eq!(limud.daf, 26);
        assert_eq!(limud.tractate, BavliTractate::Tamid);
    }

    #[test]
    fn daf_yomi_bavli_second_day_of_midos() {
        // Note: Midos starts at page 34 on 2019-10-21. This tests the second day.
        let test_date = from_gregorian_date(2019, 10, 22);
        let limud = DafYomiBavli.limud(test_date).expect("limud exists");
        assert_eq!(limud.daf, 35);
        assert_eq!(limud.tractate, BavliTractate::Midos);
    }

    #[test]
    fn daf_yomi_bavli_after_midos() {
        let test_date = from_gregorian_date(2019, 10, 25);
        let limud = DafYomiBavli.limud(test_date).expect("limud exists");
        assert_eq!(limud.daf, 2);
        assert_eq!(limud.tractate, BavliTractate::Niddah);
    }
}
