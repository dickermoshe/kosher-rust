use core::convert::Infallible;

use jiff::{
    Timestamp,
    tz::{Offset, TimeZone},
};
use thiserror::Error;

/// A time zone: anything that can say what the UTC offset is at a given instant.
///
/// [`jiff::tz::TimeZone`] implements it. Implement it yourself to take time zone rules from
/// somewhere else, such as the operating system or another language through FFI, then build the
/// location with [`Location::new_with_timezone`].
///
/// ```
/// use jiff::{Timestamp, tz::Offset};
/// use kosher_rust::{Location, TimeZoneLike};
///
/// /// Israel Standard Time all year, ignoring daylight saving.
/// struct IsraelStandardTime;
///
/// impl TimeZoneLike for IsraelStandardTime {
///     fn offset_at(&self, _timestamp: Timestamp) -> Offset {
///         Offset::constant(2)
///     }
/// }
///
/// let jerusalem = Location::new_with_timezone(31.778, 35.235, Some(IsraelStandardTime))?;
/// # Ok::<(), kosher_rust::LocationError>(())
/// ```
pub trait TimeZoneLike {
    /// The offset from UTC in effect at `timestamp`.
    fn offset_at(&self, timestamp: Timestamp) -> Offset;
}

impl TimeZoneLike for TimeZone {
    fn offset_at(&self, timestamp: Timestamp) -> Offset {
        self.to_offset(timestamp)
    }
}

impl<T: TimeZoneLike + ?Sized> TimeZoneLike for &T {
    fn offset_at(&self, timestamp: Timestamp) -> Offset {
        (**self).offset_at(timestamp)
    }
}

/// A location's coordinates and heights without its time zone: what the solar calculations take.
pub(crate) type Place = Location<Infallible>;

/// A place on earth: where to calculate *zmanim* for.
///
/// Start with [`Location::new`], then add the observer's height with
/// [`with_elevation`](Self::with_elevation) and, in hilly terrain, the heights of the horizons with
/// [`with_horizons`](Self::with_horizons). Each step validates its input.
///
/// ```
/// use jiff::tz::TimeZone;
/// use kosher_rust::Location;
///
/// let jerusalem = Location::new(31.778, 35.235, Some(TimeZone::get("Asia/Jerusalem")?))?
///     .with_elevation(754.0)?
///     .with_horizons(754.0, 0.0)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// The time zone is a [`jiff::tz::TimeZone`] by default; [`Location::new_with_timezone`] takes any
/// [`TimeZoneLike`].
#[derive(Debug, Clone, PartialEq)]
pub struct Location<Tz = TimeZone> {
    pub(crate) latitude: f64,
    pub(crate) longitude: f64,
    pub(crate) elevation: f64,
    pub(crate) eastern_horizon_elevation: f64,
    pub(crate) western_horizon_elevation: f64,
    pub(crate) timezone: Option<Tz>,
}

/// Why a [`Location`] was rejected.
#[derive(Debug, Error, PartialEq, Eq, Clone, Copy, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LocationError {
    /// The latitude is not between -90° and 90°.
    #[error("latitude must be between -90 and 90 degrees")]
    InvalidLatitude,
    /// The longitude is not between -180° and 180°.
    #[error("longitude must be between -180 and 180 degrees")]
    InvalidLongitude,
    /// An elevation is negative or not a finite number.
    #[error("elevation must be a finite number of meters, 0 or more")]
    InvalidElevation,
    /// The longitude is more than 150° from Greenwich, so a time zone is needed to know which
    /// day a date means there.
    #[error("locations more than 150 degrees from Greenwich need a time zone")]
    TimeZoneRequired,
}

impl Location {
    /// A location at sea level, latitude and longitude in degrees (north and east positive).
    ///
    /// The time zone may be `None`, except within 30° of the date line, where it decides which
    /// day a date means. The Kiddush Levana times also need one.
    ///
    /// # Errors
    ///
    /// - [`LocationError::InvalidLatitude`] outside -90° to 90°.
    /// - [`LocationError::InvalidLongitude`] outside -180° to 180°.
    /// - [`LocationError::TimeZoneRequired`] when `timezone` is `None` and the longitude is
    ///   beyond ±150°.
    pub fn new(latitude: f64, longitude: f64, timezone: Option<TimeZone>) -> Result<Self, LocationError> {
        Self::new_with_timezone(latitude, longitude, timezone)
    }
}

impl<Tz: TimeZoneLike> Location<Tz> {
    /// Like [`Location::new`], with any [`TimeZoneLike`] as the time zone.
    ///
    /// # Errors
    ///
    /// The same as [`Location::new`].
    pub fn new_with_timezone(latitude: f64, longitude: f64, timezone: Option<Tz>) -> Result<Self, LocationError> {
        if timezone.is_none() && longitude.abs() > 150.0 {
            return Err(LocationError::TimeZoneRequired);
        }
        if longitude.is_nan() || longitude.abs() > 180.0 {
            return Err(LocationError::InvalidLongitude);
        }
        if latitude.is_nan() || latitude.abs() > 90.0 {
            return Err(LocationError::InvalidLatitude);
        }
        Ok(Self {
            latitude,
            longitude,
            elevation: 0.0,
            eastern_horizon_elevation: 0.0,
            western_horizon_elevation: 0.0,
            timezone,
        })
    }
}

impl<Tz> Location<Tz> {
    /// The same place without its time zone.
    pub(crate) fn place(&self) -> Place {
        Location {
            latitude: self.latitude,
            longitude: self.longitude,
            elevation: self.elevation,
            eastern_horizon_elevation: self.eastern_horizon_elevation,
            western_horizon_elevation: self.western_horizon_elevation,
            timezone: None,
        }
    }

    /// The same location with the observer `meters` above sea level.
    ///
    /// Elevation makes sunrise earlier and sunset later. Which *zmanim* use it depends on
    /// [`Config::use_elevation`](crate::zmanim::Config::use_elevation).
    ///
    /// # Errors
    ///
    /// [`LocationError::InvalidElevation`] when `meters` is negative, NaN or infinite.
    pub fn with_elevation(mut self, meters: f64) -> Result<Self, LocationError> {
        self.elevation = valid_elevation(meters)?;
        Ok(self)
    }

    /// The same location with the eastern horizon, over which the sun rises, and the western
    /// horizon, behind which it sets, at the given heights above sea level in meters.
    ///
    /// By default both are at sea level. In hilly places they can differ by hundreds of meters,
    /// and using the observer's elevation for both shifts sunrise or sunset by minutes. Sunrise
    /// is then adjusted for the observer's height above the eastern horizon, and sunset for the
    /// height above the western one.
    ///
    /// # Errors
    ///
    /// [`LocationError::InvalidElevation`] when either height is negative, NaN or infinite.
    pub fn with_horizons(mut self, east_meters: f64, west_meters: f64) -> Result<Self, LocationError> {
        self.eastern_horizon_elevation = valid_elevation(east_meters)?;
        self.western_horizon_elevation = valid_elevation(west_meters)?;
        Ok(self)
    }

    /// Latitude in degrees, north positive.
    pub fn latitude(&self) -> f64 {
        self.latitude
    }

    /// Longitude in degrees, east positive.
    pub fn longitude(&self) -> f64 {
        self.longitude
    }

    /// The observer's height above sea level, in meters.
    pub fn elevation(&self) -> f64 {
        self.elevation
    }

    /// The height of the eastern horizon above sea level, in meters.
    pub fn eastern_horizon_elevation(&self) -> f64 {
        self.eastern_horizon_elevation
    }

    /// The height of the western horizon above sea level, in meters.
    pub fn western_horizon_elevation(&self) -> f64 {
        self.western_horizon_elevation
    }

    /// The time zone, if one was given.
    pub fn timezone(&self) -> Option<&Tz> {
        self.timezone.as_ref()
    }

    /// The observer's height above the eastern horizon, which adjusts sunrise. Never negative.
    pub(crate) fn sunrise_elevation(&self) -> f64 {
        (self.elevation - self.eastern_horizon_elevation).max(0.0)
    }

    /// The observer's height above the western horizon, which adjusts sunset. Never negative.
    pub(crate) fn sunset_elevation(&self) -> f64 {
        (self.elevation - self.western_horizon_elevation).max(0.0)
    }
}

fn valid_elevation(meters: f64) -> Result<f64, LocationError> {
    if meters.is_finite() && meters >= 0.0 {
        Ok(meters)
    } else {
        Err(LocationError::InvalidElevation)
    }
}

#[cfg(feature = "defmt")]
impl<Tz> defmt::Format for Location<Tz> {
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
    #![allow(clippy::unwrap_used)]

    use super::*;

    #[test]
    fn requires_a_timezone_only_near_the_date_line() {
        assert_eq!(Location::new(0.0, 150.1, None), Err(LocationError::TimeZoneRequired));
        assert!(Location::new(0.0, 150.0, None).is_ok());
        assert!(Location::new(0.0, 150.1, Some(TimeZone::UTC)).is_ok());
    }

    #[test]
    fn rejects_out_of_range_coordinates() {
        assert_eq!(
            Location::new(0.0, 181.0, Some(TimeZone::UTC)),
            Err(LocationError::InvalidLongitude)
        );
        assert_eq!(
            Location::new(91.0, 0.0, Some(TimeZone::UTC)),
            Err(LocationError::InvalidLatitude)
        );
        assert_eq!(Location::new(f64::NAN, 0.0, None), Err(LocationError::InvalidLatitude));
    }

    #[test]
    fn rejects_invalid_elevations() {
        let location = Location::new(0.0, 0.0, None).unwrap();
        for bad in [-1.0, f64::NAN, f64::INFINITY] {
            assert_eq!(
                location.clone().with_elevation(bad),
                Err(LocationError::InvalidElevation)
            );
            assert_eq!(
                location.clone().with_horizons(bad, 0.0),
                Err(LocationError::InvalidElevation)
            );
            assert_eq!(
                location.clone().with_horizons(0.0, bad),
                Err(LocationError::InvalidElevation)
            );
        }
    }

    #[test]
    fn horizons_default_to_sea_level() {
        let location = Location::new(0.0, 0.0, None).unwrap().with_elevation(754.0).unwrap();
        assert_eq!(location.eastern_horizon_elevation(), 0.0);
        assert_eq!(location.western_horizon_elevation(), 0.0);
        assert_eq!(location.sunrise_elevation(), 754.0);
        assert_eq!(location.sunset_elevation(), 754.0);
    }

    #[test]
    fn height_above_a_higher_horizon_is_zero() {
        let location = Location::new(0.0, 0.0, None)
            .unwrap()
            .with_elevation(1000.0)
            .unwrap()
            .with_horizons(300.0, 1500.0)
            .unwrap();
        assert_eq!(location.sunrise_elevation(), 700.0);
        assert_eq!(location.sunset_elevation(), 0.0);
    }
}
