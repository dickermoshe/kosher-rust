use thiserror::Error;

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
    /// The time needs a time zone and the location has none. Only the Kiddush Levana times do.
    #[error("this time needs a location with a time zone")]
    TimeZoneRequired,
    /// A calculation failed in a way that should not happen. Please report it with the location
    /// and date.
    #[error("internal calculation error; please report the location and date")]
    Internal,
}
