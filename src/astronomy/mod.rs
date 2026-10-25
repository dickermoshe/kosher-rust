//! Solar position algorithms behind sunrise, sunset, solar noon / midnight and
//! depression-angle zmanim.
//!
//! The public selector is [`AstronomicalCalculator`]. Each variant maps to one
//! [`SolarAlgorithm`] implementation ported from KosherJava:
//!
//! - [`noaa::Noaa`] — the NOAA low-accuracy Meeus formulas (KosherJava's default).
//! - [`meeus::Meeus`] — the higher-accuracy VSOP87 algorithm with ΔT.
//! - [`spa::Spa`] — the NREL Solar Position Algorithm with a topocentric rise/set solve.
//!
//! Everything the algorithms share — the zenith adjustment for refraction, solar radius
//! and elevation, Julian-day conversion, the KosherJava date-rollover rules and the
//! conversion from UTC hours to a [`Timestamp`] — lives in this module so the
//! backends only differ in their solar-position mathematics.

use core_maths::*;
use jiff::{SignedDuration, Timestamp, civil::Date, tz::Offset};

use crate::Zone;

use crate::{
    Location,
    zmanim::{AstronomicalCalculator, ZmanimError},
};

pub(crate) mod meeus;
mod noaa;
mod solar_radius_table;
mod spa;
mod vsop87;

use solar_radius_table::SOLAR_RADIUS_BY_DAY_OF_YEAR;

pub(crate) const GEOMETRIC_ZENITH: f64 = 90.0;
pub(super) const REFRACTION: f64 = 34.0 / 60.0;
const EARTH_RADIUS_KM: f64 = 6371.0088;
pub(super) const JULIAN_DAY_JAN_1_2000: f64 = 2_451_545.0;
pub(super) const JULIAN_DAYS_PER_CENTURY: f64 = 36_525.0;
const HOUR_NANOS: f64 = 3_600_000.0 * 1_000_000.0;

/// The solar event being solved for.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SolarEvent {
    Sunrise,
    Sunset,
    Noon,
    Midnight,
}

/// A solar position algorithm, in KosherJava's `AstronomicalCalculator` conventions.
///
/// `date` is the already timezone-adjusted civil date (see [`adjusted_local_date`]),
/// `longitude_west` is positive west, and results are expressed relative to 0:00 UTC of
/// `date`. Implementations must not normalise their results into `[0, 24)`; the callers
/// in this module do that so the KosherJava date-rollover logic sees the same values.
pub(crate) trait SolarAlgorithm {
    /// UTC minutes after 0:00 at which the sun's center reaches `adjusted_zenith`.
    ///
    /// `adjusted_zenith` already includes refraction, solar radius and elevation dip when
    /// the event is true sunrise / sunset.
    fn rise_set_utc_minutes(
        &self,
        date: Date,
        location: &Location,
        adjusted_zenith: f64,
        event: SolarEvent,
    ) -> Result<f64, ZmanimError>;

    /// UTC minutes after 0:00 of solar noon, or of the solar midnight that follows it.
    fn noon_midnight_utc_minutes(&self, date: Date, longitude_west: f64, event: SolarEvent) -> f64;

    /// UTC hours (not normalised) at which the sun reaches an azimuth of exactly 90° or 270°.
    fn time_at_azimuth_utc_hours(&self, date: Date, location: &Location, azimuth: f64) -> Result<f64, ZmanimError>;
}

impl SolarAlgorithm for AstronomicalCalculator {
    fn rise_set_utc_minutes(
        &self,
        date: Date,
        location: &Location,
        adjusted_zenith: f64,
        event: SolarEvent,
    ) -> Result<f64, ZmanimError> {
        match *self {
            Self::Noaa => noaa::Noaa.rise_set_utc_minutes(date, location, adjusted_zenith, event),
            Self::Meeus { apply_delta_t } => {
                meeus::Meeus::standard(apply_delta_t).rise_set_utc_minutes(date, location, adjusted_zenith, event)
            }
            Self::Spa { apply_delta_t } => {
                spa::Spa { apply_delta_t }.rise_set_utc_minutes(date, location, adjusted_zenith, event)
            }
        }
    }

    fn noon_midnight_utc_minutes(&self, date: Date, longitude_west: f64, event: SolarEvent) -> f64 {
        match *self {
            Self::Noaa => noaa::Noaa.noon_midnight_utc_minutes(date, longitude_west, event),
            Self::Meeus { apply_delta_t } => {
                meeus::Meeus::standard(apply_delta_t).noon_midnight_utc_minutes(date, longitude_west, event)
            }
            Self::Spa { apply_delta_t } => {
                spa::Spa { apply_delta_t }.noon_midnight_utc_minutes(date, longitude_west, event)
            }
        }
    }

    fn time_at_azimuth_utc_hours(&self, date: Date, location: &Location, azimuth: f64) -> Result<f64, ZmanimError> {
        match *self {
            Self::Noaa => noaa::Noaa.time_at_azimuth_utc_hours(date, location, azimuth),
            Self::Meeus { apply_delta_t } => {
                meeus::Meeus::standard(apply_delta_t).time_at_azimuth_utc_hours(date, location, azimuth)
            }
            Self::Spa { apply_delta_t } => {
                spa::Spa { apply_delta_t }.time_at_azimuth_utc_hours(date, location, azimuth)
            }
        }
    }
}

pub(crate) fn sunrise(
    calculator: AstronomicalCalculator,
    date: Date,
    location: &Location,
    timezone: Option<&dyn Zone>,
    adjust_for_elevation: bool,
) -> Result<Timestamp, ZmanimError> {
    rise_set(
        calculator,
        date,
        location,
        timezone,
        GEOMETRIC_ZENITH,
        adjust_for_elevation,
        SolarEvent::Sunrise,
    )
}

pub(crate) fn sunset(
    calculator: AstronomicalCalculator,
    date: Date,
    location: &Location,
    timezone: Option<&dyn Zone>,
    adjust_for_elevation: bool,
) -> Result<Timestamp, ZmanimError> {
    rise_set(
        calculator,
        date,
        location,
        timezone,
        GEOMETRIC_ZENITH,
        adjust_for_elevation,
        SolarEvent::Sunset,
    )
}

pub(crate) fn sunrise_offset_by_degrees(
    calculator: AstronomicalCalculator,
    date: Date,
    location: &Location,
    timezone: Option<&dyn Zone>,
    degrees: f64,
) -> Result<Timestamp, ZmanimError> {
    rise_set(
        calculator,
        date,
        location,
        timezone,
        GEOMETRIC_ZENITH + degrees,
        false,
        SolarEvent::Sunrise,
    )
}

pub(crate) fn sunset_offset_by_degrees(
    calculator: AstronomicalCalculator,
    date: Date,
    location: &Location,
    timezone: Option<&dyn Zone>,
    degrees: f64,
) -> Result<Timestamp, ZmanimError> {
    rise_set(
        calculator,
        date,
        location,
        timezone,
        GEOMETRIC_ZENITH + degrees,
        false,
        SolarEvent::Sunset,
    )
}

pub(crate) fn solar_noon(
    calculator: AstronomicalCalculator,
    date: Date,
    location: &Location,
    timezone: Option<&dyn Zone>,
) -> Result<Timestamp, ZmanimError> {
    let adjusted_date = adjusted_local_date(date, location, timezone)?;
    let noon = utc_noon_midnight_hours(calculator, adjusted_date, location, SolarEvent::Noon);
    instant_from_utc_hours(noon, adjusted_date, location, SolarEvent::Noon)
}

pub(crate) fn solar_midnight(
    calculator: AstronomicalCalculator,
    date: Date,
    location: &Location,
    timezone: Option<&dyn Zone>,
) -> Result<Timestamp, ZmanimError> {
    let adjusted_date = adjusted_local_date(date, location, timezone)?;
    let midnight = utc_noon_midnight_hours(calculator, adjusted_date, location, SolarEvent::Midnight);
    instant_from_utc_hours(midnight, adjusted_date, location, SolarEvent::Midnight)
}

pub(crate) fn time_at_azimuth(
    calculator: AstronomicalCalculator,
    date: Date,
    location: &Location,
    timezone: Option<&dyn Zone>,
    target_azimuth: f64,
) -> Result<Timestamp, ZmanimError> {
    let adjusted_date = adjusted_local_date(date, location, timezone)?;
    let hours = utc_time_at_azimuth_hours(calculator, adjusted_date, location, target_azimuth)?;
    let event = if target_azimuth == 90.0 {
        SolarEvent::Sunrise
    } else {
        SolarEvent::Sunset
    };
    instant_from_utc_hours(hours, adjusted_date, location, event)
}

fn rise_set(
    calculator: AstronomicalCalculator,
    date: Date,
    location: &Location,
    timezone: Option<&dyn Zone>,
    zenith: f64,
    adjust_for_elevation: bool,
    event: SolarEvent,
) -> Result<Timestamp, ZmanimError> {
    let adjusted_date = adjusted_local_date(date, location, timezone)?;
    let hours = utc_rise_set_hours(calculator, adjusted_date, location, zenith, adjust_for_elevation, event)?;
    instant_from_utc_hours(hours, adjusted_date, location, event)
}

/// Sunrise or sunset of `algorithm` on an already adjusted civil date for a fully adjusted
/// zenith: nothing is added for refraction, solar radius or dip here. Callers with their
/// own horizon model (see [`crate::myzmanim`]) build the zenith themselves.
pub(crate) fn rise_set_on_adjusted_date(
    algorithm: &impl SolarAlgorithm,
    adjusted_date: Date,
    location: &Location,
    adjusted_zenith: f64,
    event: SolarEvent,
) -> Result<Timestamp, ZmanimError> {
    let utc_minutes = algorithm.rise_set_utc_minutes(adjusted_date, location, adjusted_zenith, event)?;
    if utc_minutes.is_nan() {
        return Err(ZmanimError::Internal);
    }
    instant_from_utc_hours(normalize_utc_hours(utc_minutes / 60.0), adjusted_date, location, event)
}

/// Mirrors KosherJava's `AstronomicalCalculator.getUTCSunrise` / `getUTCSunset`: the UTC
/// time of the event in hours, normalised to `[0, 24)`, for the given (already adjusted)
/// civil date.
pub(crate) fn utc_rise_set_hours(
    calculator: AstronomicalCalculator,
    date: Date,
    location: &Location,
    zenith: f64,
    adjust_for_elevation: bool,
    event: SolarEvent,
) -> Result<f64, ZmanimError> {
    let elevation = match (adjust_for_elevation, event) {
        (false, _) => 0.0,
        (true, SolarEvent::Sunrise) => location.sunrise_elevation(),
        (true, SolarEvent::Sunset) => location.sunset_elevation(),
        (true, SolarEvent::Noon | SolarEvent::Midnight) => return Err(ZmanimError::Internal),
    };
    let adjusted_zenith = adjust_zenith(zenith, elevation, date);
    let utc_minutes = calculator.rise_set_utc_minutes(date, location, adjusted_zenith, event)?;
    if utc_minutes.is_nan() {
        return Err(ZmanimError::Internal);
    }
    Ok(normalize_utc_hours(utc_minutes / 60.0))
}

/// Mirrors KosherJava's `AstronomicalCalculator.getUTCNoon` / `getUTCMidnight`: the UTC
/// time in hours, normalised to `[0, 24)`, for the given (already adjusted) civil date.
pub(crate) fn utc_noon_midnight_hours(
    calculator: AstronomicalCalculator,
    date: Date,
    location: &Location,
    event: SolarEvent,
) -> f64 {
    let minutes = calculator.noon_midnight_utc_minutes(date, -location.longitude, event);
    normalize_utc_hours(minutes / 60.0)
}

/// Mirrors KosherJava's `AstronomicalCalculator.getTimeAtAzimuth`: the UTC time in hours,
/// normalised to `[0, 24)`, at which the sun is due east (90°) or due west (270°).
pub(crate) fn utc_time_at_azimuth_hours(
    calculator: AstronomicalCalculator,
    date: Date,
    location: &Location,
    target_azimuth: f64,
) -> Result<f64, ZmanimError> {
    if target_azimuth != 90.0 && target_azimuth != 270.0 {
        return Err(ZmanimError::Internal);
    }
    let hours = calculator.time_at_azimuth_utc_hours(date, location, target_azimuth)?;
    Ok(normalize_utc_hours(hours))
}

/// Shared body of every calculator's `getTimeAtAzimuth`: iterate on the hour angle at
/// which the sun's declination places it due east / west, re-sampling the declination and
/// equation of time (both supplied by `declination_and_eot` for a UT Julian day) each pass.
pub(super) fn time_at_azimuth_iterative(
    date: Date,
    location: &Location,
    target_azimuth: f64,
    iterations: usize,
    mut declination_and_eot: impl FnMut(f64) -> (f64, f64),
) -> Result<f64, ZmanimError> {
    let julian_day = julian_day(date);
    let solar_noon_base = 0.5 - location.longitude / 360.0;
    let mut date_time = solar_noon_base + if target_azimuth == 90.0 { 0.25 } else { 0.75 };

    for _ in 0..iterations {
        let (declination, equation_of_time) = declination_and_eot(julian_day + date_time);
        let ratio = CoreFloat::tan(declination.to_radians()) / CoreFloat::tan(location.latitude.to_radians());

        if ratio.is_nan() || !(-1.0..=1.0).contains(&ratio) {
            return Err(ZmanimError::NotApplicable);
        }

        let direction = if target_azimuth == 90.0 { -1.0 } else { 1.0 };
        let offset = direction * CoreFloat::acos(ratio).to_degrees() / 360.0;
        date_time = solar_noon_base + offset - equation_of_time / 1440.0;
    }

    Ok(date_time * 24.0)
}

/// `date`, moved a day where the time zone's clock is a day or so from solar time, as across the
/// date line. The time zone's offset is read at the local midnight starting `date`, as KosherJava's
/// `GeoLocation.getAntimeridianAdjustment` reads it.
pub(crate) fn adjusted_local_date(
    date: Date,
    location: &Location,
    timezone: Option<&dyn Zone>,
) -> Result<Date, ZmanimError> {
    let Some(timezone) = timezone else {
        return if location.longitude.abs() > 150.0 {
            Err(ZmanimError::TimeZoneRequired)
        } else {
            Ok(date)
        };
    };

    let offset = timezone.offset(timezone.start_of_day(date)?).seconds();
    let local_hours_offset = (location.longitude * 240.0 - f64::from(offset)) / 3600.0;

    if local_hours_offset >= 20.0 {
        add_days(date, 1)
    } else if local_hours_offset <= -20.0 {
        add_days(date, -1)
    } else {
        Ok(date)
    }
}

pub(crate) fn instant_from_utc_hours(
    time: f64,
    mut date: Date,
    location: &Location,
    event: SolarEvent,
) -> Result<Timestamp, ZmanimError> {
    if time.is_nan() {
        return Err(ZmanimError::Internal);
    }

    // `time` is only the UTC time of day, a number from 0 to 24. It doesn't say which UTC day the event
    // is on, so start from `date` and correct it.
    //
    // `local_time_hours` is that time in local mean time, which is UTC plus the longitude's hours. It can
    // come out below 0 or above 24, which means the event is really on the day before or the day after
    // `date`. Each event has a window of local mean time that belongs to the date, and these arms move
    // the date by a day when the event falls outside it.
    //
    // - Sunrise belongs to the date from 18:00 the evening before through 18:00 of the date. After 18:00
    //   it is really the evening before (an Arctic sunrise just after midnight, say), so go back a day.
    //   At -6:00 or earlier the UTC time wrapped around midnight, so go forward a day. For example, just
    //   west of the date line local mean time is about 12 hours behind UTC. A sunrise at noon local mean
    //   time is then 24:00 UTC, which arrives as 0:00 and reads as -12:00 local mean time.
    // - Sunset belongs from 06:00 of the date through 06:00 the next morning. Before 06:00 go forward a
    //   day. At 30:00, which is 06:00 the next morning, or later the UTC time wrapped the other way, so go
    //   back a day.
    // - Solar midnight is the one after the date's noon, so one before 12:00 moves forward a day.
    // - Solar noon belongs between 0:00 and 24:00. Before 0:00 go forward a day, and after 24:00 go back a
    //   day.
    let local_time_hours = location.longitude / 15.0 + time;
    match event {
        SolarEvent::Sunrise if local_time_hours > 18.0 => date = add_days(date, -1)?,
        SolarEvent::Sunrise if local_time_hours <= -6.0 => date = add_days(date, 1)?,
        SolarEvent::Sunset if local_time_hours < 6.0 => date = add_days(date, 1)?,
        SolarEvent::Sunset if local_time_hours >= 30.0 => date = add_days(date, -1)?,
        SolarEvent::Midnight if local_time_hours < 12.0 => date = add_days(date, 1)?,
        SolarEvent::Noon if local_time_hours < 0.0 => date = add_days(date, 1)?,
        SolarEvent::Noon if local_time_hours > 24.0 => date = add_days(date, -1)?,
        _ => {}
    }

    let nanos = (time * HOUR_NANOS).round() as i64;
    let date_time = date.at(0, 0, 0, 0).checked_add(SignedDuration::from_nanos(nanos))?;
    Ok(Offset::UTC.to_timestamp(date_time)?)
}

fn add_days(date: Date, days: i64) -> Result<Date, ZmanimError> {
    Ok(date.checked_add(SignedDuration::from_hours(24 * days))?)
}

pub(crate) fn normalize_utc_hours(time: f64) -> f64 {
    (time % 24.0 + 24.0) % 24.0
}

#[allow(clippy::unwrap_used, clippy::expect_used)]
pub(crate) fn apparent_solar_radius(date: Date) -> f64 {
    let month = date.month();
    let day = date.day();
    let reference = if month == 2 && day == 29 {
        Date::new(2050, 2, 28).unwrap()
    } else {
        Date::new(2050, month, day).unwrap()
    };
    let jan1 = Date::new(2050, 1, 1).unwrap();
    let day_index = jan1
        .until(reference)
        .expect("reference date is on or after Jan 1 2050")
        .get_days() as usize;
    SOLAR_RADIUS_BY_DAY_OF_YEAR[day_index]
}

/// Adds refraction, the date-based apparent solar radius and the elevation dip to the
/// zenith, but only for the geometric 90° zenith of true sunrise / sunset.
fn adjust_zenith(zenith: f64, elevation: f64, date: Date) -> f64 {
    if zenith == GEOMETRIC_ZENITH {
        zenith + apparent_solar_radius(date) + REFRACTION + elevation_adjustment(elevation)
    } else {
        zenith
    }
}

fn elevation_adjustment(elevation_m: f64) -> f64 {
    CoreFloat::acos(EARTH_RADIUS_KM / (EARTH_RADIUS_KM + elevation_m / 1000.0)).to_degrees()
}

/// Julian day for 0h UT of the date (Meeus chapter 7).
pub(super) fn julian_day(date: Date) -> f64 {
    let mut year = i32::from(date.year());
    let mut month = i32::from(date.month());
    let day = i32::from(date.day());

    if month <= 2 {
        year -= 1;
        month += 12;
    }

    let a = year / 100;
    let b = 2 - a + a / 4;

    (365.25 * f64::from(year + 4716)).floor() + (30.6001 * f64::from(month + 1)).floor() + f64::from(day) + f64::from(b)
        - 1524.5
}

/// Julian centuries since J2000.0, with no ΔT applied.
pub(super) fn julian_centuries_from_julian_day(julian_day: f64) -> f64 {
    (julian_day - JULIAN_DAY_JAN_1_2000) / JULIAN_DAYS_PER_CENTURY
}

/// Hour angle of the sun in radians for the given latitude, declination and zenith.
///
/// Returns [`ZmanimError::AllNight`] when the sun never rises to the zenith and
/// [`ZmanimError::AllDay`] when it never sets below it (KosherJava returns `NaN` for both).
pub(super) fn sun_hour_angle(
    latitude: f64,
    solar_declination: f64,
    zenith: f64,
    event: SolarEvent,
) -> Result<f64, ZmanimError> {
    let lat_rad = latitude.to_radians();
    let sd_rad = solar_declination.to_radians();
    let cos_hour_angle = CoreFloat::cos(zenith.to_radians()) / (CoreFloat::cos(lat_rad) * CoreFloat::cos(sd_rad))
        - CoreFloat::tan(lat_rad) * CoreFloat::tan(sd_rad);
    hour_angle_from_cos(cos_hour_angle, event)
}

/// Converts `cos(H₀)` into a signed hour angle in radians (negative for sunset), mapping
/// out-of-range values to the polar-day / polar-night errors.
pub(super) fn hour_angle_from_cos(cos_hour_angle: f64, event: SolarEvent) -> Result<f64, ZmanimError> {
    if cos_hour_angle > 1.0 {
        return Err(ZmanimError::AllNight);
    }
    if cos_hour_angle < -1.0 {
        return Err(ZmanimError::AllDay);
    }

    let mut hour_angle = CoreFloat::acos(cos_hour_angle);
    if event == SolarEvent::Sunset {
        hour_angle = -hour_angle;
    }
    Ok(hour_angle)
}

#[cfg(test)]
mod tests;
