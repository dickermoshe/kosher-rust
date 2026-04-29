//! Compare Java and Rust zman results.

use std::{error::Error, str::FromStr};

use chrono::{Duration, NaiveDate};
use chrono_tz::Tz;

use crate::{
    calculator::ZmanLike,
    java_tests::{jni::calculate_java_zman, policy::max_diff_ms_for_preset},
    prelude::ZmanimError,
    presets::ZmanPreset,
    types::{config::CalculatorConfig, location::Location},
};

use super::{cases::TestCase, policy};

#[derive(Clone, Debug)]
pub(crate) struct ZmanResult {
    pub(crate) formatted: String,
    pub(crate) timestamp_ms: i64,
}

/// Identifies one random test case so a failure can be replayed.
#[derive(Clone, Copy, Debug)]
pub(crate) struct CaseRun {
    pub(crate) seed: u64,
    pub(crate) iteration: u64,
}

pub(crate) fn run_test_case(
    case: TestCase,
    preset: &'static ZmanPreset<'static>,
    random_run: Option<CaseRun>,
) -> Result<(), Box<dyn Error>> {
    let java = calculate_java_zman(case, preset)?;
    let rust = calculate_rust_zman(case, preset)?;
    assert_results_match(case, java, rust, &format_message(case, random_run))
}

fn calculate_rust_zman(
    case: TestCase,
    preset: &'static ZmanPreset<'static>,
) -> Result<Option<ZmanResult>, ZmanimError> {
    let timezone = Tz::from_str(case.timezone).unwrap();
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

fn format_message(case: TestCase, case_run: Option<CaseRun>) -> String {
    case_run
        .map(|run| {
            format!(
                "seed={} iteration={}\nPaste into REGRESSION_CASES:\n{},",
                run.seed,
                run.iteration,
                case.code_literal()
            )
        })
        .unwrap_or_default()
}

fn assert_results_match(
    case: TestCase,
    java: Option<ZmanResult>,
    rust: Option<ZmanResult>,
    replay_message: &str,
) -> Result<(), Box<dyn Error>> {
    match (java, rust) {
        (None, None) => Ok(()),
        (Some(java), Some(rust)) => {
            let difference = (java.timestamp_ms - rust.timestamp_ms).abs();
            let max_diff_ms = max_diff_ms_for_preset(case.preset_name);
            assert!(
                difference <= max_diff_ms,
                "zman mismatch for {} on {} at ({}, {}) in {}: java={} rust={} diff={}ms max={}ms\n{}",
                case.preset_name,
                NaiveDate::from_ymd_opt(case.year, case.month, case.day).unwrap(),
                case.latitude,
                case.longitude,
                case.timezone,
                java.formatted,
                rust.formatted,
                difference,
                max_diff_ms,
                replay_message
            );
            Ok(())
        }
        (java, rust) if policy::allows_intentional_null_mismatch(case.preset_name) => Ok(()),
        (java, rust) => panic!(
            "null mismatch for {} on {} in {}: java={:?} rust={:?}\n{}",
            case.preset_name,
            NaiveDate::from_ymd_opt(case.year, case.month, case.day).unwrap(),
            case.timezone,
            java,
            rust,
            replay_message
        ),
    }
}
