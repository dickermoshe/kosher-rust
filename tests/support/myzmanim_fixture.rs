//! The fixture format both MyZmanim conformance tests use, and the comparison they share.
//!
//! A fixture is four CSV files (`places.csv`, `heights.csv`, `offsets.csv`, `days.csv`) as a
//! [`SiteData`] value, postcard-encoded and deflated. Times, heights and offsets are delta-coded
//! against the previous row of the same place, so a year of data stays small.
//!
//! - `places.csv`: place, latitude, longitude, standard UTC offset in hours, candle-lighting
//!   minutes, then the name in quotes.
//! - `heights.csv`: place, date, observer, eastern horizon, western horizon, in meters.
//! - `offsets.csv`: place, date, the UTC offset in seconds that the day's times are in.
//! - `days.csv`: place, date, then one local wall-clock time per column, empty where MyZmanim
//!   shows none.

#![allow(dead_code, clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::{
    collections::HashMap,
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
use serde::{Deserialize, Serialize};

/// A time MyZmanim did not show.
const MISSING: i32 = i32::MIN;

/// The fixture. Rows are sorted by place, then date.
#[derive(Serialize, Deserialize)]
struct SiteData {
    /// The `days.csv` column names, in the order of each row's `times`.
    columns: Vec<String>,
    places: Vec<Place>,
    rows: Vec<Row>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Place {
    pub id: String,
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    /// The standard UTC offset in seconds.
    pub offset_seconds: i32,
    /// MyZmanim's candle-lighting minutes for the place, in tenths of a minute.
    pub candle_tenths: i32,
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
    /// The UTC offset of the row's times in seconds.
    offset_seconds: i32,
    /// Local wall-clock seconds after the row's midnight, one per column.
    times: Vec<i32>,
}

/// The running absolute values of one place while delta-coding: date, heights, offset, times.
type Running = (i32, [i32; 3], i32, Vec<i32>);

/// One `days.csv` row before delta-coding: place, date, heights, offset, times.
type ParsedRow = (u16, i32, [i32; 3], i32, Vec<i32>);

/// A decoded place-day.
pub struct PlaceDay {
    pub place: Place,
    pub date: Date,
    pub heights: [f64; 3],
    /// The UTC offset MyZmanim's times are in that day.
    pub offset: Offset,
    /// `(column, local wall-clock time)` for the times MyZmanim showed.
    pub times: Vec<(&'static str, DateTime)>,
}

impl PlaceDay {
    /// The place, with the day's observer and horizon heights.
    pub fn location(&self) -> Location {
        Location::new(self.place.latitude, self.place.longitude)
            .and_then(|l| l.with_elevation(self.heights[0]))
            .and_then(|l| l.with_horizons(self.heights[1], self.heights[2]))
            .unwrap()
    }

    /// The place's candle-lighting minutes as a duration.
    pub fn candle_lighting(&self) -> std::time::Duration {
        std::time::Duration::from_secs(u64::from(self.place.candle_tenths.unsigned_abs()) * 6)
    }

    /// The local wall-clock time MyZmanim showed in `column` that day.
    pub fn time(&self, column: &str) -> Option<String> {
        self.times
            .iter()
            .find(|(c, _)| *c == column)
            .map(|(_, t)| t.to_string())
    }
}

fn epoch() -> Date {
    jiff::civil::date(1970, 1, 1)
}

/// Every place-day in `bytes`. `columns` lists the column names the caller compares; other
/// columns are dropped.
pub fn decode(bytes: &[u8], columns: &[&'static str]) -> Vec<PlaceDay> {
    let mut raw = Vec::new();
    DeflateDecoder::new(bytes).read_to_end(&mut raw).unwrap();
    let data: SiteData = postcard::from_bytes(&raw).unwrap();
    let names: Vec<Option<&'static str>> = data
        .columns
        .iter()
        .map(|name| columns.iter().find(|column| *column == name).copied())
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
            if let Some(column) = names[index] {
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

fn read_csv(dir: &Path, name: &str) -> Vec<String> {
    let path = dir.join(name);
    fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("{}: {err}", path.display()))
        .lines()
        .filter(|line| !line.is_empty())
        .map(str::to_owned)
        .collect()
}

/// The fixture bytes for the four CSV files in `dir`.
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

/// Rebuilds `tests/fixtures/<file>` from the CSV files in the directory the environment
/// variable `dir_variable` names, and reports its size. `columns` is used only for the report.
pub fn write_fixture(dir_variable: &str, file: &str, columns: &[&'static str]) {
    let dir = PathBuf::from(env::var_os(dir_variable).unwrap_or_else(|| panic!("{dir_variable}")));
    let bytes = encode(&dir);
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join(file);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(&path, &bytes).unwrap();
    let days = decode(&bytes, columns);
    eprintln!(
        "wrote {} ({} bytes, {} place-days, {} times)",
        path.display(),
        bytes.len(),
        days.len(),
        days.iter().map(|day| day.times.len()).sum::<usize>()
    );
}

/// One compared time.
pub struct Diff {
    pub place: String,
    pub date: Date,
    pub column: &'static str,
    /// Ours minus MyZmanim's, in seconds, after any whole hours explained by [`Diffs::am_pm`].
    pub seconds: f64,
}

/// The comparison of every time: the differences, and the times that could not be explained.
pub struct Diffs {
    pub diffs: Vec<Diff>,
    /// Times exactly 12 hours off, which look like AM/PM mistakes in scraped data.
    pub am_pm: usize,
    /// Times off by any other whole number of hours.
    pub far_off: Vec<String>,
}

/// Compares every time in `days` with `ours`, which gives our time for a place-day and column,
/// or `None` where we give no time.
///
/// Each time is read in the UTC offset MyZmanim used that day. A time may differ from ours by
/// exactly 12 hours, an AM/PM mistake in scraped data. Any other whole-hour difference is a real
/// error and lands in `far_off`.
pub fn compare(days: &[PlaceDay], ours: impl Fn(&PlaceDay, &TimeZone, &str) -> Option<Timestamp>) -> Diffs {
    let mut diffs = Vec::new();
    let mut am_pm = 0;
    let mut far_off = Vec::new();
    for day in days {
        let timezone = TimeZone::fixed(day.offset);
        for (column, theirs) in &day.times {
            let Some(ours) = ours(day, &timezone, column) else {
                continue;
            };
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
                seconds,
            });
        }
    }
    Diffs { diffs, am_pm, far_off }
}

/// The share of differences within 1 s and within 2 s, the mean, and the worst difference.
pub struct Summary<'a> {
    pub within_1: f64,
    pub within_2: f64,
    pub mean: f64,
    pub worst: &'a Diff,
}

/// Summarizes `diffs` and prints one line about them.
pub fn summarize(diffs: &[Diff]) -> Summary<'_> {
    let total = diffs.len() as f64;
    let within = |limit: f64| diffs.iter().filter(|d| d.seconds.abs() <= limit).count() as f64 / total;
    let worst = diffs
        .iter()
        .max_by(|a, b| a.seconds.abs().total_cmp(&b.seconds.abs()))
        .unwrap();
    let summary = Summary {
        within_1: within(1.0),
        within_2: within(2.0),
        mean: diffs.iter().map(|d| d.seconds).sum::<f64>() / total,
        worst,
    };
    eprintln!(
        "{} comparisons: {:.3}% within 1 s, {:.3}% within 2 s, mean {:+.3} s, worst {:+.1} s ({} {} {})",
        diffs.len(),
        summary.within_1 * 100.0,
        summary.within_2 * 100.0,
        summary.mean,
        worst.seconds,
        worst.place,
        worst.date,
        worst.column
    );
    summary
}

/// The mean difference of each column in `columns`, for columns with any times.
pub fn column_means<'a>(diffs: &[Diff], columns: impl IntoIterator<Item = &'a str>) -> Vec<(&'a str, f64, usize)> {
    columns
        .into_iter()
        .filter_map(|column| {
            let values: Vec<f64> = diffs.iter().filter(|d| d.column == column).map(|d| d.seconds).collect();
            (!values.is_empty()).then(|| (column, values.iter().sum::<f64>() / values.len() as f64, values.len()))
        })
        .collect()
}
