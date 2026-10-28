#![allow(clippy::unwrap_used, clippy::panic)]

use jiff::Timestamp;

use super::*;

/// The continuous Julian date of an RFC 3339 timestamp.
fn julian_date(timestamp: &str, delta_t: f64) -> JulianDate {
    let seconds = timestamp.parse::<Timestamp>().unwrap().as_millisecond() as f64 / 1000.0;
    JulianDate {
        julian_date: 2440587.5 + seconds / 86400.0,
        delta_t,
    }
}

/// The SPA refraction correction applied to an unrefracted elevation, as a zenith angle, for
/// comparing with refracted reference values. No refraction is applied below -0.83337°.
fn refracted_zenith(elevation: f64, pressure: f64, temperature: f64) -> f64 {
    if elevation > -0.83337 {
        let phase = (elevation + 10.3 / (elevation + 5.11)).to_radians();
        let correction = (pressure / 1010.0) * (283.0 / (273.0 + temperature)) * 1.02 / (60.0 * phase.tan());
        90.0 - elevation - correction
    } else {
        90.0 - elevation
    }
}

/// The worked example of Reda and Andreas (NREL/TP-560-34302, Table A5.1).
#[test]
fn nrel_example() {
    let time = julian_date("2003-10-17T19:30:30Z", 67.0);
    let position = event_position_at_height(time, 39.742476, -105.1786, 1830.14);
    let azimuth = Topocentric::new(time, 39.742476, -105.1786, 1830.14).azimuth();
    assert!((azimuth - 194.340241).abs() < 1e-4, "{azimuth}");
    assert!(
        (90.0 - position.elevation - 50.1279).abs() < 1e-4,
        "{}",
        90.0 - position.elevation
    );
    assert!(
        (refracted_zenith(position.elevation, 820.0, 11.0) - 50.111622).abs() < 1e-6,
        "{}",
        refracted_zenith(position.elevation, 820.0, 11.0)
    );
}

/// Zenith and azimuth angles from NREL's reference implementation, `spa.c` (ΔT 0, sea level, 1000 hPa, 10 °C).
#[test]
fn nrel_reference_angles() {
    let data = include_str!("../../../tests/fixtures/spa_reference_testdata.csv");
    let mut checked = 0;
    for line in data.lines().filter(|line| !line.starts_with('#') && !line.is_empty()) {
        let fields: Vec<&str> = line.split(',').collect();
        let (latitude, longitude) = (fields[1].parse::<f64>().unwrap(), fields[2].parse::<f64>().unwrap());
        let (reference_azimuth, reference_zenith) =
            (fields[3].parse::<f64>().unwrap(), fields[4].parse::<f64>().unwrap());
        let topocentric = Topocentric::new(julian_date(fields[0], 0.0), latitude, longitude, 0.0);
        let zenith = refracted_zenith(topocentric.elevation(), 1000.0, 10.0);
        assert!((zenith - reference_zenith).abs() < 1e-6, "{line}: zenith {zenith}");
        let azimuth = topocentric.azimuth();
        let azimuth_error = (azimuth - reference_azimuth + 540.0) % 360.0 - 180.0;
        assert!(azimuth_error.abs() < 1e-6, "{line}: azimuth {azimuth}");
        checked += 1;
    }
    assert_eq!(checked, 512);
}

/// `limit_to` keeps results in `[0, max)`, including for tiny negative remainders.
#[test]
fn limit_to_stays_below_max() {
    assert_eq!(limit_to(-1e-20, 360.0), 0.0);
    assert_eq!(limit_to(-90.0, 360.0), 270.0);
    assert_eq!(limit_to(725.0, 360.0), 5.0);
}
