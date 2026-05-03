//! Calculation logic for zmanim.
//!
//! [`ZmanimCalculator`] is the main entry point: construct one with [`ZmanimCalculator::new`],
//! then pass any [`ZmanLike`] implementor (typically a value from [`crate::presets`]) to
//! [`ZmanimCalculator::calculate`] to obtain a [`jiff::Timestamp`].

use crate::types::{
    config::CalculatorConfig,
    error::{IntoDateTimeResult, ZmanimError},
    location::Location,
};
use astronomical_calculator::{AstronomicalCalculator, Refraction};
#[allow(unused_imports)]
use core_maths::*;
use jiff::{
    civil::Date,
    tz::{AmbiguousOffset, TimeZone},
    SignedDuration, Timestamp,
};

/// Calculates zmanim for a given [`Location`] and [`Date`].
///
/// Construct once for a location/date pair, then call [`ZmanimCalculator::calculate`]
/// with one or more values implementing [`ZmanLike`].
///
/// Most users should pass one of the ready-made definitions from [`crate::presets`]
/// (for example `presets::SUNRISE`) instead of implementing custom zman logic.
#[derive(Clone, Debug)]
pub struct ZmanimCalculator {
    /// The location to calculate for.
    pub(crate) location: Location,
    /// The civil date at `location` for which zmanim are calculated.
    pub(crate) date: Date,
    /// Calculation configuration options.
    pub(crate) config: CalculatorConfig,
    pub(crate) elevation_adjusted_calculator: AstronomicalCalculator,
    pub(crate) sea_level_calculator: AstronomicalCalculator,
    pub(crate) sea_level_calculator_no_refraction: AstronomicalCalculator,
}

impl ZmanimCalculator {
    /// Creates a new calculator for the given `location`, `date`, and `config`.
    ///
    /// Use this as your main entry point before calculating any zmanim.
    ///
    /// # Errors
    ///
    /// Returns an error when the calculator cannot be initialized from the provided
    /// location/date/config values.
    pub fn new(
        location: Location,
        date: Date,
        config: CalculatorConfig,
    ) -> Result<Self, ZmanimError> {
        // Regular test builds use NOAA refraction to line up with KosherJava's
        // default calculator. The private test feature below opts tests back
        // into the same SPA/Bennett refraction model used by production builds.
        #[cfg(all(test, not(feature = "__test-spa-refraction")))]
        let refraction = Refraction::NOAA;
        #[cfg(any(not(test), feature = "__test-spa-refraction"))]
        let refraction = Refraction::ApSolposBennet;
        let localnoon = Self::local_noon(date, &location)?;
        let elevation_adjusted_calculator = AstronomicalCalculator::new(
            localnoon,
            None,
            0.0,
            location.longitude,
            location.latitude,
            location.elevation,
            22.0,
            1013.25,
            None,
            refraction,
        )
        .map_err(ZmanimError::AstronomicalCalculatorError)?;
        let sea_level_calculator = AstronomicalCalculator::new(
            localnoon,
            None,
            0.0,
            location.longitude,
            location.latitude,
            0.0,
            22.0,
            1013.25,
            None,
            refraction,
        )
        .map_err(ZmanimError::AstronomicalCalculatorError)?;
        let sea_level_calculator_no_refraction = AstronomicalCalculator::new(
            localnoon,
            None,
            0.0,
            location.longitude,
            location.latitude,
            0.0,
            22.0,
            1013.25,
            None,
            Refraction::NoRefraction,
        )
        .map_err(ZmanimError::AstronomicalCalculatorError)?;
        Ok(Self {
            location,
            date,
            config,
            elevation_adjusted_calculator,
            sea_level_calculator,
            sea_level_calculator_no_refraction,
        })
    }

    /// Calculates a single zman using the current calculator state.
    ///
    /// Pass any value implementing [`ZmanLike`] and receive the resulting instant in UTC.
    ///
    /// This method takes `&mut self` so repeated calls can reuse internal intermediate
    /// computation state for better performance.
    ///
    /// If borrow rules make your call sites awkward, clone the calculator and use each
    /// clone independently (for example, one clone for sunrise and another for sunset).
    pub fn calculate(&mut self, zman: &impl ZmanLike) -> Result<Timestamp, ZmanimError> {
        zman.calculate(self)
    }

    fn local_noon(date: Date, location: &Location) -> Result<Timestamp, ZmanimError> {
        // Preferred: convert 12:00:00 in the location's timezone to UTC.
        if let Some(tz) = location.timezone.as_ref() {
            let result = tz.to_ambiguous_timestamp(date.at(12, 0, 0, 0));
            match result.offset() {
                AmbiguousOffset::Unambiguous { .. } | AmbiguousOffset::Fold { .. } => {
                    return result
                        .earlier()
                        .map_err(|_| ZmanimError::TimeConversionError);
                }
                // Noon falls inside a DST gap on this date; fall through to the longitude estimate.
                AmbiguousOffset::Gap { .. } => {}
            }
        }

        // Fallback: estimate UTC noon from longitude (4 min per degree).
        // Not valid near the anti-meridian where the date itself is ambiguous.
        if !Location::near_anti_meridian(location.longitude) {
            let utc_noon = date
                .at(12, 0, 0, 0)
                .to_zoned(TimeZone::UTC)
                .map_err(|_| ZmanimError::TimeConversionError)?
                .timestamp();
            let offset = SignedDuration::from_secs((location.longitude * 4.0 * 60.0) as i64);
            return utc_noon
                .checked_sub(offset)
                .map_err(|_| ZmanimError::TimeConversionError);
        }

        Err(ZmanimError::LocalNoonError)
    }

    /// Converts local mean time (LMT) hours for a date/location into UTC.
    ///
    /// `hours` is interpreted in the half-open range `[0.0, 24.0)`, where:
    /// - `0.0` is local mean midnight
    /// - `12.0` is local mean noon
    ///
    /// # Errors
    ///
    /// Returns:
    /// - [`ZmanimError::InvalidHours`] when `hours` is outside `[0.0, 24.0)`,
    /// - [`ZmanimError::TimeConversionError`] if midnight construction fails.
    pub(crate) fn local_mean_time(
        &mut self,
        date: Date,
        location: &Location,
        hours: f64,
    ) -> Result<Timestamp, ZmanimError> {
        if !(0.0..24.0).contains(&hours) {
            return Err(ZmanimError::InvalidHours);
        }

        let midnight = date.at(0, 0, 0, 0);
        let lmt_nanos = (hours * 3600.0 * 1_000_000_000.0).round() as i64;
        let offset_nanos = (location.longitude * 240.0 * 1_000_000_000.0).round() as i64;
        let lmt_dt = midnight
            .checked_add(SignedDuration::from_nanos(lmt_nanos))
            .and_then(|dt| dt.checked_sub(SignedDuration::from_nanos(offset_nanos)))
            .map_err(|_| ZmanimError::TimeConversionError)?;
        let mut utc = lmt_dt
            .to_zoned(TimeZone::UTC)
            .map_err(|_| ZmanimError::TimeConversionError)?
            .timestamp();

        if let Some(timezone) = &location.timezone {
            for _ in 0..4 {
                let local_date = utc.to_zoned(timezone.clone()).date();
                if local_date == date {
                    break;
                }
                if local_date > date {
                    utc -= SignedDuration::from_hours(24);
                } else {
                    utc += SignedDuration::from_hours(24);
                }
            }
        }

        Ok(utc)
    }
    pub(crate) fn configured_calculator(&mut self) -> &mut AstronomicalCalculator {
        if self.config.use_elevation {
            &mut self.elevation_adjusted_calculator
        } else {
            &mut self.sea_level_calculator
        }
    }
    pub(crate) fn elevation_adjusted_sunrise(&mut self) -> Result<Timestamp, ZmanimError> {
        self.elevation_adjusted_calculator
            .get_sunrise()
            .into_date_time_result()
    }
    pub(crate) fn elevation_adjusted_sunset(&mut self) -> Result<Timestamp, ZmanimError> {
        self.elevation_adjusted_calculator
            .get_sunset()
            .into_date_time_result()
    }
    pub(crate) fn sea_level_sunrise(&mut self) -> Result<Timestamp, ZmanimError> {
        self.sea_level_calculator
            .get_sea_level_sunrise()
            .into_date_time_result()
    }
    pub(crate) fn sea_level_sunset(&mut self) -> Result<Timestamp, ZmanimError> {
        self.sea_level_calculator
            .get_sea_level_sunset()
            .into_date_time_result()
    }
    pub(crate) fn solar_transit(&mut self) -> Result<Timestamp, ZmanimError> {
        // Solar transit calculations do not use elevation, so the calculator used here is irrelevant
        self.elevation_adjusted_calculator
            .get_solar_transit()
            .into_date_time_result()
    }
    pub(crate) fn solar_midnight(&mut self) -> Result<Timestamp, ZmanimError> {
        // Solar midnight calculations do not use elevation, so the calculator used here is irrelevant
        self.elevation_adjusted_calculator
            .get_next_solar_midnight()
            .into_date_time_result()
    }
}

/// A value that can be calculated by a [`ZmanimCalculator`].
///
/// Most consumers should use predefined values from [`crate::presets`]. Implement this trait
/// when you need a custom zman definition not already provided there.
pub trait ZmanLike {
    /// Computes this zman for the current calculator state.
    ///
    /// Implement this trait for custom zman definitions that can run through
    /// [`ZmanimCalculator::calculate`].
    fn calculate(&self, calculator: &mut ZmanimCalculator) -> Result<Timestamp, ZmanimError>;
}

#[cfg(feature = "defmt")]
impl defmt::Format for ZmanimCalculator {
    fn format(&self, fmt: defmt::Formatter) {
        let y = self.date.year();
        let m = self.date.month();
        let d = self.date.day();
        defmt::write!(
            fmt,
            "ZmanimCalculator {{ location: {}, date: {=i16}-{=i8}-{=i8}, config: {} }}",
            self.location,
            y,
            m,
            d,
            self.config
        )
    }
}
