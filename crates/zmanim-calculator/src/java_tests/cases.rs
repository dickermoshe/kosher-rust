//! Case generation and regression fixtures for Java parity tests.

use chrono::Month;
use rand::rngs::StdRng;
use std::{collections::HashSet, str::FromStr, sync::OnceLock};

use chrono_tz::Tz;
use rand::RngExt;
use tzf_rs::DefaultFinder;

use super::{jni::java_supported_timezones, policy};

/// Complete input needed to evaluate one preset against both Java and Rust.
#[derive(Clone, Copy, Debug)]
pub(crate) struct TestCase {
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

impl TestCase {
    /// Formats this case as a Rust literal suitable for [`REGRESSION_CASES`].
    pub fn code_literal(&self) -> String {
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
    /// Generates one randomized input shared by the Java and Rust calculators.
    pub fn random(rng: &mut StdRng, preset_name: &'static str) -> Self {
        let max_latitude = policy::max_latitude_for_preset(preset_name);
        let (year_start, year_end) = policy::random_year_range_for_preset(preset_name);

        for _ in 0..policy::MAX_TIMEZONE_ATTEMPTS {
            let year = rng.random_range(year_start..=year_end);
            let month: u32 = rng.random_range(1..=12);
            let days_in_month = Month::try_from(month as u8)
                .unwrap()
                .num_days(year)
                .unwrap() as u32;
            let day: u32 = rng.random_range(1..=days_in_month);
            let latitude = rng.random_range(-max_latitude..=max_latitude);
            let longitude = rng.random_range(-179.999_999..=179.999_999);
            let Some(timezone) = timezone_for_coordinates(longitude, latitude) else {
                continue;
            };

            return TestCase {
                year,
                month,
                day,
                latitude,
                longitude,
                elevation: rng.random_range(0.0..=policy::MAX_RANDOM_ELEVATION_METERS),
                timezone,
                preset_name,
                ateret_torah_sunset_offset_minutes: rng.random_range(0..60),
                candle_lighting_offset_minutes: rng.random_range(0..60),
                use_astronomical_chatzos_for_other_zmanim: rng.random_bool(0.5),
                use_elevation: rng.random_bool(0.5),
            };
        }

        panic!(
            "failed to find a supported timezone after {} attempts",
            policy::MAX_TIMEZONE_ATTEMPTS
        )
    }
}

static TIMEZONE_FINDER: OnceLock<DefaultFinder> = OnceLock::new();
static SHARED_TIMEZONES: OnceLock<HashSet<String>> = OnceLock::new();

/// Returns a shared Java/Rust timezone for the given coordinates.
///
/// `None` means the coordinate lookup produced a timezone name that either Java
/// or `chrono-tz` cannot use.
fn timezone_for_coordinates(longitude: f64, latitude: f64) -> Option<&'static str> {
    let timezone_name = TIMEZONE_FINDER
        .get_or_init(DefaultFinder::new)
        .get_tz_name(longitude, latitude);
    let timezone = Tz::from_str(timezone_name).ok()?;
    let timezone_name = timezone.name();
    let shared_timezones = SHARED_TIMEZONES.get_or_init(|| {
        let java_timezones = java_supported_timezones();

        chrono_tz::TZ_VARIANTS
            .iter()
            .map(|timezone| timezone.name().to_string())
            .filter(|timezone| java_timezones.contains(timezone))
            .collect()
    });
    if let Some(timezone) = shared_timezones.get(timezone_name) {
        Some(timezone)
    } else {
        None
    }
}

/// Deterministic cases captured from past random failures or known edge cases.
pub(crate) const REGRESSION_CASES: &[TestCase] = &[
    TestCase {
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
    TestCase {
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
    TestCase {
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
    TestCase {
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
    TestCase {
        year: 2037,
        month: 12,
        day: 29,
        latitude: -32.93056753553307,
        longitude: -125.36776050346323,
        elevation: 0.0,
        timezone: "Etc/GMT+8",
        preset_name: "getChatzos",
        ateret_torah_sunset_offset_minutes: 21,
        candle_lighting_offset_minutes: 9,
        use_astronomical_chatzos_for_other_zmanim: true,
        use_elevation: false,
    },
    TestCase {
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
    TestCase {
        year: 1915,
        month: 9,
        day: 5,
        latitude: 38.30176121408502,
        longitude: -126.08439530892548,
        elevation: 1563.902186131589,
        timezone: "Etc/GMT+8",
        preset_name: "getSofZmanShma3HoursBeforeChatzos",
        ateret_torah_sunset_offset_minutes: 2,
        candle_lighting_offset_minutes: 44,
        use_astronomical_chatzos_for_other_zmanim: false,
        use_elevation: false,
    },
    TestCase {
        year: 2000,
        month: 1,
        day: 15,
        latitude: 21.743167270756928,
        longitude: 27.55334330024661,
        elevation: 486.715389268106,
        timezone: "Africa/Khartoum",
        preset_name: "getFixedLocalChatzos",
        ateret_torah_sunset_offset_minutes: 23,
        candle_lighting_offset_minutes: 36,
        use_astronomical_chatzos_for_other_zmanim: false,
        use_elevation: false,
    },
    TestCase {
        year: 2065,
        month: 4,
        day: 20,
        latitude: -30.49487088258978,
        longitude: -26.02045885793035,
        elevation: 2531.3575346139123,
        timezone: "Etc/GMT+2",
        preset_name: "getSofZmanBiurChametzMGA16Point1Degrees",
        ateret_torah_sunset_offset_minutes: 6,
        candle_lighting_offset_minutes: 14,
        use_astronomical_chatzos_for_other_zmanim: true,
        use_elevation: true,
    },
];
