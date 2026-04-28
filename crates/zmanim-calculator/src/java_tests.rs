#![allow(clippy::unwrap_used)]

use std::{env::join_paths, error::Error, path::PathBuf, str::FromStr, sync::OnceLock};

use chrono::{Duration, NaiveDate};
use chrono_tz::Tz;
use jni::{
    jni_sig, jni_str,
    objects::{JClassLoader, JObject, JString, JValue, LoaderContext},
    sys::{jboolean, JNI_FALSE, JNI_TRUE},
    InitArgsBuilder, JNIVersion, JavaVM,
};

use crate::{
    calculator::ZmanLike,
    java_bindings::com::kosherjava::zmanim::{util::GeoLocation, ComprehensiveZmanimCalendar},
    presets::{ZmanPreset, ALL},
    types::{config::CalculatorConfig, location::Location},
};

type TestResult<T = ()> = Result<T, Box<dyn Error>>;

const DEFAULT_MAX_DIFF_MS: i64 = 10_000;
const EDGE_CASE_MAX_DIFF_MS: i64 = 30_000;

static JVM: OnceLock<JavaVM> = OnceLock::new();

#[derive(Clone, Copy, Debug)]
struct RegressionCase {
    year: i32,
    month: u32,
    day: u32,
    latitude: f64,
    longitude: f64,
    elevation: f64,
    timezone: &'static str,
    preset_name: &'static str,
    ateret_torah_sunset_offset_minutes: i64,
    candle_lighting_offset_minutes: i64,
    use_astronomical_chatzos_for_other_zmanim: bool,
    use_elevation: bool,
}

#[derive(Debug)]
struct ZmanResult {
    formatted: String,
    timestamp_ms: i64,
}

const REGRESSION_CASES: &[RegressionCase] = &[
    RegressionCase {
        year: 2024,
        month: 4,
        day: 22,
        latitude: 31.778,
        longitude: 35.235,
        elevation: 754.0,
        timezone: "Asia/Jerusalem",
        preset_name: "getSofZmanAchilasChametzGRA",
        ateret_torah_sunset_offset_minutes: 0,
        candle_lighting_offset_minutes: 18,
        use_astronomical_chatzos_for_other_zmanim: false,
        use_elevation: false,
    },
    RegressionCase {
        year: 2024,
        month: 4,
        day: 22,
        latitude: 31.778,
        longitude: 35.235,
        elevation: 754.0,
        timezone: "Asia/Jerusalem",
        preset_name: "getSofZmanBiurChametzGRA",
        ateret_torah_sunset_offset_minutes: 0,
        candle_lighting_offset_minutes: 18,
        use_astronomical_chatzos_for_other_zmanim: false,
        use_elevation: false,
    },
    RegressionCase {
        year: 2026,
        month: 1,
        day: 3,
        latitude: 39.36463,
        longitude: -76.70222,
        elevation: 0.0,
        timezone: "America/New_York",
        preset_name: "getSofZmanKidushLevanaBetweenMoldos",
        ateret_torah_sunset_offset_minutes: 0,
        candle_lighting_offset_minutes: 18,
        use_astronomical_chatzos_for_other_zmanim: false,
        use_elevation: false,
    },
    RegressionCase {
        year: 2058,
        month: 7,
        day: 31,
        latitude: -18.88480386694347,
        longitude: -174.522379072958,
        elevation: 2671.332842032057,
        timezone: "Pacific/Tongatapu",
        preset_name: "getFixedLocalChatzos",
        ateret_torah_sunset_offset_minutes: 19,
        candle_lighting_offset_minutes: 6,
        use_astronomical_chatzos_for_other_zmanim: true,
        use_elevation: false,
    },
];

#[test]
fn regression_cases_match_java_bindings() -> TestResult {
    for case in REGRESSION_CASES {
        assert_case_matches_java(*case)?;
    }
    Ok(())
}

fn assert_case_matches_java(case: RegressionCase) -> TestResult {
    let java = calculate_java_zman(case)?;
    let rust = calculate_rust_zman(case)?;

    match (java, rust) {
        (None, None) => Ok(()),
        (Some(java), Some(rust)) => {
            let difference = (java.timestamp_ms - rust.timestamp_ms).abs();
            let max_diff_ms = max_allowed_difference_ms(case.preset_name);
            assert!(
                difference <= max_diff_ms,
                "zman mismatch for {} on {} at ({}, {}) in {}: java={} rust={} diff={}ms max={}ms",
                case.preset_name,
                NaiveDate::from_ymd_opt(case.year, case.month, case.day).unwrap(),
                case.latitude,
                case.longitude,
                case.timezone,
                java.formatted,
                rust.formatted,
                difference,
                max_diff_ms
            );
            Ok(())
        }
        (java, rust) => panic!(
            "null mismatch for {} on {} in {}: java={:?} rust={:?}",
            case.preset_name,
            NaiveDate::from_ymd_opt(case.year, case.month, case.day).unwrap(),
            case.timezone,
            java,
            rust
        ),
    }
}

fn calculate_java_zman(case: RegressionCase) -> TestResult<Option<ZmanResult>> {
    java_vm()
        .attach_current_thread(|env| -> TestResult<Option<ZmanResult>> {
            let loader = JClassLoader::get_system_class_loader(env)?;
            let loader_context = LoaderContext::Loader(&loader);
            jni::__test_bindings_init(env, &loader_context);
            crate::java_bindings::jni_init(env, &loader_context)?;

            let timezone = new_zone_id(env, case.timezone)?;
            let location_name = env.new_string("")?;
            let location = GeoLocation::new5(
                env,
                location_name,
                case.latitude,
                case.longitude,
                case.elevation,
                timezone,
            )?;
            let calendar = ComprehensiveZmanimCalendar::new_geo_location(env, location)?;
            env.call_method(
                &calendar,
                jni_str!("setUseElevation"),
                jni_sig!("(Z)V"),
                &[JValue::Bool(bool_to_jboolean(case.use_elevation))],
            )?;
            env.call_method(
                &calendar,
                jni_str!("setCandleLightingOffset"),
                jni_sig!("(D)V"),
                &[JValue::Double(case.candle_lighting_offset_minutes as f64)],
            )?;
            env.call_method(
                &calendar,
                jni_str!("setUseAstronomicalChatzos"),
                jni_sig!("(Z)V"),
                &[JValue::Bool(JNI_TRUE)],
            )?;
            env.call_method(
                &calendar,
                jni_str!("setUseAstronomicalChatzosForOtherZmanim"),
                jni_sig!("(Z)V"),
                &[JValue::Bool(bool_to_jboolean(
                    case.use_astronomical_chatzos_for_other_zmanim,
                ))],
            )?;
            env.call_method(
                &calendar,
                jni_str!("setAteretTorahSunsetOffset"),
                jni_sig!("(D)V"),
                &[JValue::Double(
                    case.ateret_torah_sunset_offset_minutes as f64,
                )],
            )?;

            let local_date = new_local_date(env, case.year, case.month as i32, case.day as i32)?;
            env.call_method(
                &calendar,
                jni_str!("setLocalDate"),
                jni_sig!("(Ljava/time/LocalDate;)V"),
                &[JValue::Object(&local_date)],
            )?;

            let instant = match case.preset_name {
                "getSofZmanAchilasChametzGRA" => calendar.get_sof_zman_achilas_chametz_gra(env)?,
                "getSofZmanBiurChametzGRA" => calendar.get_sof_zman_biur_chametz_gra(env)?,
                "getSofZmanKidushLevanaBetweenMoldos" => {
                    calendar.get_sof_zman_kidush_levana_between_moldos(env)?
                }
                "getFixedLocalChatzos" => calendar.get_fixed_local_chatzos(env)?,
                other => panic!("missing Java binding mapping for {other}"),
            };

            if instant.is_null() {
                return Ok(None);
            }

            let timestamp_ms = env
                .call_method(&instant, jni_str!("toEpochMilli"), jni_sig!("()J"), &[])?
                .j()?;
            let formatted = format_in_timezone(env, &instant, case.timezone)?;
            Ok(Some(ZmanResult {
                formatted,
                timestamp_ms,
            }))
        })
}

fn calculate_rust_zman(case: RegressionCase) -> TestResult<Option<ZmanResult>> {
    let timezone = Tz::from_str(case.timezone)?;
    let location = Location::new(
        case.latitude,
        case.longitude,
        case.elevation,
        Some(timezone),
    )?;
    let date = NaiveDate::from_ymd_opt(case.year, case.month, case.day).unwrap();
    let config = CalculatorConfig {
        candle_lighting_offset: Duration::minutes(case.candle_lighting_offset_minutes),
        use_astronomical_chatzos_for_other_zmanim: case.use_astronomical_chatzos_for_other_zmanim,
        use_elevation: case.use_elevation,
        ateret_torah_sunset_offset: Duration::minutes(case.ateret_torah_sunset_offset_minutes),
    };
    let mut calculator = crate::calculator::ZmanimCalculator::new(location, date, config)?;
    let preset = preset_by_name(case.preset_name);

    let dt = match preset.calculate(&mut calculator) {
        Ok(dt) => dt,
        Err(_) => return Ok(None),
    };
    let formatted = dt.with_timezone(&timezone).to_rfc3339();
    Ok(Some(ZmanResult {
        formatted,
        timestamp_ms: dt.timestamp_millis(),
    }))
}

fn preset_by_name(name: &str) -> &'static ZmanPreset<'static> {
    ALL.iter()
        .copied()
        .find(|preset| preset.name == name)
        .unwrap_or_else(|| panic!("missing Rust preset mapping for {name}"))
}

fn max_allowed_difference_ms(preset_name: &str) -> i64 {
    match preset_name {
        "getSunriseWithElevation"
        | "getSeaLevelSunrise"
        | "getSunsetWithElevation"
        | "getSeaLevelSunset"
        | "getChatzos"
        | "getChatzosAsHalfDay"
        | "getFixedLocalChatzos" => EDGE_CASE_MAX_DIFF_MS,
        _ => DEFAULT_MAX_DIFF_MS,
    }
}

fn java_vm() -> &'static JavaVM {
    JVM.get_or_init(|| {
        let classpath = join_paths([
            java_path("target/zmanim-2.6.0-SNAPSHOT.jar"),
        ])
        .expect("failed to build Java classpath");
        let classpath_opt = format!("-Djava.class.path={}", classpath.to_string_lossy());
        let args = InitArgsBuilder::new()
            .version(JNIVersion::V1_8)
            .option(&classpath_opt)
            .build()
            .expect("failed to build JVM init args");
        JavaVM::new(args).expect("failed to create JVM for java parity tests")
    })
}

fn java_path(relative: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("java")
        .join(relative)
}

fn new_zone_id<'local>(
    env: &mut jni::Env<'local>,
    timezone: &str,
) -> jni::errors::Result<JObject<'local>> {
    let timezone = env.new_string(timezone)?;
    env.call_static_method(
        jni_str!("java/time/ZoneId"),
        jni_str!("of"),
        jni_sig!("(Ljava/lang/String;)Ljava/time/ZoneId;"),
        &[JValue::Object(&timezone)],
    )?
    .l()
}

fn new_local_date<'local>(
    env: &mut jni::Env<'local>,
    year: i32,
    month: i32,
    day: i32,
) -> jni::errors::Result<JObject<'local>> {
    env.call_static_method(
        jni_str!("java/time/LocalDate"),
        jni_str!("of"),
        jni_sig!("(III)Ljava/time/LocalDate;"),
        &[JValue::Int(year), JValue::Int(month), JValue::Int(day)],
    )?
    .l()
}

fn format_in_timezone<'local>(
    env: &mut jni::Env<'local>,
    instant: &JObject<'local>,
    timezone: &str,
) -> jni::errors::Result<String> {
    let zone_id = new_zone_id(env, timezone)?;
    let zoned = env
        .call_static_method(
            jni_str!("java/time/ZonedDateTime"),
            jni_str!("ofInstant"),
            jni_sig!("(Ljava/time/Instant;Ljava/time/ZoneId;)Ljava/time/ZonedDateTime;"),
            &[JValue::Object(instant), JValue::Object(&zone_id)],
        )?
        .l()?;
    let text = env
        .call_method(
            &zoned,
            jni_str!("toString"),
            jni_sig!("()Ljava/lang/String;"),
            &[],
        )?
        .l()?;
    let text = env.cast_local::<JString>(text)?;
    Ok(text.to_string())
}

fn bool_to_jboolean(value: bool) -> jboolean {
    if value {
        JNI_TRUE
    } else {
        JNI_FALSE
    }
}
