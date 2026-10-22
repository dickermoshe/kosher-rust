//! A C ABI for kosher-rust, and through WeaveFFI, bindings for Python, Node, Swift, Kotlin,
//! Dart, .NET and the rest.
//!
//! This crate is a thin, FFI-shaped facade: everything crosses the boundary as primitives,
//! strings, records and reference-counted objects, so it re-states the parts of the API it
//! exposes rather than tagging kosher-rust's own types.
//!
//! - Times are milliseconds since the Unix epoch (UTC).
//! - Time zones are IANA names such as `"America/New_York"`.
//! - A [`Zman`](kosher_rust::Zman) is named by its KosherJava getter, for example
//!   `"getSofZmanShmaGRA"`; [`kosher::zmanim_catalog`] lists them.

#[weaveffi::module]
pub mod kosher {

    use jiff::{civil, tz::TimeZone};
    use kosher_rust::{
        calendar::{self, HebrewYear},
        zmanim::{self, AstronomicalCalculator},
    };

    /// Why a call failed.
    #[weaveffi::error]
    #[derive(Debug)]
    #[repr(i32)]
    pub enum KosherError {
        /// The latitude is not between -90 and 90 degrees.
        InvalidLatitude = 1,
        /// The longitude is not between -180 and 180 degrees.
        InvalidLongitude = 2,
        /// An elevation is negative or not a finite number.
        InvalidElevation = 3,
        /// The time zone is not a known IANA name, or one is needed and none was given.
        InvalidTimeZone = 4,
        /// The date does not exist.
        InvalidDate = 5,
        /// No zman has this KosherJava name.
        UnknownZman = 6,
        /// The sun stays above the angle the time needs all day.
        AllDay = 10,
        /// The sun stays below the angle the time needs all day.
        AllNight = 11,
        /// This time does not apply on this date.
        NotApplicable = 12,
        /// This time needs a location with a time zone.
        TimeZoneRequired = 13,
        /// Internal calculation error; please report the location and date.
        Internal = 14,
    }

    impl std::fmt::Display for KosherError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(match self {
                Self::InvalidLatitude => "latitude must be between -90 and 90 degrees",
                Self::InvalidLongitude => "longitude must be between -180 and 180 degrees",
                Self::InvalidElevation => "elevation must be a finite number of meters, 0 or more",
                Self::InvalidTimeZone => "unknown or missing time zone",
                Self::InvalidDate => "no such date",
                Self::UnknownZman => "no zman has this name",
                Self::AllDay => "the sun does not go low enough for this time on this date",
                Self::AllNight => "the sun does not rise high enough for this time on this date",
                Self::NotApplicable => "this time does not apply on this date",
                Self::TimeZoneRequired => "this time needs a location with a time zone",
                Self::Internal => "internal calculation error; please report the location and date",
            })
        }
    }

    impl From<kosher_rust::LocationError> for KosherError {
        fn from(error: kosher_rust::LocationError) -> Self {
            use kosher_rust::LocationError as E;
            match error {
                E::InvalidLatitude => Self::InvalidLatitude,
                E::InvalidLongitude => Self::InvalidLongitude,
                E::InvalidElevation => Self::InvalidElevation,
                E::TimeZoneRequired => Self::InvalidTimeZone,
            }
        }
    }

    impl From<zmanim::ZmanimError> for KosherError {
        fn from(error: zmanim::ZmanimError) -> Self {
            use zmanim::ZmanimError as E;
            match error {
                E::AllDay => Self::AllDay,
                E::AllNight => Self::AllNight,
                E::NotApplicable => Self::NotApplicable,
                E::TimeZoneRequired => Self::TimeZoneRequired,
                E::Internal => Self::Internal,
            }
        }
    }

    /// Israel (one day of Yom Tov) or the diaspora (two).
    #[weaveffi::enumeration]
    #[repr(i32)]
    pub enum Region {
        /// Eretz Yisrael.
        Israel = 0,
        /// Everywhere else.
        Diaspora = 1,
    }

    impl From<Region> for calendar::Region {
        fn from(region: Region) -> Self {
            match region {
                Region::Israel => Self::Israel,
                Region::Diaspora => Self::Diaspora,
            }
        }
    }

    /// A month of the Hebrew calendar, in order from Tishrei. `Adar` is Adar II in a leap year.
    #[weaveffi::enumeration]
    #[repr(i32)]
    pub enum HebrewMonth {
        Tishrei = 1,
        Cheshvan = 2,
        Kislev = 3,
        Teves = 4,
        Shevat = 5,
        AdarI = 6,
        Adar = 7,
        Nisan = 8,
        Iyar = 9,
        Sivan = 10,
        Tammuz = 11,
        Av = 12,
        Elul = 13,
    }

    impl From<HebrewMonth> for calendar::HebrewMonth {
        fn from(month: HebrewMonth) -> Self {
            use calendar::HebrewMonth as M;
            match month {
                HebrewMonth::Tishrei => M::Tishrei,
                HebrewMonth::Cheshvan => M::Cheshvan,
                HebrewMonth::Kislev => M::Kislev,
                HebrewMonth::Teves => M::Teves,
                HebrewMonth::Shevat => M::Shevat,
                HebrewMonth::AdarI => M::AdarI,
                HebrewMonth::Adar => M::Adar,
                HebrewMonth::Nisan => M::Nisan,
                HebrewMonth::Iyar => M::Iyar,
                HebrewMonth::Sivan => M::Sivan,
                HebrewMonth::Tammuz => M::Tammuz,
                HebrewMonth::Av => M::Av,
                HebrewMonth::Elul => M::Elul,
            }
        }
    }

    impl From<calendar::HebrewMonth> for HebrewMonth {
        fn from(month: calendar::HebrewMonth) -> Self {
            use calendar::HebrewMonth as M;
            match month {
                M::Tishrei => Self::Tishrei,
                M::Cheshvan => Self::Cheshvan,
                M::Kislev => Self::Kislev,
                M::Teves => Self::Teves,
                M::Shevat => Self::Shevat,
                M::AdarI => Self::AdarI,
                M::Adar => Self::Adar,
                M::Nisan => Self::Nisan,
                M::Iyar => Self::Iyar,
                M::Sivan => Self::Sivan,
                M::Tammuz => Self::Tammuz,
                M::Av => Self::Av,
                M::Elul => Self::Elul,
            }
        }
    }

    /// A place on earth. Elevations are in meters; leave them 0 for sea level.
    #[weaveffi::record]
    pub struct Location {
        /// Degrees, north positive.
        pub latitude: f64,
        /// Degrees, east positive.
        pub longitude: f64,
        /// IANA time zone name. Needed beyond 150 degrees from Greenwich and for Kiddush Levana.
        pub timezone: Option<String>,
        /// The observer's height above sea level.
        pub elevation: f64,
        /// Height of the eastern horizon.
        pub eastern_horizon_elevation: f64,
        /// Height of the western horizon.
        pub western_horizon_elevation: f64,
    }

    impl Location {
        fn to_kosher(&self) -> Result<kosher_rust::Location, KosherError> {
            let timezone = match &self.timezone {
                Some(name) => {
                    Some(TimeZone::get(name).map_err(|_| KosherError::InvalidTimeZone)?)
                }
                None => None,
            };
            Ok(
                kosher_rust::Location::new(self.latitude, self.longitude, timezone)?
                    .with_elevation(self.elevation)?
                    .with_horizons(
                        self.eastern_horizon_elevation,
                        self.western_horizon_elevation,
                    )?,
            )
        }
    }

    /// The solar position algorithm.
    #[weaveffi::enumeration]
    #[repr(i32)]
    pub enum Calculator {
        /// NOAA (KosherJava's default).
        Noaa = 0,
        /// Jean Meeus VSOP87, with the delta-T correction.
        Meeus = 1,
        /// NREL SPA, with the delta-T correction.
        Spa = 2,
    }

    /// How zmanim are calculated. [`default_config`] matches KosherJava's defaults.
    #[weaveffi::record]
    pub struct Config {
        /// Minutes before sunset for candle lighting.
        pub candle_lighting_minutes: i64,
        /// Divide the day at astronomical chatzos for proportional zmanim.
        pub use_astronomical_chatzos_for_other_zmanim: bool,
        /// Let elevation adjust every zman built on sunrise and sunset.
        pub use_elevation: bool,
        /// Minutes after sunset for Tzais Ateret Torah.
        pub ateret_torah_sunset_minutes: i64,
        /// Chatzos is the sun's transit rather than the midpoint of sunrise and sunset.
        pub use_astronomical_chatzos: bool,
        /// The solar position algorithm.
        pub calculator: Calculator,
    }

    impl Config {
        fn to_kosher(&self) -> zmanim::Config {
            zmanim::Config {
                candle_lighting_offset: jiff::SignedDuration::from_mins(
                    self.candle_lighting_minutes,
                ),
                use_astronomical_chatzos_for_other_zmanim: self
                    .use_astronomical_chatzos_for_other_zmanim,
                use_elevation: self.use_elevation,
                ateret_torah_sunset_offset: jiff::SignedDuration::from_mins(
                    self.ateret_torah_sunset_minutes,
                ),
                use_astronomical_chatzos: self.use_astronomical_chatzos,
                astronomical_calculator: match self.calculator {
                    Calculator::Noaa => AstronomicalCalculator::Noaa,
                    Calculator::Meeus => AstronomicalCalculator::Meeus {
                        apply_delta_t: true,
                    },
                    Calculator::Spa => AstronomicalCalculator::Spa {
                        apply_delta_t: true,
                    },
                },
            }
        }
    }

    /// KosherJava's default configuration.
    #[weaveffi::export]
    pub fn default_config() -> Config {
        let d = zmanim::Config::default();
        Config {
            candle_lighting_minutes: d.candle_lighting_offset.as_mins(),
            use_astronomical_chatzos_for_other_zmanim: d.use_astronomical_chatzos_for_other_zmanim,
            use_elevation: d.use_elevation,
            ateret_torah_sunset_minutes: d.ateret_torah_sunset_offset.as_mins(),
            use_astronomical_chatzos: d.use_astronomical_chatzos,
            calculator: Calculator::Noaa,
        }
    }

    /// One entry of the zmanim catalog.
    #[weaveffi::record]
    pub struct ZmanInfo {
        /// The KosherJava getter, which names the zman in [`Zmanim::get`].
        pub java_name: String,
        /// The display name, for example "Sof Zman Shma (GR'A)".
        pub name: String,
        /// Whether KosherJava deprecates it.
        pub deprecated: bool,
    }

    /// Every zman this library can calculate.
    #[weaveffi::export]
    pub fn zmanim_catalog() -> Vec<ZmanInfo> {
        kosher_rust::Zman::ALL
            .into_iter()
            .map(|zman| ZmanInfo {
                java_name: zman.java_name().into(),
                name: zman.name().into(),
                deprecated: zman.is_deprecated(),
            })
            .collect()
    }

    fn civil_date(year: i32, month: i32, day: i32) -> Result<civil::Date, KosherError> {
        let year = i16::try_from(year).map_err(|_| KosherError::InvalidDate)?;
        let month = i8::try_from(month).map_err(|_| KosherError::InvalidDate)?;
        let day = i8::try_from(day).map_err(|_| KosherError::InvalidDate)?;
        civil::Date::new(year, month, day).map_err(|_| KosherError::InvalidDate)
    }

    /// The zmanim for a location on a Gregorian date.
    #[weaveffi::interface]
    pub struct Zmanim {
        inner: zmanim::Zmanim,
    }

    impl Zmanim {
        /// The zmanim at `location` on `year`-`month`-`day`, with `config`.
        pub fn new(
            location: Location,
            year: i32,
            month: i32,
            day: i32,
            config: Config,
        ) -> Result<Self, KosherError> {
            Ok(Self {
                inner: zmanim::Zmanim::with_config(
                    location.to_kosher()?,
                    civil_date(year, month, day)?,
                    config.to_kosher(),
                ),
            })
        }

        /// The time of the zman named `java_name`, in milliseconds since the Unix epoch.
        pub fn get(&self, java_name: &str) -> Result<i64, KosherError> {
            let zman =
                kosher_rust::Zman::from_java_name(java_name).ok_or(KosherError::UnknownZman)?;
            Ok(self.inner.get(zman)?.as_millisecond())
        }

        /// When the sun is `degrees` below the horizon before sunrise, in Unix milliseconds.
        pub fn sunrise_offset_by_degrees(&self, degrees: f64) -> Result<i64, KosherError> {
            Ok(self.inner.sunrise_offset_by_degrees(degrees)?.as_millisecond())
        }

        /// When the sun is `degrees` below the horizon after sunset, in Unix milliseconds.
        pub fn sunset_offset_by_degrees(&self, degrees: f64) -> Result<i64, KosherError> {
            Ok(self.inner.sunset_offset_by_degrees(degrees)?.as_millisecond())
        }
    }

    /// A day on the Hebrew calendar.
    #[weaveffi::interface]
    pub struct HebrewDate {
        inner: calendar::HebrewDate,
    }

    impl HebrewDate {
        /// The Hebrew date `day` `month` `year`.
        pub fn new(year: i32, month: HebrewMonth, day: i32) -> Result<Self, KosherError> {
            let day = u8::try_from(day).map_err(|_| KosherError::InvalidDate)?;
            calendar::HebrewDate::new(year, month.into(), day)
                .map(|inner| Self { inner })
                .ok_or(KosherError::InvalidDate)
        }

        /// The Hebrew date of the Gregorian `year`-`month`-`day`.
        pub fn from_gregorian(year: i32, month: i32, day: i32) -> Result<Self, KosherError> {
            Ok(Self {
                inner: civil_date(year, month, day)?.into(),
            })
        }

        /// The Hebrew year, for example 5785.
        pub fn year(&self) -> i32 {
            self.inner.year().get()
        }

        /// The month.
        pub fn month(&self) -> HebrewMonth {
            self.inner.month().into()
        }

        /// The day of the month, from 1.
        pub fn day(&self) -> i32 {
            i32::from(self.inner.day())
        }

        /// Whether the year has 13 months.
        pub fn is_leap_year(&self) -> bool {
            HebrewYear::new(self.inner.year().get()).is_leap()
        }

        /// The date `days` later (or earlier, if negative).
        pub fn add_days(&self, days: i32) -> Result<Self, KosherError> {
            self.inner
                .add_days(days)
                .map(|inner| Self { inner })
                .ok_or(KosherError::InvalidDate)
        }

        /// The holidays on this day, by English name.
        pub fn holidays(&self, region: Region) -> Vec<String> {
            self.inner
                .holidays(region.into())
                .map(|holiday| holiday.to_string())
                .collect()
        }

        /// Whether melacha is forbidden: Shabbos or Yom Tov.
        pub fn is_assur_bemelacha(&self, region: Region) -> bool {
            self.inner.is_assur_bemelacha(region.into())
        }

        /// The weekly parsha read this Shabbos, by English name.
        pub fn parsha(&self, region: Region) -> Option<String> {
            self.inner.parsha(region.into()).map(|p| p.to_string())
        }

        /// The date, for example "15 Nisan 5784".
        pub fn display(&self) -> String {
            self.inner.to_string()
        }
    }
}

weaveffi::export_runtime!();
