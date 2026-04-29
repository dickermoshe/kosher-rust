//! JNI bridge used by the Java parity tests.
//!
//! This module owns the JVM lifecycle and the object setup needed to ask
//! KosherJava for one preset result.  The rest of the parity harness should not
//! need to know about class loaders, generated JNI bindings, or Java object
//! construction details.

use std::{collections::HashSet, env::join_paths, error::Error, path::PathBuf, sync::OnceLock};

use jni::{
    jni_sig, jni_str,
    objects::{JClassLoader, JObject, JObjectArray, JString, JValue, LoaderContext},
    sys::{jboolean, JNI_FALSE, JNI_TRUE},
    InitArgsBuilder, JNIVersion, JavaVM,
};

use crate::{
    java_bindings::com::kosherjava::zmanim::{util::GeoLocation, ComprehensiveZmanimCalendar},
    java_tests::parity::ZmanResult,
    presets::ZmanPreset,
};

use super::cases::TestCase;

static JVM: OnceLock<JavaVM> = OnceLock::new();

/// Returns the process-wide JVM used by all parity tests.
///
/// JNI permits only one JVM per process, so the test harness initializes it once
/// with the local KosherJava jar on the classpath and reuses it for every case.
pub(crate) fn java_vm() -> &'static JavaVM {
    JVM.get_or_init(|| {
        let java_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("java")
            .join("target/zmanim-2.6.0-SNAPSHOT.jar");
        // A process can only create one JVM.
        let classpath = join_paths([java_path]).expect("failed to build Java classpath");
        let classpath_opt = format!("-Djava.class.path={}", classpath.to_string_lossy());
        let args = InitArgsBuilder::new()
            .version(JNIVersion::V1_8)
            .option(&classpath_opt)
            .build()
            .expect("failed to build JVM init args");
        JavaVM::new(args).expect("failed to create JVM for java parity tests")
    })
}

/// Initializes generated Java bindings for the current JNI environment.
pub(crate) fn init_bindings(env: &mut jni::Env<'_>) -> Result<(), Box<dyn Error>> {
    let loader = JClassLoader::get_system_class_loader(env)?;
    let loader_context = LoaderContext::Loader(&loader);
    jni::__test_bindings_init(env, &loader_context);
    crate::java_bindings::jni_init(env, &loader_context)?;
    Ok(())
}

static JAVA_SUPPORTED_TIMEZONES: OnceLock<HashSet<String>> = OnceLock::new();

/// Returns the timezone IDs known to the Java runtime.
///
/// Random case generation intersects this set with `chrono-tz` so timezone-name
/// compatibility does not masquerade as a zman calculation mismatch.
pub(crate) fn java_supported_timezones() -> &'static HashSet<String> {
    JAVA_SUPPORTED_TIMEZONES.get_or_init(|| {
        java_vm()
            .attach_current_thread(|env| -> Result<HashSet<String>, Box<dyn Error>> {
                let zone_ids = env
                    .call_static_method(
                        jni_str!("java/time/ZoneId"),
                        jni_str!("getAvailableZoneIds"),
                        jni_sig!("()Ljava/util/Set;"),
                        &[],
                    )?
                    .l()?;
                let zone_array = env
                    .call_method(
                        &zone_ids,
                        jni_str!("toArray"),
                        jni_sig!("()[Ljava/lang/Object;"),
                        &[],
                    )?
                    .l()?;
                let zone_array = env.cast_local::<JObjectArray<'_, JObject<'_>>>(zone_array)?;
                let zone_array_len = zone_array.len(env)?;
                let mut timezones = HashSet::with_capacity(zone_array_len);

                for index in 0..zone_array_len {
                    let zone = zone_array.get_element(env, index)?;
                    let zone = env.cast_local::<JString>(zone)?;
                    timezones.insert(zone.to_string());
                }

                Ok(timezones)
            })
            .expect("failed to load Java-supported timezone IDs")
    })
}

/// Calculates one preset with KosherJava for a parity [`TestCase`].
///
/// The returned timestamp is in epoch milliseconds for comparison, while the
/// formatted value is only for human-readable failure messages.
pub(crate) fn calculate_java_zman(
    case: TestCase,
    zman: &'static ZmanPreset<'static>,
) -> Result<Option<ZmanResult>, Box<dyn Error>> {
    java_vm().attach_current_thread(
        |env: &mut jni::Env<'_>| -> Result<Option<ZmanResult>, Box<dyn Error>> {
            init_bindings(env)?;

            let timezone = env.new_string(case.timezone)?;
            let timezone = env
                .call_static_method(
                    jni_str!("java/time/ZoneId"),
                    jni_str!("of"),
                    jni_sig!("(Ljava/lang/String;)Ljava/time/ZoneId;"),
                    &[JValue::Object(&timezone)],
                )?
                .l()?;

            let location_name = env.new_string("")?;
            let location = GeoLocation::new5(
                env,
                location_name,
                case.latitude,
                case.longitude,
                case.elevation,
                &timezone,
            )?;
            let calendar = ComprehensiveZmanimCalendar::new_geo_location(env, location)?;
            let local_date = new_local_date(env, case.year, case.month as i32, case.day as i32)?;
            env.call_method(
                &calendar,
                jni_str!("setLocalDate"),
                jni_sig!("(Ljava/time/LocalDate;)V"),
                &[JValue::Object(&local_date)],
            )?;
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
            let instant = zman.calc.call(env, &calendar)?;
            if instant.is_null() {
                return Ok(None);
            }

            let timestamp_ms = env
                .call_method(&instant, jni_str!("toEpochMilli"), jni_sig!("()J"), &[])?
                .j()?;
            let formatted = formatted_instant(env, &instant, &timezone)?;

            Ok(Some(ZmanResult {
                formatted,
                timestamp_ms,
            }))
        },
    )
}

pub(crate) fn new_local_date<'local>(
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

fn formatted_instant<'local>(
    env: &mut jni::Env<'local>,
    instant: &JObject<'local>,
    zone_id: &JObject<'local>,
) -> jni::errors::Result<String> {
    let zoned = env
        .call_static_method(
            jni_str!("java/time/ZonedDateTime"),
            jni_str!("ofInstant"),
            jni_sig!("(Ljava/time/Instant;Ljava/time/ZoneId;)Ljava/time/ZonedDateTime;"),
            &[JValue::Object(instant), JValue::Object(zone_id)],
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

pub(crate) fn bool_to_jboolean(value: bool) -> jboolean {
    if value {
        JNI_TRUE
    } else {
        JNI_FALSE
    }
}
