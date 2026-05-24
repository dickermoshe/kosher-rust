//! Calculation logic for zmanim.
//!
//! [`ZmanimCalculator`] is the main entry point: construct one with [`ZmanimCalculator::new`],
//! then pass any [`ZmanLike`] implementor (typically a value from [`crate::presets`]) to
//! [`ZmanimCalculator::calculate`] to obtain a [`jiff::Timestamp`].

use crate::types::{config::CalculatorConfig, error::ZmanimError, location::Location};
#[allow(unused_imports)]
use core_maths::*;
use jiff::{civil::Date, tz::TimeZone, SignedDuration, Timestamp};

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
        validate_location(&location)?;
        Ok(Self {
            location,
            date,
            config,
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

        let adjusted_date = crate::astronomy::adjusted_local_date(date, location)?;
        let midnight = adjusted_date.at(0, 0, 0, 0);
        let lmt_nanos = (hours * 3600.0 * 1_000_000_000.0).round() as i64;
        let offset_nanos = (location.longitude * 240.0 * 1_000_000_000.0).round() as i64;
        let lmt_dt = midnight
            .checked_add(SignedDuration::from_nanos(lmt_nanos))
            .and_then(|dt| dt.checked_sub(SignedDuration::from_nanos(offset_nanos)))
            .map_err(|_| ZmanimError::TimeConversionError)?;
        let utc = lmt_dt
            .to_zoned(TimeZone::UTC)
            .map_err(|_| ZmanimError::TimeConversionError)?
            .timestamp();

        Ok(utc)
    }
    pub(crate) fn configured_sunrise(&self) -> Result<Timestamp, ZmanimError> {
        if self.config.use_elevation {
            self.elevation_adjusted_sunrise()
        } else {
            self.sea_level_sunrise()
        }
    }
    pub(crate) fn configured_sunset(&self) -> Result<Timestamp, ZmanimError> {
        if self.config.use_elevation {
            self.elevation_adjusted_sunset()
        } else {
            self.sea_level_sunset()
        }
    }
    pub(crate) fn elevation_adjusted_sunrise(&self) -> Result<Timestamp, ZmanimError> {
        crate::astronomy::sunrise(self.date, &self.location, true)
    }
    pub(crate) fn elevation_adjusted_sunset(&self) -> Result<Timestamp, ZmanimError> {
        crate::astronomy::sunset(self.date, &self.location, true)
    }
    pub(crate) fn sea_level_sunrise(&self) -> Result<Timestamp, ZmanimError> {
        crate::astronomy::sunrise(self.date, &self.location, false)
    }
    pub(crate) fn sea_level_sunset(&self) -> Result<Timestamp, ZmanimError> {
        crate::astronomy::sunset(self.date, &self.location, false)
    }
    pub(crate) fn sunrise_offset_by_degrees(&self, degrees: f64) -> Result<Timestamp, ZmanimError> {
        crate::astronomy::sunrise_offset_by_degrees(self.date, &self.location, degrees)
    }
    pub(crate) fn sunset_offset_by_degrees(&self, degrees: f64) -> Result<Timestamp, ZmanimError> {
        crate::astronomy::sunset_offset_by_degrees(self.date, &self.location, degrees)
    }
    pub(crate) fn solar_transit(&self) -> Result<Timestamp, ZmanimError> {
        crate::astronomy::solar_noon(self.date, &self.location)
    }
    pub(crate) fn solar_midnight(&self) -> Result<Timestamp, ZmanimError> {
        crate::astronomy::solar_midnight(self.date, &self.location)
    }
}

fn validate_location(location: &Location) -> Result<(), ZmanimError> {
    if location.timezone.is_none() && Location::near_anti_meridian(location.longitude) {
        return Err(ZmanimError::TimeZoneRequired);
    }
    if location.longitude.abs() > 180.0 || location.longitude.is_nan() {
        return Err(ZmanimError::InvalidLongitude);
    }
    if location.latitude.abs() > 90.0 || location.latitude.is_nan() {
        return Err(ZmanimError::InvalidLatitude);
    }
    if location.elevation.is_nan() || location.elevation < 0.0 {
        return Err(ZmanimError::InvalidElevation);
    }
    Ok(())
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
