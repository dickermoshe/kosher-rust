//! Java-side reference path for zmanim Java parity tests.

use std::{collections::HashSet, error::Error, sync::OnceLock};

use jni::{
    jni_sig, jni_str,
    objects::{JObject, JObjectArray, JString, JValue},
    strings::JNIString,
    sys::{JNI_FALSE, JNI_TRUE, jboolean},
};

use crate::java_bindings::com::kosherjava::zmanim::{ComprehensiveZmanimCalendar, util::GeoLocation};

use crate::{init_bindings, java_vm};

use kosher_rust::prelude::*;

use super::types::{TestCase, ZmanResult};

static JAVA_SUPPORTED_TIMEZONES: OnceLock<HashSet<String>> = OnceLock::new();

/// Returns the timezone IDs known to the Java runtime.

pub(super) fn java_supported_timezones() -> &'static HashSet<String> {
    JAVA_SUPPORTED_TIMEZONES.get_or_init(|| {
        java_vm()
            .attach_current_thread(|env| -> Result<HashSet<String>, Box<dyn Error>> {
                init_bindings(env)?;

                let zone_ids = env
                    .call_static_method(
                        jni_str!("java/time/ZoneId"),
                        jni_str!("getAvailableZoneIds"),
                        jni_sig!("()Ljava/util/Set;"),
                        &[],
                    )?
                    .l()?;

                let zone_array = env
                    .call_method(&zone_ids, jni_str!("toArray"), jni_sig!("()[Ljava/lang/Object;"), &[])?
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

pub(super) fn calculate_java_zman(
    case: TestCase,

    zman: &'static ZmanPreset,
) -> Result<Option<ZmanResult>, Box<dyn Error>> {
    java_vm().attach_current_thread(|env: &mut jni::Env<'_>| -> Result<Option<ZmanResult>, Box<dyn Error>> {
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
            jni_str!("setUseAstronomicalChatzosForOtherZmanim"),
            jni_sig!("(Z)V"),
            &[JValue::Bool(bool_to_jboolean(
                case.use_astronomical_chatzos_for_other_zmanim,
            ))],
        )?;

        env.call_method(
            &calendar,
            jni_str!("setUseAstronomicalChatzos"),
            jni_sig!("(Z)V"),
            &[JValue::Bool(bool_to_jboolean(case.use_astronomical_chatzos))],
        )?;

        env.call_method(
            &calendar,
            jni_str!("setAteretTorahSunsetOffset"),
            jni_sig!("(D)V"),
            &[JValue::Double(case.ateret_torah_sunset_offset_minutes as f64)],
        )?;

        let instant = env
            .call_method(
                &calendar,
                JNIString::from(zman.method_name),
                jni_sig!("()Ljava/time/Instant;"),
                &[],
            )?
            .l()?;

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
    })
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
        .call_method(&zoned, jni_str!("toString"), jni_sig!("()Ljava/lang/String;"), &[])?
        .l()?;

    let text = env.cast_local::<JString>(text)?;

    Ok(text.to_string())
}

pub(crate) fn bool_to_jboolean(value: bool) -> jboolean {
    if value { JNI_TRUE } else { JNI_FALSE }
}
