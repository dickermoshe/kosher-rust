//! Checks [`kosher_rust::myzmanim`], set to match the web app, against MyZmanim's web app.
//!
//! `tests/fixtures/myzmanim-app.bin` holds every place MyZmanim's place search marks popular:
//! 746 places in 75 countries, each on every day of a year, the 368 days from 2026-11-09 to
//! 2027-11-11. For every
//! day it has each time the web app's reply gave for the rows [`MyZman`] covers, candle lighting
//! at seven offsets as well as at the place's own, the observer and horizon heights the web app
//! used that day, and the UTC offset its times are in. The fixture's format is described in
//! `support/myzmanim_fixture.rs`. The CSV files come from the saved web app replies, two
//! 184-day requests per place, starting 2026-11-09 and 2027-05-12. Rebuild the fixture from them
//! with
//!
//! ```text
//! MYZMANIM_APP_DIR=<dir with the csv files> cargo test --test myzmanim_app -- --ignored encode_fixture
//! ```
//!
//! A horizon the web app has not surveyed is level with the observer, as the web app calculates
//! it, though its reply says 0 m. The offsets are the web app's own: the place's offset, plus an
//! hour on the days it says are daylight-saving time. They are MyZmanim's, not the tz
//! database's, because this test checks the calculation and not the time-zone rules. The one
//! exception is Melilla (84069548). Every time the web app shows for it is an hour behind the
//! offset its record states, UTC+0 in winter and UTC+1 in summer, so its times are read an hour
//! behind.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

#[path = "support/myzmanim_fixture.rs"]
mod fixture;

use std::{collections::HashSet, time::Duration};

use jiff::{civil::Date, tz::Offset};
use kosher_rust::myzmanim::{MyZman, MyZmanim, Source};

use fixture::{Diffs, PlaceDay};

const FIXTURE: &[u8] = include_bytes!("fixtures/myzmanim-app.bin");

/// Each web app column, the row it is, and for candle lighting the minutes before sunset when
/// the column fixes them rather than using the place's own.
const COLUMNS: &[(&str, MyZman, Option<u64>)] = &[
    ("dawn90eq", MyZman::Dawn90Degrees, None),
    ("dawn72fix", MyZman::Dawn72Fixed, None),
    ("dawn72eq", MyZman::Dawn72Degrees, None),
    ("yakir115", MyZman::Talis11Point5Degrees, None),
    ("yakir110", MyZman::Talis11Degrees, None),
    ("yakir102", MyZman::Talis10Point2Degrees, None),
    ("sunriseLevel", MyZman::SunriseLevel, None),
    ("sunriseDefault", MyZman::SunriseElevated, None),
    ("shemaMA90eq", MyZman::ShemaMA90Degrees, None),
    ("shemaMA72fix", MyZman::ShemaMA72Fixed, None),
    ("shemaMA72eq", MyZman::ShemaMA72Degrees, None),
    ("shemaGra", MyZman::ShemaGra, None),
    ("shachrisMA90eq", MyZman::ShacharisMA90Degrees, None),
    ("shachrisMA72fix", MyZman::ShacharisMA72Fixed, None),
    ("shachrisMA72eq", MyZman::ShacharisMA72Degrees, None),
    ("shachrisGra", MyZman::ShacharisGra, None),
    ("midday", MyZman::Midday, None),
    ("mincha30fix", MyZman::Mincha30Fixed, None),
    ("minchaGra", MyZman::MinchaGra, None),
    ("minchaStrict", MyZman::MinchaLechumra, None),
    ("minchaMA72fix", MyZman::MinchaMA72Fixed, None),
    ("ketanaGra", MyZman::KetanaGra, None),
    ("ketanaMA72fix", MyZman::KetanaMA72Fixed, None),
    ("plagGra", MyZman::PlagGra, None),
    ("plagMA72fix", MyZman::PlagMA72Fixed, None),
    ("candles", MyZman::CandleLighting, None),
    ("candles10", MyZman::CandleLighting, Some(10)),
    ("candles15", MyZman::CandleLighting, Some(15)),
    ("candles18", MyZman::CandleLighting, Some(18)),
    ("candles20", MyZman::CandleLighting, Some(20)),
    ("candles22", MyZman::CandleLighting, Some(22)),
    ("candles30", MyZman::CandleLighting, Some(30)),
    ("candles40", MyZman::CandleLighting, Some(40)),
    ("sunsetLevel", MyZman::SunsetLevel, None),
    ("sunsetElevated", MyZman::SunsetElevated, None),
    ("sunsetDefault", MyZman::SunsetElevated, None),
    ("nightGra180eq", MyZman::NightGra18Minutes, None),
    ("nightGra225eq", MyZman::NightGra22Point5Minutes, None),
    ("nightGra240eq", MyZman::NightGra24Minutes, None),
    ("nightZalman", MyZman::NightZalman, None),
    ("nightFastTuc", MyZman::NightFastTukaccinsky, None),
    ("nightFastRMF", MyZman::NightFastFeinstein, None),
    ("nightMoed", MyZman::NightMoed, None),
    ("nightShabbos", MyZman::NightShabbos, None),
    ("nightChazonIsh", MyZman::NightChazonIsh, None),
    ("night72fix", MyZman::NightRTam72Fixed, None),
    ("night72eq", MyZman::NightRTam72Degrees, None),
    ("midnight", MyZman::Midnight, None),
];

fn column_names() -> Vec<&'static str> {
    COLUMNS.iter().map(|(name, ..)| *name).collect()
}

fn decode() -> Vec<PlaceDay> {
    fixture::decode(FIXTURE, &column_names())
}

/// Rebuilds `tests/fixtures/myzmanim-app.bin` from the CSV files in `MYZMANIM_APP_DIR`.
#[test]
#[ignore = "rebuilds the fixture; needs MYZMANIM_APP_DIR"]
fn encode_fixture() {
    fixture::write_fixture("MYZMANIM_APP_DIR", "myzmanim-app.bin", &column_names());
}

/// Every difference, ours set to match `source` minus the web app's.
fn diffs(days: &[PlaceDay], source: Source) -> Diffs {
    fixture::compare(days, |day, timezone, column| {
        let (_, zman, minutes) = COLUMNS.iter().find(|(name, ..)| *name == column).unwrap();
        let candles = minutes.map_or_else(|| day.candle_lighting(), |m| Duration::from_secs(m * 60));
        MyZmanim::new(day.location(), day.date, Some(timezone.clone()))
            .candle_lighting(candles)
            .matching(source)
            .get(*zman)
            .ok()
    })
}

/// Every difference, ours set to match the web app minus the web app's.
fn app_diffs() -> Diffs {
    diffs(&decode(), Source::WebApp)
}

#[test]
fn fixture_decodes_to_the_whole_data_set() {
    let days = decode();
    let places: HashSet<_> = days.iter().map(|day| day.place.id.clone()).collect();
    assert_eq!(places.len(), 746);
    assert_eq!(days.len(), 746 * 368);
    let first = days.iter().map(|day| day.date).min().unwrap();
    let last = days.iter().map(|day| day.date).max().unwrap();
    assert_eq!(
        (first, last),
        (jiff::civil::date(2026, 11, 9), jiff::civil::date(2027, 11, 11))
    );

    let day = |id: &str, date: Date| days.iter().find(|day| day.place.id == id && day.date == date).unwrap();
    // Jerusalem, 2026-11-09: 650 m up, the eastern horizon not surveyed and so level with the
    // observer, the western at sea level.
    let jerusalem = day("27526341", jiff::civil::date(2026, 11, 9));
    assert_eq!(jerusalem.place.name, "Jerusalem, Israel");
    assert_eq!(jerusalem.heights, [650.0, 650.0, 0.0]);
    assert_eq!(jerusalem.place.candle_tenths, 400);
    assert_eq!(jerusalem.offset, Offset::from_seconds(2 * 3600).unwrap());
    assert_eq!(jerusalem.time("dawn72eq").as_deref(), Some("2026-11-09T04:47:49"));
    assert_eq!(jerusalem.time("sunsetElevated").as_deref(), Some("2026-11-09T16:47:54"));
    // In winter the western horizon is 100 m.
    assert_eq!(
        day("27526341", jiff::civil::date(2026, 12, 10)).heights,
        [650.0, 650.0, 100.0]
    );
    // Lakewood is flat and shows no elevated sunset. Daylight time starts there on 2027-03-14.
    let lakewood = day("69581767", jiff::civil::date(2027, 3, 13));
    assert_eq!(lakewood.heights, [0.0, 0.0, 0.0]);
    assert_eq!(lakewood.time("sunsetElevated"), None);
    assert_eq!(lakewood.offset.seconds() / 3600, -5);
    assert_eq!(
        day("69581767", jiff::civil::date(2027, 3, 14)).offset.seconds() / 3600,
        -4
    );
    // Melilla's times are an hour behind its record's offset, in winter and in summer.
    assert_eq!(day("84069548", jiff::civil::date(2026, 11, 9)).offset.seconds(), 0);
    assert_eq!(day("84069548", jiff::civil::date(2027, 4, 9)).offset.seconds(), 3600);
}

#[test]
fn reproduces_the_web_app_to_the_second() {
    let Diffs { diffs, am_pm, far_off } = app_diffs();
    assert!(
        far_off.is_empty(),
        "{} times off by whole hours:\n{}",
        far_off.len(),
        far_off.iter().take(40).cloned().collect::<Vec<_>>().join("\n")
    );
    // The web app's times come from its own reply, not scraped pages, so none is 12 hours off.
    assert_eq!(am_pm, 0);
    assert_eq!(diffs.len(), 10_981_402, "every time the web app showed");

    let summary = fixture::summarize(&diffs);
    let worst = summary.worst;
    // This measures 99.991% within 1 s and 99.998% within 2 s. Only 45 times are more than 5 s
    // off. All of them are rows measured in degrees, on the first or last nights of the year the
    // sun gets that far below the horizon, so it crosses the angle slowly and the time is hard to
    // pin down. Most are far north in spring and summer. The worst is 40 s: dawn at 19.860° in
    // Oslo (41135580, 2027-08-27).
    assert!(summary.within_1 >= 0.9995, "{:.5}", summary.within_1);
    assert!(summary.within_2 >= 0.9999, "{:.5}", summary.within_2);
    assert!(
        worst.seconds.abs() <= 45.0,
        "{} {} {}: {:+.1} s",
        worst.place,
        worst.date,
        worst.column,
        worst.seconds
    );
    assert!(summary.mean.abs() < 0.1, "{}", summary.mean);

    // No single zman should be systematically off.
    for (column, mean, count) in fixture::column_means(&diffs, column_names()) {
        assert!(mean.abs() <= 0.5, "{column}: mean {mean:+.2} s over {count} rows");
    }
}

/// Set to match the website, the times agree with the web app wherever the two calculate the
/// same way, and miss it where they don't.
#[test]
fn the_website_setting_does_not_match_the_web_app() {
    let site = diffs(&decode(), Source::LegacySite);
    let means = fixture::column_means(&site.diffs, column_names());
    let mean = |column: &str| means.iter().find(|(c, ..)| *c == column).unwrap().1;
    // The website's 16.1° dawn is earlier than the web app's 16.053°, and its 10.2° talis
    // earlier than the web app's 10.129°.
    assert!(mean("dawn72eq") < -10.0, "{}", mean("dawn72eq"));
    assert!(mean("yakir102") < -15.0, "{}", mean("yakir102"));
    // Sunset, which both calculate the same way, agrees.
    assert!(mean("sunsetLevel").abs() < 0.5, "{}", mean("sunsetLevel"));
}
