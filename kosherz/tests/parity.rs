//! The bindings give the same answers as kosher-rust.
//!
//! The zmanim tests run the bindings with a `LocalDay` built from a jiff time zone and kosher-rust
//! with the time zone itself, so they also check that the local day is all the calculations need.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, missing_docs)]

use jiff::{civil, tz::TimeZone};
use kosherz::api::{
    calendar::{
        self, CalendarError, GregorianDate, HebrewDate, Region,
        limudim::{self, LimudimError},
    },
    times::{
        self, Calculator, LocalDay, Location, ZmanFailure, Zmanim, ZmanimError,
        myzmanim::{self, MyZmanim},
        sea_level_location, validate_location,
    },
};

/// `date` in `zone` as a `LocalDay`: its two local midnights, resolved the way Java does, and the
/// offsets at them.
fn local_day(zone: &TimeZone, date: civil::Date) -> LocalDay {
    let midnight = |date: civil::Date| zone.to_ambiguous_timestamp(date.at(0, 0, 0, 0)).compatible().unwrap();
    let start = midnight(date);
    let end = midnight(date.tomorrow().unwrap());
    LocalDay {
        start_ms: start.as_millisecond(),
        start_offset_seconds: zone.to_offset(start).seconds(),
        end_ms: end.as_millisecond(),
        end_offset_seconds: zone.to_offset(end).seconds(),
    }
}

struct Place {
    name: &'static str,
    latitude: f64,
    longitude: f64,
    elevation: f64,
    zone: &'static str,
}

const PLACES: [Place; 6] = [
    Place {
        name: "Lakewood",
        latitude: 40.09,
        longitude: -74.22,
        elevation: 0.0,
        zone: "America/New_York",
    },
    Place {
        name: "Jerusalem",
        latitude: 31.778,
        longitude: 35.235,
        elevation: 754.0,
        zone: "Asia/Jerusalem",
    },
    Place {
        name: "Tromsø",
        latitude: 69.65,
        longitude: 18.96,
        elevation: 0.0,
        zone: "Europe/Oslo",
    },
    Place {
        name: "Suva",
        latitude: -18.14,
        longitude: 178.44,
        elevation: 0.0,
        zone: "Pacific/Fiji",
    },
    Place {
        name: "Honolulu",
        latitude: 21.31,
        longitude: -157.86,
        elevation: 0.0,
        zone: "Pacific/Honolulu",
    },
    Place {
        name: "Melbourne",
        latitude: -37.81,
        longitude: 144.96,
        elevation: 30.0,
        zone: "Australia/Melbourne",
    },
];

fn location(place: &Place) -> Location {
    Location {
        elevation: place.elevation,
        eastern_horizon_elevation: place.elevation,
        ..sea_level_location(place.latitude, place.longitude)
    }
}

fn kosher_location(place: &Place) -> kosher_rust::Location {
    kosher_rust::Location::new(place.latitude, place.longitude)
        .unwrap()
        .with_elevation(place.elevation)
        .unwrap()
        .with_horizons(place.elevation, 0.0)
        .unwrap()
}

fn place_day(place: &Place, date: civil::Date) -> Option<LocalDay> {
    Some(local_day(&TimeZone::get(place.zone).unwrap(), date))
}

/// Every 9th day of 2024, which lands on DST changes, the solstices, Erev Pesach and
/// Kiddush Levana windows, plus the days the clocks changed in New York.
fn dates() -> Vec<civil::Date> {
    let mut dates: Vec<civil::Date> = (0..366)
        .step_by(9)
        .map(|day| {
            civil::date(2024, 1, 1)
                .checked_add(jiff::Span::new().days(day))
                .unwrap()
        })
        .collect();
    dates.extend([
        civil::date(2024, 3, 10),
        civil::date(2024, 11, 3),
        civil::date(2024, 11, 4),
    ]);
    dates
}

fn expected(result: Result<jiff::Timestamp, kosher_rust::zmanim::ZmanimError>) -> (Option<i64>, Option<ZmanFailure>) {
    use kosher_rust::zmanim::ZmanimError as E;
    match result {
        Ok(time) => (Some(time.as_millisecond()), None),
        Err(error) => (
            None,
            Some(match error {
                E::AllDay => ZmanFailure::AllDay,
                E::AllNight => ZmanFailure::AllNight,
                E::NotApplicable => ZmanFailure::NotApplicable,
                E::TimeZoneRequired => ZmanFailure::TimeZoneRequired,
                E::Internal | E::InvalidDegrees => ZmanFailure::Unexpected,
            }),
        ),
    }
}

#[test]
fn zmanim_match_kosher_rust_for_every_calculator() {
    let mut checked = 0;
    for place in &PLACES {
        for date in dates() {
            for (calculator, kosher_calculator) in [
                (Calculator::Noaa, kosher_rust::zmanim::AstronomicalCalculator::Noaa),
                (
                    Calculator::Meeus,
                    kosher_rust::zmanim::AstronomicalCalculator::Meeus { apply_delta_t: true },
                ),
                (
                    Calculator::Spa,
                    kosher_rust::zmanim::AstronomicalCalculator::Spa { apply_delta_t: true },
                ),
            ] {
                let config = times::Config {
                    calculator,
                    ..times::default_config()
                };
                let ours = Zmanim::new(location(place), date.into_gregorian(), config, place_day(place, date)).unwrap();
                let theirs = kosher_rust::Zmanim::with_config(
                    kosher_location(place),
                    date,
                    Some(TimeZone::get(place.zone).unwrap()),
                    kosher_rust::zmanim::Config {
                        astronomical_calculator: kosher_calculator,
                        ..Default::default()
                    },
                );
                for row in ours.all() {
                    let zman = kosher_rust::Zman::from(row.zman);
                    let (time, failure) = expected(theirs.get(zman));
                    assert!(
                        row.time == time && row.failure == failure,
                        "{} {date} {calculator:?} {}: bindings {:?} {:?}, kosher-rust {time:?} {failure:?}",
                        place.name,
                        zman.java_name(),
                        row.time,
                        row.failure,
                    );
                    checked += 1;
                }
            }
        }
    }
    assert!(checked > 50_000, "{checked}");
}

#[test]
fn my_zmanim_match_kosher_rust() {
    for place in &PLACES {
        for date in dates() {
            let ours = MyZmanim::new(location(place), date.into_gregorian(), 40.0, place_day(place, date)).unwrap();
            let theirs = kosher_rust::myzmanim::MyZmanim::new(
                kosher_location(place),
                date,
                Some(TimeZone::get(place.zone).unwrap()),
            )
            .candle_lighting(std::time::Duration::from_mins(40));
            for row in ours.all() {
                let zman = kosher_rust::myzmanim::MyZman::from(row.zman);
                let (time, failure) = expected(theirs.get(zman));
                assert_eq!(
                    (row.time, row.failure),
                    (time, failure),
                    "{} {date} {zman:?}",
                    place.name
                );
            }
            assert_eq!(
                ours.proportional_hour_gra().ok(),
                theirs.proportional_hour_gra().ok().map(|d| d.as_millis() as i64),
            );
        }
    }
}

trait IntoGregorian {
    fn into_gregorian(self) -> GregorianDate;
}

impl IntoGregorian for civil::Date {
    fn into_gregorian(self) -> GregorianDate {
        GregorianDate {
            year: i32::from(self.year()),
            month: i32::from(self.month()),
            day: i32::from(self.day()),
        }
    }
}

#[test]
fn zman_values_are_positions_and_round_trip() {
    let all = times::all_zmanim();
    assert_eq!(all.len(), kosher_rust::Zman::ALL.len());
    for (position, zman) in all.into_iter().enumerate() {
        assert_eq!(zman as i32, position as i32);
        let theirs = kosher_rust::Zman::from(zman);
        assert_eq!(theirs, kosher_rust::Zman::ALL[position]);
        assert_eq!(times::zman_java_name(zman), theirs.java_name());
        assert_eq!(times::zman_name(zman), theirs.name());
        assert_eq!(times::zman_is_deprecated(zman), theirs.is_deprecated());
        assert_eq!(
            kosher_rust::zmanim::Category::from(times::zman_category(zman)),
            theirs.category()
        );
        assert_eq!(
            times::zman_description(zman, times::default_config()),
            theirs.description(&Default::default())
        );
    }
    let my = myzmanim::all_my_zmanim();
    assert_eq!(my.len(), kosher_rust::myzmanim::MyZman::ALL.len());
    for (position, zman) in my.into_iter().enumerate() {
        assert_eq!(zman as i32, position as i32);
        assert_eq!(
            kosher_rust::myzmanim::MyZman::from(zman),
            kosher_rust::myzmanim::MyZman::ALL[position]
        );
    }
}

#[test]
fn config_round_trips() {
    assert_eq!(
        kosher_rust::zmanim::Config::from(times::default_config()),
        kosher_rust::zmanim::Config::default()
    );
    let custom = times::Config {
        candle_lighting_minutes: 40,
        use_astronomical_chatzos_for_other_zmanim: true,
        use_elevation: true,
        ateret_torah_sunset_minutes: 25,
        use_astronomical_chatzos: false,
        calculator: Calculator::Spa,
        apply_delta_t: false,
    };
    assert_eq!(times::Config::from(kosher_rust::zmanim::Config::from(custom)), custom);
    assert_eq!(
        times::calculator_name(Calculator::Meeus),
        "Jean Meeus Higher-Accuracy (VSOP87) Algorithm"
    );

    // NOAA ignores delta-T, and `config` still gives back what was passed in.
    let noaa = times::Config {
        calculator: Calculator::Noaa,
        apply_delta_t: false,
        ..custom
    };
    let date = GregorianDate {
        year: 2024,
        month: 4,
        day: 23,
    };
    let day = Zmanim::new(sea_level_location(40.09, -74.22), date, noaa, None).unwrap();
    assert_eq!(day.config(), noaa);
}

#[test]
fn errors() {
    let lakewood = sea_level_location(40.09, -74.22);
    let date = GregorianDate {
        year: 2024,
        month: 11,
        day: 4,
    };
    let config = times::default_config();
    let levana = times::Zman::TchilasZmanKidushLevana3Days;

    let utc = || Some(local_day(&TimeZone::UTC, civil::date(2024, 11, 4)));
    let bad = |location: Location| Zmanim::new(location, date, config, utc()).err();
    assert_eq!(bad(sea_level_location(91.0, 0.0)), Some(ZmanimError::InvalidLatitude));
    assert_eq!(bad(sea_level_location(0.0, 181.0)), Some(ZmanimError::InvalidLongitude));
    assert_eq!(
        bad(Location {
            elevation: -1.0,
            ..lakewood
        }),
        Some(ZmanimError::InvalidElevation)
    );
    assert_eq!(
        Zmanim::new(sea_level_location(0.0, 179.0), date, config, None).err(),
        Some(ZmanimError::TimeZoneRequired)
    );
    assert_eq!(validate_location(sea_level_location(0.0, 179.0), true), Ok(()));
    assert_eq!(
        Zmanim::new(lakewood, GregorianDate { day: 31, ..date }, config, None).err(),
        Some(ZmanimError::InvalidGregorianDate)
    );

    let no_zone = Zmanim::new(lakewood, date, config, None).unwrap();
    assert_eq!(no_zone.get(levana), Err(ZmanimError::TimeZoneRequired));
    assert!(no_zone.get(times::Zman::Sunrise).is_ok());

    let utc_day = local_day(&TimeZone::UTC, civil::date(2024, 11, 4));
    let impossible = |day: LocalDay| Zmanim::new(lakewood, date, config, Some(day)).err();
    assert_eq!(
        impossible(LocalDay {
            end_ms: utc_day.start_ms - 1,
            ..utc_day
        }),
        Some(ZmanimError::InvalidLocalDay)
    );
    assert_eq!(
        impossible(LocalDay {
            end_ms: utc_day.start_ms + 48 * 3_600_000 + 1,
            ..utc_day
        }),
        Some(ZmanimError::InvalidLocalDay)
    );
    for seconds in [i32::MIN, -86_400, 86_400, i32::MAX] {
        assert_eq!(
            impossible(LocalDay {
                start_offset_seconds: seconds,
                ..utc_day
            }),
            Some(ZmanimError::InvalidLocalDay),
            "{seconds}"
        );
        assert_eq!(
            impossible(LocalDay {
                end_offset_seconds: seconds,
                ..utc_day
            }),
            Some(ZmanimError::InvalidLocalDay),
            "{seconds}"
        );
    }
    assert_eq!(
        MyZmanim::new(
            lakewood,
            date,
            18.0,
            Some(LocalDay {
                end_ms: utc_day.start_ms - 1,
                ..utc_day
            })
        )
        .err(),
        Some(ZmanimError::InvalidLocalDay)
    );
    // A day the clocks skip starts and ends at the same instant.
    assert_eq!(
        impossible(LocalDay {
            end_ms: utc_day.start_ms,
            ..utc_day
        }),
        None
    );
}

/// Numbers at and past the limits the bindings accept.
#[test]
fn inputs_at_their_limits() {
    let date = GregorianDate {
        year: 2024,
        month: 4,
        day: 23,
    };
    let config = times::default_config();
    // Longitude 170 needs the time zone, so every time asks it.
    let offset = |seconds: i32| {
        let day = local_day(
            &TimeZone::fixed(jiff::tz::Offset::from_seconds(seconds).unwrap()),
            civil::date(2024, 4, 23),
        );
        Zmanim::new(sea_level_location(0.0, 170.0), date, config, Some(day))
            .unwrap()
            .get(times::Zman::Sunrise)
    };
    for seconds in [-86_399, 0, 86_399] {
        assert!(offset(seconds).is_ok(), "{seconds}");
    }

    assert_eq!(times::shaah_zmanis(0, 12 * 3_600_000), 3_600_000);
    assert_eq!(times::shaah_zmanis(i64::MIN, i64::MAX), 1_537_228_672_809_129_301);
    assert_eq!(times::shaah_zmanis(i64::MAX, i64::MIN), -1_537_228_672_809_129_301);

    let lakewood = sea_level_location(40.09, -74.22);
    let with_minutes = |candle_lighting_minutes: i32, ateret_torah_sunset_minutes: i32| {
        let config = times::Config {
            candle_lighting_minutes,
            ateret_torah_sunset_minutes,
            ..config
        };
        Zmanim::new(lakewood, date, config, None).map(|_| ())
    };
    assert_eq!(with_minutes(0, -1440), Ok(()));
    assert_eq!(with_minutes(1440, 1440), Ok(()));
    assert_eq!(with_minutes(-1, 40), Err(ZmanimError::InvalidMinutes));
    assert_eq!(with_minutes(1441, 40), Err(ZmanimError::InvalidMinutes));
    assert_eq!(with_minutes(18, -1441), Err(ZmanimError::InvalidMinutes));
    assert_eq!(with_minutes(i32::MAX, i32::MIN), Err(ZmanimError::InvalidMinutes));

    let candle_lighting = |minutes: f64| MyZmanim::new(lakewood, date, minutes, None).map(|_| ());
    for minutes in [0.0, 40.5, 1440.0] {
        assert_eq!(candle_lighting(minutes), Ok(()), "{minutes}");
    }
    for minutes in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY, 1440.5, -0.5, -1440.0, -1e15] {
        assert_eq!(candle_lighting(minutes), Err(ZmanimError::InvalidMinutes), "{minutes}");
    }
}

/// Days the clocks treated unusually, near the date line and on half-hour changes: the local day
/// still gives every time kosher-rust gives with the full time zone.
#[test]
fn unusual_days_match_kosher_rust() {
    let cases = [
        // Samoa skipped 30 December 2011, crossing the date line.
        (
            "Pacific/Apia",
            -13.83,
            -171.76,
            [(2011, 12, 29), (2011, 12, 30), (2011, 12, 31)],
        ),
        // Kiritimati skipped 31 December 1994.
        (
            "Pacific/Kiritimati",
            1.87,
            -157.4,
            [(1994, 12, 30), (1994, 12, 31), (1995, 1, 1)],
        ),
        // Kwajalein skipped 21 August 1993.
        (
            "Pacific/Kwajalein",
            8.72,
            167.73,
            [(1993, 8, 20), (1993, 8, 21), (1993, 8, 22)],
        ),
        // Lord Howe Island moves its clocks by half an hour.
        (
            "Australia/Lord_Howe",
            -31.55,
            159.08,
            [(2024, 4, 6), (2024, 4, 7), (2024, 10, 6)],
        ),
        // New York's changes, the night they happen and the day after.
        (
            "America/New_York",
            40.09,
            -74.22,
            [(2024, 3, 10), (2024, 11, 3), (2024, 11, 4)],
        ),
    ];
    for (name, latitude, longitude, dates) in cases {
        let zone = TimeZone::get(name).unwrap();
        for (year, month, day) in dates {
            let date = civil::date(year, month, day);
            let ours = Zmanim::new(
                sea_level_location(latitude, longitude),
                date.into_gregorian(),
                times::default_config(),
                Some(local_day(&zone, date)),
            )
            .unwrap();
            let theirs = kosher_rust::Zmanim::new(
                kosher_rust::Location::new(latitude, longitude).unwrap(),
                date,
                Some(zone.clone()),
            );
            for row in ours.all() {
                let zman = kosher_rust::Zman::from(row.zman);
                let (time, failure) = expected(theirs.get(zman));
                assert_eq!(
                    (row.time, row.failure),
                    (time, failure),
                    "{name} {date} {}",
                    zman.java_name()
                );
            }
            let my_ours = MyZmanim::new(
                sea_level_location(latitude, longitude),
                date.into_gregorian(),
                18.0,
                Some(local_day(&zone, date)),
            )
            .unwrap();
            let my_theirs = kosher_rust::myzmanim::MyZmanim::new(
                kosher_rust::Location::new(latitude, longitude).unwrap(),
                date,
                Some(zone.clone()),
            );
            for row in my_ours.all() {
                let zman = kosher_rust::myzmanim::MyZman::from(row.zman);
                assert_eq!(
                    (row.time, row.failure),
                    expected(my_theirs.get(zman)),
                    "{name} {date} {zman:?}"
                );
            }
        }
    }
}

/// Kiddush Levana on the day after the clocks went back. The day check compares the time with the
/// local midnights that start and end the day.
#[test]
fn kiddush_levana_after_a_clock_change() {
    let lakewood = sea_level_location(40.09, -74.22);
    let date = GregorianDate {
        year: 2024,
        month: 11,
        day: 4,
    };
    let zone = local_day(&TimeZone::get("America/New_York").unwrap(), civil::date(2024, 11, 4));
    let zmanim = Zmanim::new(lakewood, date, times::default_config(), Some(zone)).unwrap();
    let time = zmanim.get(times::Zman::TchilasZmanKidushLevana3Days).unwrap();
    let local = jiff::Timestamp::from_millisecond(time)
        .unwrap()
        .to_zoned(TimeZone::get("America/New_York").unwrap());
    assert_eq!(local.strftime("%Y-%m-%d %H:%M:%S").to_string(), "2024-11-04 08:44:50");
}

#[test]
fn calendar_matches_kosher_rust() {
    let mut date = civil::date(2023, 9, 1);
    while date < civil::date(2025, 10, 1) {
        let ours = HebrewDate::from_gregorian(date.into_gregorian()).unwrap();
        let theirs = kosher_rust::HebrewDate::try_from(date).unwrap();
        assert_eq!(ours.year(), theirs.year().get());
        assert_eq!(kosher_rust::calendar::HebrewMonth::from(ours.month()), theirs.month());
        assert_eq!(ours.day(), i32::from(theirs.day()));
        assert_eq!(ours.day_of_year(), i32::from(theirs.day_of_year()));
        assert_eq!(ours.display(), theirs.to_string());
        assert_eq!(ours.to_gregorian(), Some(date.into_gregorian()));
        assert_eq!(ours.molad(), theirs.molad().map(|m| m.timestamp().as_millisecond()));
        assert_eq!(ours.is_aseres_yemei_teshuva(), theirs.is_aseres_yemei_teshuva());
        for (region, kosher_region) in [
            (Region::Israel, kosher_rust::Region::Israel),
            (Region::Diaspora, kosher_rust::Region::Diaspora),
        ] {
            let special_days: Vec<kosher_rust::calendar::SpecialDay> = ours
                .special_days(region)
                .into_iter()
                .map(|special_day| special_day.try_into().unwrap())
                .collect();
            assert_eq!(
                special_days,
                theirs.special_days(kosher_region).collect::<Vec<_>>(),
                "{date}"
            );
            assert_eq!(
                ours.is_assur_bemelacha(region),
                theirs.is_assur_bemelacha(kosher_region)
            );
            assert_eq!(
                ours.has_candle_lighting(region),
                theirs.has_candle_lighting(kosher_region)
            );
            assert_eq!(ours.parsha(region).map(Into::into), theirs.parsha(kosher_region));
            assert_eq!(
                ours.special_parsha(region).map(Into::into),
                theirs.special_parsha(kosher_region)
            );
            assert_eq!(
                ours.next_parsha(region).map(Into::into),
                theirs.next_parsha(kosher_region)
            );
            assert_eq!(
                limudim::pirkei_avos(&ours, region).map(|unit| limudim::pirkei_avos_display(unit).unwrap()),
                kosher_rust::limudim::Schedule::on(&kosher_rust::limudim::PirkeiAvos(kosher_region), date)
                    .map(|unit| unit.to_string())
            );
        }
        let next = ours.add_days(1).unwrap();
        assert_eq!(next.compare(&ours), 1);
        assert_eq!(ours.compare(&ours), 0);
        assert!(ours.equals(&HebrewDate::from_gregorian(date.into_gregorian()).unwrap()));
        date = date.tomorrow().unwrap();
    }
}

#[test]
fn limudim_match_kosher_rust() {
    use kosher_rust::limudim::{self as k, Schedule};

    let day = HebrewDate::from_gregorian(GregorianDate {
        year: 2024,
        month: 4,
        day: 23,
    })
    .unwrap();
    assert_eq!(limudim::daf_yomi_bavli(&day).unwrap().display, "Bava Metzia 55");
    assert_eq!(limudim::mishna_yomis(&day).unwrap().display, "Nazir 5:5-6");

    let mut date = civil::date(2023, 1, 1);
    while date < civil::date(2025, 1, 1) {
        let ours = HebrewDate::from_gregorian(date.into_gregorian()).unwrap();
        assert_eq!(
            limudim::daf_yomi_bavli(&ours).map(|d| d.display),
            k::DafYomiBavli.on(date).map(|d| d.to_string())
        );
        assert_eq!(
            limudim::daf_hashavua_bavli(&ours).map(|d| d.display),
            k::DafHashavuaBavli.on(date).map(|d| d.to_string())
        );
        assert_eq!(
            limudim::daf_yomi_yerushalmi(&ours).map(|d| d.display),
            k::DafYomiYerushalmi.on(date).map(|d| d.to_string())
        );
        assert_eq!(
            limudim::amud_yomi_dirshu(&ours).map(|a| a.display),
            k::AmudYomiDirshu.on(date).map(|a| a.to_string())
        );
        assert_eq!(
            limudim::mishna_yomis(&ours).map(|m| m.display),
            k::MishnaYomis.on(date).map(|m| m.to_string())
        );
        assert_eq!(
            limudim::tehillim_monthly(&ours).map(|unit| limudim::tehillim_display(unit).unwrap()),
            k::TehillimMonthly.on(date).map(|t| t.to_string())
        );
        if let Some(unit) = limudim::tehillim_monthly(&ours) {
            assert_eq!(
                k::TehillimUnit::try_from(unit),
                Ok(k::TehillimMonthly.on(date).unwrap())
            );
        }
        if let Some(daf) = limudim::daf_yomi_bavli(&ours) {
            let theirs = k::DafYomiBavli.on(date).unwrap();
            assert_eq!(k::Tractate::from(daf.tractate), theirs.tractate);
            assert_eq!(limudim::tractate_name(daf.tractate), theirs.tractate.to_string());
            assert_eq!(limudim::tractate_hebrew(daf.tractate), theirs.tractate.hebrew());
        }
        date = date.tomorrow().unwrap();
    }
}

#[test]
fn enums_round_trip() {
    for value in 0..=69u8 {
        let theirs = kosher_rust::calendar::Parsha::try_from(value).unwrap();
        let ours = calendar::Parsha::from(theirs);
        assert_eq!(ours as i32, i32::from(value));
        assert_eq!(kosher_rust::calendar::Parsha::from(ours), theirs);
        assert_eq!(calendar::parsha_name(ours), theirs.to_string());
        assert_eq!(calendar::parsha_hebrew(ours), theirs.hebrew());
    }
    for year in [5784, 5785] {
        let info = calendar::year_info(year).unwrap();
        let theirs = kosher_rust::calendar::HebrewYear::new(year).unwrap();
        assert_eq!(info.is_leap, theirs.is_leap());
        assert_eq!(info.days, i32::from(theirs.days()));
        assert_eq!(info.months.len(), theirs.months().len());
        for &month in &info.months {
            assert_eq!(
                calendar::days_in_month(year, month),
                theirs.days_in(month.into()).map(i32::from)
            );
            assert_eq!(
                calendar::month_name(month),
                kosher_rust::calendar::HebrewMonth::from(month).to_string()
            );
        }
        let rosh_hashana = calendar::rosh_hashana(year).unwrap();
        assert_eq!((rosh_hashana.day(), rosh_hashana.year()), (1, year));
    }
    assert_eq!(calendar::days_in_month(5785, calendar::HebrewMonth::AdarI), None);
    let chanukah = calendar::SpecialDay::Chanukah { day: 3 };
    assert_eq!(
        kosher_rust::calendar::SpecialDay::try_from(chanukah),
        Ok(kosher_rust::calendar::SpecialDay::Chanukah(3))
    );
    assert_eq!(
        calendar::special_day_is_fast_day(calendar::SpecialDay::YomKippur),
        Ok(true)
    );
    assert_eq!(
        calendar::special_day_is_modern(calendar::SpecialDay::YomHaatzmaut),
        Ok(true)
    );
}

/// Out-of-range numbers from outside are rejected, not clamped.
#[test]
fn out_of_range_values_are_rejected() {
    use calendar::{HebrewMonth, SpecialDay};
    use limudim::{PirkeiAvosUnit, TehillimUnit};

    let day = Zmanim::new(
        sea_level_location(40.09, -74.22),
        GregorianDate {
            year: 2024,
            month: 4,
            day: 23,
        },
        times::default_config(),
        None,
    )
    .unwrap();
    for degrees in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY, 90.5, -90.5] {
        assert_eq!(day.sunrise_offset_by_degrees(degrees), Err(ZmanimError::InvalidDegrees));
        assert_eq!(day.sunset_offset_by_degrees(degrees), Err(ZmanimError::InvalidDegrees));
    }
    assert!(day.sunrise_offset_by_degrees(16.1).is_ok());

    for special_day in [
        SpecialDay::Chanukah { day: 0 },
        SpecialDay::Chanukah { day: 9 },
        SpecialDay::CountOfTheOmer { day: 50 },
        SpecialDay::CountOfTheOmer { day: -1 },
    ] {
        assert_eq!(
            calendar::special_day_name(special_day),
            Err(CalendarError::InvalidSpecialDay),
            "{special_day:?}"
        );
    }
    assert_eq!(
        calendar::special_day_name(SpecialDay::Chanukah { day: 8 }).as_deref(),
        Ok("Chanukah")
    );

    for unit in [
        TehillimUnit::Psalms { start: 0, end: 9 },
        TehillimUnit::Psalms { start: 1, end: 151 },
        TehillimUnit::PsalmVerses {
            psalm: 119,
            start_verse: 1,
            end_verse: 177,
        },
    ] {
        assert_eq!(
            limudim::tehillim_display(unit),
            Err(LimudimError::InvalidUnit),
            "{unit:?}"
        );
    }
    assert_eq!(
        limudim::tehillim_display(TehillimUnit::Psalms { start: 140, end: 150 }).as_deref(),
        Ok("Tehillim 140-150")
    );
    for unit in [
        PirkeiAvosUnit::Single { chapter: 0 },
        PirkeiAvosUnit::Combined { first: 6, second: 7 },
    ] {
        assert_eq!(
            limudim::pirkei_avos_display(unit),
            Err(LimudimError::InvalidUnit),
            "{unit:?}"
        );
    }

    for year in [i32::MIN, -1, 0, 1, 3760, 10_000, i32::MAX] {
        assert_eq!(calendar::year_info(year), Err(CalendarError::InvalidDate), "{year}");
        assert_eq!(calendar::days_in_month(year, HebrewMonth::Nisan), None, "{year}");
        assert!(calendar::rosh_hashana(year).is_err(), "{year}");
        assert!(HebrewDate::new(year, HebrewMonth::Nisan, 1).is_err(), "{year}");
    }
    for (year, month, day) in [(-3761, 1, 1), (6239, 12, 31), (9999, 12, 31)] {
        assert!(
            HebrewDate::from_gregorian(GregorianDate { year, month, day }).is_err(),
            "{year}-{month}-{day}"
        );
    }
    let last = HebrewDate::new(9999, HebrewMonth::Elul, 29).unwrap();
    assert!(last.add_days(1).is_err());
    let first = HebrewDate::new(3761, HebrewMonth::Teves, 18).unwrap();
    assert!(first.add_days(-1).is_err());
    assert!(HebrewDate::new(3761, HebrewMonth::Teves, 17).is_err());
    assert!(HebrewDate::new(1, HebrewMonth::Tishrei, 1).is_err());
    assert_eq!(
        first.to_gregorian().unwrap(),
        GregorianDate {
            year: 1,
            month: 1,
            day: 1
        }
    );
    assert_eq!(
        HebrewDate::from_gregorian(last.to_gregorian().unwrap()).unwrap().year(),
        9999
    );
}

#[test]
fn every_variant_converts() {
    // kosher-rust's enums are non-exhaustive, so a variant the bindings lack would be skipped or
    // replaced. These fail first.
    for zman in kosher_rust::Zman::ALL {
        assert!(kosherz::api::times::Zman::try_from(zman).is_ok(), "{zman:?}");
        assert!(
            kosherz::api::times::Category::try_from(zman.category()).is_ok(),
            "{:?}",
            zman.category()
        );
    }
    for zman in kosher_rust::myzmanim::MyZman::ALL {
        assert!(
            kosherz::api::times::myzmanim::MyZman::try_from(zman).is_ok(),
            "{zman:?}"
        );
    }
}
