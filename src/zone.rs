use jiff::{
    SignedDuration, Timestamp,
    civil::{Date, Time},
    tz::{Offset, TimeZone},
};

/// A time zone, described by its offset from UTC at each instant.
///
/// [`Zmanim`](crate::Zmanim) and [`MyZmanim`](crate::myzmanim::MyZmanim) take a
/// [`jiff::tz::TimeZone`], which implements this trait. Implement it yourself when the time zone
/// comes from somewhere else, such as a device's settings or another language's date library, and
/// build the calculator with [`Zmanim::with_zone`](crate::Zmanim::with_zone).
///
/// ```
/// use jiff::{Timestamp, civil::date, tz::Offset};
/// use kosher_rust::{Location, Zman, Zmanim, Zone, zmanim::Config};
///
/// /// Eastern Standard Time, all year.
/// struct Eastern;
///
/// impl Zone for Eastern {
///     fn offset(&self, _: Timestamp) -> Offset {
///         Offset::constant(-5)
///     }
/// }
///
/// let lakewood = Location::new(40.09, -74.22)?;
/// let zmanim = Zmanim::with_zone(lakewood, date(2024, 1, 15), Some(Eastern), Config::default());
/// assert!(zmanim.get(Zman::SofZmanKidushLevana15Days).is_err());
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub trait Zone {
    /// The offset from UTC in effect at `instant`.
    fn offset(&self, instant: Timestamp) -> Offset;

    /// The instant `date` begins on this time zone's clocks. That is local midnight, or the end of
    /// the gap when the clocks skip over midnight. When midnight happens twice, it is the first.
    ///
    /// The default reads [`offset`](Self::offset) a day before and a day after midnight, so it
    /// works for a time zone whose offset changes at most once in those two days.
    ///
    /// # Errors
    ///
    /// When the instant is outside the range a [`Timestamp`] holds.
    fn start_of_day(&self, date: Date) -> Result<Timestamp, jiff::Error> {
        // Midnight read as UTC, moved by each candidate offset.
        let midnight = Offset::UTC.to_timestamp(date.to_datetime(Time::midnight()))?;
        let at = |offset: Offset| midnight.checked_sub(SignedDuration::from_secs(i64::from(offset.seconds())));
        let before = self.offset(midnight.checked_sub(SignedDuration::from_hours(24))?);
        let after = self.offset(midnight.checked_add(SignedDuration::from_hours(24))?);
        let early = at(before)?;
        if self.offset(early) == before {
            return Ok(early);
        }
        let late = at(after)?;
        if self.offset(late) == after {
            return Ok(late);
        }
        // The clocks skipped midnight. Reading it with the offset from before the gap lands just
        // after the gap.
        Ok(early)
    }
}

impl Zone for TimeZone {
    fn offset(&self, instant: Timestamp) -> Offset {
        self.to_offset(instant)
    }

    fn start_of_day(&self, date: Date) -> Result<Timestamp, jiff::Error> {
        Ok(date.to_zoned(self.clone())?.timestamp())
    }
}

impl Zone for Offset {
    fn offset(&self, _: Timestamp) -> Offset {
        *self
    }
}

impl<Z: Zone + ?Sized> Zone for &Z {
    fn offset(&self, instant: Timestamp) -> Offset {
        (**self).offset(instant)
    }

    fn start_of_day(&self, date: Date) -> Result<Timestamp, jiff::Error> {
        (**self).start_of_day(date)
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use jiff::{ToSpan, civil::date};

    use super::*;

    /// A time zone that only has the default `start_of_day`.
    struct Offsets<'a>(&'a TimeZone);

    impl Zone for Offsets<'_> {
        fn offset(&self, instant: Timestamp) -> Offset {
            self.0.to_offset(instant)
        }
    }

    /// The default `start_of_day` agrees with jiff in every time zone of the database, on each day
    /// from two days before to two days after each change of offset, and on New Year's Day, when
    /// many zones left local mean time. That includes São Paulo's 4 November 2018, when the clocks
    /// skipped midnight, and Apia's skipped 30 December 2011.
    #[test]
    fn default_start_of_day_matches_jiff_in_every_zone() {
        let mut checked = 0;
        for name in jiff::tz::db().available() {
            let zone = TimeZone::get(name.as_str()).unwrap();
            let mut days: Vec<Date> = [1870, 1880, 1900, 1912, 1920, 1946, 1970, 2000, 2025]
                .map(|year| date(year, 1, 1))
                .to_vec();
            for change in zone
                .following(Timestamp::from_second(-5_000_000_000).unwrap())
                .take_while(|change| change.timestamp().as_second() < 4_200_000_000)
            {
                let day = Offset::UTC.to_datetime(change.timestamp()).date();
                days.extend(day.checked_sub(2.days()).unwrap().series(1.days()).take(5));
            }
            for day in days {
                assert_eq!(
                    Offsets(&zone).start_of_day(day).unwrap(),
                    zone.start_of_day(day).unwrap(),
                    "{name} {day}"
                );
                checked += 1;
            }
        }
        assert!(checked > 100_000, "{checked}");
    }
}
