//! Performance tests: every calendar, limud, and zmanim calculation must complete within 10 ms.
//!
//! These tests only run in release builds where the 10 ms budget applies.
//! Run with: `cargo test --test benchmarks --release`

#![cfg(not(debug_assertions))]

use std::time::{Duration, Instant};

use icu_calendar::{Date, cal::Hebrew};
use jiff::{civil::Date as CivilDate, tz::TimeZone};
use kosher_rust::calendar::{HebrewCalendar, HebrewCalendarDate, HebrewHolidayCalendar, month::*};
use kosher_rust::limudim::prelude::*;
use kosher_rust::zmanim::prelude::presets::*;
use kosher_rust::zmanim::prelude::*;

const MAX_DURATION: Duration = Duration::from_millis(10);
const WARMUP_ITERATIONS: usize = 50;
const SAMPLE_ITERATIONS: usize = 500;

/// Runs `operation` after warmup and asserts every timed sample stays under [`MAX_DURATION`].
fn assert_max_duration(name: &str, mut operation: impl FnMut()) {
    for _ in 0..WARMUP_ITERATIONS {
        operation();
    }

    let mut max_elapsed = Duration::ZERO;
    for _ in 0..SAMPLE_ITERATIONS {
        let start = Instant::now();
        operation();
        max_elapsed = max_elapsed.max(start.elapsed());
    }

    assert!(
        max_elapsed <= MAX_DURATION,
        "{name}: max {max_elapsed:?} exceeds {MAX_DURATION:?} limit"
    );
}

fn sample_hebrew_dates() -> [Date<Hebrew>; 6] {
    [
        Date::try_new_hebrew_v2(5784, TISHREI, 1).expect("valid date"),
        Date::try_new_hebrew_v2(5784, NISAN, 15).expect("valid date"),
        Date::try_new_hebrew_v2(5784, KISLEV, 25).expect("valid date"),
        Date::try_new_hebrew_v2(5784, ADAR, 14).expect("valid date"),
        Date::try_new_hebrew_v2(5784, AV, 9).expect("valid date"),
        Date::try_new_hebrew_v2(5784, ADARI, 10).expect("valid date"),
    ]
}

fn sample_gregorian_dates() -> [Date<icu_calendar::cal::Gregorian>; 3] {
    [
        Date::try_new_gregorian(2024, 1, 20).expect("valid date"),
        Date::try_new_gregorian(2025, 4, 13).expect("valid date"),
        Date::try_new_gregorian(2024, 10, 3).expect("valid date"),
    ]
}

mod calendar {
    use super::*;

    #[test]
    fn hebrew_date_conversion() {
        let dates = sample_gregorian_dates();
        for (index, date) in dates.iter().enumerate() {
            let date = *date;
            assert_max_duration(&format!("hebrew_date conversion [{index}]"), move || {
                let _ = date.hebrew_date();
            });
        }
    }

    #[test]
    fn holidays() {
        let dates = sample_hebrew_dates();
        for (index, date) in dates.iter().enumerate() {
            let date = *date;
            assert_max_duration(&format!("holidays [{index}]"), move || {
                let _ = date.holidays(false, true).count();
            });
        }
    }

    #[test]
    fn is_assur_bemelacha() {
        let dates = sample_hebrew_dates();
        for (index, date) in dates.iter().enumerate() {
            let date = *date;
            assert_max_duration(&format!("is_assur_bemelacha [{index}]"), move || {
                let _ = date.is_assur_bemelacha(false);
            });
        }
    }

    #[test]
    fn has_candle_lighting() {
        let dates = sample_hebrew_dates();
        for (index, date) in dates.iter().enumerate() {
            let date = *date;
            assert_max_duration(&format!("has_candle_lighting [{index}]"), move || {
                let _ = date.has_candle_lighting(false);
            });
        }
    }

    #[test]
    fn is_aseres_yemei_teshuva() {
        let dates = sample_hebrew_dates();
        for (index, date) in dates.iter().enumerate() {
            let date = *date;
            assert_max_duration(&format!("is_aseres_yemei_teshuva [{index}]"), move || {
                let _ = date.is_aseres_yemei_teshuva();
            });
        }
    }

    #[test]
    fn todays_parsha() {
        let dates = sample_hebrew_dates();
        for (index, date) in dates.iter().enumerate() {
            let date = *date;
            assert_max_duration(&format!("todays_parsha [{index}]"), move || {
                let _ = date.todays_parsha(false);
            });
        }
    }

    #[test]
    fn special_parsha() {
        let dates = sample_hebrew_dates();
        for (index, date) in dates.iter().enumerate() {
            let date = *date;
            assert_max_duration(&format!("special_parsha [{index}]"), move || {
                let _ = date.special_parsha(false);
            });
        }
    }

    #[test]
    fn upcoming_parsha() {
        let dates = sample_hebrew_dates();
        for (index, date) in dates.iter().enumerate() {
            let date = *date;
            assert_max_duration(&format!("upcoming_parsha [{index}]"), move || {
                let _ = date.upcoming_parsha(false);
            });
        }
    }

    #[test]
    fn day_of_chanukah() {
        let dates = sample_hebrew_dates();
        for (index, date) in dates.iter().enumerate() {
            let date = *date;
            assert_max_duration(&format!("day_of_chanukah [{index}]"), move || {
                let _ = date.day_of_chanukah();
            });
        }
    }

    #[test]
    fn day_of_the_omer() {
        let dates = sample_hebrew_dates();
        for (index, date) in dates.iter().enumerate() {
            let date = *date;
            assert_max_duration(&format!("day_of_the_omer [{index}]"), move || {
                let _ = date.day_of_the_omer();
            });
        }
    }

    #[test]
    fn year_and_month_queries() {
        assert_max_duration("days_in_hebrew_year", || {
            let _ = Hebrew::days_in_hebrew_year(5784);
        });
        assert_max_duration("days_in_hebrew_month", || {
            let _ = Hebrew::days_in_hebrew_month(5784, NISAN);
        });
        assert_max_duration("is_hebrew_leap_year", || {
            let _ = Hebrew::is_hebrew_leap_year(5784);
        });
        assert_max_duration("cheshvan_kislev_kviah", || {
            let _ = Hebrew::cheshvan_kislev_kviah(5784);
        });
    }
}

mod limudim {
    use super::*;

    fn limud_dates() -> [Date<Hebrew>; 4] {
        [
            Date::try_new_hebrew_v2(5784, TISHREI, 10).expect("valid date"),
            Date::try_new_hebrew_v2(5784, NISAN, 15).expect("valid date"),
            Date::try_new_hebrew_v2(5784, AV, 9).expect("valid date"),
            Date::try_new_hebrew_v2(5785, IYYAR, 18).expect("valid date"),
        ]
    }

    #[test]
    fn daf_yomi_bavli() {
        let dates = limud_dates();
        for (index, date) in dates.iter().enumerate() {
            let date = *date;
            assert_max_duration(&format!("DafYomiBavli [{index}]"), move || {
                let _ = date.limud(DafYomiBavli::default());
            });
        }
    }

    #[test]
    fn daf_yomi_yerushalmi() {
        let dates = limud_dates();
        for (index, date) in dates.iter().enumerate() {
            let date = *date;
            assert_max_duration(&format!("DafYomiYerushalmiVilna [{index}]"), move || {
                let _ = date.limud(DafYomiYerushalmiVilna::default());
            });
        }
    }

    #[test]
    fn daf_hashavua_bavli() {
        let dates = limud_dates();
        for (index, date) in dates.iter().enumerate() {
            let date = *date;
            assert_max_duration(&format!("DafHashavuaBavli [{index}]"), move || {
                let _ = date.limud(DafHashavuaBavli::default());
            });
        }
    }

    #[test]
    fn amud_yomi_bavli_dirshu() {
        let dates = limud_dates();
        for (index, date) in dates.iter().enumerate() {
            let date = *date;
            assert_max_duration(&format!("AmudYomiBavliDirshu [{index}]"), move || {
                let _ = date.limud(AmudYomiBavliDirshu::default());
            });
        }
    }

    #[test]
    fn mishna_yomis() {
        let dates = limud_dates();
        for (index, date) in dates.iter().enumerate() {
            let date = *date;
            assert_max_duration(&format!("MishnaYomis [{index}]"), move || {
                let _ = date.limud(MishnaYomis);
            });
        }
    }

    #[test]
    fn pirkei_avos() {
        let dates = limud_dates();
        for (index, date) in dates.iter().enumerate() {
            let date = *date;
            assert_max_duration(&format!("PirkeiAvos diaspora [{index}]"), move || {
                let _ = date.limud(PirkeiAvos { in_israel: false });
            });
            assert_max_duration(&format!("PirkeiAvos israel [{index}]"), move || {
                let _ = date.limud(PirkeiAvos { in_israel: true });
            });
        }
    }

    #[test]
    fn tehillim_monthly() {
        let dates = limud_dates();
        for (index, date) in dates.iter().enumerate() {
            let date = *date;
            assert_max_duration(&format!("TehillimMonthly [{index}]"), move || {
                let _ = date.limud(TehillimMonthly);
            });
        }
    }
}

mod zmanim {
    use super::*;

    struct ZmanimCase {
        label: &'static str,
        location: Location,
        date: CivilDate,
        config: CalculatorConfig,
    }

    fn sample_zmanim_cases() -> [ZmanimCase; 4] {
        [
            ZmanimCase {
                label: "lakewood",
                location: Location::new(
                    40.0721087,
                    -74.2400243,
                    15.0,
                    Some(TimeZone::get("America/New_York").expect("valid timezone")),
                )
                .expect("valid location"),
                date: CivilDate::new(2017, 10, 17).expect("valid date"),
                config: CalculatorConfig::default(),
            },
            ZmanimCase {
                label: "jerusalem",
                location: Location::new(
                    31.778,
                    35.235,
                    754.0,
                    Some(TimeZone::get("Asia/Jerusalem").expect("valid timezone")),
                )
                .expect("valid location"),
                date: CivilDate::new(2024, 4, 22).expect("valid date"),
                config: CalculatorConfig::default(),
            },
            ZmanimCase {
                label: "reykjavik_equinox",
                location: Location::new(
                    64.1466,
                    -21.9426,
                    0.0,
                    Some(TimeZone::get("Atlantic/Reykjavik").expect("valid timezone")),
                )
                .expect("valid location"),
                date: CivilDate::new(2017, 3, 21).expect("valid date"),
                config: CalculatorConfig::default(),
            },
            ZmanimCase {
                label: "baltimore_kidush_levana",
                location: Location::new(
                    39.36463,
                    -76.70222,
                    0.0,
                    Some(TimeZone::get("America/New_York").expect("valid timezone")),
                )
                .expect("valid location"),
                date: CivilDate::new(2026, 1, 3).expect("valid date"),
                config: CalculatorConfig::default(),
            },
        ]
    }

    fn calculator(case: &ZmanimCase) -> ZmanimCalculator {
        ZmanimCalculator::new(case.location.clone(), case.date, case.config)
    }

    #[test]
    fn sun_times() {
        let cases = sample_zmanim_cases();
        for case in cases.iter() {
            let calc = calculator(case);
            let label = case.label;
            assert_max_duration(&format!("ELEVATION_ADJUSTED_SUNRISE [{label}]"), {
                let calc = calc.clone();
                move || {
                    let _ = calc.calculate(&ELEVATION_ADJUSTED_SUNRISE);
                }
            });
            assert_max_duration(&format!("ELEVATION_ADJUSTED_SUNSET [{label}]"), {
                let calc = calc.clone();
                move || {
                    let _ = calc.calculate(&ELEVATION_ADJUSTED_SUNSET);
                }
            });
            assert_max_duration(&format!("SEA_LEVEL_SUNRISE [{label}]"), {
                let calc = calc.clone();
                move || {
                    let _ = calc.calculate(&SEA_LEVEL_SUNRISE);
                }
            });
            assert_max_duration(&format!("SEA_LEVEL_SUNSET [{label}]"), move || {
                let _ = calc.calculate(&SEA_LEVEL_SUNSET);
            });
        }
    }

    #[test]
    fn twilight() {
        let cases = sample_zmanim_cases();
        for case in cases.iter() {
            let calc = calculator(case);
            let label = case.label;
            assert_max_duration(&format!("ALOS_16_POINT_1_DEGREES [{label}]"), {
                let calc = calc.clone();
                move || {
                    let _ = calc.calculate(&ALOS_16_POINT_1_DEGREES);
                }
            });
            assert_max_duration(&format!("TZAIS_16_POINT_1_DEGREES [{label}]"), {
                let calc = calc.clone();
                move || {
                    let _ = calc.calculate(&TZAIS_16_POINT_1_DEGREES);
                }
            });
        }
    }

    #[test]
    fn chatzos() {
        let cases = sample_zmanim_cases();
        for case in cases.iter() {
            let calc = calculator(case);
            let label = case.label;
            assert_max_duration(&format!("CHATZOS_HAYOM [{label}]"), {
                let calc = calc.clone();
                move || {
                    let _ = calc.calculate(&CHATZOS_HAYOM);
                }
            });
            assert_max_duration(&format!("CHATZOS_HALAYLA [{label}]"), {
                let calc = calc.clone();
                move || {
                    let _ = calc.calculate(&CHATZOS_HALAYLA);
                }
            });
        }
    }

    #[test]
    fn daily_zmanim() {
        let cases = sample_zmanim_cases();
        for case in cases.iter() {
            let calc = calculator(case);
            let label = case.label;
            assert_max_duration(&format!("SOF_ZMAN_SHMA_GRA [{label}]"), {
                let calc = calc.clone();
                move || {
                    let _ = calc.calculate(&SOF_ZMAN_SHMA_GRA);
                }
            });
            assert_max_duration(&format!("MINCHA_GEDOLA_GRA [{label}]"), {
                let calc = calc.clone();
                move || {
                    let _ = calc.calculate(&MINCHA_GEDOLA_GRA);
                }
            });
            assert_max_duration(&format!("PLAG_HAMINCHA_16_POINT_1_DEGREES [{label}]"), {
                let calc = calc.clone();
                move || {
                    let _ = calc.calculate(&PLAG_HAMINCHA_16_POINT_1_DEGREES);
                }
            });
            assert_max_duration(&format!("CANDLE_LIGHTING [{label}]"), {
                let calc = calc.clone();
                move || {
                    let _ = calc.calculate(&CANDLE_LIGHTING);
                }
            });
        }
    }

    #[test]
    fn kidush_levana() {
        let cases = sample_zmanim_cases();
        for case in cases.iter() {
            let calc = calculator(case);
            let label = case.label;
            assert_max_duration(&format!("SOF_ZMAN_KIDUSH_LEVANA_BETWEEN_MOLDOS [{label}]"), {
                let calc = calc.clone();
                move || {
                    let _ = calc.calculate(&SOF_ZMAN_KIDUSH_LEVANA_BETWEEN_MOLDOS);
                }
            });
            assert_max_duration(&format!("SOF_ZMAN_KIDUSH_LEVANA_15_DAYS [{label}]"), {
                let calc = calc.clone();
                move || {
                    let _ = calc.calculate(&SOF_ZMAN_KIDUSH_LEVANA_15_DAYS);
                }
            });
        }
    }

    #[test]
    fn full_day_sheet() {
        let cases = sample_zmanim_cases();
        for case in cases.iter() {
            let calc = calculator(case);
            let label = case.label;
            assert_max_duration(&format!("full_day_sheet [{label}]"), move || {
                let _ = calc.calculate(&ALOS_16_POINT_1_DEGREES);
                let _ = calc.calculate(&SOF_ZMAN_SHMA_GRA);
                let _ = calc.calculate(&CHATZOS_HAYOM);
                let _ = calc.calculate(&MINCHA_GEDOLA_GRA);
                let _ = calc.calculate(&PLAG_HAMINCHA_16_POINT_1_DEGREES);
                let _ = calc.calculate(&CANDLE_LIGHTING);
                let _ = calc.calculate(&ELEVATION_ADJUSTED_SUNSET);
                let _ = calc.calculate(&TZAIS_16_POINT_1_DEGREES);
            });
        }
    }
}
