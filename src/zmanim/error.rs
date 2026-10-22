use thiserror::Error;

use crate::calendar::DateOutOfRange;

/// Why a time could not be calculated for this place and date.
#[derive(Debug, Error, PartialEq, Eq, Clone, Copy, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ZmanimError {
    /// The sun stays above the angle the time needs all day, as in the far north in summer.
    #[error("the sun does not go low enough for this time on this date")]
    AllDay,
    /// The sun stays below the angle the time needs all day, as in the far north in winter.
    #[error("the sun does not rise high enough for this time on this date")]
    AllNight,
    /// The time only exists on certain dates, and this is not one: the latest time to eat
    /// chametz outside Erev Pesach, Kiddush Levana outside its window, the polar times on a day
    /// with a normal sunrise.
    #[error("this time does not apply on this date")]
    NotApplicable,
    /// The time needs a time zone and none was given.
    ///
    /// The four Kiddush Levana times always need a time zone. So does `get_zoned`. At a location
    /// more than 150° from Greenwich every time needs one.
    #[error("this time needs a time zone")]
    TimeZoneRequired,
    /// A calculation failed in a way that should not happen. Please report it with the location
    /// and date.
    #[error("internal calculation error; please report the location and date")]
    Internal,
}

/// A date outside the range a [`HebrewDate`] covers, which a calculation should not reach.
///
/// [`HebrewDate`]: crate::HebrewDate
impl From<DateOutOfRange> for ZmanimError {
    fn from(_: DateOutOfRange) -> Self {
        Self::Internal
    }
}

/// Date and time arithmetic fails only past the ends of the range jiff holds, which a calculation
/// should not reach.
impl From<jiff::Error> for ZmanimError {
    fn from(_: jiff::Error) -> Self {
        Self::Internal
    }
}
