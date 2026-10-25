//! Writes `test/fixtures/parity.json`: what the crate itself calculates for a set of places, dates
//! and time zones, using the real time zone database. The JavaScript tests check that the package,
//! with time zones from `Intl`, gives the same answers.
//!
//! Run `UPDATE_GENERATED=1 cargo test -p kr4js` after changing a calculation in the crate.

#![cfg(not(target_arch = "wasm32"))]
#![allow(clippy::expect_used, clippy::unwrap_used)]

use std::fmt::Write;
use std::path::Path;
use std::time::Duration;

use jiff::{SignedDuration, ToSpan, civil::Date, civil::date, tz::TimeZone};
use kosher_rust::limudim::{
    AmudYomiDirshu, DafHashavuaBavli, DafYomiBavli, DafYomiYerushalmi, MishnaYomis, PirkeiAvos, Schedule,
    TehillimMonthly,
};
use kosher_rust::myzmanim::{MyZman, MyZmanim};
use kosher_rust::zmanim::{AstronomicalCalculator, Config, Zman, Zmanim, ZmanimError};
use kosher_rust::{HebrewDate, Location, Region};

/// `text` as a JSON string.
fn json(text: &str) -> String {
    let mut out = String::from('"');
    for c in text.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            c if (c as u32) < 0x20 => write!(out, "\\u{:04x}", c as u32).unwrap(),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

/// `value` as a JSON string, or `null`.
fn json_or_null(value: Option<impl ToString>) -> String {
    value.map_or_else(|| "null".into(), |v| json(&v.to_string()))
}

/// The reason JavaScript gives for `error`.
fn reason(error: ZmanimError) -> &'static str {
    match error {
        ZmanimError::AllDay => "allDay",
        ZmanimError::AllNight => "allNight",
        ZmanimError::NotApplicable => "notApplicable",
        ZmanimError::DateOutOfRange(_) => "dateOutOfRange",
        ZmanimError::InvalidDegrees => "invalidDegrees",
        ZmanimError::TimeZoneRequired => "timeZoneRequired",
        ZmanimError::Internal => "internal",
    }
}

/// The times as a JSON array of Unix milliseconds or reasons.
fn times(results: impl Iterator<Item = Result<jiff::Timestamp, ZmanimError>>) -> String {
    let items: Vec<String> = results
        .map(|result| match result {
            Ok(time) => time.as_millisecond().to_string(),
            Err(error) => json(reason(error)),
        })
        .collect();
    format!("[{}]", items.join(","))
}

struct Place {
    name: &'static str,
    latitude: f64,
    longitude: f64,
    elevation: f64,
    horizons: (f64, f64),
    zone: &'static str,
}

const LAKEWOOD: Place = Place {
    name: "Lakewood",
    latitude: 40.0961,
    longitude: -74.2224,
    elevation: 30.0,
    horizons: (0.0, 0.0),
    zone: "America/New_York",
};

const JERUSALEM: Place = Place {
    name: "Jerusalem",
    latitude: 31.778,
    longitude: 35.235,
    elevation: 754.0,
    horizons: (754.0, 0.0),
    zone: "Asia/Jerusalem",
};

impl Place {
    fn location(&self) -> Location {
        Location::new(self.latitude, self.longitude)
            .unwrap()
            .with_elevation(self.elevation)
            .unwrap()
            .with_horizons(self.horizons.0, self.horizons.1)
            .unwrap()
    }

    fn json(&self) -> String {
        format!(
            r#"{{"latitude":{},"longitude":{},"elevation":{},"easternHorizon":{},"westernHorizon":{}}}"#,
            self.latitude, self.longitude, self.elevation, self.horizons.0, self.horizons.1
        )
    }
}

/// A config, and the same settings as the JavaScript options object.
fn configs() -> Vec<(Config, &'static str)> {
    vec![
        (Config::default(), "{}"),
        (
            Config {
                use_elevation: true,
                candle_lighting_offset: Duration::from_mins(40),
                astronomical_calculator: AstronomicalCalculator::Meeus { apply_delta_t: true },
                ..Config::default()
            },
            r#"{"useElevation":true,"candleLightingOffset":40,"calculator":"meeus"}"#,
        ),
        (
            Config {
                use_astronomical_chatzos: false,
                use_astronomical_chatzos_for_other_zmanim: true,
                ateret_torah_sunset_offset: SignedDuration::from_mins(-12),
                astronomical_calculator: AstronomicalCalculator::Spa { apply_delta_t: false },
                ..Config::default()
            },
            r#"{"useAstronomicalChatzos":false,"useAstronomicalChatzosForOtherZmanim":true,"ateretTorahSunsetOffset":-12,"calculator":"spa","applyDeltaT":false}"#,
        ),
    ]
}

fn zmanim_cases(out: &mut Vec<String>) {
    let places = [
        LAKEWOOD,
        JERUSALEM,
        Place {
            name: "London",
            latitude: 51.5074,
            longitude: -0.1278,
            elevation: 0.0,
            horizons: (0.0, 0.0),
            zone: "Europe/London",
        },
        Place {
            name: "Tromsø",
            latitude: 69.6492,
            longitude: 18.9553,
            elevation: 0.0,
            horizons: (0.0, 0.0),
            zone: "Europe/Oslo",
        },
        Place {
            name: "Apia",
            latitude: -13.8599,
            longitude: -171.7513,
            elevation: 0.0,
            horizons: (0.0, 0.0),
            zone: "Pacific/Apia",
        },
        Place {
            name: "Auckland",
            latitude: -36.8485,
            longitude: 174.7633,
            elevation: 0.0,
            horizons: (0.0, 0.0),
            zone: "Pacific/Auckland",
        },
        Place {
            name: "Sydney",
            latitude: -33.8688,
            longitude: 151.2093,
            elevation: 0.0,
            horizons: (0.0, 0.0),
            zone: "Australia/Sydney",
        },
        Place {
            name: "Kathmandu",
            latitude: 27.9881,
            longitude: 86.925,
            elevation: 8826.0,
            horizons: (0.0, 0.0),
            zone: "Asia/Kathmandu",
        },
        Place {
            name: "São Paulo",
            latitude: -23.5505,
            longitude: -46.6333,
            elevation: 760.0,
            horizons: (0.0, 0.0),
            zone: "America/Sao_Paulo",
        },
    ];
    let dates = [
        date(2024, 4, 23),
        date(2024, 3, 10),
        date(2024, 3, 29),
        date(2024, 3, 31),
        date(2024, 4, 7),
        date(2024, 6, 21),
        date(2024, 9, 29),
        date(2024, 11, 3),
        date(2024, 12, 21),
        date(2018, 11, 4),
        date(2011, 12, 29),
    ];
    for (place_index, place) in places.iter().enumerate() {
        let zone = TimeZone::get(place.zone).unwrap();
        for (date_index, &day) in dates.iter().enumerate() {
            let (config, config_json) = &configs()[(place_index + date_index) % 3];
            let zmanim = Zmanim::with_config(place.location(), day, Some(zone.clone()), *config);
            out.push(case(place, place.zone, day, config_json, &zmanim));
        }
    }

    // Every day of a Hebrew month in Lakewood, for the Kiddush Levana times.
    let zone = TimeZone::get(LAKEWOOD.zone).unwrap();
    let mut day = date(2024, 4, 8);
    while day <= date(2024, 5, 8) {
        let zmanim = Zmanim::new(LAKEWOOD.location(), day, Some(zone.clone()));
        out.push(case(&LAKEWOOD, LAKEWOOD.zone, day, "{}", &zmanim));
        day = day.tomorrow().unwrap();
    }

    // Without a time zone.
    let zmanim = Zmanim::new(LAKEWOOD.location(), date(2024, 4, 23), None);
    out.push(case(&LAKEWOOD, "", date(2024, 4, 23), "{}", &zmanim));
}

fn case(place: &Place, zone: &str, day: Date, config: &str, zmanim: &Zmanim) -> String {
    let zone = if zone.is_empty() { "null".into() } else { json(zone) };
    format!(
        r#"{{"name":{},"location":{},"timeZone":{zone},"date":"{day}","config":{config},"times":{},"sunriseOffsetBy11Degrees":{},"sunsetOffsetBy8Point5Degrees":{}}}"#,
        json(place.name),
        place.json(),
        times(Zman::ALL.into_iter().map(|zman| zmanim.get(zman))),
        times(std::iter::once(zmanim.sunrise_offset_by_degrees(11.0))),
        times(std::iter::once(zmanim.sunset_offset_by_degrees(8.5))),
    )
}

fn my_zmanim_cases(out: &mut Vec<String>) {
    let safed = Place {
        name: "Safed",
        latitude: 32.96464803,
        longitude: 35.49599712,
        elevation: 570.0,
        horizons: (570.0, 0.0),
        zone: "Asia/Jerusalem",
    };
    let places = [JERUSALEM, LAKEWOOD, safed];
    for place in &places {
        let zone = TimeZone::get(place.zone).unwrap();
        for day in [date(2026, 11, 6), date(2027, 3, 1), date(2024, 3, 29)] {
            let zmanim =
                MyZmanim::new(place.location(), day, Some(zone.clone())).candle_lighting(Duration::from_mins(30));
            out.push(format!(
                r#"{{"name":{},"location":{},"timeZone":{},"date":"{day}","candleLightingOffset":30,"times":{},"proportionalHourGra":{}}}"#,
                json(place.name),
                place.json(),
                json(place.zone),
                times(MyZman::ALL.into_iter().map(|zman| zmanim.get(zman))),
                zmanim.proportional_hour_gra().map_or(f64::NAN, |hour| hour.as_millis_f64()),
            ));
        }
    }
}

/// `value` as the JSON the package gives for it, such as `"bereshis"` or `{"id":"chanukah","day":3}`.
fn id(value: impl serde::Serialize) -> String {
    serde_json::to_string(&value).unwrap()
}

fn special_days(day: HebrewDate, region: Region) -> String {
    let ids: Vec<String> = day.special_days(region).map(id).collect();
    format!("[{}]", ids.join(","))
}

fn calendar_cases(out: &mut Vec<String>) {
    let mut dates: Vec<Date> = Vec::new();
    // Every day of 5784, a leap year, and every third day of 5785, a common one.
    let mut day = date(2023, 9, 16);
    while day < date(2024, 10, 3) {
        dates.push(day);
        day = day.tomorrow().unwrap();
    }
    while day < date(2025, 9, 23) {
        dates.push(day);
        day = day.checked_add(3.days()).unwrap();
    }
    dates.extend([
        date(1, 1, 1),
        date(1923, 9, 11),
        date(1947, 5, 20),
        date(1980, 2, 2),
        date(6239, 9, 1),
    ]);

    for day in dates {
        let hebrew = HebrewDate::try_from(day).unwrap();
        let parsha = |region| id(hebrew.parsha(region));
        let special_parsha = |region| id(hebrew.special_parsha(region));
        let next_parsha = |region| id(hebrew.next_parsha(region));
        let molad = hebrew.molad().unwrap();
        out.push(format!(
            concat!(
                r#"{{"date":"{}","hebrew":{},"dayOfYear":{},"weekday":{},"#,
                r#""specialDays":{{"israel":{},"diaspora":{}}},"#,
                r#""assur":[{},{}],"candles":[{},{}],"aseres":{},"#,
                r#""parsha":[{},{}],"specialParsha":[{},{}],"nextParsha":[{},{}],"#,
                r#""molad":[{},{},{},{},{}],"#,
                r#""dafYomiBavli":{},"dafHashavuaBavli":{},"dafYomiYerushalmi":{},"amudYomiDirshu":{},"#,
                r#""mishnaYomis":{},"tehillimMonthly":{},"pirkeiAvos":[{},{}]}}"#
            ),
            day,
            json(&hebrew.to_string()),
            hebrew.day_of_year(),
            hebrew.weekday().to_sunday_zero_offset(),
            special_days(hebrew, Region::Israel),
            special_days(hebrew, Region::Diaspora),
            hebrew.is_assur_bemelacha(Region::Israel),
            hebrew.is_assur_bemelacha(Region::Diaspora),
            hebrew.has_candle_lighting(Region::Israel),
            hebrew.has_candle_lighting(Region::Diaspora),
            hebrew.is_aseres_yemei_teshuva(),
            parsha(Region::Israel),
            parsha(Region::Diaspora),
            special_parsha(Region::Israel),
            special_parsha(Region::Diaspora),
            next_parsha(Region::Israel),
            next_parsha(Region::Diaspora),
            molad.timestamp().as_millisecond(),
            molad.weekday().to_sunday_zero_offset(),
            molad.hours(),
            molad.minutes(),
            molad.chalakim(),
            json_or_null(DafYomiBavli.on(day)),
            json_or_null(DafHashavuaBavli.on(day)),
            json_or_null(DafYomiYerushalmi.on(day)),
            json_or_null(AmudYomiDirshu.on(day)),
            json_or_null(MishnaYomis.on(day)),
            json_or_null(TehillimMonthly.on(day)),
            json_or_null(PirkeiAvos(Region::Israel).on(day)),
            json_or_null(PirkeiAvos(Region::Diaspora).on(day)),
        ));
    }
}

fn fixture() -> String {
    let (mut zmanim, mut my_zmanim, mut calendar) = (Vec::new(), Vec::new(), Vec::new());
    zmanim_cases(&mut zmanim);
    my_zmanim_cases(&mut my_zmanim);
    calendar_cases(&mut calendar);
    format!(
        "{{\n\"zmanim\":[\n{}\n],\n\"myZmanim\":[\n{}\n],\n\"calendar\":[\n{}\n]\n}}\n",
        zmanim.join(",\n"),
        my_zmanim.join(",\n").replace("NaN", "null"),
        calendar.join(",\n"),
    )
}

#[test]
fn generated_parity_fixture_is_current() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("test/fixtures/parity.json");
    let expected = fixture();
    if std::env::var_os("UPDATE_GENERATED").is_some() {
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(&path, &expected).expect("write test/fixtures/parity.json");
        return;
    }
    let actual = std::fs::read_to_string(&path).unwrap_or_default().replace("\r\n", "\n");
    assert!(
        actual == expected,
        "test/fixtures/parity.json is out of date. Run `UPDATE_GENERATED=1 cargo test -p kr4js`."
    );
}
