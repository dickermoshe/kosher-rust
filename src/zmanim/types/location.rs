use jiff::tz::TimeZone;

use crate::zmanim::types::error::ZmanimError;

/// A geographic location (latitude/longitude/elevation) and an optional timezone.
///
/// Three elevations describe a location:
///
/// - [`elevation`](Self::elevation): how high the observer stands above sea level.
/// - [`eastern_horizon_elevation`](Self::eastern_horizon_elevation): how high the
///   horizon the sun rises over stands above sea level.
/// - [`western_horizon_elevation`](Self::western_horizon_elevation): how high the
///   horizon the sun sets behind stands above sea level.
///
/// When elevation adjustment is enabled, sunrise uses the observer's height above
/// the eastern horizon ([`sunrise_elevation`](Self::sunrise_elevation)) and sunset
/// uses the height above the western horizon ([`sunset_elevation`](Self::sunset_elevation)).
/// Both horizon elevations default to sea level, which reproduces the plain
/// observer-elevation adjustment.
#[derive(Debug, Clone, PartialEq)]
pub struct Location {
    /// Latitude in degrees. Valid range: `[-90.0, 90.0]` (positive = North).
    pub latitude: f64,
    /// Longitude in degrees. Valid range: `[-180.0, 180.0]` (positive = East).
    /// Must be provided with a timezone when `abs(longitude) > 150°`.
    pub longitude: f64,
    /// Observer elevation above sea level in meters. Must be `>= 0.0`.
    pub elevation: f64,
    /// Elevation above sea level in meters of the eastern horizon, over which the
    /// sun rises. Must be `>= 0.0` and finite. Defaults to `0.0` (sea level).
    ///
    /// In hilly terrain the eastern and western horizons can sit at very
    /// different heights, and a single observer elevation would then shift one
    /// of sunrise or sunset by minutes.
    pub eastern_horizon_elevation: f64,
    /// Elevation above sea level in meters of the western horizon, behind which
    /// the sun sets. Must be `>= 0.0` and finite. Defaults to `0.0` (sea level).
    pub western_horizon_elevation: f64,
    /// Timezone of the location. Required when near the anti-meridian (`abs(longitude) > 150°`).
    /// Also required for calculating kiddush levena times.
    pub timezone: Option<TimeZone>,
}

impl Location {
    /// Creates a new `Location` with both horizons at sea level, returning a
    /// [`ZmanimError`] if any value is out of range.
    ///
    /// # Errors
    /// - [`ZmanimError::InvalidLatitude`] — `latitude` outside `[-90.0, 90.0]`
    /// - [`ZmanimError::InvalidLongitude`] — `longitude` outside `[-180.0, 180.0]`
    /// - [`ZmanimError::InvalidElevation`] — `elevation` below `0.0`
    /// - [`ZmanimError::TimeZoneRequired`] — `timezone` is `None` and `abs(longitude) > 150°`
    pub fn new(latitude: f64, longitude: f64, elevation: f64, timezone: Option<TimeZone>) -> Result<Self, ZmanimError> {
        Self::with_horizon_elevations(latitude, longitude, elevation, 0.0, 0.0, timezone)
    }

    /// Creates a new `Location` whose eastern (sunrise) and western (sunset)
    /// horizons stand at the given elevations above sea level.
    ///
    /// # Errors
    /// Same as [`Location::new`]. Additionally returns
    /// [`ZmanimError::InvalidElevation`] if either horizon elevation is below
    /// `0.0`, `NaN`, or infinite.
    pub fn with_horizon_elevations(
        latitude: f64,
        longitude: f64,
        elevation: f64,
        eastern_horizon_elevation: f64,
        western_horizon_elevation: f64,
        timezone: Option<TimeZone>,
    ) -> Result<Self, ZmanimError> {
        if timezone.is_none() && Self::near_anti_meridian(longitude) {
            return Err(ZmanimError::TimeZoneRequired);
        }

        if longitude.abs() > 180.0 || longitude.is_nan() {
            return Err(ZmanimError::InvalidLongitude);
        }
        if latitude.abs() > 90.0 || latitude.is_nan() {
            return Err(ZmanimError::InvalidLatitude);
        }
        if elevation.is_nan() || elevation < 0.0 {
            return Err(ZmanimError::InvalidElevation);
        }
        for horizon in [eastern_horizon_elevation, western_horizon_elevation] {
            if !horizon.is_finite() || horizon < 0.0 {
                return Err(ZmanimError::InvalidElevation);
            }
        }

        Ok(Self {
            latitude,
            longitude,
            elevation,
            eastern_horizon_elevation,
            western_horizon_elevation,
            timezone,
        })
    }

    /// The observer's height in meters above the eastern horizon, used to
    /// adjust sunrise. Never negative: a horizon higher than the observer
    /// yields `0.0`.
    #[must_use]
    pub fn sunrise_elevation(&self) -> f64 {
        (self.elevation - self.eastern_horizon_elevation).max(0.0)
    }

    /// The observer's height in meters above the western horizon, used to
    /// adjust sunset. Never negative: a horizon higher than the observer
    /// yields `0.0`.
    #[must_use]
    pub fn sunset_elevation(&self) -> f64 {
        (self.elevation - self.western_horizon_elevation).max(0.0)
    }

    pub(crate) fn near_anti_meridian(longitude: f64) -> bool {
        const ANTI_MERIDIAN_THRESHOLD: f64 = 150.0;
        longitude.abs() > ANTI_MERIDIAN_THRESHOLD
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for Location {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(
            fmt,
            "Location {{ latitude: {}, longitude: {}, elevation: {}, eastern_horizon_elevation: {}, western_horizon_elevation: {}, has_timezone: {} }}",
            self.latitude,
            self.longitude,
            self.elevation,
            self.eastern_horizon_elevation,
            self.western_horizon_elevation,
            self.timezone.is_some(),
        )
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;
    use jiff::tz::TimeZone;

    #[test]
    fn test_location_rejects_anti_meridian_without_timezone() {
        let location = Location::new(0.0, 150.1, 0.0, None);
        assert!(location.is_err());
    }

    #[test]
    fn test_location_rejects_out_of_range_coords() {
        let bad_longitude = Location::new(0.0, 181.0, 0.0, Some(TimeZone::UTC));
        assert!(bad_longitude.is_err());

        let bad_latitude = Location::new(91.0, 0.0, 0.0, Some(TimeZone::UTC));
        assert!(bad_latitude.is_err());
    }

    #[test]
    fn test_location_rejects_negative_elevation() {
        let location = Location::new(0.0, 0.0, -1.0, Some(TimeZone::UTC));
        assert!(location.is_err());
    }

    #[test]
    fn test_location_rejects_invalid_horizon_elevations() {
        for bad in [-1.0, f64::NAN, f64::INFINITY] {
            assert!(Location::with_horizon_elevations(0.0, 0.0, 10.0, bad, 0.0, Some(TimeZone::UTC)).is_err());
            assert!(Location::with_horizon_elevations(0.0, 0.0, 10.0, 0.0, bad, Some(TimeZone::UTC)).is_err());
        }
    }

    #[test]
    fn test_default_horizons_are_sea_level() {
        let location = Location::new(0.0, 0.0, 754.0, Some(TimeZone::UTC)).expect("valid location");
        assert_eq!(location.eastern_horizon_elevation, 0.0);
        assert_eq!(location.western_horizon_elevation, 0.0);
        assert_eq!(location.sunrise_elevation(), 754.0);
        assert_eq!(location.sunset_elevation(), 754.0);
    }

    #[test]
    fn test_horizon_elevations_reduce_effective_elevation_and_clamp_at_zero() {
        let location = Location::with_horizon_elevations(0.0, 0.0, 1000.0, 300.0, 1500.0, Some(TimeZone::UTC))
            .expect("valid location");
        assert_eq!(location.sunrise_elevation(), 700.0);
        assert_eq!(location.sunset_elevation(), 0.0);
    }
}
