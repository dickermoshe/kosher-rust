//! Behaviour of the public limudim API. Conformance against KosherJava and hebcal over thousands of
//! random dates lives in interop-tests/.

#![allow(clippy::unwrap_used, missing_docs)]

use icu_calendar::Date;
use jiff::civil::date;
use kosher_rust::calendar::{HebrewDate, HebrewMonth, Region};
use kosher_rust::limudim::{
    AmudYomiDirshu, DafHashavuaBavli, DafYomiBavli, DafYomiYerushalmi, Mishna, MishnaPair, MishnaYomis, PirkeiAvos,
    PirkeiAvosUnit, Schedule, TehillimMonthly, TehillimUnit, Tractate,
};

#[test]
fn accepts_jiff_icu_and_hebrew_dates() {
    let from_jiff = DafYomiBavli.on(date(2017, 12, 28));
    let from_icu = DafYomiBavli.on(Date::try_new_gregorian(2017, 12, 28).unwrap());
    let from_hebrew = DafYomiBavli.on(HebrewDate::try_from(date(2017, 12, 28)).unwrap());
    assert_eq!(from_jiff, from_icu);
    assert_eq!(from_jiff, from_hebrew);
    assert_eq!(from_jiff.unwrap().to_string(), "Shevuos 30");
}

#[test]
fn readme_examples() {
    let day = date(2024, 4, 23);
    assert_eq!(DafYomiBavli.on(day).unwrap().to_string(), "Bava Metzia 55");
    assert_eq!(MishnaYomis.on(day).unwrap().to_string(), "Nazir 5:5-6");
    assert_eq!(TehillimMonthly.on(day).unwrap().to_string(), "Tehillim 77-78");
}

#[test]
fn nothing_before_a_program_began() {
    assert_eq!(DafYomiBavli.on(date(1923, 9, 10)), None);
    assert_eq!(DafHashavuaBavli.on(date(2005, 3, 5)), None);
    assert_eq!(AmudYomiDirshu.on(date(2023, 1, 1)), None);
}

#[test]
fn units_print_like_a_calendar() {
    assert_eq!(AmudYomiDirshu.on(date(2024, 5, 30)).unwrap().to_string(), "Shabbos 53a");
    assert_eq!(
        DafHashavuaBavli.on(date(2018, 10, 10)).unwrap().to_string(),
        "Megillah 2"
    );
    assert!(DafYomiYerushalmi.on(date(2024, 4, 23)).is_some());

    let psalm_119 = TehillimMonthly
        .on(HebrewDate::new(5784, HebrewMonth::Nisan, 25).unwrap())
        .unwrap();
    assert_eq!(
        psalm_119,
        TehillimUnit::PsalmVerses {
            psalm: 119,
            start_verse: 1,
            end_verse: 96
        }
    );
    assert_eq!(psalm_119.to_string(), "Tehillim 119:1-96");

    let avos = PirkeiAvos(Region::Diaspora).on(HebrewDate::new(5778, HebrewMonth::Elul, 20).unwrap());
    assert_eq!(avos, Some(PirkeiAvosUnit::Combined(3, 4)));
    assert_eq!(avos.unwrap().to_string(), "Pirkei Avos 3-4");
    assert_eq!(PirkeiAvosUnit::Single(6).to_string(), "Pirkei Avos 6");
}

#[test]
fn mishna_pairs_across_chapters_and_tractates() {
    let mishna = |tractate, chapter, mishna| Mishna {
        tractate,
        chapter,
        mishna,
    };
    let across_chapters = MishnaPair {
        first: mishna(Tractate::Nazir, 5, 10),
        second: mishna(Tractate::Nazir, 6, 1),
    };
    assert_eq!(across_chapters.to_string(), "Nazir 5:10-6:1");

    let across_tractates = MishnaPair {
        first: mishna(Tractate::Nazir, 9, 5),
        second: mishna(Tractate::Sotah, 1, 1),
    };
    assert_eq!(across_tractates.to_string(), "Nazir 9:5-Sotah 1:1");
}

#[test]
fn tractate_names() {
    assert_eq!(Tractate::BavaMetzia.to_string(), "Bava Metzia");
    assert_eq!(Tractate::BavaMetzia.hebrew(), "בבא מציעא");
    assert_eq!(Tractate::RoshHashanah.hebrew(), "ראש השנה");
}

#[test]
fn tractate_names_parse_back() {
    for tractate in Tractate::ALL {
        assert_eq!(Tractate::from_name(tractate.name()), Some(tractate));
    }
    assert_eq!(Tractate::from_name(""), None);
}

#[test]
fn pirkei_avos_weekdays_of_pesach_give_the_first_chapter() {
    let diaspora = PirkeiAvos(Region::Diaspora);
    // 21 Nisan 5671, before the Shabbos of 24 Nisan.
    assert_eq!(diaspora.on(date(1911, 4, 19)), Some(PirkeiAvosUnit::Single(1)));
    // 18 Nisan 5846, before the Shabbos of 22 Nisan. That Shabbos is after Pesach in Israel, and
    // the last day of Pesach in the diaspora, where it has no reading.
    assert_eq!(PirkeiAvos(Region::Israel).on(date(2086, 4, 2)), Some(PirkeiAvosUnit::Single(1)));
    assert_eq!(diaspora.on(date(2086, 4, 2)), None);
    // In 2024 the Shabbos of 19 Nisan is Chol Hamoed, so its week has no reading. The next
    // week starts on Sunday, 20 Nisan.
    assert_eq!(diaspora.on(date(2024, 4, 24)), None);
    assert_eq!(diaspora.on(date(2024, 4, 27)), None);
    assert_eq!(diaspora.on(date(2024, 4, 28)), Some(PirkeiAvosUnit::Single(1)));
}
