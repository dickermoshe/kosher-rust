use kosher_rust::{LocationError, zmanim::ZmanimError};

use crate::api::kosher::{KosherError, zmanim::ZmanFailure};

pub(crate) fn message(error: KosherError) -> &'static str {
    match error {
        KosherError::InvalidLatitude => "latitude must be between -90 and 90 degrees",
        KosherError::InvalidLongitude => "longitude must be between -180 and 180 degrees",
        KosherError::InvalidElevation => "elevation must be a finite number of meters, 0 or more",
        KosherError::InvalidDate => "no such date, or outside the supported range",
        KosherError::TimeZoneRequired => "this needs a time zone",
        KosherError::TimeZoneCallbackFailed => "the time zone callback failed or gave an offset of a day or more",
        KosherError::AllDay => "the sun does not go low enough for this time on this date",
        KosherError::AllNight => "the sun does not rise high enough for this time on this date",
        KosherError::NotApplicable => "this time does not apply on this date",
        KosherError::Internal => "internal calculation error; please report the location and date",
    }
}

impl From<LocationError> for KosherError {
    fn from(error: LocationError) -> Self {
        match error {
            LocationError::InvalidLatitude => Self::InvalidLatitude,
            LocationError::InvalidLongitude => Self::InvalidLongitude,
            LocationError::InvalidElevation => Self::InvalidElevation,
            LocationError::TimeZoneRequired => Self::TimeZoneRequired,
        }
    }
}

impl From<ZmanimError> for KosherError {
    fn from(error: ZmanimError) -> Self {
        match error {
            ZmanimError::AllDay => Self::AllDay,
            ZmanimError::AllNight => Self::AllNight,
            ZmanimError::NotApplicable => Self::NotApplicable,
            ZmanimError::TimeZoneRequired => Self::TimeZoneRequired,
            ZmanimError::Internal => Self::Internal,
        }
    }
}

impl From<KosherError> for ZmanFailure {
    fn from(error: KosherError) -> Self {
        match error {
            KosherError::AllDay => Self::AllDay,
            KosherError::AllNight => Self::AllNight,
            KosherError::NotApplicable => Self::NotApplicable,
            KosherError::TimeZoneRequired => Self::TimeZoneRequired,
            KosherError::TimeZoneCallbackFailed => Self::TimeZoneCallbackFailed,
            KosherError::InvalidLatitude
            | KosherError::InvalidLongitude
            | KosherError::InvalidElevation
            | KosherError::InvalidDate
            | KosherError::Internal => Self::Internal,
        }
    }
}
