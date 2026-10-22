//! The bindings give the same answers as kosher-rust.
//!
//! The zmanim tests run the bindings with a callback time zone and kosher-rust with jiff's, so
//! they also check that a `TimeZoneOffset` callback is as good as a real time zone.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, missing_docs)]

use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

use jiff::{civil, tz::TimeZone};
use kosher_rust_bindings::api::kosher::{
    GregorianDate, KosherError, Location,
    calendar::{self, HebrewDate, Region},
    limudim,
    myzmanim::{self, MyZmanim},
    sea_level_location, validate_location,
    zmanim::{self, Calculator, TimeZoneOffset, ZmanFailure, Zmanim},
};

/// A jiff time zone, seen only through the callback.
struct Jiff(TimeZone);

impl TimeZoneOffset for Jiff {
    fn offset_seconds(&self, unix_seconds: i64) -> Result<i32, weaveffi::ForeignError> {
        let timestamp = jiff::Timestamp::from_second(unix_seconds).unwrap();
        Ok(self.0.to_offset(timestamp).seconds())
    }
}

/// A callback that always fails, counting its calls.
#[derive(Default)]
struct Broken(AtomicUsize);

impl TimeZoneOffset for Broken {
    fn offset_seconds(&self, _unix_seconds: i64) -> Result<i32, weaveffi::ForeignError> {
        self.0.fetch_add(1, Ordering::Relaxed);
        Err(weaveffi::ForeignError::new(1, "no offset"))
    }
}

/// A callback giving an offset no time zone has.
struct TooFar;

impl TimeZoneOffset for TooFar {
    fn offset_seconds(&self, _unix_seconds: i64) -> Result<i32, weaveffi::ForeignError> {
        Ok(90_000)
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
    kosher_rust::Location::new(
        place.latitude,
        place.longitude,
        Some(TimeZone::get(place.zone).unwrap()),
    )
    .unwrap()
    .with_elevation(place.elevation)
    .unwrap()
    .with_horizons(place.elevation, 0.0)
    .unwrap()
}

fn time_zone(place: &Place) -> Option<Arc<dyn TimeZoneOffset>> {
    Some(Arc::new(Jiff(TimeZone::get(place.zone).unwrap())))
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
                E::Internal => ZmanFailure::Internal,
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
                let config = zmanim::Config {
                    calculator,
                    ..zmanim::default_config()
                };
                let ours = Zmanim::new(location(place), date.into_gregorian(), config, time_zone(place)).unwrap();
                let theirs = kosher_rust::Zmanim::with_config(
                    kosher_location(place),
                    date,
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
            let ours = MyZmanim::new(location(place), date.into_gregorian(), 40.0, time_zone(place)).unwrap();
            let theirs = kosher_rust::myzmanim::MyZmanim::new(kosher_location(place), date).candle_lighting(40.0);
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
    let all = zmanim::all_zmanim();
    assert_eq!(all.len(), kosher_rust::Zman::ALL.len());
    for (position, zman) in all.into_iter().enumerate() {
        assert_eq!(zman as i32, position as i32);
        let theirs = kosher_rust::Zman::from(zman);
        assert_eq!(theirs, kosher_rust::Zman::ALL[position]);
        assert_eq!(zmanim::zman_java_name(zman), theirs.java_name());
        assert_eq!(zmanim::zman_from_java_name(theirs.java_name()), Some(zman));
        assert_eq!(zmanim::zman_name(zman), theirs.name());
        assert_eq!(zmanim::zman_is_deprecated(zman), theirs.is_deprecated());
        assert_eq!(
            kosher_rust::zmanim::Category::from(zmanim::zman_category(zman)),
            theirs.category()
        );
        assert_eq!(
            zmanim::zman_description(zman, zmanim::default_config()),
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
        kosher_rust::zmanim::Config::from(zmanim::default_config()),
        kosher_rust::zmanim::Config::default()
    );
    let custom = zmanim::Config {
        candle_lighting_minutes: 40,
        use_astronomical_chatzos_for_other_zmanim: true,
        use_elevation: true,
        ateret_torah_sunset_minutes: 25,
        use_astronomical_chatzos: false,
        calculator: Calculator::Spa,
        apply_delta_t: false,
    };
    assert_eq!(zmanim::Config::from(kosher_rust::zmanim::Config::from(custom)), custom);
    assert_eq!(
        zmanim::calculator_name(Calculator::Meeus),
        "Jean Meeus Higher-Accuracy (VSOP87) Algorithm"
    );
}

#[test]
fn errors() {
    let lakewood = sea_level_location(40.09, -74.22);
    let date = GregorianDate {
        year: 2024,
        month: 11,
        day: 4,
    };
    let config = zmanim::default_config();
    let levana = zmanim::Zman::TchilasZmanKidushLevana3Days;

    let utc = || Some(Arc::new(Jiff(TimeZone::UTC)) as Arc<dyn TimeZoneOffset>);
    let bad = |location: Location| Zmanim::new(location, date, config, utc()).err();
    assert_eq!(bad(sea_level_location(91.0, 0.0)), Some(KosherError::InvalidLatitude));
    assert_eq!(bad(sea_level_location(0.0, 181.0)), Some(KosherError::InvalidLongitude));
    assert_eq!(
        bad(Location {
            elevation: -1.0,
            ..lakewood
        }),
        Some(KosherError::InvalidElevation)
    );
    assert_eq!(
        Zmanim::new(sea_level_location(0.0, 179.0), date, config, None).err(),
        Some(KosherError::TimeZoneRequired)
    );
    assert_eq!(validate_location(sea_level_location(0.0, 179.0), true), Ok(()));
    assert_eq!(
        Zmanim::new(lakewood, GregorianDate { day: 31, ..date }, config, None).err(),
        Some(KosherError::InvalidDate)
    );

    let no_zone = Zmanim::new(lakewood, date, config, None).unwrap();
    assert_eq!(no_zone.get(levana), Err(KosherError::TimeZoneRequired));
    assert!(no_zone.get(zmanim::Zman::Sunrise).is_ok());

    let broken = Arc::new(Broken::default());
    let failing = Zmanim::new(lakewood, date, config, Some(broken.clone())).unwrap();
    assert_eq!(failing.get(levana), Err(KosherError::TimeZoneCallbackFailed));
    assert!(broken.0.load(Ordering::Relaxed) > 0);
    // Every time asks the time zone for the date-line check, so every time reports the failure.
    assert_eq!(
        failing.get(zmanim::Zman::Sunrise),
        Err(KosherError::TimeZoneCallbackFailed)
    );

    let too_far = Zmanim::new(lakewood, date, config, Some(Arc::new(TooFar))).unwrap();
    assert_eq!(too_far.get(levana), Err(KosherError::TimeZoneCallbackFailed));
    let failures: Vec<_> = too_far.all().into_iter().filter_map(|row| row.failure).collect();
    // Only times that turn out not to apply without looking at the sun skip the callback.
    assert!(
        failures.iter().all(|&failure| matches!(
            failure,
            ZmanFailure::TimeZoneCallbackFailed | ZmanFailure::NotApplicable
        )),
        "{failures:?}"
    );
    assert!(failures.contains(&ZmanFailure::TimeZoneCallbackFailed));
}

/// Kiddush Levana on the day after the clocks went back, through a callback: the day check
/// needs the offset at the molad, not at the start of the day.
#[test]
fn kiddush_levana_after_a_clock_change() {
    let lakewood = sea_level_location(40.09, -74.22);
    let date = GregorianDate {
        year: 2024,
        month: 11,
        day: 4,
    };
    let zone = Some(Arc::new(Jiff(TimeZone::get("America/New_York").unwrap())) as Arc<dyn TimeZoneOffset>);
    let zmanim = Zmanim::new(lakewood, date, zmanim::default_config(), zone).unwrap();
    let time = zmanim.get(zmanim::Zman::TchilasZmanKidushLevana3Days).unwrap();
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
        let theirs = kosher_rust::HebrewDate::from(date);
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
            let holidays: Vec<kosher_rust::calendar::Holiday> =
                ours.holidays(region).into_iter().map(Into::into).collect();
            assert_eq!(holidays, theirs.holidays(kosher_region).collect::<Vec<_>>(), "{date}");
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
                limudim::pirkei_avos(&ours, region).map(limudim::pirkei_avos_display),
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
            limudim::tehillim_monthly(&ours).map(limudim::tehillim_display),
            k::TehillimMonthly.on(date).map(|t| t.to_string())
        );
        if let Some(unit) = limudim::tehillim_monthly(&ours) {
            assert_eq!(k::TehillimUnit::from(unit), k::TehillimMonthly.on(date).unwrap());
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
        let info = calendar::year_info(year);
        let theirs = kosher_rust::calendar::HebrewYear::new(year);
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
    let chanukah = calendar::Holiday::Chanukah { day: 3 };
    assert_eq!(
        kosher_rust::calendar::Holiday::from(chanukah),
        kosher_rust::calendar::Holiday::Chanukah(3)
    );
    assert!(calendar::holiday_is_fast_day(calendar::Holiday::YomKippur));
    assert!(calendar::holiday_is_modern(calendar::Holiday::YomHaatzmaut));
}
