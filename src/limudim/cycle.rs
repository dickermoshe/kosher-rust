use icu_calendar::{Date, cal::Hebrew};

use crate::limudim::{HebrewDateExt, limud::PerpetualCycleFinder};

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

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use icu_calendar::{Date, cal::Hebrew};

    use crate::limudim::{HebrewDateExt, from_gregorian_date};

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
}
