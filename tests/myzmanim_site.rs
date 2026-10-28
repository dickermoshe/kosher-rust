//! Checks [`kosher_rust::myzmanim`] against MyZmanim's website.
//!
//! `tests/fixtures/myzmanim-site.bin` holds the whole data set gathered for the KosherJava
//! `MyZmanim`: 284 places over a year, every row of the site's "All zmanim" table with
//! the observer and horizon heights the site used that day, the UTC offset its times are in that
//! day, and each place's coordinates, standard UTC offset and candle-lighting minutes. It is the
//! site's four CSV files (`places.csv`, `heights.csv`, `offsets.csv`, `days.csv`) as a
//! [`SiteData`] value, postcard-encoded and deflated; times, heights and offsets are delta-coded
//! against the previous row of the same place so the whole year fits in well under a megabyte.
//! Rebuild it from the CSV files with
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
//! difference fails. The heights were checked against the web app's for every place-day.
//! Set `MYZMANIM_SITE_DIFFS` to the Java comparison's `diffs.csv` to also check every row against
//! the Java port's own difference.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::{
    collections::{HashMap, HashSet},
    env, fs,
    io::{Read, Write},
    path::{Path, PathBuf},
};

use flate2::{Compression, read::DeflateDecoder, write::DeflateEncoder};
use jiff::{
    SignedDuration, Timestamp,
    civil::{Date, DateTime},
    tz::{Offset, TimeZone},
};
use kosher_rust::Location;
use kosher_rust::myzmanim::{MyZman, MyZmanim};
use serde::{Deserialize, Serialize};

const FIXTURE: &[u8] = include_bytes!("fixtures/myzmanim-site.bin");

/// A time the site did not show.
const MISSING: i32 = i32::MIN;

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

// ---------------------------------------------------------------------------------------
// The fixture's schema
// ---------------------------------------------------------------------------------------

/// The site data. Rows are sorted by place, then date.
#[derive(Serialize, Deserialize)]
struct SiteData {
    /// The `days.csv` column names, in the order of each row's `times`.
    columns: Vec<String>,
    places: Vec<Place>,
    rows: Vec<Row>,
}

#[derive(Serialize, Deserialize, Clone)]
struct Place {
    id: String,
    name: String,
    latitude: f64,
    longitude: f64,
    /// The standard UTC offset in seconds.
    offset_seconds: i32,
    /// MyZmanim's candle-lighting minutes for the place, in tenths of a minute.
    candle_tenths: i32,
}

/// One place-day. Every value is a delta against the previous row of the same place (the
/// first row of a place is absolute), except that a [`MISSING`] time is stored as is and does
/// not update the running value.
#[derive(Serialize, Deserialize)]
struct Row {
    place: u16,
    /// Days since the Unix epoch.
    date_days: i32,
    /// Observer, eastern horizon and western horizon heights in decimeters.
    heights_dm: [i32; 3],
    /// The UTC offset of the row's times in seconds: the standard offset, plus an hour on the
    /// days MyZmanim keeps daylight-saving time.
    offset_seconds: i32,
    /// Local wall-clock seconds after the row's midnight, one per column.
    times: Vec<i32>,
}

/// The running absolute values of one place while delta-coding: date, heights, offset, times.
type Running = (i32, [i32; 3], i32, Vec<i32>);

/// One `days.csv` row before delta-coding: place, date, heights, offset, times.
type ParsedRow = (u16, i32, [i32; 3], i32, Vec<i32>);

/// A decoded place-day.
struct PlaceDay {
    place: Place,
    date: Date,
    heights: [f64; 3],
    /// The UTC offset the site's times are in that day.
    offset: Offset,
    /// `(column, local wall-clock time)` for the times the site showed.
    times: Vec<(&'static str, DateTime)>,
}

fn epoch() -> Date {
    jiff::civil::date(1970, 1, 1)
}

fn decode(bytes: &[u8]) -> Vec<PlaceDay> {
    let mut raw = Vec::new();
    DeflateDecoder::new(bytes).read_to_end(&mut raw).unwrap();
    let data: SiteData = postcard::from_bytes(&raw).unwrap();
    let columns: Vec<Option<&'static str>> = data
        .columns
        .iter()
        .map(|name| {
            COLUMNS
                .iter()
                .find(|(column, _)| column == name)
                .map(|(column, _)| *column)
        })
        .collect();

    let mut days = Vec::with_capacity(data.rows.len());
    let mut running: HashMap<u16, Running> = HashMap::new();
    for row in &data.rows {
        let state = running
            .entry(row.place)
            .or_insert_with(|| (0, [0; 3], 0, vec![0; row.times.len()]));
        state.0 += row.date_days;
        for (height, delta) in state.1.iter_mut().zip(row.heights_dm) {
            *height += delta;
        }
        state.2 += row.offset_seconds;
        let date = epoch()
            .checked_add(SignedDuration::from_hours(24 * i64::from(state.0)))
            .unwrap();
        let mut times = Vec::new();
        for (index, delta) in row.times.iter().enumerate() {
            if *delta == MISSING {
                continue;
            }
            state.3[index] += delta;
            if let Some(column) = columns[index] {
                let time = date
                    .at(0, 0, 0, 0)
                    .checked_add(SignedDuration::from_secs(i64::from(state.3[index])))
                    .unwrap();
                times.push((column, time));
            }
        }
        days.push(PlaceDay {
            place: data.places[usize::from(row.place)].clone(),
            date,
            heights: state.1.map(|dm| f64::from(dm) / 10.0),
            offset: Offset::from_seconds(state.2).unwrap(),
            times,
        });
    }
    days
}

// ---------------------------------------------------------------------------------------
// Encoding the fixture from the site's CSV files
// ---------------------------------------------------------------------------------------

fn read_csv(dir: &Path, name: &str) -> Vec<String> {
    let path = dir.join(name);
    fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("{}: {err}", path.display()))
        .lines()
        .filter(|line| !line.is_empty())
        .map(str::to_owned)
        .collect()
}

fn encode(dir: &Path) -> Vec<u8> {
    let places: Vec<Place> = read_csv(dir, "places.csv")
        .iter()
        .skip(1)
        .map(|line| {
            let mut fields = line.splitn(6, ',');
            let id = fields.next().unwrap().to_owned();
            let latitude = fields.next().unwrap().parse().unwrap();
            let longitude = fields.next().unwrap().parse().unwrap();
            let offset_hours: f64 = fields.next().unwrap().parse().unwrap();
            let candle_minutes: f64 = fields.next().unwrap().parse().unwrap();
            let name = fields.next().unwrap_or("").trim_matches('"').to_owned();
            Place {
                id,
                name,
                latitude,
                longitude,
                offset_seconds: (offset_hours * 3600.0).round() as i32,
                candle_tenths: (candle_minutes * 10.0).round() as i32,
            }
        })
        .collect();
    let place_index: HashMap<&str, u16> = places
        .iter()
        .enumerate()
        .map(|(index, place)| (place.id.as_str(), u16::try_from(index).unwrap()))
        .collect();

    let heights: HashMap<(String, String), [i32; 3]> = read_csv(dir, "heights.csv")
        .iter()
        .skip(1)
        .map(|line| {
            let fields: Vec<&str> = line.split(',').collect();
            let decimeters = |index: usize| {
                let meters: f64 = fields[index].parse().unwrap();
                let dm = (meters * 10.0).round() as i32;
                assert!(
                    (f64::from(dm) / 10.0 - meters).abs() < 1e-9,
                    "{line}: height is not whole decimeters"
                );
                dm
            };
            (
                (fields[0].to_owned(), fields[1].to_owned()),
                [decimeters(2), decimeters(3), decimeters(4)],
            )
        })
        .collect();

    let offsets: HashMap<(String, String), i32> = read_csv(dir, "offsets.csv")
        .iter()
        .skip(1)
        .map(|line| {
            let fields: Vec<&str> = line.split(',').collect();
            ((fields[0].to_owned(), fields[1].to_owned()), fields[2].parse().unwrap())
        })
        .collect();

    let days = read_csv(dir, "days.csv");
    let columns: Vec<String> = days[0].split(',').skip(2).map(str::to_owned).collect();
    let mut parsed: Vec<ParsedRow> = days
        .iter()
        .skip(1)
        .map(|line| {
            let cells: Vec<&str> = line.split(',').collect();
            let place = place_index[cells[0]];
            let date: Date = cells[1].parse().unwrap();
            let date_days = epoch().until(date).unwrap().get_days();
            let key = (cells[0].to_owned(), cells[1].to_owned());
            let heights = heights[&key];
            let offset = *offsets
                .get(&key)
                .unwrap_or_else(|| panic!("{} {}: no offset", cells[0], cells[1]));
            let midnight = date.at(0, 0, 0, 0);
            let times = cells[2..]
                .iter()
                .map(|cell| {
                    if cell.is_empty() {
                        MISSING
                    } else {
                        let time: DateTime = cell.parse().unwrap();
                        i32::try_from(time.duration_since(midnight).as_secs()).unwrap()
                    }
                })
                .collect();
            (place, date_days, heights, offset, times)
        })
        .collect();
    parsed.sort_by_key(|(place, date_days, ..)| (*place, *date_days));

    let mut rows = Vec::with_capacity(parsed.len());
    let mut running: HashMap<u16, Running> = HashMap::new();
    for (place, date_days, heights, offset, times) in parsed {
        let state = running
            .entry(place)
            .or_insert_with(|| (0, [0; 3], 0, vec![0; times.len()]));
        let row = Row {
            place,
            date_days: date_days - state.0,
            heights_dm: [
                heights[0] - state.1[0],
                heights[1] - state.1[1],
                heights[2] - state.1[2],
            ],
            offset_seconds: offset - state.2,
            times: times
                .iter()
                .zip(state.3.iter_mut())
                .map(|(&time, last)| {
                    if time == MISSING {
                        MISSING
                    } else {
                        let delta = time - *last;
                        *last = time;
                        delta
                    }
                })
                .collect(),
        };
        state.0 = date_days;
        state.1 = heights;
        state.2 = offset;
        rows.push(row);
    }

    let data = SiteData { columns, places, rows };
    let raw = postcard::to_allocvec(&data).unwrap();
    let mut encoder = DeflateEncoder::new(Vec::new(), Compression::best());
    encoder.write_all(&raw).unwrap();
    encoder.finish().unwrap()
}

/// Rebuilds `tests/fixtures/myzmanim-site.bin` from the CSV files in `MYZMANIM_SITE_DIR`.
#[test]
#[ignore = "rebuilds the fixture; needs MYZMANIM_SITE_DIR"]
fn encode_fixture() {
    let dir = PathBuf::from(env::var_os("MYZMANIM_SITE_DIR").expect("MYZMANIM_SITE_DIR"));
    let bytes = encode(&dir);
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("myzmanim-site.bin");
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(&path, &bytes).unwrap();
    let days = decode(&bytes);
    eprintln!(
        "wrote {} ({} bytes, {} place-days, {} times)",
        path.display(),
        bytes.len(),
        days.len(),
        days.iter().map(|day| day.times.len()).sum::<usize>()
    );
}

// ---------------------------------------------------------------------------------------
// The comparison
// ---------------------------------------------------------------------------------------

/// Cells the website shows exactly 12 hours off.
const AM_PM_CELLS: usize = 10_261;

struct Diff {
    place: String,
    date: Date,
    column: &'static str,
    seconds: f64,
}

/// The comparison of every cell: the differences, and the cells that could not be explained.
struct SiteDiffs {
    /// Ours minus the website's, in seconds, after the whole hours explained below.
    diffs: Vec<Diff>,
    /// Cells the website shows 12 hours off, which look like AM/PM mistakes in the scraped data.
    am_pm: usize,
    /// Cells off by any other whole number of hours.
    far_off: Vec<String>,
}

/// Every difference, ours minus the website's.
///
/// Each time is read in the UTC offset the site used that day. A cell may differ from ours by
/// exactly 12 hours, an AM/PM mistake in the scraped data. Any other whole-hour difference is a
/// real error and lands in `far_off`.
fn site_diffs() -> SiteDiffs {
    let mut diffs = Vec::new();
    let mut am_pm = 0;
    let mut far_off = Vec::new();
    for day in decode(FIXTURE) {
        let timezone = TimeZone::fixed(day.offset);
        let location = Location::new(day.place.latitude, day.place.longitude)
            .and_then(|l| l.with_elevation(day.heights[0]))
            .and_then(|l| l.with_horizons(day.heights[1], day.heights[2]))
            .unwrap();
        let calendar = MyZmanim::new(location, day.date, Some(timezone.clone())).candle_lighting(
            std::time::Duration::from_secs(u64::from(day.place.candle_tenths.unsigned_abs()) * 6),
        );

        for (column, theirs) in &day.times {
            let zman = COLUMNS.iter().find(|(name, _)| name == column).unwrap().1;
            let Ok(ours) = calendar.get(zman) else { continue };
            let theirs: Timestamp = theirs.to_zoned(timezone.clone()).unwrap().timestamp();
            let seconds = (ours.as_millisecond() - theirs.as_millisecond()) as f64 / 1000.0;
            let hours = (seconds / 3600.0).round() as i64;
            let seconds = seconds - hours as f64 * 3600.0;
            if hours.abs() == 12 {
                am_pm += 1;
            } else if hours != 0 {
                far_off.push(format!(
                    "{} {} {column}: {hours} h {seconds:+.1} s off",
                    day.place.id, day.date
                ));
                continue;
            }
            diffs.push(Diff {
                place: day.place.id.clone(),
                date: day.date,
                column,
                // The Java comparison's tenth of a second.
                seconds: (seconds * 10.0).round() / 10.0,
            });
        }
    }
    SiteDiffs { diffs, am_pm, far_off }
}

#[test]
fn fixture_decodes_to_the_whole_data_set() {
    let days = decode(FIXTURE);
    assert_eq!(days.len(), 24_140);
    assert_eq!(
        days.iter()
            .map(|day| day.place.id.clone())
            .collect::<HashSet<_>>()
            .len(),
        284
    );
    // Anvers (Antwerp), 2026-11-06, the first row of the site's days.csv.
    let anvers = days
        .iter()
        .find(|day| day.place.id == "26244298" && day.date == jiff::civil::date(2026, 11, 6))
        .unwrap();
    assert_eq!(anvers.place.name, "Anvers, Belgium");
    assert_eq!(anvers.heights, [0.0, 0.0, 0.0]);
    let time = |column: &str| {
        anvers
            .times
            .iter()
            .find(|(c, _)| *c == column)
            .map(|(_, t)| t.to_string())
    };
    assert_eq!(time("sunriseLevel").as_deref(), Some("2026-11-06T07:43:40"));
    assert_eq!(time("midnight").as_deref(), Some("2026-11-07T00:26:29"));
    assert_eq!(time("candles").as_deref(), Some("2026-11-06T16:49:33"));
    assert_eq!(time("sunsetElevated"), None);
    // Safed, a hill town: 570 m up, eastern horizon level with the observer.
    let safed = days.iter().find(|day| day.place.id == "27491341").unwrap();
    assert_eq!(safed.heights, [570.0, 570.0, 0.0]);
    assert_eq!(safed.place.candle_tenths, 300);
    assert_eq!(anvers.offset, Offset::from_seconds(3600).unwrap());

    // Each day's times are in the offset the site used that day. Edmonton's times were scraped
    // before MyZmanim moved it to permanent standard time, so they still switch to daylight time
    // in March.
    let offset_hours = |id: &str, date: Date| {
        days.iter()
            .find(|day| day.place.id == id && day.date == date)
            .map(|day| day.offset.seconds() / 3600)
            .unwrap()
    };
    assert_eq!(offset_hours("CAT5J4Y8", jiff::civil::date(2027, 3, 5)), -7);
    assert_eq!(offset_hours("CAT5J4Y8", jiff::civil::date(2027, 3, 26)), -6);
    assert_eq!(offset_hours("27526341", jiff::civil::date(2027, 3, 26)), 3);
}

#[test]
fn reproduces_the_website_to_the_second() {
    let SiteDiffs { diffs, am_pm, far_off } = site_diffs();
    assert!(
        far_off.is_empty(),
        "{} cells off by whole hours:\n{}",
        far_off.len(),
        far_off.iter().take(40).cloned().collect::<Vec<_>>().join("\n")
    );
    assert_eq!(diffs.len(), 944_603, "every comparison the Java port makes");
    // Pinned so that a real 12-hour error can't hide among the website's AM/PM mistakes.
    assert_eq!(am_pm, AM_PM_CELLS, "cells 12 hours off");

    let total = diffs.len() as f64;
    let within = |limit: f64| diffs.iter().filter(|d| d.seconds.abs() <= limit).count() as f64 / total;
    let worst = diffs
        .iter()
        .max_by(|a, b| a.seconds.abs().total_cmp(&b.seconds.abs()))
        .unwrap();
    let mean = diffs.iter().map(|d| d.seconds).sum::<f64>() / total;

    eprintln!(
        "{} comparisons: {:.2}% within 1 s, {:.2}% within 2 s, mean {mean:+.3} s, worst {:+.1} s ({} {} {})",
        diffs.len(),
        within(1.0) * 100.0,
        within(2.0) * 100.0,
        worst.seconds,
        worst.place,
        worst.date,
        worst.column
    );

    // The Java port measures 92.2% within 1 s, 99.8% within 2 s and a worst case of 9.9 s. The
    // worst case here is 13.6 s: misheyakir at 11.5° on a day the sun only just reaches that angle
    // (75245600, 2027-04-29), so it crosses the angle slowly and the time is harder to pin down.
    assert!(within(1.0) >= 0.92, "{:.3}", within(1.0));
    assert!(within(2.0) >= 0.997, "{:.4}", within(2.0));
    assert!(
        worst.seconds.abs() <= 15.0,
        "{} {} {}: {:+.1} s",
        worst.place,
        worst.date,
        worst.column,
        worst.seconds
    );
    assert!(mean.abs() < 0.5, "{mean}");

    // No single zman should be systematically off. The one known bias is the elevated sunset
    // from Israel's hill towns, which still sets 2–3 s early (about +1.1 s on average).
    for (column, _) in COLUMNS {
        let values: Vec<f64> = diffs
            .iter()
            .filter(|d| d.column == *column)
            .map(|d| d.seconds)
            .collect();
        if values.is_empty() {
            continue;
        }
        let column_mean = values.iter().sum::<f64>() / values.len() as f64;
        assert!(
            column_mean.abs() <= 1.5,
            "{column}: mean {column_mean:+.2} s over {} rows",
            values.len()
        );
    }
}

#[test]
fn matches_the_java_port_row_for_row() {
    let Some(diffs_path) = env::var_os("MYZMANIM_SITE_DIFFS") else {
        eprintln!("MYZMANIM_SITE_DIFFS not set; skipping the row-for-row check against the Java port");
        return;
    };
    let java: HashMap<(String, String, String), f64> = fs::read_to_string(diffs_path)
        .unwrap()
        .lines()
        .skip(1)
        .filter(|line| !line.is_empty())
        .map(|line| {
            let fields: Vec<&str> = line.split(',').collect();
            (
                (fields[0].to_owned(), fields[1].to_owned(), fields[2].to_owned()),
                fields[4].parse().unwrap(),
            )
        })
        .collect();

    let diffs = site_diffs().diffs;
    let mut compared = 0usize;
    let mut mismatches = Vec::new();
    for diff in &diffs {
        let key = (diff.place.clone(), diff.date.to_string(), diff.column.to_owned());
        let Some(&java_seconds) = java.get(&key) else { continue };
        compared += 1;
        // Java's diffs are printed to a tenth of a second.
        if (diff.seconds - java_seconds).abs() > 0.15 {
            mismatches.push(format!(
                "{} {} {}: rust {:+.2} java {:+.1}",
                key.0, key.1, key.2, diff.seconds, java_seconds
            ));
        }
    }
    assert_eq!(compared, diffs.len(), "every row should have a Java counterpart");
    eprintln!("{compared} rows compared with the Java port");
    assert!(
        mismatches.is_empty(),
        "{} rows differ from the Java port:\n{}",
        mismatches.len(),
        mismatches.iter().take(40).cloned().collect::<Vec<_>>().join("\n")
    );
}
