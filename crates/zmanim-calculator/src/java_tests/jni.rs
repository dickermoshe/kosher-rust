//! Calls KosherJava through JNI.

use std::{collections::HashSet, env::join_paths, error::Error, path::PathBuf, sync::OnceLock};

use jni::{
    jni_sig, jni_str,
    objects::{JClassLoader, JObject, JObjectArray, JString, JValue, LoaderContext},
    strings::JNIString,
    sys::{jboolean, JNI_FALSE, JNI_TRUE},
    InitArgsBuilder, JNIVersion, JavaVM,
};

use crate::java_bindings::com::kosherjava::zmanim::{
    util::GeoLocation, ComprehensiveZmanimCalendar,
};

use super::cases::RegressionCase;

static JVM: OnceLock<JavaVM> = OnceLock::new();
static JAVA_SUPPORTED_TIMEZONES: OnceLock<HashSet<String>> = OnceLock::new();

#[derive(Clone, Debug)]
pub(crate) struct JavaZmanResult {
    pub(crate) formatted: String,
    pub(crate) timestamp_ms: i64,
}

/// Calls several KosherJava preset methods for the same date and place.
///
/// The results are in the same order as `preset_names`.
pub(crate) fn calculate_java_zman_batch(
    case: RegressionCase,
    preset_names: &[&'static str],
) -> Result<Vec<Option<JavaZmanResult>>, Box<dyn Error>> {
    java_vm().attach_current_thread(
        |env: &mut jni::Env<'_>| -> Result<Vec<Option<JavaZmanResult>>, Box<dyn Error>> {
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

            let mut results = Vec::with_capacity(preset_names.len());
            for &preset_name in preset_names {
                let preset_method = JNIString::new(preset_name);
                let instant: JObject<'_> = env
                    .call_method(
                        &calendar,
                        preset_method,
                        jni_sig!("()Ljava/time/Instant;"),
                        &[],
                    )?
                    .l()?;

                if instant.is_null() {
                    results.push(None);
                    continue;
                }

                let timestamp_ms = env
                    .call_method(&instant, jni_str!("toEpochMilli"), jni_sig!("()J"), &[])?
                    .j()?;
                let formatted = format_in_timezone(env, &instant, case.timezone)?;
                results.push(Some(JavaZmanResult {
                    formatted,
                    timestamp_ms,
                }));
            }

            Ok(results)
        },
    )
}

/// Returns the timezones known to Java.
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

fn java_vm() -> &'static JavaVM {
    JVM.get_or_init(|| {
        // A process can only create one JVM.
        let classpath = join_paths([java_path("target/zmanim-2.6.0-SNAPSHOT.jar")])
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
