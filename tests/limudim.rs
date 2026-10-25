//! Behaviour of the public limudim API. Conformance against KosherJava and hebcal over thousands of
//! random dates lives in interop-tests/.

#![allow(clippy::unwrap_used, missing_docs)]

use icu_calendar::Date;
use jiff::civil::date;
use kosher_rust::calendar::{HebrewDate, HebrewMonth, Region};
use kosher_rust::limudim::{
    AmudYomiDirshu, BavliTractate, Daf, DafHashavuaBavli, DafYomiBavli, DafYomiYerushalmi, Mishna, MishnaPair,
    MishnaTractate, MishnaYomis, PirkeiAvos, PirkeiAvosUnit, Schedule, TehillimMonthly, TehillimUnit,
    YerushalmiTractate,
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
fn yerushalmi_cycles_ending_on_a_skip_day_keep_every_daf() {
    // Tishah B'Av 5932 falls in the days this cycle is extended by. It still ends with Niddah 13.
    let daf = |day| DafYomiYerushalmi.on(day).map(|daf| daf.to_string());
    assert_eq!(daf(date(2172, 8, 6)).as_deref(), Some("Niddah 12"));
    assert_eq!(daf(date(2172, 8, 7)).as_deref(), Some("Niddah 13"));
    assert_eq!(daf(date(2172, 8, 8)).as_deref(), Some("Berachos 1"));
}

#[test]
fn yerushalmi_cycle_with_tishah_bav_before_its_last_daf_keeps_every_daf() {
    // Tishah B'Av 6240 falls the day before the last daf of the cycle.
    let daf = |day| DafYomiYerushalmi.on(day).map(|daf| daf.to_string());
    assert_eq!(daf(date(2480, 8, 14)).as_deref(), Some("Niddah 12"));
    assert_eq!(daf(date(2480, 8, 15)), None);
    assert_eq!(daf(date(2480, 8, 16)).as_deref(), Some("Niddah 13"));
    assert_eq!(daf(date(2480, 8, 17)).as_deref(), Some("Berachos 1"));
}

#[test]
fn yerushalmi_runs_to_the_end_of_the_range() {
    // The last cycle starts on 21 Tishrei 9997 and ends after the range.
    assert!(DafYomiYerushalmi.on(date(6238, 10, 1)).is_some());
    assert!(
        DafYomiYerushalmi
            .on(HebrewDate::new(9999, HebrewMonth::Elul, 29).unwrap())
            .is_some()
    );
}

#[test]
fn mishna_pairs_across_chapters_and_tractates() {
    let mishna = |tractate, chapter, mishna| Mishna {
        tractate,
        chapter,
        mishna,
    };
    let across_chapters = MishnaPair {
        first: mishna(MishnaTractate::Nazir, 5, 10),
        second: mishna(MishnaTractate::Nazir, 6, 1),
    };
    assert_eq!(across_chapters.to_string(), "Nazir 5:10-6:1");

    let across_tractates = MishnaPair {
        first: mishna(MishnaTractate::Nazir, 9, 5),
        second: mishna(MishnaTractate::Sotah, 1, 1),
    };
    assert_eq!(across_tractates.to_string(), "Nazir 9:5-Sotah 1:1");
}

#[test]
fn tractate_names() {
    assert_eq!(MishnaTractate::BavaMetzia.to_string(), "Bava Metzia");
    assert_eq!(MishnaTractate::BavaMetzia.hebrew(), "בבא מציעא");
    assert_eq!(MishnaTractate::RoshHashanah.hebrew(), "ראש השנה");
}

#[test]
fn tractate_names_parse_back() {
    for tractate in MishnaTractate::ALL {
        assert_eq!(MishnaTractate::from_name(tractate.name()), Some(tractate));
    }
    assert_eq!(MishnaTractate::from_name(""), None);
}

#[test]
fn pirkei_avos_weekdays_of_pesach_give_the_first_chapter() {
    let diaspora = PirkeiAvos(Region::Diaspora);
    // 21 Nisan 5671, before the Shabbos of 24 Nisan.
    assert_eq!(diaspora.on(date(1911, 4, 19)), Some(PirkeiAvosUnit::Single(1)));
    // 18 Nisan 5846, before the Shabbos of 22 Nisan. That Shabbos is after Pesach in Israel, and
    // the last day of Pesach in the diaspora, where it has no reading.
    assert_eq!(
        PirkeiAvos(Region::Israel).on(date(2086, 4, 2)),
        Some(PirkeiAvosUnit::Single(1))
    );
    assert_eq!(diaspora.on(date(2086, 4, 2)), None);
    // In 2024 the Shabbos of 19 Nisan is Chol Hamoed, so its week has no reading. The next
    // week starts on Sunday, 20 Nisan.
    assert_eq!(diaspora.on(date(2024, 4, 24)), None);
    assert_eq!(diaspora.on(date(2024, 4, 27)), None);
    assert_eq!(diaspora.on(date(2024, 4, 28)), Some(PirkeiAvosUnit::Single(1)));
}

#[test]
fn yerushalmi_skips_tishah_bav_when_it_is_postponed() {
    // 9 Av 5775 was a Shabbos, so the fast and the skipped day were Sunday. Checked against hebcal.
    let daf = |day| DafYomiYerushalmi.on(day).map(|daf| daf.to_string());
    assert_eq!(daf(date(2015, 7, 24)).as_deref(), Some("Shabbos 62"));
    assert_eq!(daf(date(2015, 7, 25)).as_deref(), Some("Shabbos 63"));
    assert_eq!(daf(date(2015, 7, 26)), None);
    assert_eq!(daf(date(2015, 7, 27)).as_deref(), Some("Shabbos 64"));
}

/// Every Shabbos of a summer, from hebcal. 9 Av was on Shabbos in 5775 and 5778, and 8 Av in 5776;
/// those Shabbosos have no chapter.
#[test]
fn pirkei_avos_summers_with_a_shabbos_on_8_or_9_av() {
    let summers = [
        (
            Region::Diaspora,
            (2015, 4, 18),
            "1 2 3 4 5 6 1 2 3 4 5 6 1 2 - 3 4 5 6 1-2 3-4 5-6",
        ),
        (
            Region::Israel,
            (2015, 4, 11),
            "1 2 3 4 5 6 1 2 3 4 5 6 1 2 3 - 4 5 6 1 2 3-4 5-6",
        ),
        (
            Region::Diaspora,
            (2016, 5, 7),
            "1 2 3 4 5 6 1 2 3 4 5 6 1 2 - 3 4 5 6 1-2 3-4",
        ),
        (
            Region::Israel,
            (2016, 4, 30),
            "1 2 3 4 5 6 1 2 3 4 5 6 1 2 3 - 4 5 6 1 2 3-4",
        ),
        (
            Region::Diaspora,
            (2018, 4, 14),
            "1 2 3 4 5 6 1 2 3 4 5 6 1 2 - 3 4 5 6 1-2 3-4 5-6",
        ),
        (
            Region::Israel,
            (2018, 4, 7),
            "1 2 3 4 5 6 1 2 3 4 5 6 1 2 3 - 4 5 6 1 2 3-4 5-6",
        ),
    ];
    for (region, (year, month, day), chapters) in summers {
        let mut shabbos = date(year, month, day);
        for expected in chapters.split(' ') {
            let actual = PirkeiAvos(region).on(shabbos).map(|unit| match unit {
                PirkeiAvosUnit::Single(chapter) => chapter.to_string(),
                PirkeiAvosUnit::Combined(first, second) => format!("{first}-{second}"),
            });
            let expected = (expected != "-").then(|| expected.to_string());
            assert_eq!(actual, expected, "{shabbos} ({region:?})");
            shabbos = shabbos.checked_add(jiff::Span::new().days(7)).unwrap();
        }
    }
}

#[test]
fn each_work_has_its_own_order() {
    assert_eq!(MishnaTractate::ALL.len(), 63);
    assert_eq!(BavliTractate::ALL.len(), 40);
    assert_eq!(YerushalmiTractate::ALL.len(), 39);
    assert!(MishnaTractate::ALL.is_sorted());
    assert!(BavliTractate::ALL.is_sorted());
    assert!(YerushalmiTractate::ALL.is_sorted());
    // Kinnim is after Tamid and Midos in the Mishnah and before them in the Bavli.
    assert!(MishnaTractate::Midos < MishnaTractate::Kinnim);
    assert!(BavliTractate::Kinnim < BavliTractate::Tamid);
    // Shekalim follows Pesachim in the Bavli and Taanis in the Yerushalmi.
    assert!(BavliTractate::Shekalim < BavliTractate::Yoma);
    assert!(YerushalmiTractate::Taanis < YerushalmiTractate::Shekalim);
    assert!(YerushalmiTractate::Yoma < YerushalmiTractate::Shekalim);
}

#[test]
fn talmud_tractates_share_the_mishnahs_names() {
    for tractate in BavliTractate::ALL {
        assert_eq!(tractate.name(), MishnaTractate::from(tractate).name());
        assert_eq!(BavliTractate::from_name(tractate.name()), Some(tractate));
    }
    for tractate in YerushalmiTractate::ALL {
        assert_eq!(tractate.hebrew(), tractate.mishna().hebrew());
        assert_eq!(YerushalmiTractate::from_name(tractate.name()), Some(tractate));
    }
    assert_eq!(YerushalmiTractate::BavaMetzia.to_string(), "Bava Metzia");
    assert_eq!(BavliTractate::from_name("Peah"), None);
}

#[test]
fn units_sort_in_their_works_order() {
    let daf = |tractate, daf| Daf { tractate, daf };
    let mut dafim = [
        daf(BavliTractate::Tamid, 26),
        daf(BavliTractate::Kinnim, 24),
        daf(BavliTractate::Kinnim, 23),
    ];
    dafim.sort();
    assert_eq!(
        dafim,
        [
            daf(BavliTractate::Kinnim, 23),
            daf(BavliTractate::Kinnim, 24),
            daf(BavliTractate::Tamid, 26)
        ]
    );
    // Each day of a cycle is later than the one before, across tractates too.
    let mut day = date(2024, 1, 1);
    while day < date(2024, 12, 31) {
        let tomorrow = day.tomorrow().unwrap();
        assert!(DafYomiBavli.on(day) < DafYomiBavli.on(tomorrow), "{day}");
        assert!(MishnaYomis.on(day) < MishnaYomis.on(tomorrow), "{day}");
        if let (Some(today), Some(next)) = (DafYomiYerushalmi.on(day), DafYomiYerushalmi.on(tomorrow)) {
            assert!(today < next, "{day}");
        }
        day = tomorrow;
    }
    let mishna = |chapter, mishna| Mishna {
        tractate: MishnaTractate::Nazir,
        chapter,
        mishna,
    };
    assert!(mishna(5, 6) < mishna(5, 10));
    assert!(mishna(5, 10) < mishna(6, 1));
}
