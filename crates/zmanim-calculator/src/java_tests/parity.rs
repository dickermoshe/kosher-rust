//! Compare Java and Rust zman results.

use std::{error::Error, str::FromStr};

use chrono::{Duration, NaiveDate};
use chrono_tz::Tz;

use crate::{
    calculator::ZmanLike,
    java_tests::policy::{max_diff_ms_for_preset, DEFAULT_MAX_DIFF_MS},
    presets::{ZmanPreset, ALL},
    types::{config::CalculatorConfig, location::Location},
};

use super::{
    cases::{regression_case_literal, RegressionCase},
    jni::{calculate_java_zman_batch, JavaZmanResult},
    policy,
};

pub(crate) type TestResult<T = ()> = Result<T, Box<dyn Error>>;

/// Identifies one random test case so a failure can be replayed.
#[derive(Clone, Copy, Debug)]
pub(crate) struct RandomRun {
    pub(crate) seed: u64,
    pub(crate) iteration: u64,
}

pub(crate) fn assert_case_matches_java(
    case: RegressionCase,
    random_run: Option<RandomRun>,
) -> TestResult {
    let java = calculate_java_zman(case)?;
    let rust = calculate_rust_zman(case)?;
    let replay_message = format_replay_message(case, random_run);
    assert_results_match(case, java, rust, &replay_message)
}

pub(crate) fn assert_presets_match_java(
    base_case: RegressionCase,
    preset_names: &[&'static str],
    random_run: Option<RandomRun>,
) -> TestResult {
    let java_results = calculate_java_zman_batch(base_case, preset_names)?;

    for (preset_name, java) in preset_names.iter().copied().zip(java_results.into_iter()) {
        let case = RegressionCase {
            preset_name,
            ..base_case
        };
        let rust = calculate_rust_zman(case)?;
        let replay_message = format_replay_message(case, random_run);
        assert_results_match(case, java, rust, &replay_message)?;
    }

    Ok(())
}

fn calculate_java_zman(case: RegressionCase) -> TestResult<Option<ZmanResult>> {
    let mut results = calculate_java_zman_batch(case, &[case.preset_name])?;
    Ok(results
        .pop()
        .expect("single-preset batch should return one result"))
}

type ZmanResult = JavaZmanResult;

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

fn format_replay_message(case: RegressionCase, random_run: Option<RandomRun>) -> String {
    random_run
        .map(|run| {
            format!(
                "seed={} iteration={}\nPaste into REGRESSION_CASES:\n{},",
                run.seed,
                run.iteration,
                regression_case_literal(case)
            )
        })
        .unwrap_or_default()
}

fn assert_results_match(
    case: RegressionCase,
    java: Option<ZmanResult>,
    rust: Option<ZmanResult>,
    replay_message: &str,
) -> TestResult {
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
