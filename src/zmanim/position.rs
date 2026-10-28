use jiff::Timestamp;

use super::AstronomicalCalculator;
use crate::{Location, astronomy};

/// Where the sun is in the sky, as returned by [`AstronomicalCalculator::solar_position`].
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[non_exhaustive]
pub struct SolarPosition {
    /// Degrees above the horizon, or below it when negative. This is where the center of the sun
    /// appears, so it includes the bending of light by the atmosphere. Near the horizon that
    /// raises the sun by about half a degree.
    pub elevation: f64,
    /// Degrees clockwise from true north, from 0 up to 360. East is 90, south is 180 and west
    /// is 270.
    pub azimuth: f64,
}

impl AstronomicalCalculator {
    /// Where the sun is in the sky, seen from `location` at `instant`.
    ///
    /// Only [`Spa`](Self::Spa) uses the location's elevation, to correct for the observer's
    /// height above the center of the earth. That changes the result by less than a hundredth
    /// of a degree. The other algorithms give the position seen from sea level.
    ///
    /// ```
    /// use jiff::Timestamp;
    /// use kosher_rust::{Location, zmanim::AstronomicalCalculator};
    ///
    /// let jerusalem = Location::new(31.778, 35.235)?;
    /// let noon: Timestamp = "2024-06-21T09:39:00Z".parse()?;
    /// let sun = AstronomicalCalculator::Noaa.solar_position(&jerusalem, noon);
    /// assert!(sun.elevation > 81.0); // nearly overhead on the longest day
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[must_use]
    pub fn solar_position(self, location: &Location, instant: Timestamp) -> SolarPosition {
        astronomy::solar_position(self, location, instant)
    }
}
