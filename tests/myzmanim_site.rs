//! Checks [`kosher_rust::myzmanim`], set to match the website, against MyZmanim's website.
//!
//! `tests/fixtures/myzmanim-site.bin` holds the zmanim device's 245 preset places on 135 days
//! from September 2026 to August 2027: every row of the site's "All zmanim" table with the
//! observer and horizon heights the site used that day, the UTC offset its times are in that
//! day, and each place's coordinates, standard UTC offset and candle-lighting minutes. Each
//! place is the MyZmanim location id the device stores. The website redirects ten of those ids
//! (Antwerp, Munich, Rome and other cities with an English name) to the local-language place they
//! are a name variant of, which the web app gives the same coordinates, heights and settings;
//! their pages were fetched from that place and filed under the device's id. The fixture's
//! format is described in `support/myzmanim_fixture.rs`. Rebuild it from the site's CSV files
//! with
//!
//! ```text
//! MYZMANIM_SITE_DIR=<dir with the csv files> cargo test --test myzmanim_site -- --ignored encode_fixture
//! ```
//!
//! The offsets come from MyZmanim's web app, which states for every day whether the place is on
//! daylight-saving time. They are MyZmanim's, not the tz database's: MyZmanim's time-zone rules are
//! out of date in places, and this test checks the calculation, not the time-zone rules, so each
//! time is read in the offset the site itself used. A time may then differ from ours by exactly
//! 12 hours, an AM/PM mistake in the scraped data, which is counted; any other whole-hour
//! difference fails. The heights are the ones the site's labels show. They match the web app's
//! on every place-day but 22: in winter the site draws Ramat Shlomo's elevated sunset against a
//! 100 m western horizon, where the web app says 0 m all year.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

#[path = "support/myzmanim_fixture.rs"]
mod fixture;

use std::collections::HashSet;

use jiff::{civil::Date, tz::Offset};
use kosher_rust::myzmanim::{MyZman, MyZmanim, Source};

use fixture::{Diffs, PlaceDay};

const FIXTURE: &[u8] = include_bytes!("fixtures/myzmanim-site.bin");

const COLUMNS: &[(&str, MyZman)] = &[
    ("siteDawn90", MyZman::Dawn90Degrees),
    ("dawn72fix", MyZman::Dawn72Fixed),
    ("siteDawn161", MyZman::Dawn72Degrees),
    ("siteYakir115", MyZman::Talis11Point5Degrees),
    ("siteYakir110", MyZman::Talis11Degrees),
    ("siteYakir102", MyZman::Talis10Point2Degrees),
    ("sunriseLevel", MyZman::SunriseLevel),
    ("sunriseElevated", MyZman::SunriseElevated),
    ("sunriseDefault", MyZman::SunriseElevated),
    ("siteShemaMA90", MyZman::ShemaMA90Degrees),
    ("shemaMA72fix", MyZman::ShemaMA72Fixed),
    ("siteShemaMA161", MyZman::ShemaMA72Degrees),
    ("shemaGra", MyZman::ShemaGra),
    ("siteShachrisMA90", MyZman::ShacharisMA90Degrees),
    ("shachrisMA72fix", MyZman::ShacharisMA72Fixed),
    ("siteShachrisMA161", MyZman::ShacharisMA72Degrees),
    ("shachrisGra", MyZman::ShacharisGra),
    ("midday", MyZman::Midday),
    ("mincha30fix", MyZman::Mincha30Fixed),
    ("minchaGra", MyZman::MinchaGra),
    ("minchaStrict", MyZman::MinchaLechumra),
    ("minchaMA72fix", MyZman::MinchaMA72Fixed),
    ("ketanaGra", MyZman::KetanaGra),
    ("ketanaMA72fix", MyZman::KetanaMA72Fixed),
    ("plagGra", MyZman::PlagGra),
    ("plagMA72fix", MyZman::PlagMA72Fixed),
    ("candles", MyZman::CandleLighting),
    ("sunsetLevel", MyZman::SunsetLevel),
    ("sunsetElevated", MyZman::SunsetElevated),
    ("sunsetDefault", MyZman::SunsetElevated),
    ("nightGra180eq", MyZman::NightGra18Minutes),
    ("nightGra225eq", MyZman::NightGra22Point5Minutes),
    ("nightGra240eq", MyZman::NightGra24Minutes),
    ("nightZalman", MyZman::NightZalman),
    ("nightFastTuc", MyZman::NightFastTukaccinsky),
    ("nightFastRMF", MyZman::NightFastFeinstein),
    ("nightMoed", MyZman::NightMoed),
    ("nightShabbos", MyZman::NightShabbos),
    ("nightChazonIsh", MyZman::NightChazonIsh),
    ("night72fix", MyZman::NightRTam72Fixed),
    ("siteNight161", MyZman::NightRTam72Degrees),
    ("midnight", MyZman::Midnight),
];

/// Cells the website shows exactly 12 hours off.
const AM_PM_CELLS: usize = 13_506;

fn column_names() -> Vec<&'static str> {
    COLUMNS.iter().map(|(name, _)| *name).collect()
}

fn decode() -> Vec<PlaceDay> {
    fixture::decode(FIXTURE, &column_names())
}

/// Rebuilds `tests/fixtures/myzmanim-site.bin` from the CSV files in `MYZMANIM_SITE_DIR`.
#[test]
#[ignore = "rebuilds the fixture; needs MYZMANIM_SITE_DIR"]
fn encode_fixture() {
    fixture::write_fixture("MYZMANIM_SITE_DIR", "myzmanim-site.bin", &column_names());
}

/// Every difference, ours minus the website's.
fn site_diffs() -> Diffs {
    fixture::compare(&decode(), |day, timezone, column| {
        let zman = COLUMNS.iter().find(|(name, _)| *name == column).unwrap().1;
        MyZmanim::new(day.location(), day.date, Some(timezone.clone()))
            .candle_lighting(day.candle_lighting())
            .matching(Source::LegacySite)
            .get(zman)
            .ok()
    })
}

#[test]
fn fixture_decodes_to_the_whole_data_set() {
    let days = decode();
    assert_eq!(days.len(), 33_075);
    assert_eq!(
        days.iter()
            .map(|day| day.place.id.clone())
            .collect::<HashSet<_>>()
            .len(),
        245
    );
    // Antwerp, 2026-11-06: one of the ids the website redirects (to Antwerpen, 39842001).
    let antwerp = days
        .iter()
        .find(|day| day.place.id == "39847741" && day.date == jiff::civil::date(2026, 11, 6))
        .unwrap();
    assert_eq!(antwerp.place.name, "Antwerp, Belgium");
    assert_eq!(antwerp.heights, [0.0, 0.0, 0.0]);
    assert_eq!(antwerp.time("sunriseLevel").as_deref(), Some("2026-11-06T07:43:40"));
    assert_eq!(antwerp.time("midnight").as_deref(), Some("2026-11-07T00:26:29"));
    assert_eq!(antwerp.time("candles").as_deref(), Some("2026-11-06T16:49:33"));
    assert_eq!(antwerp.time("sunsetElevated"), None);
    // Safed, a hill town: 570 m up, eastern horizon level with the observer.
    let safed = days.iter().find(|day| day.place.id == "27491341").unwrap();
    assert_eq!(safed.heights, [570.0, 570.0, 0.0]);
    assert_eq!(safed.place.candle_tenths, 300);
    assert_eq!(antwerp.offset, Offset::from_seconds(3600).unwrap());

    let day = |id: &str, date: Date| days.iter().find(|day| day.place.id == id && day.date == date).unwrap();
    // Ramat Shlomo's western horizon is 100 m in winter on the website only.
    assert_eq!(
        day("27470600", jiff::civil::date(2026, 11, 10)).heights,
        [650.0, 650.0, 0.0]
    );
    assert_eq!(
        day("27470600", jiff::civil::date(2026, 12, 10)).heights,
        [650.0, 650.0, 100.0]
    );

    // Each day's times are in the offset the site used that day: daylight time starts on
    // 2027-03-14 in Lakewood, 2027-03-26 in Har Nof and 2027-03-28 in Antwerp.
    let offset_hours = |id: &str, date: Date| day(id, date).offset.seconds() / 3600;
    assert_eq!(offset_hours("69581767", jiff::civil::date(2027, 3, 5)), -5);
    assert_eq!(offset_hours("69581767", jiff::civil::date(2027, 3, 26)), -4);
    assert_eq!(offset_hours("27470376", jiff::civil::date(2027, 3, 5)), 2);
    assert_eq!(offset_hours("27470376", jiff::civil::date(2027, 3, 26)), 3);
    assert_eq!(offset_hours("39847741", jiff::civil::date(2027, 3, 27)), 1);
    assert_eq!(offset_hours("39847741", jiff::civil::date(2027, 3, 28)), 2);
}

#[test]
fn reproduces_the_website_to_the_second() {
    let Diffs { diffs, am_pm, far_off } = site_diffs();
    assert!(
        far_off.is_empty(),
        "{} cells off by whole hours:\n{}",
        far_off.len(),
        far_off.iter().take(40).cloned().collect::<Vec<_>>().join("\n")
    );
    assert_eq!(diffs.len(), 1_295_195, "every time the site showed");
    // Pinned so that a real 12-hour error can't hide among the website's AM/PM mistakes.
    assert_eq!(am_pm, AM_PM_CELLS, "cells 12 hours off");

    let summary = fixture::summarize(&diffs);
    let worst = summary.worst;

    // This measures 99.89% within 1 s and 100.00% within 2 s. The worst case is 9.0 s: dawn at
    // 16.1° in Saint Petersburg (86290690, 2027-04-28), a night the sun only just gets that far
    // below the horizon, so it crosses the angle slowly and the time is harder to pin down.
    assert!(summary.within_1 >= 0.998, "{:.4}", summary.within_1);
    assert!(summary.within_2 >= 0.9999, "{:.5}", summary.within_2);
    assert!(
        worst.seconds.abs() <= 15.0,
        "{} {} {}: {:+.1} s",
        worst.place,
        worst.date,
        worst.column,
        worst.seconds
    );
    assert!(summary.mean.abs() < 0.1, "{}", summary.mean);

    // No single zman should be systematically off.
    for (column, mean, count) in fixture::column_means(&diffs, COLUMNS.iter().map(|(name, _)| *name)) {
        assert!(mean.abs() <= 1.0, "{column}: mean {mean:+.2} s over {count} rows");
    }
}
