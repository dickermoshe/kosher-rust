use icu_calendar::{Date, cal::Hebrew};

use crate::limudim::{cycle::Cycle, interval::Interval};

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
/// Trait for calculators that can be used to calculate the limud for a given date.
#[allow(private_bounds)]
pub trait Limud<T>: InternalLimud<T> {}
