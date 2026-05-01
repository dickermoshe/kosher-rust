//! Assertions for zmanim Java parity tests.

use std::error::Error;

use chrono::NaiveDate;

use crate::presets::ZmanPreset;

use super::{
    java_reference::calculate_java_zman,
    policy,
    rust_reference::calculate_rust_zman,
    types::{CaseRun, TestCase, ZmanResult},
};

/// Calculates one case with both implementations and asserts policy-compliant parity.
pub(super) fn run_test_case(
    case: TestCase,
    preset: &'static ZmanPreset<'static>,
    random_run: Option<CaseRun>,
) -> Result<(), Box<dyn Error>> {
    let java = calculate_java_zman(case, preset)?;
    let rust = calculate_rust_zman(case, preset)?;

    assert_results_match(case, java, rust, random_run)
}

/// Asserts that the results match according to the policy.
fn assert_results_match(
    case: TestCase,
    java: Option<ZmanResult>,
    rust: Option<ZmanResult>,
    random_run: Option<CaseRun>,
) -> Result<(), Box<dyn Error>> {
    let run_info = random_run
        .map(|run| {
            format!(
                "seed={} iteration={}\nPaste into REGRESSION_CASES:\n{},",
                run.seed,
                run.iteration,
                case.code_literal()
            )
        })
        .unwrap_or_default();
    match (java, rust) {
        (None, None) => Ok(()),
        (Some(java), Some(rust)) => {
            let difference = (java.timestamp_ms - rust.timestamp_ms).abs();
            let max_diff_ms = policy::max_diff_ms_for_preset(case.preset_name);
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
                run_info
            );
            Ok(())
        }
        (_, _) if policy::allows_intentional_null_mismatch(case.preset_name) => Ok(()),
        (java, rust) => panic!(
            "null mismatch for {} on {} in {}: java={:?} rust={:?}\n{}",
            case.preset_name,
            NaiveDate::from_ymd_opt(case.year, case.month, case.day).unwrap(),
            case.timezone,
            java,
            rust,
            run_info
        ),
    }
}
