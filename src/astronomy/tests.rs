#![allow(clippy::unwrap_used, clippy::panic)]

use jiff::{civil::Date, tz::TimeZone};

use super::*;

const NOAA: AstronomicalCalculator = AstronomicalCalculator::Noaa;
const MEEUS: AstronomicalCalculator = AstronomicalCalculator::Meeus { apply_delta_t: true };
const MEEUS_NO_DELTA_T: AstronomicalCalculator = AstronomicalCalculator::Meeus { apply_delta_t: false };
const SPA: AstronomicalCalculator = AstronomicalCalculator::Spa { apply_delta_t: true };
const SPA_NO_DELTA_T: AstronomicalCalculator = AstronomicalCalculator::Spa { apply_delta_t: false };

/// KosherJava asserts its calculator unit tests to eight decimals of an hour (36 µs).
const JAVA_DELTA: f64 = 1e-8;

/// Mirrors KosherJava's `TestLocations` (latitude, longitude, elevation in meters).
#[derive(Clone, Copy)]
enum TestLocation {
    Nj,
    La,
    Jerusalem,
    Norway,
    Sydney,
    Macapa,
    Suva,
    Ushuaia,
}

impl TestLocation {
    fn location(self) -> Location {
        let (latitude, longitude, elevation) = match self {
            Self::Nj => (41.1181036, -74.0840691, 167.0),
            Self::La => (34.0201613, -118.6919095, 71.0),
            Self::Jerusalem => (31.7962994, 35.1053185, 754.0),
            Self::Norway => (70.1498248, 9.1456867, 0.0),
            Self::Sydney => (-33.8688, 151.2093, 58.0),
            Self::Macapa => (0.0349, -51.0694, 15.0),
            Self::Suva => (-18.1416, 178.4419, 6.0),
            Self::Ushuaia => (-54.8019, -68.3030, 23.0),
        };
        Location::new(latitude, longitude, Some(TimeZone::UTC))
            .and_then(|l| l.with_elevation(elevation))
            .unwrap()
    }
}

/// `(date, location, expected UTC hours)`; `None` means KosherJava returns `NaN`.
type HourCase = (&'static str, TestLocation, Option<f64>);

fn date(iso: &str) -> Date {
    iso.parse().unwrap()
}

fn assert_java_hours(label: &str, expected: Option<f64>, actual: Result<f64, ZmanimError>) {
    match (expected, actual) {
        (None, Err(_)) => {}
        (Some(expected), Ok(actual)) => assert!(
            (expected - actual).abs() <= JAVA_DELTA,
            "{label}: expected {expected} got {actual} (diff {})",
            (expected - actual).abs()
        ),
        (expected, actual) => panic!("{label}: expected {expected:?} got {actual:?}"),
    }
}

fn check_rise_set(calculator: AstronomicalCalculator, event: SolarEvent, cases: &[HourCase]) {
    for (iso, place, expected) in cases {
        let actual = utc_rise_set_hours(calculator, date(iso), &place.location(), GEOMETRIC_ZENITH, true, event);
        assert_java_hours(&format!("{calculator:?} {event:?} {iso}"), *expected, actual);
    }
}

fn check_noon_midnight(calculator: AstronomicalCalculator, event: SolarEvent, cases: &[HourCase]) {
    for (iso, place, expected) in cases {
        let actual = utc_noon_midnight_hours(calculator, date(iso), &place.location(), event);
        assert_java_hours(&format!("{calculator:?} {event:?} {iso}"), *expected, Ok(actual));
    }
}

fn check_azimuth(calculator: AstronomicalCalculator, cases: &[(&str, TestLocation, f64, Option<f64>)]) {
    for (iso, place, azimuth, expected) in cases {
        let actual = utc_time_at_azimuth_hours(calculator, date(iso), &place.location(), *azimuth);
        assert_java_hours(&format!("{calculator:?} azimuth {azimuth} {iso}"), *expected, actual);
    }
}

fn check_sunrise_at_zenith(calculator: AstronomicalCalculator, cases: &[(&str, TestLocation, f64, Option<f64>)]) {
    for (iso, place, zenith, expected) in cases {
        let actual = utc_rise_set_hours(
            calculator,
            date(iso),
            &place.location(),
            *zenith,
            true,
            SolarEvent::Sunrise,
        );
        assert_java_hours(&format!("{calculator:?} zenith {zenith} {iso}"), *expected, actual);
    }
}

fn check_sunrise_without_elevation(calculator: AstronomicalCalculator, cases: &[HourCase]) {
    for (iso, place, expected) in cases {
        let actual = utc_rise_set_hours(
            calculator,
            date(iso),
            &place.location(),
            GEOMETRIC_ZENITH,
            false,
            SolarEvent::Sunrise,
        );
        assert_java_hours(&format!("{calculator:?} no-elevation sunrise {iso}"), *expected, actual);
    }
}

// ---------------------------------------------------------------------------------------
// NOAACalculatorTest
// ---------------------------------------------------------------------------------------

#[test]
fn noaa_utc_sunrise_matches_java() {
    use TestLocation::*;
    check_rise_set(
        NOAA,
        SolarEvent::Sunrise,
        &[
            ("2017-10-17", Nj, Some(11.13540831)),
            ("2017-10-17", La, Some(14.00704301)),
            ("1955-02-26", Jerusalem, Some(4.11872483)),
            ("2017-06-21", Norway, None),
            ("2020-02-29", Sydney, Some(19.68246214)),
            ("2000-01-01", Macapa, Some(9.39008078)),
            ("2023-06-21", Suva, Some(18.60488873)),
            ("2017-06-21", Ushuaia, Some(12.95743787)),
            ("2100-07-04", Nj, Some(9.44903860)),
        ],
    );
}

#[test]
fn noaa_utc_sunset_matches_java() {
    use TestLocation::*;
    check_rise_set(
        NOAA,
        SolarEvent::Sunset,
        &[
            ("2017-10-17", Nj, Some(22.24081501)),
            ("2017-10-17", La, Some(1.31823827)),
            ("1955-02-26", Jerusalem, Some(15.64544001)),
            ("2017-06-21", Norway, None),
            ("2020-02-29", Sydney, Some(8.56373515)),
            ("2000-01-01", Macapa, Some(21.53146217)),
            ("2023-06-21", Suva, Some(5.65781259)),
            ("2017-06-21", Ushuaia, Some(20.21256620)),
            ("2100-07-04", Nj, Some(0.58065990)),
        ],
    );
}

#[test]
fn noaa_utc_noon_matches_java() {
    use TestLocation::*;
    check_noon_midnight(
        NOAA,
        SolarEvent::Noon,
        &[
            ("2017-10-17", Nj, Some(16.69315478)),
            ("2017-10-17", La, Some(19.66659743)),
            ("1955-02-26", Jerusalem, Some(9.87832210)),
            ("2017-06-21", Norway, Some(11.42090460)),
            ("2020-02-29", Sydney, Some(2.12779094)),
            ("2000-01-01", Macapa, Some(15.46078241)),
            ("2023-06-21", Suva, Some(0.13137581)),
            ("2017-06-21", Ushuaia, Some(16.58492880)),
            ("2100-07-04", Nj, Some(17.01703523)),
        ],
    );
}

#[test]
fn noaa_utc_midnight_matches_java() {
    use TestLocation::*;
    check_noon_midnight(
        NOAA,
        SolarEvent::Midnight,
        &[
            ("2017-10-17", Nj, Some(4.69150247)),
            ("2017-10-17", La, Some(7.66495567)),
            ("1955-02-26", Jerusalem, Some(21.87692914)),
            ("2017-06-21", Norway, Some(23.42271259)),
            ("2020-02-29", Sydney, Some(14.12619634)),
            ("2000-01-01", Macapa, Some(3.46470789)),
            ("2023-06-21", Suva, Some(12.13318970)),
            ("2017-06-21", Ushuaia, Some(4.58673539)),
            ("2100-07-04", Nj, Some(5.01848806)),
        ],
    );
}

#[test]
fn noaa_time_at_azimuth_matches_java() {
    use TestLocation::*;
    check_azimuth(
        NOAA,
        &[
            ("2017-10-17", Nj, 90.0, Some(9.96599151)),
            ("2017-10-17", Nj, 270.0, Some(23.43667951)),
            ("2017-10-17", La, 90.0, Some(12.71769344)),
            ("2017-10-17", La, 270.0, Some(2.63764230)),
            ("2020-02-29", Sydney, 90.0, Some(20.92605878)),
            ("2020-02-29", Sydney, 270.0, Some(7.34651923)),
            ("2000-01-01", Macapa, 90.0, None),
            ("2017-06-21", Norway, 90.0, Some(6.02028371)),
        ],
    );
}

#[test]
fn noaa_utc_sunrise_at_zenith_matches_java() {
    use TestLocation::*;
    check_sunrise_at_zenith(
        NOAA,
        &[
            ("2017-10-17", Nj, 96.0, Some(10.70891556)),
            ("2017-10-17", Nj, 102.0, Some(10.17562194)),
            ("2017-10-17", Nj, 108.0, Some(9.64406486)),
            ("2020-02-29", Sydney, 96.0, Some(19.27827196)),
        ],
    );
}

#[test]
fn noaa_utc_sunrise_without_elevation_matches_java() {
    use TestLocation::*;
    check_sunrise_without_elevation(
        NOAA,
        &[
            ("2017-10-17", Nj, Some(11.17291440)),
            ("1955-02-26", Jerusalem, Some(4.18884065)),
        ],
    );
}

// ---------------------------------------------------------------------------------------
// MeeusCalculatorTest
// ---------------------------------------------------------------------------------------

#[test]
fn meeus_utc_sunrise_matches_java() {
    use TestLocation::*;
    check_rise_set(
        MEEUS,
        SolarEvent::Sunrise,
        &[
            ("2017-10-17", Nj, Some(11.13565507)),
            ("2017-10-17", La, Some(14.00730639)),
            ("1955-02-26", Jerusalem, Some(4.11849856)),
            ("2017-06-21", Norway, None),
            ("2020-02-29", Sydney, Some(19.68266925)),
            ("2000-01-01", Macapa, Some(9.38974837)),
            ("2023-06-21", Suva, Some(18.60510228)),
            ("2017-06-21", Ushuaia, Some(12.95697826)),
            ("2100-07-04", Nj, Some(9.44873166)),
        ],
    );
}

#[test]
fn meeus_utc_sunset_matches_java() {
    use TestLocation::*;
    check_rise_set(
        MEEUS,
        SolarEvent::Sunset,
        &[
            ("2017-10-17", Nj, Some(22.24115497)),
            ("2017-10-17", La, Some(1.31857880)),
            ("1955-02-26", Jerusalem, Some(15.64527430)),
            ("2017-06-21", Norway, None),
            ("2020-02-29", Sydney, Some(8.56381339)),
            ("2000-01-01", Macapa, Some(21.53114700)),
            ("2023-06-21", Suva, Some(5.65803778)),
            ("2017-06-21", Ushuaia, Some(20.21223110)),
            ("2100-07-04", Nj, Some(0.58039250)),
        ],
    );
}

#[test]
fn meeus_utc_noon_matches_java() {
    use TestLocation::*;
    check_noon_midnight(
        MEEUS,
        SolarEvent::Noon,
        &[
            ("2017-10-17", Nj, Some(16.69344845)),
            ("2017-10-17", La, Some(19.66689969)),
            ("1955-02-26", Jerusalem, Some(9.87812628)),
            ("2017-06-21", Norway, Some(11.42050168)),
            ("2020-02-29", Sydney, Some(2.12793408)),
            ("2000-01-01", Macapa, Some(15.46045983)),
            ("2023-06-21", Suva, Some(0.13159547)),
            ("2017-06-21", Ushuaia, Some(16.58453220)),
            ("2100-07-04", Nj, Some(17.01674900)),
        ],
    );
}

#[test]
fn meeus_utc_midnight_matches_java() {
    use TestLocation::*;
    check_noon_midnight(
        MEEUS,
        SolarEvent::Midnight,
        &[
            ("2017-10-17", Nj, Some(4.69183111)),
            ("2017-10-17", La, Some(7.66529295)),
            ("1955-02-26", Jerusalem, Some(21.87675832)),
            ("2017-06-21", Norway, Some(23.42232487)),
            ("2020-02-29", Sydney, Some(14.12636235)),
            ("2000-01-01", Macapa, Some(3.46440271)),
            ("2023-06-21", Suva, Some(12.13342359)),
            ("2017-06-21", Ushuaia, Some(4.58635433)),
            ("2100-07-04", Nj, Some(5.01821298)),
        ],
    );
}

#[test]
fn meeus_time_at_azimuth_matches_java() {
    use TestLocation::*;
    check_azimuth(
        MEEUS,
        &[
            ("2017-10-17", Nj, 90.0, Some(9.96630242)),
            ("2017-10-17", Nj, 270.0, Some(23.43696025)),
            ("2017-10-17", La, 90.0, Some(12.71802243)),
            ("2017-10-17", La, 270.0, Some(2.63792382)),
            ("2020-02-29", Sydney, 90.0, Some(20.92601400)),
            ("2020-02-29", Sydney, 270.0, Some(7.34685529)),
            ("2000-01-01", Macapa, 90.0, None),
            ("2017-06-21", Norway, 90.0, Some(6.01986159)),
        ],
    );
}

#[test]
fn meeus_utc_sunrise_at_zenith_matches_java() {
    use TestLocation::*;
    check_sunrise_at_zenith(
        MEEUS,
        &[
            ("2017-10-17", Nj, 96.0, Some(10.70916170)),
            ("2017-10-17", Nj, 102.0, Some(10.17586674)),
            ("2017-10-17", Nj, 108.0, Some(9.64430764)),
            ("2020-02-29", Sydney, 96.0, Some(19.27848095)),
        ],
    );
}

#[test]
fn meeus_utc_sunrise_without_elevation_matches_java() {
    use TestLocation::*;
    check_sunrise_without_elevation(
        MEEUS,
        &[
            ("2017-10-17", Nj, Some(11.17316120)),
            ("1955-02-26", Jerusalem, Some(4.18861444)),
        ],
    );
}

#[test]
fn meeus_utc_sunrise_with_delta_t_disabled_matches_java() {
    use TestLocation::*;
    check_rise_set(
        MEEUS_NO_DELTA_T,
        SolarEvent::Sunrise,
        &[
            ("2017-10-17", Nj, Some(11.13563996)),
            ("1955-02-26", Jerusalem, Some(4.11850671)),
        ],
    );
}

// ---------------------------------------------------------------------------------------
// SPACalculatorTest
// ---------------------------------------------------------------------------------------

#[test]
fn spa_utc_sunrise_matches_java() {
    use TestLocation::*;
    check_rise_set(
        SPA,
        SolarEvent::Sunrise,
        &[
            ("2017-10-17", Nj, Some(11.13593689)),
            ("2017-10-17", La, Some(14.00756754)),
            ("1955-02-26", Jerusalem, Some(4.11873932)),
            ("2017-06-21", Norway, None),
            ("2020-02-29", Sydney, Some(19.68293143)),
            ("2000-01-01", Macapa, Some(9.38998685)),
            ("2023-06-21", Suva, Some(18.60534965)),
            ("2017-06-21", Ushuaia, Some(12.95741199)),
            ("2100-07-04", Nj, Some(9.44912615)),
        ],
    );
}

#[test]
fn spa_utc_sunset_matches_java() {
    use TestLocation::*;
    check_rise_set(
        SPA,
        SolarEvent::Sunset,
        &[
            ("2017-10-17", Nj, Some(22.24099415)),
            ("2017-10-17", La, Some(1.31843857)),
            ("1955-02-26", Jerusalem, Some(15.64512300)),
            ("2017-06-21", Norway, None),
            ("2020-02-29", Sydney, Some(8.56367402)),
            ("2000-01-01", Macapa, Some(21.53102552)),
            ("2023-06-21", Suva, Some(5.65791427)),
            ("2017-06-21", Ushuaia, Some(20.21191830)),
            ("2100-07-04", Nj, Some(0.58028674)),
        ],
    );
}

#[test]
fn spa_utc_noon_matches_java() {
    use TestLocation::*;
    check_noon_midnight(
        SPA,
        SolarEvent::Noon,
        &[
            ("2017-10-17", Nj, Some(16.69350885)),
            ("2017-10-17", La, Some(19.66696009)),
            ("1955-02-26", Jerusalem, Some(9.87817106)),
            ("2017-06-21", Norway, Some(11.42056214)),
            ("2020-02-29", Sydney, Some(2.12799525)),
            ("2000-01-01", Macapa, Some(15.46051831)),
            ("2023-06-21", Suva, Some(0.13165740)),
            ("2017-06-21", Ushuaia, Some(16.58459266)),
            ("2100-07-04", Nj, Some(17.01689317)),
        ],
    );
}

#[test]
fn spa_utc_midnight_matches_java() {
    use TestLocation::*;
    check_noon_midnight(
        SPA,
        SolarEvent::Midnight,
        &[
            ("2017-10-17", Nj, Some(4.69189152)),
            ("2017-10-17", La, Some(7.66535335)),
            ("1955-02-26", Jerusalem, Some(21.87680310)),
            ("2017-06-21", Norway, Some(23.42238534)),
            ("2020-02-29", Sydney, Some(14.12642351)),
            ("2000-01-01", Macapa, Some(3.46446119)),
            ("2023-06-21", Suva, Some(12.13348552)),
            ("2017-06-21", Ushuaia, Some(4.58641480)),
            ("2100-07-04", Nj, Some(5.01835716)),
        ],
    );
}

#[test]
fn spa_time_at_azimuth_matches_java() {
    use TestLocation::*;
    check_azimuth(
        SPA,
        &[
            ("2017-10-17", Nj, 90.0, Some(9.96630382)),
            ("2017-10-17", Nj, 270.0, Some(23.43696229)),
            ("2017-10-17", La, 90.0, Some(12.71802376)),
            ("2017-10-17", La, 270.0, Some(2.63792595)),
            ("2020-02-29", Sydney, 90.0, Some(20.92601636)),
            ("2020-02-29", Sydney, 270.0, Some(7.34685680)),
            ("2000-01-01", Macapa, 90.0, None),
            ("2017-06-21", Norway, 90.0, Some(6.01986373)),
        ],
    );
}

#[test]
fn spa_utc_sunrise_at_zenith_matches_java() {
    use TestLocation::*;
    check_sunrise_at_zenith(
        SPA,
        &[
            ("2017-10-17", Nj, 96.0, Some(10.70943974)),
            ("2017-10-17", Nj, 102.0, Some(10.17613961)),
            ("2017-10-17", Nj, 108.0, Some(9.64457480)),
            ("2020-02-29", Sydney, 96.0, Some(19.27874463)),
        ],
    );
}

#[test]
fn spa_utc_sunrise_without_elevation_matches_java() {
    use TestLocation::*;
    check_sunrise_without_elevation(
        SPA,
        &[
            ("2017-10-17", Nj, Some(11.17344334)),
            ("1955-02-26", Jerusalem, Some(4.18885558)),
        ],
    );
}

#[test]
fn spa_utc_sunrise_with_delta_t_disabled_matches_java() {
    use TestLocation::*;
    check_rise_set(
        SPA_NO_DELTA_T,
        SolarEvent::Sunrise,
        &[
            ("2017-10-17", Nj, Some(11.13586818)),
            ("1955-02-26", Jerusalem, Some(4.11871824)),
        ],
    );
}

// ---------------------------------------------------------------------------------------
// Cross-calculator behaviour
// ---------------------------------------------------------------------------------------

const ALL_CALCULATORS: [AstronomicalCalculator; 5] = [NOAA, MEEUS, MEEUS_NO_DELTA_T, SPA, SPA_NO_DELTA_T];

#[test]
fn every_calculator_rejects_unsupported_azimuth() {
    for calculator in ALL_CALCULATORS {
        let result = utc_time_at_azimuth_hours(calculator, date("2017-10-17"), &TestLocation::Nj.location(), 123.0);
        assert_eq!(result, Err(ZmanimError::Internal), "{calculator:?}");
    }
}

#[test]
fn every_calculator_reports_polar_day_and_night() {
    let norway = TestLocation::Norway.location();
    for calculator in ALL_CALCULATORS {
        for event in [SolarEvent::Sunrise, SolarEvent::Sunset] {
            let summer = utc_rise_set_hours(calculator, date("2017-06-21"), &norway, GEOMETRIC_ZENITH, true, event);
            assert_eq!(summer, Err(ZmanimError::AllDay), "{calculator:?} {event:?} midsummer");
            let winter = utc_rise_set_hours(calculator, date("2017-12-21"), &norway, GEOMETRIC_ZENITH, true, event);
            assert_eq!(winter, Err(ZmanimError::AllNight), "{calculator:?} {event:?} midwinter");
        }
    }
}

#[test]
fn calculators_agree_to_within_a_minute_for_ordinary_sunrise() {
    let nj = TestLocation::Nj.location();
    let day = date("2017-10-17");
    let noaa = utc_rise_set_hours(NOAA, day, &nj, GEOMETRIC_ZENITH, true, SolarEvent::Sunrise).unwrap();
    for calculator in [MEEUS, MEEUS_NO_DELTA_T, SPA, SPA_NO_DELTA_T] {
        let other = utc_rise_set_hours(calculator, day, &nj, GEOMETRIC_ZENITH, true, SolarEvent::Sunrise).unwrap();
        assert!(
            (noaa - other).abs() * 3600.0 < 60.0,
            "{calculator:?} differs from NOAA by {} s",
            (noaa - other).abs() * 3600.0
        );
        assert_ne!(noaa, other, "{calculator:?} should not be bit-identical to NOAA");
    }
}

#[test]
fn delta_t_changes_meeus_and_spa_results() {
    let nj = TestLocation::Nj.location();
    let day = date("2017-10-17");
    for (with, without) in [(MEEUS, MEEUS_NO_DELTA_T), (SPA, SPA_NO_DELTA_T)] {
        let a = utc_rise_set_hours(with, day, &nj, GEOMETRIC_ZENITH, true, SolarEvent::Sunrise).unwrap();
        let b = utc_rise_set_hours(without, day, &nj, GEOMETRIC_ZENITH, true, SolarEvent::Sunrise).unwrap();
        assert_ne!(a, b, "{with:?} vs {without:?}");
    }
}

#[test]
fn spa_sunrise_depends_on_observer_elevation_even_at_sea_level_horizon() {
    // SPA feeds the observer elevation into its parallax term, so the sea-level sunrise
    // (no horizon dip) still moves with elevation; NOAA and Meeus ignore it entirely.
    let day = date("2017-10-17");
    let low = Location::new(41.1181036, -74.0840691, Some(TimeZone::UTC)).unwrap();
    let high = Location::new(41.1181036, -74.0840691, Some(TimeZone::UTC))
        .and_then(|l| l.with_elevation(4000.0))
        .unwrap();
    let spa_low = utc_rise_set_hours(SPA, day, &low, GEOMETRIC_ZENITH, false, SolarEvent::Sunrise).unwrap();
    let spa_high = utc_rise_set_hours(SPA, day, &high, GEOMETRIC_ZENITH, false, SolarEvent::Sunrise).unwrap();
    assert_ne!(spa_low, spa_high);
    assert!(
        (spa_low - spa_high).abs() * 3600.0 < 1.0,
        "parallax shift should be sub-second"
    );
    for calculator in [NOAA, MEEUS] {
        let a = utc_rise_set_hours(calculator, day, &low, GEOMETRIC_ZENITH, false, SolarEvent::Sunrise).unwrap();
        let b = utc_rise_set_hours(calculator, day, &high, GEOMETRIC_ZENITH, false, SolarEvent::Sunrise).unwrap();
        assert_eq!(a, b, "{calculator:?}");
    }
}

#[test]
fn calculator_names_match_java() {
    assert_eq!(
        NOAA.name(),
        "US National Oceanic and Atmospheric Administration Algorithm"
    );
    assert_eq!(MEEUS.name(), "Jean Meeus Higher-Accuracy (VSOP87) Algorithm");
    assert_eq!(SPA.name(), "NREL Solar Position Algorithm");
}

// ---------------------------------------------------------------------------------------
// Shared helpers
// ---------------------------------------------------------------------------------------

fn test_location() -> Location {
    Location::new(
        40.0721087,
        -74.2400243,
        Some(TimeZone::get("America/New_York").unwrap()),
    )
    .unwrap()
}

#[test]
fn normalize_utc_hours_zero_stays_zero() {
    assert_eq!(normalize_utc_hours(0.0), 0.0);
}

#[test]
fn normalize_utc_hours_negative_and_overflow() {
    assert!((normalize_utc_hours(-1.0) - 23.0).abs() < f64::EPSILON);
    assert!((normalize_utc_hours(25.0) - 1.0).abs() < f64::EPSILON);
}

#[test]
fn apparent_solar_radius_varies_through_year() {
    let perihelion = Date::new(2024, 1, 3).unwrap();
    let aphelion = Date::new(2024, 7, 5).unwrap();
    let fixed = 16.0 / 60.0;
    assert!(apparent_solar_radius(perihelion) > fixed);
    assert!(apparent_solar_radius(aphelion) < fixed);
}

#[test]
fn apparent_solar_radius_changes_sunrise_vs_fixed_radius() {
    for calculator in ALL_CALCULATORS {
        let location = test_location();
        let date = Date::new(2024, 1, 3).unwrap();
        let apparent = sunrise(calculator, date, &location, false).unwrap();

        let fixed_zenith = GEOMETRIC_ZENITH + (16.0 / 60.0) + REFRACTION;
        let adjusted_date = adjusted_local_date(date, &location).unwrap();
        let utc_minutes = calculator
            .rise_set_utc_minutes(adjusted_date, &location, fixed_zenith, SolarEvent::Sunrise)
            .unwrap();
        let fixed = instant_from_utc_hours(
            normalize_utc_hours(utc_minutes / 60.0),
            adjusted_date,
            &location,
            SolarEvent::Sunrise,
        )
        .unwrap();

        assert_ne!(apparent, fixed, "{calculator:?}");
        assert!(apparent < fixed, "{calculator:?}");
    }
}

#[test]
fn solar_midnight_succeeds_near_antimeridian() {
    let location = Location::new(-14.2750, -170.7020, Some(TimeZone::get("Pacific/Apia").unwrap())).unwrap();
    let date = Date::new(2011, 12, 29).unwrap();
    for calculator in ALL_CALCULATORS {
        assert!(solar_midnight(calculator, date, &location).is_ok(), "{calculator:?}");
    }
}

#[test]
fn time_at_azimuth_normalizes_hours_before_timestamp() {
    let location = Location::new(69.6492, 18.9553, Some(TimeZone::get("Europe/Oslo").unwrap())).unwrap();
    let date = Date::new(2017, 6, 21).unwrap();
    for calculator in ALL_CALCULATORS {
        let azimuth_time = time_at_azimuth(calculator, date, &location, 90.0).unwrap();
        let local = azimuth_time.to_zoned(TimeZone::get("Europe/Oslo").unwrap());
        assert_eq!(local.date(), Date::new(2017, 6, 21).unwrap(), "{calculator:?}");
    }
}
