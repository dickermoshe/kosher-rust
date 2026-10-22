use kosher_rust::LocationError;

use crate::api::{
    calendar::{CalendarError, limudim::LimudimError},
    zmanim::{ZmanFailure, ZmanimError},
};

/// The text of an error, as its `Display` shows it.
pub(crate) trait Message {
    fn message(self) -> &'static str;
}

pub(crate) fn message(error: impl Message) -> &'static str {
    error.message()
}

impl Message for CalendarError {
    fn message(self) -> &'static str {
        match self {
            Self::InvalidDate => "no such date, or outside Hebrew years 1 to 9999",
            Self::InvalidHoliday => "the day of Chanukah or of the Omer is out of range",
        }
    }
}

impl Message for LimudimError {
    fn message(self) -> &'static str {
        match self {
            Self::InvalidUnit => "a psalm, verse or chapter number is out of range",
        }
    }
}

impl Message for ZmanimError {
    fn message(self) -> &'static str {
        match self {
            Self::InvalidLatitude => "latitude must be between -90 and 90 degrees",
            Self::InvalidLongitude => "longitude must be between -180 and 180 degrees",
            Self::InvalidElevation => "elevation must be a finite number of meters, 0 or more",
            Self::InvalidGregorianDate => "no such date, or outside the supported range",
            Self::TimeZoneRequired => "this needs a time zone",
            Self::TimeZoneCallbackFailed => "the time zone callback failed or gave an offset of a day or more",
            Self::InvalidMinutes => "minutes must be a finite number from -1440 to 1440",
            Self::InvalidDegrees => "degrees must be a finite number",
            Self::AllDay => "the sun does not go low enough for this time on this date",
            Self::AllNight => "the sun does not rise high enough for this time on this date",
            Self::NotApplicable => "this time does not apply on this date",
            Self::Unexpected => "unexpected calculation error; please report the location and date",
        }
    }
}

impl From<LocationError> for ZmanimError {
    fn from(error: LocationError) -> Self {
        match error {
            LocationError::InvalidLatitude => Self::InvalidLatitude,
            LocationError::InvalidLongitude => Self::InvalidLongitude,
            LocationError::InvalidElevation => Self::InvalidElevation,
            LocationError::TimeZoneRequired => Self::TimeZoneRequired,
        }
    }
}

impl From<kosher_rust::zmanim::ZmanimError> for ZmanimError {
    fn from(error: kosher_rust::zmanim::ZmanimError) -> Self {
        match error {
            kosher_rust::zmanim::ZmanimError::AllDay => Self::AllDay,
            kosher_rust::zmanim::ZmanimError::AllNight => Self::AllNight,
            kosher_rust::zmanim::ZmanimError::NotApplicable => Self::NotApplicable,
            kosher_rust::zmanim::ZmanimError::TimeZoneRequired => Self::TimeZoneRequired,
            kosher_rust::zmanim::ZmanimError::Internal => Self::Unexpected,
        }
    }
}

impl From<ZmanimError> for ZmanFailure {
    fn from(error: ZmanimError) -> Self {
        match error {
            ZmanimError::AllDay => Self::AllDay,
            ZmanimError::AllNight => Self::AllNight,
            ZmanimError::NotApplicable => Self::NotApplicable,
            ZmanimError::TimeZoneRequired => Self::TimeZoneRequired,
            ZmanimError::TimeZoneCallbackFailed => Self::TimeZoneCallbackFailed,
            ZmanimError::InvalidLatitude
            | ZmanimError::InvalidLongitude
            | ZmanimError::InvalidElevation
            | ZmanimError::InvalidGregorianDate
            | ZmanimError::InvalidMinutes
            | ZmanimError::InvalidDegrees
            | ZmanimError::Unexpected => Self::Unexpected,
        }
    }
}
