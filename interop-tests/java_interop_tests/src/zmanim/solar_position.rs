//! Java parity for [`AstronomicalCalculator::solar_position`] against KosherJava's
//! `getSolarElevation` and `getSolarAzimuth`.

use std::error::Error;

use jiff::Timestamp;
use jni::{jni_sig, jni_str, objects::JValue};
use rand::{RngExt, SeedableRng, rngs::StdRng};

use kosher_rust::{Location, zmanim::AstronomicalCalculator};

use super::{java_reference::new_java_calculator, policy};
use crate::{init_bindings, java_bindings::com::kosherjava::zmanim::util::GeoLocation, java_vm};

/// Largest difference allowed in either angle, in degrees.
const MAX_DIFF_DEGREES: f64 = 1e-6;

/// Above this elevation the azimuth changes quickly with tiny differences in position, so only
/// the elevation is compared.
const MAX_AZIMUTH_ELEVATION: f64 = 89.0;

/// Compares random instants and places with KosherJava for one solar algorithm.
pub fn test_solar_position(calculator: AstronomicalCalculator) -> Result<(), Box<dyn Error>> {
    let seed = policy::test_seed();
    let mut rng = StdRng::seed_from_u64(seed);
    for iteration in 0..policy::test_iterations() {
        let latitude = rng.random_range(-90.0..=90.0);
        let longitude = rng.random_range(-180.0..=180.0);
        let elevation = rng.random_range(0.0..=policy::MAX_RANDOM_ELEVATION_METERS);
        // 1800 to 2200.
        let millisecond = rng.random_range(-5_364_662_400_000..=7_258_118_400_000_i64);

        let location = Location::new(latitude, longitude)?.with_elevation(elevation)?;
        let instant = Timestamp::from_millisecond(millisecond)?;
        let rust = calculator.solar_position(&location, instant);
        let (java_elevation, java_azimuth) =
            java_solar_position(calculator, latitude, longitude, elevation, millisecond)?;

        let context = || {
            format!(
                "{calculator:?} at {instant}, latitude {latitude}, longitude {longitude}, elevation {elevation} \
                 (seed {seed}, iteration {iteration})"
            )
        };
        if (rust.elevation - java_elevation).abs() > MAX_DIFF_DEGREES {
            return Err(format!(
                "elevation {} != Java {java_elevation} for {}",
                rust.elevation,
                context()
            )
            .into());
        }
        let azimuth_diff = (rust.azimuth - java_azimuth + 540.0) % 360.0 - 180.0;
        if java_elevation < MAX_AZIMUTH_ELEVATION && azimuth_diff.abs() > MAX_DIFF_DEGREES {
            return Err(format!("azimuth {} != Java {java_azimuth} for {}", rust.azimuth, context()).into());
        }
    }
    Ok(())
}

/// KosherJava's solar elevation and azimuth.
fn java_solar_position(
    calculator: AstronomicalCalculator,
    latitude: f64,
    longitude: f64,
    elevation: f64,
    millisecond: i64,
) -> Result<(f64, f64), Box<dyn Error>> {
    java_vm().attach_current_thread(|env: &mut jni::Env<'_>| -> Result<(f64, f64), Box<dyn Error>> {
        init_bindings(env)?;

        let utc = env.new_string("UTC")?;
        let timezone = env
            .call_static_method(
                jni_str!("java/time/ZoneId"),
                jni_str!("of"),
                jni_sig!("(Ljava/lang/String;)Ljava/time/ZoneId;"),
                &[JValue::Object(&utc)],
            )?
            .l()?;
        let name = env.new_string("")?;
        let location = GeoLocation::new7(env, name, latitude, longitude, elevation, 0.0, 0.0, &timezone)?;
        let instant = env
            .call_static_method(
                jni_str!("java/time/Instant"),
                jni_str!("ofEpochMilli"),
                jni_sig!("(J)Ljava/time/Instant;"),
                &[JValue::Long(millisecond)],
            )?
            .l()?;
        let java_calculator = new_java_calculator(env, calculator)?;

        let mut angle = |method| -> Result<f64, Box<dyn Error>> {
            Ok(env
                .call_method(
                    &java_calculator,
                    method,
                    jni_sig!("(Ljava/time/Instant;Lcom/kosherjava/zmanim/util/GeoLocation;)D"),
                    &[JValue::Object(&instant), JValue::Object(&location)],
                )?
                .d()?)
        };
        Ok((
            angle(jni_str!("getSolarElevation"))?,
            angle(jni_str!("getSolarAzimuth"))?,
        ))
    })
}
