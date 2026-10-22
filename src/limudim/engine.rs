// The walk every schedule shares. A schedule is a sequence of cycles (Daf Yomi's 7.5-year
// cycle, a Hebrew month for Tehillim, a summer for Pirkei Avos). Each cycle is split into
// intervals, one learning unit each; some intervals are skipped (Tisha b'Av, Yom Kippur).
// `InternalLimud::limud` finds the cycle containing a date, walks its intervals to the one
// containing the date, and asks the schedule for that interval's unit.

use icu_calendar::{
    Date,
    cal::Hebrew,
    options::{DateAddOptions, DateDifferenceOptions},
    types::DateDuration,
};

pub(crate) trait HebrewDateExt {
    /// Return a copy of this date with `days` added.
    fn add_days(&self, days: i32) -> Option<Date<Hebrew>>;
    /// Calculate the number of days between two Hebrew dates, inclusive of partial spans.
    ///
    /// Returns `None` when `self > end`, or when ICU cannot compute the difference
    /// (e.g. dates outside the supported Hebrew calendar range).
    fn days_until(&self, end: &Date<Hebrew>) -> Option<u32>;
}

impl HebrewDateExt for Date<Hebrew> {
    fn add_days(&self, days: i32) -> Option<Date<Hebrew>> {
        self.try_added_with_options(DateDuration::for_days(days), DateAddOptions::default())
            .ok()
    }

    fn days_until(&self, end: &Date<Hebrew>) -> Option<u32> {
        if self > end {
            return None;
        }
        match self.try_until_with_options(end, DateDifferenceOptions::default()) {
            Ok(duration) => Some(duration.days),
            Err(error) => match error {},
        }
    }
}

pub type CycleEndCalculation = fn(Date<Hebrew>, Option<i32>) -> Option<Date<Hebrew>>;

#[derive(Clone, Copy)]
pub struct Cycle {
    pub start_date: Date<Hebrew>,
    pub end_date: Date<Hebrew>,
    pub iteration: Option<i32>,
}
impl Cycle {
    pub fn from_perpetual(finder: PerpetualCycleFinder, date: Date<Hebrew>) -> Option<Self> {
        let (start_date, end_date) = finder(date)?;
        Some(Self {
            start_date,
            end_date,
            iteration: None,
        })
    }
    pub fn from_cycle_initiation(
        initial_cycle_date: Date<Hebrew>,
        cycle_end_calculation: CycleEndCalculation,
        date: Date<Hebrew>,
    ) -> Option<Self> {
        if initial_cycle_date > date {
            return None;
        }
        let iteration = 1;
        let end_date = cycle_end_calculation(initial_cycle_date, Some(iteration))?;
        let mut cycle = Self {
            start_date: initial_cycle_date,
            end_date,
            iteration: Some(iteration),
        };
        while date > cycle.end_date {
            cycle = cycle.next(cycle_end_calculation)?;
        }
        Some(cycle)
    }

    pub fn next(&self, cycle_end_calculation: CycleEndCalculation) -> Option<Self> {
        if let Some(iteration) = self.iteration {
            let new_iteration = iteration + 1;
            let new_start_date = self.end_date.add_days(1)?;
            let new_end_date = cycle_end_calculation(new_start_date, Some(new_iteration))?;
            Some(Self {
                start_date: new_start_date,
                end_date: new_end_date,
                iteration: Some(new_iteration),
            })
        } else {
            None
        }
    }
}

pub type IntervalEndCalculation = fn(Cycle, Date<Hebrew>) -> Option<Date<Hebrew>>;

pub struct Interval {
    pub start_date: Date<Hebrew>,
    pub end_date: Date<Hebrew>,
    pub iteration: i32,
    pub cycle: Cycle,
}
impl Interval {
    pub fn first_for_cycle(cycle: Cycle, interval_end_calculation: IntervalEndCalculation) -> Option<Self> {
        let start_date = cycle.start_date;
        let iteration = 1;
        let end_date = interval_end_calculation(cycle, start_date)?;
        Some(Self {
            start_date,
            end_date,
            iteration,
            cycle,
        })
    }
    pub fn next(&self, interval_end_calculation: IntervalEndCalculation) -> Option<Self> {
        self._next_for_iteration(self.iteration + 1, interval_end_calculation)
    }
    pub fn skip(&self, interval_end_calculation: IntervalEndCalculation) -> Option<Self> {
        self._next_for_iteration(self.iteration, interval_end_calculation)
    }
    fn _next_for_iteration(
        &self,
        new_iteration: i32,
        interval_end_calculation: IntervalEndCalculation,
    ) -> Option<Self> {
        if self.end_date >= self.cycle.end_date {
            return None;
        }
        let new_start_date = self.end_date.add_days(1)?;
        let new_end_date = interval_end_calculation(self.cycle, new_start_date)?;
        Some(Self {
            start_date: new_start_date,
            end_date: new_end_date,
            iteration: new_iteration,
            cycle: self.cycle,
        })
    }
    pub fn contains(&self, date: Date<Hebrew>) -> bool {
        self.start_date <= date && date <= self.end_date
    }
}

pub type PerpetualCycleFinder = fn(Date<Hebrew>) -> Option<(Date<Hebrew>, Date<Hebrew>)>;

pub enum CycleFinder {
    Initial(Date<Hebrew>),
    Perpetual(PerpetualCycleFinder),
}

pub(crate) trait InternalLimud<T> {
    fn limud(&self, limud_date: Date<Hebrew>) -> Option<T> {
        let cycle = self.find_cycle(limud_date)?;
        if cycle.end_date < limud_date {
            return None;
        }
        let mut interval = Interval::first_for_cycle(cycle, Self::interval_end_calculation)?;
        while !interval.contains(limud_date) {
            interval = if self.is_skip_interval(&interval) {
                interval.skip(Self::interval_end_calculation)?
            } else {
                interval.next(Self::interval_end_calculation)?
            };
        }
        if self.is_skip_interval(&interval) {
            return None;
        }
        self.unit_for_interval(&interval, &limud_date)
    }
    fn cycle_finder(&self) -> CycleFinder;
    fn find_cycle(&self, date: Date<Hebrew>) -> Option<Cycle> {
        match self.cycle_finder() {
            CycleFinder::Initial(initial_cycle_date) => {
                Cycle::from_cycle_initiation(initial_cycle_date, Self::cycle_end_calculation, date)
            }
            CycleFinder::Perpetual(finder) => Some(Cycle::from_perpetual(finder, date)?),
        }
    }
    fn cycle_end_calculation(hebrew_date: Date<Hebrew>, _iteration: Option<i32>) -> Option<Date<Hebrew>> {
        Some(hebrew_date)
    }
    fn interval_end_calculation(_cycle: Cycle, hebrew_date: Date<Hebrew>) -> Option<Date<Hebrew>> {
        Some(hebrew_date)
    }
    fn is_skip_interval(&self, _interval: &Interval) -> bool {
        false
    }
    /// Returns the learning unit for `limud_date` within `interval`.
    ///
    /// Must not call `self.limud()` here: the default [`Self::limud`] routes through
    /// `unit_for_interval` and would recurse infinitely.
    fn unit_for_interval(&self, interval: &Interval, limud_date: &Date<Hebrew>) -> Option<T>;
}

#[cfg(test)]
#[allow(clippy::expect_used)]
/// A helper function to convert a Gregorian date to a Hebrew date.
/// This is only allowed in test code.
pub(crate) fn from_gregorian_date(year: i32, month: u8, day: u8) -> Date<Hebrew> {
    Date::try_new_gregorian(year, month, day)
        .expect("hard-coded Gregorian date should be valid")
        .to_calendar(Hebrew)
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use icu_calendar::{Date, cal::Hebrew};

    use super::*;

    fn cycle_end(_start: Date<Hebrew>, _iteration: Option<i32>) -> Option<Date<Hebrew>> {
        Some(_start)
    }

    #[test]
    fn from_cycle_initiation_finds_cycle_containing_date() {
        let initial = from_gregorian_date(2000, 1, 1);

        fn ten_day_cycle(start: Date<Hebrew>, _iteration: Option<i32>) -> Option<Date<Hebrew>> {
            start.add_days(9)
        }

        let target = initial.add_days(5).expect("valid date");
        let cycle = Cycle::from_cycle_initiation(initial, ten_day_cycle, target).expect("cycle exists");
        assert_eq!(cycle.start_date, initial);
        assert_eq!(cycle.end_date, initial.add_days(9).expect("cycle end"));
        assert_eq!(cycle.iteration, Some(1));
    }

    #[test]
    fn from_cycle_initiation_advances_to_later_cycle() {
        let initial = from_gregorian_date(2000, 1, 1);

        fn three_day_cycle(start: Date<Hebrew>, _iteration: Option<i32>) -> Option<Date<Hebrew>> {
            start.add_days(2)
        }

        let target = initial.add_days(10).expect("valid date");
        let cycle = Cycle::from_cycle_initiation(initial, three_day_cycle, target).expect("cycle exists");
        assert_eq!(cycle.iteration, Some(4));
        assert!(cycle.start_date <= target);
        assert!(target <= cycle.end_date);
    }

    #[test]
    fn from_cycle_initiation_returns_none_before_initial_date() {
        let initial = from_gregorian_date(2000, 1, 1);
        let target = from_gregorian_date(1999, 12, 31);
        assert!(Cycle::from_cycle_initiation(initial, cycle_end, target).is_none());
    }

    fn single_day_interval(_cycle: Cycle, date: Date<Hebrew>) -> Option<Date<Hebrew>> {
        Some(date)
    }

    #[test]
    fn contains_is_inclusive() {
        let cycle = Cycle {
            start_date: from_gregorian_date(2020, 1, 1),
            end_date: from_gregorian_date(2020, 1, 7),
            iteration: Some(1),
        };
        let interval = Interval::first_for_cycle(cycle, single_day_interval).expect("interval");
        assert!(interval.contains(from_gregorian_date(2020, 1, 1)));
        assert!(interval.contains(interval.end_date));
        assert!(!interval.contains(from_gregorian_date(2020, 1, 2)));
    }

    #[test]
    fn next_advances_iteration_within_cycle() {
        let cycle = Cycle {
            start_date: from_gregorian_date(2020, 1, 1),
            end_date: from_gregorian_date(2020, 1, 10),
            iteration: Some(1),
        };
        let first = Interval::first_for_cycle(cycle, single_day_interval).expect("first interval");
        let second = first.next(single_day_interval).expect("second interval");
        assert_eq!(second.iteration, 2);
        assert_eq!(second.start_date, first.end_date.add_days(1).expect("next day"));
    }

    #[test]
    fn skip_keeps_iteration() {
        let cycle = Cycle {
            start_date: from_gregorian_date(2020, 1, 1),
            end_date: from_gregorian_date(2020, 1, 10),
            iteration: Some(1),
        };
        let first = Interval::first_for_cycle(cycle, single_day_interval).expect("first interval");
        let skipped = first.skip(single_day_interval).expect("skipped interval");
        assert_eq!(skipped.iteration, first.iteration);
    }

    struct DailyCounter;

    impl InternalLimud<u32> for DailyCounter {
        fn cycle_finder(&self) -> CycleFinder {
            CycleFinder::Initial(from_gregorian_date(2020, 1, 1))
        }

        fn cycle_end_calculation(hebrew_date: Date<Hebrew>, _iteration: Option<i32>) -> Option<Date<Hebrew>> {
            hebrew_date.add_days(4)
        }

        fn interval_end_calculation(_cycle: Cycle, hebrew_date: Date<Hebrew>) -> Option<Date<Hebrew>> {
            Some(hebrew_date)
        }

        fn unit_for_interval(&self, interval: &Interval, _limud_date: &Date<Hebrew>) -> Option<u32> {
            Some(interval.iteration as u32)
        }
    }

    #[test]
    fn default_limud_engine_terminates_for_toy_calculator() {
        let calculator = DailyCounter;
        let limud_date = from_gregorian_date(2020, 1, 3);
        assert_eq!(calculator.limud(limud_date), Some(3));
    }
}
