//! Shared types for zmanim Java parity tests.

#[derive(Clone, Copy, Debug)]
pub(super) struct TestCase {
    pub(super) year: i32,
    pub(super) month: u32,
    pub(super) day: u32,
    pub(super) latitude: f64,
    pub(super) longitude: f64,
    pub(super) elevation: f64,
    pub(super) timezone: &'static str,
    pub(super) preset_name: &'static str,
    pub(super) ateret_torah_sunset_offset_minutes: i64,
    pub(super) candle_lighting_offset_minutes: i64,
    pub(super) use_astronomical_chatzos_for_other_zmanim: bool,
    pub(super) use_elevation: bool,
}

impl TestCase {
    /// Formats this case as a Rust literal suitable for regression fixtures.
    pub(super) fn code_literal(&self) -> String {
        format!(
            "TestCase {{\n    year: {},\n    month: {},\n    day: {},\n    latitude: {:?},\n    longitude: {:?},\n    elevation: {:?},\n    timezone: {:?},\n    preset_name: {:?},\n    ateret_torah_sunset_offset_minutes: {},\n    candle_lighting_offset_minutes: {},\n    use_astronomical_chatzos_for_other_zmanim: {},\n    use_elevation: {},\n}}",
            self.year,
            self.month,
            self.day,
            self.latitude,
            self.longitude,
            self.elevation,
            self.timezone,
            self.preset_name,
            self.ateret_torah_sunset_offset_minutes,
            self.candle_lighting_offset_minutes,
            self.use_astronomical_chatzos_for_other_zmanim,
            self.use_elevation
        )
    }
}

#[derive(Clone, Debug)]
pub(super) struct ZmanResult {
    pub(super) formatted: String,
    pub(super) timestamp_ms: i64,
}

/// Identifies a randomized test input so a failure can be replayed.
#[derive(Clone, Copy, Debug)]
pub(super) struct CaseRun {
    pub(super) seed: u64,
    pub(super) iteration: u64,
}
