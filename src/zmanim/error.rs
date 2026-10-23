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
    /// The time only exists on certain dates, and this is not one. Examples are the latest time
    /// to eat chametz outside Erev Pesach, Kiddush Levana outside its window, and the polar times
    /// on a day with a normal sunrise.
    #[error("this time does not apply on this date")]
    NotApplicable,
    /// The time depends on the Hebrew date, and the date is outside the range a
    /// [`HebrewDate`](crate::HebrewDate) covers, 1 January of year 1 through 29 Elul 9999. The
    /// chametz and Kiddush Levana times depend on the Hebrew date.
    #[error(transparent)]
    DateOutOfRange(#[from] DateOutOfRange),
    /// An angle passed to [`Zmanim::sunrise_offset_by_degrees`] or
    /// [`Zmanim::sunset_offset_by_degrees`] is not a number from -90 to 90.
    ///
    /// [`Zmanim::sunrise_offset_by_degrees`]: crate::Zmanim::sunrise_offset_by_degrees
    /// [`Zmanim::sunset_offset_by_degrees`]: crate::Zmanim::sunset_offset_by_degrees
    #[error("the angle must be a number of degrees from -90 to 90")]
    InvalidDegrees,
    /// The time needs a time zone and none was given.
    ///
    /// The four Kiddush Levana times always need a time zone. So does `get_zoned`. At a location
    /// more than 150° from Greenwich every time needs one.
    #[error("this time needs a time zone")]
    TimeZoneRequired,
    /// An offset is a day or more. The offsets are the two in [`Config`](crate::zmanim::Config)
    /// and the one given to [`MyZmanim::candle_lighting`](crate::myzmanim::MyZmanim::candle_lighting).
    #[error("the offset must be shorter than a day")]
    InvalidOffset,
    /// The time is outside the range a [`Timestamp`](jiff::Timestamp) holds, which ends in the
    /// years -9999 and 9999. Only dates within a day or two of either end get this.
    #[error("the time is outside the years -9999 to 9999")]
    TimeOutOfRange,
    /// A calculation failed in a way that should not happen. Please report it with the location
    /// and date.
    #[error("internal calculation error; please report the location and date")]
    Internal,
}

/// Date and time arithmetic fails only past the ends of the range jiff holds.
impl From<jiff::Error> for ZmanimError {
    fn from(_: jiff::Error) -> Self {
        Self::TimeOutOfRange
    }
}

/// Adding to a date with overflow constrained fails only past the ends of the range ICU holds.
impl From<icu_calendar::error::DateAddError> for ZmanimError {
    fn from(_: icu_calendar::error::DateAddError) -> Self {
        Self::TimeOutOfRange
    }
}
