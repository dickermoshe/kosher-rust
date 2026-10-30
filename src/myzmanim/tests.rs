#![allow(clippy::unwrap_used, clippy::panic)]

use jiff::{
    SignedDuration, Timestamp,
    civil::Date,
    tz::{Offset, TimeZone},
};

use super::*;

fn ts(iso: &str) -> Timestamp {
    iso.parse().unwrap()
}

/// Anvers, Belgium (place 26244298 in the site data): flat, sea level, UTC+1.
fn anvers(date: Date) -> MyZmanim {
    let tz = TimeZone::fixed(Offset::from_hours(1).unwrap());
    let location = Location::new(51.216667, 4.416667).unwrap();
    MyZmanim::new(location, date, Some(tz))
}

/// Safed, Israel (place 27491341): 570 m up, eastern horizon level with the observer, western at sea level, UTC+2.
fn safed(date: Date) -> MyZmanim {
    let tz = TimeZone::fixed(Offset::from_hours(2).unwrap());
    let location = Location::new(32.96464803, 35.49599712)
        .and_then(|l| l.with_elevation(570.0))
        .and_then(|l| l.with_horizons(570.0, 0.0))
        .unwrap();
    MyZmanim::new(location, date, Some(tz)).candle_lighting(core::time::Duration::from_mins(30))
}

fn assert_site_second(calendar: &MyZmanim, zman: MyZman, site_local: &str, offset_hours: i8) {
    let expected = ts(&format!("{site_local}+0{offset_hours}:00"));
    let actual = calendar.get(zman).unwrap();
    let diff = actual.duration_since(expected).abs();
    assert!(
        diff <= SignedDuration::from_millis(1000),
        "{zman:?}: site {expected} ours {actual} (diff {diff:?})"
    );
}

#[test]
fn anvers_matches_site_rows() {
    // Row 26244298,2027-03-01 of the site data (a weekday, so no candle-lighting row). Every
    // row is within a second.
    let calendar = anvers(Date::new(2027, 3, 1).unwrap());
    for (zman, site) in [
        (MyZman::Dawn90Degrees, "2027-03-01T05:26:17"),
        (MyZman::Dawn72Fixed, "2027-03-01T06:15:49"),
        (MyZman::Dawn72Degrees, "2027-03-01T05:49:57"),
        (MyZman::Talis11Point5Degrees, "2027-03-01T06:19:24"),
        (MyZman::Talis11Degrees, "2027-03-01T06:22:36"),
        (MyZman::Talis10Point2Degrees, "2027-03-01T06:27:42"),
        (MyZman::SunriseLevel, "2027-03-01T07:27:49"),
        (MyZman::SunriseElevated, "2027-03-01T07:27:49"),
        (MyZman::ShemaMA90Degrees, "2027-03-01T09:10:47"),
        (MyZman::ShemaMA72Fixed, "2027-03-01T09:35:28"),
        (MyZman::ShemaMA72Degrees, "2027-03-01T09:22:35"),
        (MyZman::ShemaGra, "2027-03-01T10:11:28"),
        (MyZman::ShacharisMA90Degrees, "2027-03-01T10:25:37"),
        (MyZman::ShacharisMA72Fixed, "2027-03-01T10:42:01"),
        (MyZman::ShacharisMA72Degrees, "2027-03-01T10:33:28"),
        (MyZman::ShacharisGra, "2027-03-01T11:06:01"),
        (MyZman::Midday, "2027-03-01T12:55:07"),
        (MyZman::Mincha30Fixed, "2027-03-01T13:25:07"),
        (MyZman::MinchaGra, "2027-03-01T13:22:24"),
        (MyZman::MinchaLechumra, "2027-03-01T13:25:07"),
        (MyZman::MinchaMA72Fixed, "2027-03-01T13:28:24"),
        (MyZman::KetanaGra, "2027-03-01T16:06:03"),
        (MyZman::KetanaMA72Fixed, "2027-03-01T16:48:03"),
        (MyZman::PlagGra, "2027-03-01T17:14:14"),
        (MyZman::PlagMA72Fixed, "2027-03-01T18:11:14"),
        (MyZman::SunsetLevel, "2027-03-01T18:22:26"),
        (MyZman::SunsetElevated, "2027-03-01T18:22:26"),
        (MyZman::NightGra18Minutes, "2027-03-01T18:40:41"),
        (MyZman::NightGra22Point5Minutes, "2027-03-01T18:45:20"),
        (MyZman::NightGra24Minutes, "2027-03-01T18:46:52"),
        (MyZman::NightZalman, "2027-03-01T18:55:29"),
        (MyZman::NightFastTukaccinsky, "2027-03-01T18:58:41"),
        (MyZman::NightFastFeinstein, "2027-03-01T19:02:59"),
        (MyZman::NightMoed, "2027-03-01T19:08:36"),
        (MyZman::NightShabbos, "2027-03-01T19:11:48"),
        (MyZman::NightChazonIsh, "2027-03-01T19:16:48"),
        (MyZman::NightRTam72Fixed, "2027-03-01T19:34:26"),
        (MyZman::NightRTam72Degrees, "2027-03-01T20:00:32"),
        (MyZman::Midnight, "2027-03-02T00:54:04"),
    ] {
        assert_site_second(&calendar, zman, site, 1);
    }
}

#[test]
fn safed_hill_sunset_uses_western_horizon_and_dip() {
    let calendar = safed(Date::new(2026, 11, 6).unwrap());
    let level = calendar.get(MyZman::SunsetLevel).unwrap();
    let elevated = calendar.get(MyZman::SunsetElevated).unwrap();
    // 570 m above a sea-level western horizon: the dip (0.0347°·√570 ≈ 0.83°) sets later.
    let later_by = elevated.duration_since(level);
    assert!(
        later_by > SignedDuration::from_mins(3) && later_by < SignedDuration::from_mins(6),
        "{later_by:?}"
    );
    // The eastern horizon is level with the observer, so elevated sunrise is the level time.
    assert_eq!(
        calendar.get(MyZman::SunriseLevel).unwrap(),
        calendar.get(MyZman::SunriseElevated).unwrap()
    );
    // Candle lighting (30 minutes here) and R' Tam count from the elevated sunset.
    assert_eq!(
        calendar.get(MyZman::CandleLighting).unwrap(),
        elevated - SignedDuration::from_mins(30)
    );
    assert_eq!(
        calendar.get(MyZman::NightRTam72Fixed).unwrap(),
        elevated + SignedDuration::from_mins(72)
    );
}

#[test]
fn every_row_is_distinct_and_computes() {
    let calendar = safed(Date::new(2026, 6, 21).unwrap());
    let mut seen = std::collections::HashSet::new();
    for zman in MyZman::ALL {
        assert!(seen.insert(zman), "{zman:?} listed twice");
        assert!(!zman.name().is_empty());
        let time = calendar.get(zman).unwrap_or_else(|err| panic!("{zman:?}: {err}"));
        assert_eq!(zman.to_string(), zman.name());
        let _ = time;
    }
    assert_eq!(seen.len(), 40);
    assert!(calendar.proportional_hour_gra().unwrap() > SignedDuration::from_mins(60));
}

#[test]
fn polar_day_reports_errors_not_panics() {
    let tz = TimeZone::get("Europe/Oslo").unwrap();
    let location = Location::new(69.6492, 18.9553).unwrap();
    let calendar = MyZmanim::new(location, Date::new(2017, 6, 21).unwrap(), Some(tz));
    assert_eq!(calendar.get(MyZman::SunriseLevel), Err(ZmanimError::AllDay));
    assert_eq!(calendar.get(MyZman::Midday), Err(ZmanimError::AllDay));
    assert_eq!(calendar.get(MyZman::Midnight), Err(ZmanimError::AllDay));
    assert!(calendar.get(MyZman::Dawn90Degrees).is_err());
    assert!(calendar.proportional_hour_gra().is_err());
}

#[test]
fn candle_lighting_minutes_default_and_override() {
    let calendar = anvers(Date::new(2026, 11, 6).unwrap());
    assert_eq!(calendar.candle_lighting_offset(), DEFAULT_CANDLE_LIGHTING);
    let sunset = calendar.get(MyZman::SunsetElevated).unwrap();
    assert_eq!(
        calendar.get(MyZman::CandleLighting).unwrap(),
        sunset - SignedDuration::from_mins(18)
    );
    let calendar = calendar.candle_lighting(core::time::Duration::from_secs(40 * 60 + 30));
    assert_eq!(
        calendar.candle_lighting_offset(),
        core::time::Duration::from_secs(40 * 60 + 30)
    );
    assert_eq!(
        calendar.get(MyZman::CandleLighting).unwrap(),
        sunset - SignedDuration::from_secs(40 * 60 + 30)
    );
    let next_day = MyZmanim::new(*calendar.location(), Date::new(2026, 11, 7).unwrap(), None);
    assert_eq!(next_day.date(), Date::new(2026, 11, 7).unwrap());
    assert_ne!(next_day.get(MyZman::SunsetElevated).unwrap(), sunset);
}

#[test]
fn refraction_and_dip_follow_the_measured_rules() {
    assert!((refraction(0.0) - 33.66 / 60.0).abs() < 1e-12);
    assert!(refraction(1000.0) < refraction(0.0));
    assert!(
        (refraction(1000.0) / refraction(0.0) - 0.8870).abs() < 1e-3,
        "{}",
        refraction(1000.0)
    );
    assert_eq!(dip(0.0), 0.0);
    assert_eq!(dip(-5.0), 0.0);
    assert!((dip(100.0) - 0.347).abs() < 1e-12);
}

#[test]
fn website_arithmetic_cuts_to_whole_seconds() {
    let start = ts("2026-11-06T06:43:41Z");
    let end = ts("2026-11-06T16:07:33Z");
    // Day = 33_832 s; a twelfth is 2_819.333… s, integer-divided in nanos then scaled, so
    // three hours land 8_457.999… s in and are cut back to 8_457 s.
    let three_hours = proportional(start, end, 3.0).unwrap();
    assert_eq!(three_hours, ts("2026-11-06T09:04:38Z"));
    assert_eq!(three_hours.subsec_nanosecond(), 0);
    let mid = midpoint(start, end).unwrap();
    assert_eq!(mid, ts("2026-11-06T11:25:37Z"));
    assert_eq!(
        floor_second(ts("1969-12-31T23:59:59.9Z")).unwrap(),
        ts("1969-12-31T23:59:59Z")
    );
}

/// On the first and last days a `Timestamp` holds, with any candle-lighting minutes, every row
/// is a time or an error.
#[test]
fn range_edges_and_wild_candle_lighting_report_errors_not_panics() {
    let dates = [
        Date::new(-9999, 1, 1).unwrap(),
        Date::new(-9999, 1, 2).unwrap(),
        Date::new(2026, 11, 6).unwrap(),
        Date::new(9999, 12, 30).unwrap(),
        Date::new(9999, 12, 31).unwrap(),
    ];
    for date in dates {
        for longitude in [-179.0, -55.0, 0.0, 55.0, 179.0] {
            let zone = TimeZone::fixed(Offset::from_hours((longitude / 15.0) as i8).unwrap());
            for timezone in [None, Some(zone)] {
                let Ok(location) = Location::new(0.0, longitude) else {
                    continue;
                };
                for offset in [core::time::Duration::from_mins(18), core::time::Duration::MAX] {
                    let times = MyZmanim::new(location, date, timezone.clone()).candle_lighting(offset);
                    for zman in MyZman::ALL {
                        let _ = times.get(zman);
                    }
                }
            }
        }
    }
}

/// `get_zoned` is `get` in the *zmanim*'s time zone, and needs one.
#[test]
fn get_zoned_uses_the_time_zone() {
    let date = Date::new(2026, 11, 6).unwrap();
    let times = anvers(date);
    let candles = times.get_zoned(MyZman::CandleLighting).unwrap();
    assert_eq!(candles.timestamp(), times.get(MyZman::CandleLighting).unwrap());
    assert_eq!(candles.offset(), Offset::from_hours(1).unwrap());
    let plain = MyZmanim::new(Location::new(51.216667, 4.416667).unwrap(), date, None);
    assert_eq!(
        plain.get_zoned(MyZman::CandleLighting),
        Err(ZmanimError::TimeZoneRequired)
    );
}

/// Without a time zone, every row works except more than 150° from Greenwich.
#[test]
fn what_needs_a_time_zone() {
    let date = Date::new(2026, 11, 6).unwrap();
    let plain = MyZmanim::new(Location::new(51.216667, 4.416667).unwrap(), date, None);
    for zman in MyZman::ALL {
        assert_ne!(plain.get(zman), Err(ZmanimError::TimeZoneRequired), "{zman:?}");
    }
    let far = MyZmanim::new(Location::new(0.0, 150.1).unwrap(), date, None);
    for zman in MyZman::ALL {
        assert_eq!(far.get(zman), Err(ZmanimError::TimeZoneRequired), "{zman:?}");
    }
}

#[test]
fn horizons_above_the_air_have_no_refraction() {
    let location = Location::new(31.778, 35.235)
        .unwrap()
        .with_elevation(50_000.0)
        .unwrap()
        .with_horizons(50_000.0, 50_000.0)
        .unwrap();
    let times = MyZmanim::new(location, Date::new(2026, 11, 6).unwrap(), None);
    for zman in MyZman::ALL {
        assert_ne!(times.get(zman), Err(ZmanimError::Internal), "{zman:?}");
    }
}

#[test]
fn names_parse_back() {
    for zman in MyZman::ALL {
        assert_eq!(MyZman::from_name(zman.name()), Some(zman));
    }
    assert_eq!(MyZman::from_name(""), None);
}

#[test]
fn candle_lighting_a_day_or_more_before_sunset_is_invalid() {
    let calendar = anvers(Date::new(2026, 11, 6).unwrap());
    let day = core::time::Duration::from_hours(24);
    assert_eq!(
        calendar.clone().candle_lighting(day).get(MyZman::CandleLighting),
        Err(ZmanimError::InvalidOffset)
    );
    let almost = day - core::time::Duration::from_secs(1);
    assert!(calendar.candle_lighting(almost).get(MyZman::CandleLighting).is_ok());
}

/// An eastern horizon above the observer has no dip, and thinner air bends the light less, so
/// the elevated sunrise comes a little after the level one.
#[test]
fn a_horizon_above_the_observer_delays_the_elevated_sunrise() {
    let location = Location::new(31.778, 35.235)
        .and_then(|l| l.with_elevation(754.0))
        .and_then(|l| l.with_horizons(900.0, 754.0))
        .unwrap();
    let times = MyZmanim::new(location, Date::new(2026, 11, 6).unwrap(), None);
    let later_by = times
        .get(MyZman::SunriseElevated)
        .unwrap()
        .duration_since(times.get(MyZman::SunriseLevel).unwrap());
    assert!(
        later_by > SignedDuration::ZERO && later_by < SignedDuration::from_secs(10),
        "{later_by:?}"
    );
    assert_eq!(times.get(MyZman::SunsetElevated), times.get(MyZman::SunsetLevel));
}

#[test]
fn all_lists_every_row_once() {
    for (index, zman) in MyZman::ALL.into_iter().enumerate() {
        assert!(!MyZman::ALL[..index].contains(&zman), "{zman:?}");
    }
    assert_eq!(MyZman::ALL.first(), Some(&MyZman::Dawn90Degrees));
    assert_eq!(MyZman::ALL.last(), Some(&MyZman::Midnight));
}
