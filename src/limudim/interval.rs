use icu_calendar::{Date, cal::Hebrew};

use crate::limudim::{HebrewDateExt, cycle::Cycle};

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

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use icu_calendar::{Date, cal::Hebrew};

    use crate::limudim::{HebrewDateExt, cycle::Cycle, from_gregorian_date};

    use super::*;

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
}
