use icu_calendar::{Date, cal::Hebrew};

use crate::limudim::{
    engine::{Cycle, CycleFinder, HebrewDateExt, InternalLimud, Interval},
    units::*,
};

/// Amudim in the Bavli as Dirshu divides it, which is the length of one cycle in days.
const BAVLI_TOTAL_AMUDIM: i32 = 5407;

fn initial_cycle_date() -> Date<Hebrew> {
    #[allow(clippy::expect_used)]
    let date = Date::try_new_gregorian(2023, 10, 16)
        .expect("hard-coded Gregorian date should be valid")
        .to_calendar(Hebrew);
    date
}

pub(crate) const fn start_daf(tractate: BavliTractate) -> Amud {
    match tractate {
        BavliTractate::Kinnim => Amud::new(BavliTractate::Kinnim, 22, Side::Bet),
        BavliTractate::Tamid => Amud::new(BavliTractate::Tamid, 25, Side::Bet),
        BavliTractate::Midos => Amud::new(BavliTractate::Midos, 34, Side::Aleph),
        _ => Amud::new(tractate, 2, Side::Aleph),
    }
}

pub(crate) const fn end_daf(tractate: BavliTractate) -> Amud {
    match tractate {
        BavliTractate::Berachos => Amud::new(BavliTractate::Berachos, 64, Side::Aleph),
        BavliTractate::Shabbos => Amud::new(BavliTractate::Shabbos, 157, Side::Bet),
        BavliTractate::Eruvin => Amud::new(BavliTractate::Eruvin, 105, Side::Aleph),
        BavliTractate::Pesachim => Amud::new(BavliTractate::Pesachim, 121, Side::Bet),
        BavliTractate::Shekalim => Amud::new(BavliTractate::Shekalim, 22, Side::Bet),
        BavliTractate::Yoma => Amud::new(BavliTractate::Yoma, 88, Side::Aleph),
        BavliTractate::Sukkah => Amud::new(BavliTractate::Sukkah, 56, Side::Bet),
        BavliTractate::Beitzah => Amud::new(BavliTractate::Beitzah, 40, Side::Bet),
        BavliTractate::RoshHashanah => Amud::new(BavliTractate::RoshHashanah, 35, Side::Bet),
        BavliTractate::Taanis => Amud::new(BavliTractate::Taanis, 31, Side::Aleph),
        BavliTractate::Megillah => Amud::new(BavliTractate::Megillah, 32, Side::Aleph),
        BavliTractate::MoedKatan => Amud::new(BavliTractate::MoedKatan, 29, Side::Aleph),
        BavliTractate::Chagigah => Amud::new(BavliTractate::Chagigah, 27, Side::Aleph),
        BavliTractate::Yevamos => Amud::new(BavliTractate::Yevamos, 122, Side::Bet),
        BavliTractate::Kesubos => Amud::new(BavliTractate::Kesubos, 112, Side::Bet),
        BavliTractate::Nedarim => Amud::new(BavliTractate::Nedarim, 91, Side::Bet),
        BavliTractate::Nazir => Amud::new(BavliTractate::Nazir, 66, Side::Bet),
        BavliTractate::Sotah => Amud::new(BavliTractate::Sotah, 49, Side::Bet),
        BavliTractate::Gitin => Amud::new(BavliTractate::Gitin, 90, Side::Bet),
        BavliTractate::Kiddushin => Amud::new(BavliTractate::Kiddushin, 82, Side::Bet),
        BavliTractate::BavaKamma => Amud::new(BavliTractate::BavaKamma, 119, Side::Bet),
        BavliTractate::BavaMetzia => Amud::new(BavliTractate::BavaMetzia, 119, Side::Aleph),
        BavliTractate::BavaBasra => Amud::new(BavliTractate::BavaBasra, 176, Side::Bet),
        BavliTractate::Sanhedrin => Amud::new(BavliTractate::Sanhedrin, 113, Side::Bet),
        BavliTractate::Makkos => Amud::new(BavliTractate::Makkos, 24, Side::Bet),
        BavliTractate::Shevuos => Amud::new(BavliTractate::Shevuos, 49, Side::Bet),
        BavliTractate::AvodahZarah => Amud::new(BavliTractate::AvodahZarah, 76, Side::Bet),
        BavliTractate::Horiyos => Amud::new(BavliTractate::Horiyos, 14, Side::Aleph),
        BavliTractate::Zevachim => Amud::new(BavliTractate::Zevachim, 120, Side::Bet),
        BavliTractate::Menachos => Amud::new(BavliTractate::Menachos, 110, Side::Aleph),
        BavliTractate::Chullin => Amud::new(BavliTractate::Chullin, 142, Side::Aleph),
        BavliTractate::Bechoros => Amud::new(BavliTractate::Bechoros, 61, Side::Aleph),
        BavliTractate::Arachin => Amud::new(BavliTractate::Arachin, 34, Side::Aleph),
        BavliTractate::Temurah => Amud::new(BavliTractate::Temurah, 34, Side::Aleph),
        BavliTractate::Kerisos => Amud::new(BavliTractate::Kerisos, 28, Side::Bet),
        BavliTractate::Meilah => Amud::new(BavliTractate::Meilah, 22, Side::Aleph),
        BavliTractate::Kinnim => Amud::new(BavliTractate::Kinnim, 25, Side::Aleph),
        BavliTractate::Tamid => Amud::new(BavliTractate::Tamid, 33, Side::Bet),
        BavliTractate::Midos => Amud::new(BavliTractate::Midos, 37, Side::Bet),
        BavliTractate::Niddah => Amud::new(BavliTractate::Niddah, 73, Side::Aleph),
    }
}

/// Dirshu's Amud Yomi, one side of a page of the Bavli a day, from 16 October 2023.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct AmudYomiDirshu;

fn amud_yomi_bavli_dirshu_for_date(limud_date: Date<Hebrew>) -> Option<Amud> {
    let cycle = Cycle::from_cycle_initiation(initial_cycle_date(), AmudYomiDirshu::cycle_end_calculation, limud_date)?;
    if limud_date > cycle.end_date {
        return None;
    }
    let offset = cycle.start_date.days_until(&limud_date)? as usize;
    let mut remaining = offset;
    for tractate in BavliTractate::ALL {
        let start = start_daf(tractate);
        let end = end_daf(tractate);
        let count = start.amudim_to(end)? as usize;
        if remaining < count {
            return start.at_offset(remaining as i32);
        }
        remaining -= count;
    }
    None
}

impl InternalLimud<Amud> for AmudYomiDirshu {
    fn limud(&self, limud_date: Date<Hebrew>) -> Option<Amud> {
        amud_yomi_bavli_dirshu_for_date(limud_date)
    }

    fn cycle_finder(&self) -> CycleFinder {
        CycleFinder::Initial(initial_cycle_date())
    }
    fn cycle_end_calculation(hebrew_date: Date<Hebrew>, _iteration: Option<i32>) -> Option<Date<Hebrew>> {
        hebrew_date.add_days(BAVLI_TOTAL_AMUDIM - 1)
    }

    fn unit_for_interval(&self, _interval: &Interval, limud_date: &Date<Hebrew>) -> Option<Amud> {
        amud_yomi_bavli_dirshu_for_date(*limud_date)
    }
}
super::schedule!(AmudYomiDirshu => Amud);

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use crate::limudim::engine::from_gregorian_date;

    use super::*;

    #[test]
    fn amud_yomi_bavli_dirshu_simple_date() {
        let test_date = from_gregorian_date(2024, 5, 30);
        let limud = AmudYomiDirshu.limud(test_date).expect("limud exists");
        assert_eq!(limud.daf, 53);
        assert_eq!(limud.side, Side::Aleph);
        assert_eq!(limud.tractate, BavliTractate::Shabbos);
    }

    #[test]
    fn amud_yomi_bavli_dirshu_before_cycle_began() {
        let test_date = from_gregorian_date(2023, 1, 1);
        let limud = AmudYomiDirshu.limud(test_date);
        assert!(limud.is_none());
    }

    #[test]
    fn amud_yomi_bavli_dirshu_first_day_of_cycle() {
        let test_date = from_gregorian_date(2038, 8, 5);
        let limud = AmudYomiDirshu.limud(test_date).expect("limud exists");
        assert_eq!(limud.daf, 2);
        assert_eq!(limud.side, Side::Aleph);
        assert_eq!(limud.tractate, BavliTractate::Berachos);
    }

    #[test]
    fn amud_yomi_bavli_dirshu_last_day_of_cycle() {
        let test_date = from_gregorian_date(2038, 8, 4);
        let limud = AmudYomiDirshu.limud(test_date).expect("limud exists");
        assert_eq!(limud.daf, 73);
        assert_eq!(limud.side, Side::Aleph);
        assert_eq!(limud.tractate, BavliTractate::Niddah);
    }

    #[test]
    fn amud_yomi_bavli_dirshu_end_of_meilah() {
        let test_date = from_gregorian_date(2038, 2, 10);
        let limud = AmudYomiDirshu.limud(test_date).expect("limud exists");
        assert_eq!(limud.daf, 21);
        assert_eq!(limud.side, Side::Bet);
        assert_eq!(limud.tractate, BavliTractate::Meilah);
    }

    #[test]
    fn amud_yomi_bavli_dirshu_beginning_of_kinnim() {
        let test_date = from_gregorian_date(2038, 2, 12);
        let limud = AmudYomiDirshu.limud(test_date).expect("limud exists");
        assert_eq!(limud.daf, 22);
        assert_eq!(limud.side, Side::Bet);
        assert_eq!(limud.tractate, BavliTractate::Kinnim);
    }

    #[test]
    fn amud_yomi_bavli_dirshu_end_of_kinnim() {
        let test_date = from_gregorian_date(2038, 2, 17);
        let limud = AmudYomiDirshu.limud(test_date).expect("limud exists");
        assert_eq!(limud.daf, 25);
        assert_eq!(limud.side, Side::Aleph);
        assert_eq!(limud.tractate, BavliTractate::Kinnim);
    }

    #[test]
    fn amud_yomi_bavli_dirshu_beginning_of_tamid() {
        let test_date = from_gregorian_date(2038, 2, 18);
        let limud = AmudYomiDirshu.limud(test_date).expect("limud exists");
        assert_eq!(limud.daf, 25);
        assert_eq!(limud.side, Side::Bet);
        assert_eq!(limud.tractate, BavliTractate::Tamid);
    }

    #[test]
    fn amud_yomi_bavli_dirshu_end_of_tamid() {
        let test_date = from_gregorian_date(2038, 3, 6);
        let limud = AmudYomiDirshu.limud(test_date).expect("limud exists");
        assert_eq!(limud.daf, 33);
        assert_eq!(limud.side, Side::Bet);
        assert_eq!(limud.tractate, BavliTractate::Tamid);
    }

    #[test]
    fn amud_yomi_bavli_dirshu_beginning_of_midos() {
        let test_date = from_gregorian_date(2038, 3, 7);
        let limud = AmudYomiDirshu.limud(test_date).expect("limud exists");
        assert_eq!(limud.daf, 34);
        assert_eq!(limud.side, Side::Aleph);
        assert_eq!(limud.tractate, BavliTractate::Midos);
    }

    #[test]
    fn amud_yomi_bavli_dirshu_end_of_midos() {
        let test_date = from_gregorian_date(2038, 3, 14);
        let limud = AmudYomiDirshu.limud(test_date).expect("limud exists");
        assert_eq!(limud.daf, 37);
        assert_eq!(limud.side, Side::Bet);
        assert_eq!(limud.tractate, BavliTractate::Midos);
    }

    #[test]
    fn amud_yomi_bavli_dirshu_after_midos() {
        let test_date = from_gregorian_date(2038, 3, 15);
        let limud = AmudYomiDirshu.limud(test_date).expect("limud exists");
        assert_eq!(limud.daf, 2);
        assert_eq!(limud.side, Side::Aleph);
        assert_eq!(limud.tractate, BavliTractate::Niddah);
    }
    #[test]
    fn amud_yomi_bavli_2025_23_9() {
        let test_date = from_gregorian_date(2025, 9, 23);
        let limud = AmudYomiDirshu.limud(test_date).expect("limud exists");
        assert_eq!(limud.daf, 34);
        assert_eq!(limud.side, Side::Aleph);
        assert_eq!(limud.tractate, BavliTractate::Pesachim);
    }
}
