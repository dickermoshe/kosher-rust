use core::fmt;

use icu_calendar::{Date, cal::Hebrew};

use crate::limudim::{
    engine::{Cycle, CycleFinder, HebrewDateExt, InternalLimud, Interval},
    units::{Mishna, MishnaTractate},
};

/// Days from the first to the last day of a cycle (4,192 mishnayos at two a day is 2,096 days).
const MISHNA_YOMIS_CYCLE_DAYS: i32 = 2095;

fn initial_cycle_date() -> Date<Hebrew> {
    #[allow(clippy::expect_used)]
    let date = Date::try_new_gregorian(1947, 5, 20)
        .expect("hard-coded Gregorian date should be valid")
        .to_calendar(Hebrew);
    date
}

const fn chapters(tractate: MishnaTractate) -> usize {
    match tractate {
        MishnaTractate::Berachos => 9,
        MishnaTractate::Peah => 8,
        MishnaTractate::Demai => 7,
        MishnaTractate::Kilayim => 9,
        MishnaTractate::Sheviis => 10,
        MishnaTractate::Terumos => 11,
        MishnaTractate::Maasros => 5,
        MishnaTractate::MaaserSheni => 5,
        MishnaTractate::Chalah => 4,
        MishnaTractate::Orlah => 3,
        MishnaTractate::Bikurim => 4,
        MishnaTractate::Shabbos => 24,
        MishnaTractate::Eruvin => 10,
        MishnaTractate::Pesachim => 10,
        MishnaTractate::Shekalim => 8,
        MishnaTractate::Yoma => 8,
        MishnaTractate::Sukkah => 5,
        MishnaTractate::Beitzah => 5,
        MishnaTractate::RoshHashanah => 4,
        MishnaTractate::Taanis => 4,
        MishnaTractate::Megillah => 4,
        MishnaTractate::MoedKatan => 3,
        MishnaTractate::Chagigah => 3,
        MishnaTractate::Yevamos => 16,
        MishnaTractate::Kesubos => 13,
        MishnaTractate::Nedarim => 11,
        MishnaTractate::Nazir => 9,
        MishnaTractate::Sotah => 9,
        MishnaTractate::Gitin => 9,
        MishnaTractate::Kiddushin => 4,
        MishnaTractate::BavaKamma => 10,
        MishnaTractate::BavaMetzia => 10,
        MishnaTractate::BavaBasra => 10,
        MishnaTractate::Sanhedrin => 11,
        MishnaTractate::Makkos => 3,
        MishnaTractate::Shevuos => 8,
        MishnaTractate::Eduyos => 8,
        MishnaTractate::AvodahZarah => 5,
        MishnaTractate::Avos => 6,
        MishnaTractate::Horiyos => 3,
        MishnaTractate::Zevachim => 14,
        MishnaTractate::Menachos => 13,
        MishnaTractate::Chullin => 12,
        MishnaTractate::Bechoros => 9,
        MishnaTractate::Arachin => 9,
        MishnaTractate::Temurah => 7,
        MishnaTractate::Kerisos => 6,
        MishnaTractate::Meilah => 6,
        MishnaTractate::Tamid => 7,
        MishnaTractate::Midos => 5,
        MishnaTractate::Kinnim => 3,
        MishnaTractate::Keilim => 30,
        MishnaTractate::Ohalos => 18,
        MishnaTractate::Negaim => 14,
        MishnaTractate::Parah => 12,
        MishnaTractate::Taharos => 10,
        MishnaTractate::Mikvaos => 10,
        MishnaTractate::Niddah => 10,
        MishnaTractate::Machshirin => 6,
        MishnaTractate::Zavim => 5,
        MishnaTractate::TevulYom => 4,
        MishnaTractate::Yadayim => 4,
        MishnaTractate::Uktzin => 3,
    }
}
const fn chapter_length(tractate: MishnaTractate, chapter: usize) -> u16 {
    let chapter_index = chapter - 1;
    match tractate {
        MishnaTractate::Berachos => [5, 8, 6, 7, 5, 8, 5, 8, 5][chapter_index],
        MishnaTractate::Peah => [6, 8, 8, 11, 8, 11, 8, 9][chapter_index],
        MishnaTractate::Demai => [4, 5, 6, 7, 11, 12, 8][chapter_index],
        MishnaTractate::Kilayim => [9, 11, 7, 9, 8, 9, 8, 6, 10][chapter_index],
        MishnaTractate::Sheviis => [8, 10, 10, 10, 9, 6, 7, 11, 9, 9][chapter_index],
        MishnaTractate::Terumos => [10, 6, 9, 13, 9, 6, 7, 12, 7, 12, 10][chapter_index],
        MishnaTractate::Maasros => [8, 8, 10, 6, 8][chapter_index],
        MishnaTractate::MaaserSheni => [7, 10, 13, 12, 15][chapter_index],
        MishnaTractate::Chalah => [9, 8, 10, 11][chapter_index],
        MishnaTractate::Orlah => [9, 17, 9][chapter_index],
        MishnaTractate::Bikurim => [11, 11, 12, 5][chapter_index],
        MishnaTractate::Shabbos => {
            [11, 7, 6, 2, 4, 10, 4, 7, 7, 6, 6, 6, 7, 4, 3, 8, 8, 3, 6, 5, 3, 6, 5, 5][chapter_index]
        }
        MishnaTractate::Eruvin => [10, 6, 9, 11, 9, 10, 11, 11, 4, 15][chapter_index],
        MishnaTractate::Pesachim => [7, 8, 8, 9, 10, 6, 13, 8, 11, 9][chapter_index],
        MishnaTractate::Shekalim => [7, 5, 4, 9, 6, 6, 7, 8][chapter_index],
        MishnaTractate::Yoma => [8, 7, 11, 6, 7, 8, 5, 9][chapter_index],
        MishnaTractate::Sukkah => [11, 9, 15, 10, 8][chapter_index],
        MishnaTractate::Beitzah => [10, 10, 8, 7, 7][chapter_index],
        MishnaTractate::RoshHashanah => [9, 9, 8, 9][chapter_index],
        MishnaTractate::Taanis => [7, 10, 9, 8][chapter_index],
        MishnaTractate::Megillah => [11, 6, 6, 10][chapter_index],
        MishnaTractate::MoedKatan => [10, 5, 9][chapter_index],
        MishnaTractate::Chagigah => [8, 7, 8][chapter_index],
        MishnaTractate::Yevamos => [4, 10, 10, 13, 6, 6, 6, 6, 6, 9, 7, 6, 13, 9, 10, 7][chapter_index],
        MishnaTractate::Kesubos => [10, 10, 9, 12, 9, 7, 10, 8, 9, 6, 6, 4, 11][chapter_index],
        MishnaTractate::Nedarim => [4, 5, 11, 8, 6, 10, 9, 7, 10, 8, 12][chapter_index],
        MishnaTractate::Nazir => [7, 10, 7, 7, 7, 11, 4, 2, 5][chapter_index],
        MishnaTractate::Sotah => [9, 6, 8, 5, 5, 4, 8, 7, 15][chapter_index],
        MishnaTractate::Gitin => [6, 7, 8, 9, 9, 7, 9, 10, 10][chapter_index],
        MishnaTractate::Kiddushin => [10, 10, 13, 14][chapter_index],
        MishnaTractate::BavaKamma => [4, 6, 11, 9, 7, 6, 7, 7, 12, 10][chapter_index],
        MishnaTractate::BavaMetzia => [8, 11, 12, 12, 11, 8, 11, 9, 13, 6][chapter_index],
        MishnaTractate::BavaBasra => [6, 14, 8, 9, 11, 8, 4, 8, 10, 8][chapter_index],
        MishnaTractate::Sanhedrin => [6, 5, 8, 5, 5, 6, 11, 7, 6, 6, 6][chapter_index],
        MishnaTractate::Makkos => [10, 8, 16][chapter_index],
        MishnaTractate::Shevuos => [7, 5, 11, 13, 5, 7, 8, 6][chapter_index],
        MishnaTractate::Eduyos => [14, 10, 12, 12, 7, 3, 9, 7][chapter_index],
        MishnaTractate::AvodahZarah => [9, 7, 10, 12, 12][chapter_index],
        MishnaTractate::Avos => [18, 16, 18, 22, 23, 11][chapter_index],
        MishnaTractate::Horiyos => [5, 7, 8][chapter_index],
        MishnaTractate::Zevachim => [4, 5, 6, 6, 8, 7, 6, 12, 7, 8, 8, 6, 8, 10][chapter_index],
        MishnaTractate::Menachos => [4, 5, 7, 5, 9, 7, 6, 7, 9, 9, 9, 5, 11][chapter_index],
        MishnaTractate::Chullin => [7, 10, 7, 7, 5, 7, 6, 6, 8, 4, 2, 5][chapter_index],
        MishnaTractate::Bechoros => [7, 9, 4, 10, 6, 12, 7, 10, 8][chapter_index],
        MishnaTractate::Arachin => [4, 6, 5, 4, 6, 5, 5, 7, 8][chapter_index],
        MishnaTractate::Temurah => [6, 3, 5, 4, 6, 5, 6][chapter_index],
        MishnaTractate::Kerisos => [7, 6, 10, 3, 8, 9][chapter_index],
        MishnaTractate::Meilah => [4, 9, 8, 6, 5, 6][chapter_index],
        MishnaTractate::Tamid => [4, 5, 9, 3, 6, 3, 4][chapter_index],
        MishnaTractate::Midos => [9, 6, 8, 7, 4][chapter_index],
        MishnaTractate::Kinnim => [4, 5, 6][chapter_index],
        MishnaTractate::Keilim => [
            9, 8, 8, 4, 11, 4, 6, 11, 8, 8, 9, 8, 8, 8, 6, 8, 17, 9, 10, 7, 3, 10, 5, 17, 9, 9, 12, 10, 8, 4,
        ][chapter_index],
        MishnaTractate::Ohalos => [8, 7, 7, 3, 7, 7, 6, 6, 16, 7, 9, 8, 6, 7, 10, 5, 5, 10][chapter_index],
        MishnaTractate::Negaim => [6, 5, 8, 11, 5, 8, 5, 10, 3, 10, 12, 7, 12, 13][chapter_index],
        MishnaTractate::Parah => [4, 5, 11, 4, 9, 5, 12, 11, 9, 6, 9, 11][chapter_index],
        MishnaTractate::Taharos => [9, 8, 8, 13, 9, 10, 9, 9, 9, 8][chapter_index],
        MishnaTractate::Mikvaos => [8, 10, 4, 5, 6, 11, 7, 5, 7, 8][chapter_index],
        MishnaTractate::Niddah => [7, 7, 7, 7, 9, 14, 5, 4, 11, 8][chapter_index],
        MishnaTractate::Machshirin => [6, 11, 8, 10, 11, 8][chapter_index],
        MishnaTractate::Zavim => [6, 4, 3, 7, 12][chapter_index],
        MishnaTractate::TevulYom => [5, 8, 6, 7][chapter_index],
        MishnaTractate::Yadayim => [5, 4, 5, 8][chapter_index],
        MishnaTractate::Uktzin => [6, 10, 12][chapter_index],
    }
}

const fn mishna_count_in_tractate(tractate: MishnaTractate) -> usize {
    let mut count = 0;
    let mut chapter = 1;
    while chapter <= chapters(tractate) {
        count += chapter_length(tractate, chapter) as usize;
        chapter += 1;
    }
    count
}

fn mishna_at_offset(offset: usize) -> Option<Mishna> {
    let mut remaining = offset;
    for tractate in MishnaTractate::ALL {
        let count = mishna_count_in_tractate(tractate);
        if remaining < count {
            let mut index = remaining;
            let mut chapter = 1;
            while chapter <= chapters(tractate) {
                let len = chapter_length(tractate, chapter) as usize;
                if index < len {
                    return Some(Mishna {
                        tractate,
                        chapter: chapter as u16,
                        mishna: (index + 1) as u16,
                    });
                }
                index -= len;
                chapter += 1;
            }
            return None;
        }
        remaining -= count;
    }
    None
}

/// The two *mishnayos* of a day of Mishna Yomis. Prints as "Nazir 5:5-6", "Nazir 5:10-6:1" or
/// "Nazir 9:5-Sotah 1:1".
///
/// Pairs sort in the order of the Mishnah.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct MishnaPair {
    /// The first *mishnah*.
    pub first: Mishna,
    /// The second, the next *mishnah* in order.
    pub second: Mishna,
}

impl fmt::Display for MishnaPair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (a, b) = (self.first, self.second);
        if a.tractate != b.tractate {
            write!(f, "{a}-{b}")
        } else if a.chapter != b.chapter {
            write!(f, "{a}-{}:{}", b.chapter, b.mishna)
        } else {
            write!(f, "{a}-{}", b.mishna)
        }
    }
}

/// Mishna Yomis, two *mishnayos* a day, from 20 May 1947.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct MishnaYomis;
fn mishna_yomis_for_date(limud_date: Date<Hebrew>) -> Option<MishnaPair> {
    let cycle = Cycle::from_cycle_initiation(initial_cycle_date(), MishnaYomis::cycle_end_calculation, limud_date)?;
    if limud_date > cycle.end_date {
        return None;
    }
    let iteration = cycle.start_date.days_until(&limud_date)? + 1;
    mishnas_for_iteration(iteration)
}

impl InternalLimud<MishnaPair> for MishnaYomis {
    fn limud(&self, limud_date: Date<Hebrew>) -> Option<MishnaPair> {
        mishna_yomis_for_date(limud_date)
    }

    fn cycle_finder(&self) -> CycleFinder {
        CycleFinder::Initial(initial_cycle_date())
    }

    fn unit_for_interval(&self, _interval: &Interval, limud_date: &Date<Hebrew>) -> Option<MishnaPair> {
        mishna_yomis_for_date(*limud_date)
    }
    fn cycle_end_calculation(hebrew_date: Date<Hebrew>, _iteration: Option<i32>) -> Option<Date<Hebrew>> {
        hebrew_date.add_days(MISHNA_YOMIS_CYCLE_DAYS)
    }
}
super::schedule!(MishnaYomis => MishnaPair);

fn mishnas_for_iteration(iteration: u32) -> Option<MishnaPair> {
    let base = ((iteration - 1) * 2) as usize;
    Some(MishnaPair {
        first: mishna_at_offset(base)?,
        second: mishna_at_offset(base + 1)?,
    })
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use crate::limudim::engine::from_gregorian_date;

    use super::*;

    #[test]
    fn mishna_yomis_simple_date() {
        let test_date = from_gregorian_date(2017, 12, 28);
        let limud = MishnaYomis.limud(test_date).expect("limud exists");
        // Python test expects: 'megillah 3:4-5'
        assert_eq!(limud.first.tractate, MishnaTractate::Megillah);
        assert_eq!(limud.first.chapter, 3);
        assert_eq!(limud.first.mishna, 4);
        assert_eq!(limud.second.tractate, MishnaTractate::Megillah);
        assert_eq!(limud.second.chapter, 3);
        assert_eq!(limud.second.mishna, 5);
    }

    #[test]
    fn mishna_yomis_before_cycle_began() {
        let test_date = from_gregorian_date(1947, 1, 1);
        let limud = MishnaYomis.limud(test_date);
        assert!(limud.is_none());
    }

    #[test]
    fn mishna_yomis_first_day_of_cycle() {
        let test_date = from_gregorian_date(2016, 3, 30);
        let limud = MishnaYomis.limud(test_date).expect("limud exists");
        // Python test expects: 'berachos 1:1-2'
        assert_eq!(limud.first.tractate, MishnaTractate::Berachos);
        assert_eq!(limud.first.chapter, 1);
        assert_eq!(limud.first.mishna, 1);
        assert_eq!(limud.second.tractate, MishnaTractate::Berachos);
        assert_eq!(limud.second.chapter, 1);
        assert_eq!(limud.second.mishna, 2);
    }

    #[test]
    fn mishna_yomis_last_day_of_cycle() {
        let test_date = from_gregorian_date(2016, 3, 29);
        let limud = MishnaYomis.limud(test_date).expect("limud exists");
        // Python test expects: 'uktzin 3:11-12'
        assert_eq!(limud.first.tractate, MishnaTractate::Uktzin);
        assert_eq!(limud.first.chapter, 3);
        assert_eq!(limud.first.mishna, 11);
        assert_eq!(limud.second.tractate, MishnaTractate::Uktzin);
        assert_eq!(limud.second.chapter, 3);
        assert_eq!(limud.second.mishna, 12);
    }

    #[test]
    fn mishna_yomis_span_two_masechtos() {
        let test_date = from_gregorian_date(2016, 4, 27);
        let limud = MishnaYomis.limud(test_date).expect("limud exists");
        // Python test expects: 'berachos 9:5 - peah 1:1'
        assert_eq!(limud.first.tractate, MishnaTractate::Berachos);
        assert_eq!(limud.first.chapter, 9);
        assert_eq!(limud.first.mishna, 5);
        assert_eq!(limud.second.tractate, MishnaTractate::Peah);
        assert_eq!(limud.second.chapter, 1);
        assert_eq!(limud.second.mishna, 1);
    }

    #[test]
    fn mishna_yomis_span_two_perakim() {
        let test_date = from_gregorian_date(2017, 12, 23);
        let limud = MishnaYomis.limud(test_date).expect("limud exists");
        // Python test expects: 'megillah 1:11-2:1'
        assert_eq!(limud.first.tractate, MishnaTractate::Megillah);
        assert_eq!(limud.first.chapter, 1);
        assert_eq!(limud.first.mishna, 11);
        assert_eq!(limud.second.tractate, MishnaTractate::Megillah);
        assert_eq!(limud.second.chapter, 2);
        assert_eq!(limud.second.mishna, 1);
    }

    #[test]
    fn mishna_yomis_1947_05_20() {
        let test_date = from_gregorian_date(1947, 5, 20);
        let limud = MishnaYomis.limud(test_date).expect("limud exists");
        assert_eq!(limud.first.tractate, MishnaTractate::Berachos);
        assert_eq!(limud.first.chapter, 1);
        assert_eq!(limud.first.mishna, 1);
        assert_eq!(limud.second.tractate, MishnaTractate::Berachos);
        assert_eq!(limud.second.chapter, 1);
        assert_eq!(limud.second.mishna, 2);
    }

    #[test]
    fn mishna_yomis_1950_01_01() {
        let test_date = from_gregorian_date(1950, 1, 1);
        let limud = MishnaYomis.limud(test_date).expect("limud exists");
        assert_eq!(limud.first.tractate, MishnaTractate::BavaKamma);
        assert_eq!(limud.first.chapter, 1);
        assert_eq!(limud.first.mishna, 1);
        assert_eq!(limud.second.tractate, MishnaTractate::BavaKamma);
        assert_eq!(limud.second.chapter, 1);
        assert_eq!(limud.second.mishna, 2);
    }

    #[test]
    fn mishna_yomis_1960_01_01() {
        let test_date = from_gregorian_date(1960, 1, 1);
        let limud = MishnaYomis.limud(test_date).expect("limud exists");
        assert_eq!(limud.first.tractate, MishnaTractate::Eruvin);
        assert_eq!(limud.first.chapter, 5);
        assert_eq!(limud.first.mishna, 5);
        assert_eq!(limud.second.tractate, MishnaTractate::Eruvin);
        assert_eq!(limud.second.chapter, 5);
        assert_eq!(limud.second.mishna, 6);
    }

    #[test]
    fn mishna_yomis_1970_01_01() {
        let test_date = from_gregorian_date(1970, 1, 1);
        let limud = MishnaYomis.limud(test_date).expect("limud exists");
        assert_eq!(limud.first.tractate, MishnaTractate::Mikvaos);
        assert_eq!(limud.first.chapter, 10);
        assert_eq!(limud.first.mishna, 6);
        assert_eq!(limud.second.tractate, MishnaTractate::Mikvaos);
        assert_eq!(limud.second.chapter, 10);
        assert_eq!(limud.second.mishna, 7);
    }

    #[test]
    fn mishna_yomis_1980_01_01() {
        let test_date = from_gregorian_date(1980, 1, 1);
        let limud = MishnaYomis.limud(test_date).expect("limud exists");
        assert_eq!(limud.first.tractate, MishnaTractate::Bechoros);
        assert_eq!(limud.first.chapter, 1);
        assert_eq!(limud.first.mishna, 2);
        assert_eq!(limud.second.tractate, MishnaTractate::Bechoros);
        assert_eq!(limud.second.chapter, 1);
        assert_eq!(limud.second.mishna, 3);
    }

    #[test]
    fn mishna_yomis_1990_01_01() {
        let test_date = from_gregorian_date(1990, 1, 1);
        let limud = MishnaYomis.limud(test_date).expect("limud exists");
        assert_eq!(limud.first.tractate, MishnaTractate::Sotah);
        assert_eq!(limud.first.chapter, 9);
        assert_eq!(limud.first.mishna, 14);
        assert_eq!(limud.second.tractate, MishnaTractate::Sotah);
        assert_eq!(limud.second.chapter, 9);
        assert_eq!(limud.second.mishna, 15);
    }

    #[test]
    fn mishna_yomis_2000_01_01() {
        let test_date = from_gregorian_date(2000, 1, 1);
        let limud = MishnaYomis.limud(test_date).expect("limud exists");
        assert_eq!(limud.first.tractate, MishnaTractate::Shabbos);
        assert_eq!(limud.first.chapter, 9);
        assert_eq!(limud.first.mishna, 5);
        assert_eq!(limud.second.tractate, MishnaTractate::Shabbos);
        assert_eq!(limud.second.chapter, 9);
        assert_eq!(limud.second.mishna, 6);
    }

    #[test]
    fn mishna_yomis_2010_01_01() {
        let test_date = from_gregorian_date(2010, 1, 1);
        let limud = MishnaYomis.limud(test_date).expect("limud exists");
        assert_eq!(limud.first.tractate, MishnaTractate::Taharos);
        assert_eq!(limud.first.chapter, 4);
        assert_eq!(limud.first.mishna, 12);
        assert_eq!(limud.second.tractate, MishnaTractate::Taharos);
        assert_eq!(limud.second.chapter, 4);
        assert_eq!(limud.second.mishna, 13);
    }

    #[test]
    fn mishna_yomis_2016_04_01() {
        let test_date = from_gregorian_date(2016, 4, 1);
        let limud = MishnaYomis.limud(test_date).expect("limud exists");
        // Python: 'berachos 1:5-2:1' - spans two chapters
        assert_eq!(limud.first.tractate, MishnaTractate::Berachos);
        assert_eq!(limud.first.chapter, 1);
        assert_eq!(limud.first.mishna, 5);
        assert_eq!(limud.second.tractate, MishnaTractate::Berachos);
        assert_eq!(limud.second.chapter, 2);
        assert_eq!(limud.second.mishna, 1);
    }

    #[test]
    fn mishna_yomis_2016_05_01() {
        let test_date = from_gregorian_date(2016, 5, 1);
        let limud = MishnaYomis.limud(test_date).expect("limud exists");
        assert_eq!(limud.first.tractate, MishnaTractate::Peah);
        assert_eq!(limud.first.chapter, 2);
        assert_eq!(limud.first.mishna, 2);
        assert_eq!(limud.second.tractate, MishnaTractate::Peah);
        assert_eq!(limud.second.chapter, 2);
        assert_eq!(limud.second.mishna, 3);
    }

    #[test]
    fn mishna_yomis_2017_01_01() {
        let test_date = from_gregorian_date(2017, 1, 1);
        let limud = MishnaYomis.limud(test_date).expect("limud exists");
        assert_eq!(limud.first.tractate, MishnaTractate::Chalah);
        assert_eq!(limud.first.chapter, 2);
        assert_eq!(limud.first.mishna, 3);
        assert_eq!(limud.second.tractate, MishnaTractate::Chalah);
        assert_eq!(limud.second.chapter, 2);
        assert_eq!(limud.second.mishna, 4);
    }

    #[test]
    fn mishna_yomis_2017_06_01() {
        let test_date = from_gregorian_date(2017, 6, 1);
        let limud = MishnaYomis.limud(test_date).expect("limud exists");
        assert_eq!(limud.first.tractate, MishnaTractate::Eruvin);
        assert_eq!(limud.first.chapter, 7);
        assert_eq!(limud.first.mishna, 8);
        assert_eq!(limud.second.tractate, MishnaTractate::Eruvin);
        assert_eq!(limud.second.chapter, 7);
        assert_eq!(limud.second.mishna, 9);
    }

    #[test]
    fn mishna_yomis_2017_12_01() {
        let test_date = from_gregorian_date(2017, 12, 1);
        let limud = MishnaYomis.limud(test_date).expect("limud exists");
        assert_eq!(limud.first.tractate, MishnaTractate::Taanis);
        assert_eq!(limud.first.chapter, 1);
        assert_eq!(limud.first.mishna, 1);
        assert_eq!(limud.second.tractate, MishnaTractate::Taanis);
        assert_eq!(limud.second.chapter, 1);
        assert_eq!(limud.second.mishna, 2);
    }

    #[test]
    fn mishna_yomis_2018_01_01() {
        let test_date = from_gregorian_date(2018, 1, 1);
        let limud = MishnaYomis.limud(test_date).expect("limud exists");
        assert_eq!(limud.first.tractate, MishnaTractate::Megillah);
        assert_eq!(limud.first.chapter, 4);
        assert_eq!(limud.first.mishna, 6);
        assert_eq!(limud.second.tractate, MishnaTractate::Megillah);
        assert_eq!(limud.second.chapter, 4);
        assert_eq!(limud.second.mishna, 7);
    }

    #[test]
    fn mishna_yomis_2020_01_01() {
        let test_date = from_gregorian_date(2020, 1, 1);
        let limud = MishnaYomis.limud(test_date).expect("limud exists");
        assert_eq!(limud.first.tractate, MishnaTractate::Menachos);
        assert_eq!(limud.first.chapter, 8);
        assert_eq!(limud.first.mishna, 2);
        assert_eq!(limud.second.tractate, MishnaTractate::Menachos);
        assert_eq!(limud.second.chapter, 8);
        assert_eq!(limud.second.mishna, 3);
    }

    #[test]
    fn mishna_yomis_2016_03_31() {
        let test_date = from_gregorian_date(2016, 3, 31);
        let limud = MishnaYomis.limud(test_date).expect("limud exists");
        assert_eq!(limud.first.tractate, MishnaTractate::Berachos);
        assert_eq!(limud.first.chapter, 1);
        assert_eq!(limud.first.mishna, 3);
        assert_eq!(limud.second.tractate, MishnaTractate::Berachos);
        assert_eq!(limud.second.chapter, 1);
        assert_eq!(limud.second.mishna, 4);
    }

    #[test]
    fn mishna_yomis_2016_04_02() {
        let test_date = from_gregorian_date(2016, 4, 2);
        let limud = MishnaYomis.limud(test_date).expect("limud exists");
        assert_eq!(limud.first.tractate, MishnaTractate::Berachos);
        assert_eq!(limud.first.chapter, 2);
        assert_eq!(limud.first.mishna, 2);
        assert_eq!(limud.second.tractate, MishnaTractate::Berachos);
        assert_eq!(limud.second.chapter, 2);
        assert_eq!(limud.second.mishna, 3);
    }

    #[test]
    fn mishna_yomis_2017_12_24() {
        let test_date = from_gregorian_date(2017, 12, 24);
        let limud = MishnaYomis.limud(test_date).expect("limud exists");
        assert_eq!(limud.first.tractate, MishnaTractate::Megillah);
        assert_eq!(limud.first.chapter, 2);
        assert_eq!(limud.first.mishna, 2);
        assert_eq!(limud.second.tractate, MishnaTractate::Megillah);
        assert_eq!(limud.second.chapter, 2);
        assert_eq!(limud.second.mishna, 3);
    }

    #[test]
    fn mishna_yomis_2017_12_25() {
        let test_date = from_gregorian_date(2017, 12, 25);
        let limud = MishnaYomis.limud(test_date).expect("limud exists");
        assert_eq!(limud.first.tractate, MishnaTractate::Megillah);
        assert_eq!(limud.first.chapter, 2);
        assert_eq!(limud.first.mishna, 4);
        assert_eq!(limud.second.tractate, MishnaTractate::Megillah);
        assert_eq!(limud.second.chapter, 2);
        assert_eq!(limud.second.mishna, 5);
    }

    #[test]
    fn mishna_yomis_2017_12_26() {
        let test_date = from_gregorian_date(2017, 12, 26);
        let limud = MishnaYomis.limud(test_date).expect("limud exists");
        // Python: 'megillah 2:6-3:1' - spans two chapters
        assert_eq!(limud.first.tractate, MishnaTractate::Megillah);
        assert_eq!(limud.first.chapter, 2);
        assert_eq!(limud.first.mishna, 6);
        assert_eq!(limud.second.tractate, MishnaTractate::Megillah);
        assert_eq!(limud.second.chapter, 3);
        assert_eq!(limud.second.mishna, 1);
    }

    #[test]
    fn mishna_yomis_2017_12_27() {
        let test_date = from_gregorian_date(2017, 12, 27);
        let limud = MishnaYomis.limud(test_date).expect("limud exists");
        assert_eq!(limud.first.tractate, MishnaTractate::Megillah);
        assert_eq!(limud.first.chapter, 3);
        assert_eq!(limud.first.mishna, 2);
        assert_eq!(limud.second.tractate, MishnaTractate::Megillah);
        assert_eq!(limud.second.chapter, 3);
        assert_eq!(limud.second.mishna, 3);
    }

    #[test]
    fn mishna_yomis_2012_02_26_rosh_hashanah_chapter_boundary() {
        // Regression test: Rosh Hashanah chapter 2 has 9 mishnayot, not 8
        // This date crosses from chapter 2 to chapter 3
        let test_date = from_gregorian_date(2012, 2, 26);
        let limud = MishnaYomis.limud(test_date).expect("limud exists");
        assert_eq!(limud.first.tractate, MishnaTractate::RoshHashanah);
        assert_eq!(limud.first.chapter, 2);
        assert_eq!(limud.first.mishna, 9);
        assert_eq!(limud.second.tractate, MishnaTractate::RoshHashanah);
        assert_eq!(limud.second.chapter, 3);
        assert_eq!(limud.second.mishna, 1);
    }

    #[test]
    fn mishna_yomis_2012_02_27_rosh_hashanah_chapter_3() {
        // Regression test: Day after crossing chapter boundary
        let test_date = from_gregorian_date(2012, 2, 27);
        let limud = MishnaYomis.limud(test_date).expect("limud exists");
        assert_eq!(limud.first.tractate, MishnaTractate::RoshHashanah);
        assert_eq!(limud.first.chapter, 3);
        assert_eq!(limud.first.mishna, 2);
        assert_eq!(limud.second.tractate, MishnaTractate::RoshHashanah);
        assert_eq!(limud.second.chapter, 3);
        assert_eq!(limud.second.mishna, 3);
    }
}
