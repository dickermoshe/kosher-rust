#![allow(clippy::unwrap_used, clippy::panic)]

use jiff::{SignedDuration, civil::Date, tz::TimeZone};

use super::*;

const NOAA: AstronomicalCalculator = AstronomicalCalculator::Noaa;
const MEEUS: AstronomicalCalculator = AstronomicalCalculator::Meeus { apply_delta_t: true };
const MEEUS_NO_DELTA_T: AstronomicalCalculator = AstronomicalCalculator::Meeus { apply_delta_t: false };
const SPA: AstronomicalCalculator = AstronomicalCalculator::Spa { apply_delta_t: true };
const SPA_NO_DELTA_T: AstronomicalCalculator = AstronomicalCalculator::Spa { apply_delta_t: false };

/// KosherJava asserts its calculator unit tests to eight decimals of an hour (36 µs).
const JAVA_DELTA: f64 = 1e-8;

/// Tolerance for the SPA tests in hours (2 ms). The search finds events to one millisecond, so a
/// reference that searches differently can land up to about a millisecond away.
const SPA_DELTA: f64 = 2.0 / 3_600_000.0;

/// The tolerance for a calculator's expected values.
fn tolerance(calculator: AstronomicalCalculator) -> f64 {
    match calculator {
        AstronomicalCalculator::Spa { .. } => SPA_DELTA,
        _ => JAVA_DELTA,
    }
}

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
        Location::new(latitude, longitude)
            .and_then(|l| l.with_elevation(elevation))
            .unwrap()
    }
}

/// `(date, location, expected UTC hours)`; `None` means KosherJava returns `NaN`.
type HourCase = (&'static str, TestLocation, Option<f64>);

fn date(iso: &str) -> Date {
    iso.parse().unwrap()
}

fn assert_java_hours(label: &str, expected: Option<f64>, actual: Result<f64, ZmanimError>, tolerance: f64) {
    match (expected, actual) {
        (None, Err(_)) => {}
        (Some(expected), Ok(actual)) => assert!(
            (expected - actual).abs() <= tolerance,
            "{label}: expected {expected} got {actual} (diff {})",
            (expected - actual).abs()
        ),
        (expected, actual) => panic!("{label}: expected {expected:?} got {actual:?}"),
    }
}

fn check_rise_set(calculator: AstronomicalCalculator, event: SolarEvent, cases: &[HourCase]) {
    for (iso, place, expected) in cases {
        let actual = utc_rise_set_hours(calculator, date(iso), &place.location(), GEOMETRIC_ZENITH, true, event);
        assert_java_hours(
            &format!("{calculator:?} {event:?} {iso}"),
            *expected,
            actual,
            tolerance(calculator),
        );
    }
}

fn check_noon_midnight(calculator: AstronomicalCalculator, event: SolarEvent, cases: &[HourCase]) {
    for (iso, place, expected) in cases {
        let actual = utc_noon_midnight_hours(calculator, date(iso), &place.location(), event);
        assert_java_hours(
            &format!("{calculator:?} {event:?} {iso}"),
            *expected,
            Ok(actual),
            tolerance(calculator),
        );
    }
}

fn check_azimuth(calculator: AstronomicalCalculator, cases: &[(&str, TestLocation, f64, Option<f64>)]) {
    for (iso, place, azimuth, expected) in cases {
        let actual = utc_time_at_azimuth_hours(calculator, date(iso), &place.location(), *azimuth);
        assert_java_hours(
            &format!("{calculator:?} azimuth {azimuth} {iso}"),
            *expected,
            actual,
            tolerance(calculator),
        );
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
        assert_java_hours(
            &format!("{calculator:?} zenith {zenith} {iso}"),
            *expected,
            actual,
            tolerance(calculator),
        );
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
        assert_java_hours(
            &format!("{calculator:?} no-elevation sunrise {iso}"),
            *expected,
            actual,
            tolerance(calculator),
        );
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

/// Days when the sun only just reaches the zenith, around midnight. Using the sun's declination at
/// noon the zenith looks out of reach, but using the declination at solar midnight it is reached,
/// so these return a time rather than no event.
fn check_only_just_reached(calculator: AstronomicalCalculator, expected: [f64; 3]) {
    let london = Location::new(51.5074, -0.1278).unwrap();
    let arctic = Location::new(75.51550698948259, -0.9961193842273985)
        .and_then(|l| l.with_elevation(1516.0617032522935))
        .unwrap();
    let cases = [
        (
            "London 16.1° alos",
            &london,
            "2024-06-03",
            106.1,
            false,
            SolarEvent::Sunrise,
        ),
        (
            "London 18° tzais",
            &london,
            "2024-07-20",
            108.0,
            false,
            SolarEvent::Sunset,
        ),
        (
            "75.5°N sunrise",
            &arctic,
            "2093-04-22",
            GEOMETRIC_ZENITH,
            true,
            SolarEvent::Sunrise,
        ),
    ];
    for ((label, location, iso, zenith, adjust, event), expected) in cases.into_iter().zip(expected) {
        let actual = utc_rise_set_hours(calculator, date(iso), location, zenith, adjust, event);
        assert_java_hours(
            &format!("{calculator:?} {label}"),
            Some(expected),
            actual,
            tolerance(calculator),
        );
    }
}

#[test]
fn noaa_utc_rise_set_when_only_just_reached_matches_java() {
    check_only_just_reached(NOAA, [0.17183556, 23.85934340, 0.21591651]);
}

#[test]
fn meeus_utc_rise_set_when_only_just_reached_matches_java() {
    check_only_just_reached(MEEUS, [0.17268715, 23.85897993, 0.22098753]);
}

// ---------------------------------------------------------------------------------------
// SPA
// ---------------------------------------------------------------------------------------
//
// Expected rise, set, noon and midnight times come from Klaus Brunner's solarpositioning v3.0.3,
// run on the same targets, windows and ΔT; times at an azimuth, which it does not calculate,
// come from KosherJava's `SPACalculator`. Checked to [`SPA_DELTA`].

#[test]
fn spa_utc_sunrise_matches_solarpositioning() {
    use TestLocation::*;
    check_rise_set(
        SPA,
        SolarEvent::Sunrise,
        &[
            ("2017-10-17", Nj, Some(11.13593480)),
            ("2017-10-17", La, Some(14.00756564)),
            ("1955-02-26", Jerusalem, Some(4.11873534)),
            ("2017-06-21", Norway, None),
            ("2020-02-29", Sydney, Some(19.68292944)),
            ("2000-01-01", Macapa, Some(9.38998718)),
            ("2023-06-21", Suva, Some(18.60534643)),
            ("2017-06-21", Ushuaia, Some(12.95741135)),
            ("2100-07-04", Nj, Some(9.44905415)),
        ],
    );
}

#[test]
fn spa_utc_sunset_matches_solarpositioning() {
    use TestLocation::*;
    check_rise_set(
        SPA,
        SolarEvent::Sunset,
        &[
            ("2017-10-17", Nj, Some(22.24099365)),
            ("2017-10-17", La, Some(1.31843788)),
            ("1955-02-26", Jerusalem, Some(15.64511646)),
            ("2017-06-21", Norway, None),
            ("2020-02-29", Sydney, Some(8.56367311)),
            ("2000-01-01", Macapa, Some(21.53102584)),
            ("2023-06-21", Suva, Some(5.65791097)),
            ("2017-06-21", Ushuaia, Some(20.21191650)),
            ("2100-07-04", Nj, Some(0.58022832)),
        ],
    );
}

#[test]
fn spa_utc_noon_matches_solarpositioning() {
    use TestLocation::*;
    check_noon_midnight(
        SPA,
        SolarEvent::Noon,
        &[
            ("2017-10-17", Nj, Some(16.69350757)),
            ("2017-10-17", La, Some(19.66695881)),
            ("1955-02-26", Jerusalem, Some(9.87816577)),
            ("2017-06-21", Norway, Some(11.42056095)),
            ("2020-02-29", Sydney, Some(2.12799375)),
            ("2000-01-01", Macapa, Some(15.46051864)),
            ("2023-06-21", Suva, Some(0.13165412)),
            ("2017-06-21", Ushuaia, Some(16.58459143)),
            ("2100-07-04", Nj, Some(17.01682784)),
        ],
    );
}

#[test]
fn spa_utc_midnight_matches_solarpositioning() {
    use TestLocation::*;
    check_noon_midnight(
        SPA,
        SolarEvent::Midnight,
        &[
            ("2017-10-17", Nj, Some(4.69189023)),
            ("2017-10-17", La, Some(7.66535208)),
            ("1955-02-26", Jerusalem, Some(21.87679780)),
            ("2017-06-21", Norway, Some(23.42238409)),
            ("2020-02-29", Sydney, Some(14.12642201)),
            ("2000-01-01", Macapa, Some(3.46446151)),
            ("2023-06-21", Suva, Some(12.13348220)),
            ("2017-06-21", Ushuaia, Some(4.58641355)),
            ("2100-07-04", Nj, Some(5.01829185)),
        ],
    );
}

#[test]
fn spa_time_at_azimuth_matches_java() {
    use TestLocation::*;
    check_azimuth(
        SPA,
        &[
            ("2017-10-17", Nj, 90.0, Some(9.96630489)),
            ("2017-10-17", Nj, 270.0, Some(23.43696124)),
            ("2017-10-17", La, 90.0, Some(12.71802515)),
            ("2017-10-17", La, 270.0, Some(2.63792459)),
            ("2020-02-29", Sydney, 90.0, Some(20.92601768)),
            ("2020-02-29", Sydney, 270.0, Some(7.34685571)),
            ("2000-01-01", Macapa, 90.0, None),
            ("2017-06-21", Norway, 90.0, Some(6.01986389)),
        ],
    );
}

#[test]
fn spa_utc_sunrise_at_zenith_matches_solarpositioning() {
    use TestLocation::*;
    check_sunrise_at_zenith(
        SPA,
        &[
            ("2017-10-17", Nj, 96.0, Some(10.70943766)),
            ("2017-10-17", Nj, 102.0, Some(10.17613755)),
            ("2017-10-17", Nj, 108.0, Some(9.64457289)),
            ("2020-02-29", Sydney, 96.0, Some(19.27874263)),
        ],
    );
}

#[test]
fn spa_utc_sunrise_without_elevation_matches_solarpositioning() {
    use TestLocation::*;
    check_sunrise_without_elevation(
        SPA,
        &[
            ("2017-10-17", Nj, Some(11.17344125)),
            ("1955-02-26", Jerusalem, Some(4.18885160)),
        ],
    );
}

#[test]
fn spa_utc_sunrise_with_delta_t_disabled_matches_solarpositioning() {
    use TestLocation::*;
    check_rise_set(
        SPA_NO_DELTA_T,
        SolarEvent::Sunrise,
        &[
            ("2017-10-17", Nj, Some(11.13586772)),
            ("1955-02-26", Jerusalem, Some(4.11871827)),
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
    let low = Location::new(41.1181036, -74.0840691).unwrap();
    let high = Location::new(41.1181036, -74.0840691)
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
    Location::new(40.0721087, -74.2400243).unwrap()
}

fn new_york() -> TimeZone {
    TimeZone::get("America/New_York").unwrap()
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
        let apparent = sunrise(calculator, date, &location, Some(&new_york()), false).unwrap();

        let fixed_zenith = GEOMETRIC_ZENITH + (16.0 / 60.0) + REFRACTION;
        let adjusted_date = adjusted_local_date(date, &location, Some(&new_york())).unwrap();
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
    let location = Location::new(-14.2750, -170.7020).unwrap();
    let apia = TimeZone::get("Pacific/Apia").unwrap();
    let date = Date::new(2011, 12, 29).unwrap();
    for calculator in ALL_CALCULATORS {
        assert!(
            solar_midnight(calculator, date, &location, Some(&apia)).is_ok(),
            "{calculator:?}"
        );
    }
}

#[test]
fn time_at_azimuth_normalizes_hours_before_timestamp() {
    let location = Location::new(69.6492, 18.9553).unwrap();
    let oslo = TimeZone::get("Europe/Oslo").unwrap();
    let date = Date::new(2017, 6, 21).unwrap();
    for calculator in ALL_CALCULATORS {
        let azimuth_time = time_at_azimuth(calculator, date, &location, Some(&oslo), 90.0).unwrap();
        let local = azimuth_time.to_zoned(TimeZone::get("Europe/Oslo").unwrap());
        assert_eq!(local.date(), Date::new(2017, 6, 21).unwrap(), "{calculator:?}");
    }
}

/// Reading the offset at noon UTC picks the same date as KosherJava's
/// `GeoLocation.getAntimeridianAdjustment`, which reads it at `ZonedDateTime.of(date, MIDNIGHT,
/// zone)`: the first midnight where it happens twice, and past the jump where it's skipped (jiff's
/// `compatible`). At each zone's own place, every day from 1970 to 2040.
#[test]
fn adjusted_date_is_the_same_from_noon_as_from_local_midnight() {
    let zones = [
        ("America/New_York", -74.0),
        ("America/Havana", -82.4),      // spring forward at midnight
        ("America/Santiago", -70.7),    // transitions at 24:00
        ("America/Sao_Paulo", -46.6),   // historical midnight transitions
        ("Asia/Beirut", 35.5),          // midnight transitions
        ("Pacific/Apia", -171.8),       // skipped 30 December 2011
        ("Pacific/Fakaofo", -171.2),    // skipped 30 December 2011
        ("Pacific/Kiritimati", -157.5), // skipped 31 December 1994; UTC+14
        ("Pacific/Kwajalein", 167.7),   // skipped 21 August 1993
        ("Pacific/Tongatapu", -175.2),  // UTC+13, with DST some years
        ("Pacific/Chatham", -176.5),    // UTC+12:45, with DST
        ("America/Adak", -176.7),       // UTC-10, with DST
        ("Australia/Lord_Howe", 159.1), // half-hour DST
    ];
    let mut differences = std::vec::Vec::new();
    for (name, longitude) in zones {
        let tz = TimeZone::get(name).unwrap();
        let location = Location::new(0.0, longitude).unwrap();
        let mut date = Date::new(1970, 1, 1).unwrap();
        while date.year() < 2040 {
            let midnight = tz.to_ambiguous_timestamp(date.at(0, 0, 0, 0)).compatible().unwrap();
            let offset = f64::from(tz.to_offset(midnight).seconds());
            let hours = (longitude * 240.0 - offset) / 3600.0;
            let days = if hours >= 20.0 {
                1
            } else if hours <= -20.0 {
                -1
            } else {
                0
            };
            let expected = date.checked_add(SignedDuration::from_hours(24 * days)).unwrap();
            if adjusted_local_date(date, &location, Some(&tz)).unwrap() != expected {
                differences.push(std::format!("{name} {date}"));
            }
            date = date.tomorrow().unwrap();
        }
    }
    assert_eq!(differences, std::vec::Vec::<std::string::String>::new());
}
