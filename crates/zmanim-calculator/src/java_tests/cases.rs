//! Shared case definitions for deterministic and replayable Java parity tests.

/// Complete input needed to evaluate one preset against both Java and Rust.
#[derive(Clone, Copy, Debug)]
pub(crate) struct RegressionCase {
    pub(crate) year: i32,
    pub(crate) month: u32,
    pub(crate) day: u32,
    pub(crate) latitude: f64,
    pub(crate) longitude: f64,
    pub(crate) elevation: f64,
    pub(crate) timezone: &'static str,
    pub(crate) preset_name: &'static str,
    pub(crate) ateret_torah_sunset_offset_minutes: i64,
    pub(crate) candle_lighting_offset_minutes: i64,
    pub(crate) use_astronomical_chatzos_for_other_zmanim: bool,
    pub(crate) use_elevation: bool,
}

/// Hand-picked cases promoted out of randomized failures or known boundary bugs.
pub(crate) const REGRESSION_CASES: &[RegressionCase] = &[
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
    RegressionCase {
        year: 2037,
        month: 12,
        day: 29,
        latitude: -32.93056753553307,
        longitude: -125.36776050346323,
        elevation: 940.4531699881416,
        timezone: "Etc/GMT+8",
        preset_name: "getSofZmanShma3HoursBeforeChatzos",
        ateret_torah_sunset_offset_minutes: 21,
        candle_lighting_offset_minutes: 9,
        use_astronomical_chatzos_for_other_zmanim: true,
        use_elevation: false,
    },
];

/// Formats a failing random case so it can be pasted directly into `REGRESSION_CASES`.
pub(crate) fn regression_case_literal(case: RegressionCase) -> String {
    format!(
        "RegressionCase {{\n    year: {},\n    month: {},\n    day: {},\n    latitude: {:?},\n    longitude: {:?},\n    elevation: {:?},\n    timezone: {:?},\n    preset_name: {:?},\n    ateret_torah_sunset_offset_minutes: {},\n    candle_lighting_offset_minutes: {},\n    use_astronomical_chatzos_for_other_zmanim: {},\n    use_elevation: {},\n}}",
        case.year,
        case.month,
        case.day,
        case.latitude,
        case.longitude,
        case.elevation,
        case.timezone,
        case.preset_name,
        case.ateret_torah_sunset_offset_minutes,
        case.candle_lighting_offset_minutes,
        case.use_astronomical_chatzos_for_other_zmanim,
        case.use_elevation
    )
}
