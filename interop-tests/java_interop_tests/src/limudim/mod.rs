//! Java parity tests for limudim schedules.

use std::{env, error::Error, sync::OnceLock};

use icu_calendar::{Date as IcuDate, Gregorian, cal::Hebrew};
use jiff::civil::Date;
use jni::{jni_sig, jni_str, objects::JString};
use rand::{RngExt, SeedableRng, rngs::StdRng};

use crate::calendar::java_reference::new_local_date;
use crate::{init_bindings, java_vm};

use crate::java_bindings::com::kosherjava::zmanim::hebrewcalendar::{
    Daf as JavaDaf, JewishCalendar, YerushalmiYomiCalculator, YomiCalculator,
};
use kosher_rust::limudim::*;

/// Which of KosherJava's transliterations to read from a `Daf`.
#[derive(Clone, Copy)]
enum Talmud {
    Bavli,
    Yerushalmi,
}

const DEFAULT_LIMUDIM_PARITY_ITERATIONS: u64 = 250;
const MAX_REJECTION_ATTEMPTS: u32 = 10_000;
const JAVA_BAVLI_BEFORE_START_MESSAGE: &str = "is prior to organized Daf Yomi Bavli cycles";
const JAVA_YERUSHALMI_BEFORE_START_MESSAGE: &str = "is prior to the first Daf Yomi cycle";

fn test_iterations() -> u64 {
    match env::var("ZMANIM_LIMUDIM_JAVA_PARITY_ITERATIONS") {
        Ok(value) => value
            .parse::<u64>()
            .unwrap_or_else(|_| panic!("invalid ZMANIM_LIMUDIM_JAVA_PARITY_ITERATIONS value: {value}")),
        Err(env::VarError::NotPresent) => DEFAULT_LIMUDIM_PARITY_ITERATIONS,
        Err(err) => panic!("failed to read ZMANIM_LIMUDIM_JAVA_PARITY_ITERATIONS: {err}"),
    }
}

fn test_seed() -> u64 {
    static SEED: OnceLock<u64> = OnceLock::new();

    *SEED.get_or_init(|| match env::var("ZMANIM_JAVA_PARITY_SEED") {
        Ok(value) => value
            .parse::<u64>()
            .unwrap_or_else(|_| panic!("invalid ZMANIM_JAVA_PARITY_SEED value: {value}")),
        Err(env::VarError::NotPresent) => rand::random::<u64>(),
        Err(err) => panic!("failed to read ZMANIM_JAVA_PARITY_SEED: {err}"),
    })
}

fn date(year: i16, month: i8, day: i8) -> Date {
    Date::new(year, month, day).expect("valid Gregorian date")
}

fn random_supported_date(rng: &mut StdRng, min_date: Date, max_year: i16) -> Date {
    for attempt in 0..MAX_REJECTION_ATTEMPTS {
        let year = rng.random_range(min_date.year()..=max_year);
        let month = rng.random_range(1..=12);
        let day = rng.random_range(1..=Date::new(year, month, 1).expect("valid random month").days_in_month() as i8);
        let candidate = date(year, month, day);
        if candidate >= min_date {
            return candidate;
        }
        let _ = attempt;
    }
    panic!(
        "failed to find supported date after {MAX_REJECTION_ATTEMPTS} attempts \
         (min_date={min_date:?}, max_year={max_year}, seed={})",
        test_seed()
    );
}

#[test]
fn test_daf_yomi_bavli_against_kosher_java() -> Result<(), Box<dyn Error>> {
    let before_start = date(1923, 9, 10);
    let rust = DafYomiBavli.on(hebrew_date(before_start));
    assert!(
        rust.is_none(),
        "Daf Yomi Bavli should not exist before the first cycle for {before_start:?}: rust={rust:?}"
    );
    let java = java_bavli_daf(before_start)?;
    assert_eq!(
        rust, java,
        "Daf Yomi Bavli should not exist before the first cycle for {before_start:?}: rust={rust:?} java={java:?}"
    );

    for date in [
        date(1923, 9, 11),
        date(1969, 4, 28),
        date(1969, 4, 29),
        date(1975, 6, 24),
        date(2012, 8, 3),
        date(2017, 12, 28),
        date(2019, 10, 10),
        date(2020, 1, 4),
    ] {
        assert_bavli_matches(date, None)?;
    }

    for year in 1924..=2100 {
        for &(month, day) in &[(1, 1), (4, 15), (7, 1), (10, 1)] {
            assert_bavli_matches(date(year, month, day), None)?;
        }
    }

    Ok(())
}

#[test]
fn test_random_daf_yomi_bavli_against_kosher_java() -> Result<(), Box<dyn Error>> {
    let seed = test_seed();
    let iterations = test_iterations();
    let mut rng = StdRng::seed_from_u64(seed);

    for iteration in 0..iterations {
        let date = random_supported_date(&mut rng, date(1920, 9, 11), 2100);
        assert_bavli_matches(date, Some((seed, iteration)))?;
    }

    Ok(())
}

#[test]
fn test_daf_yomi_yerushalmi_against_kosher_java() -> Result<(), Box<dyn Error>> {
    let before_start = date(1980, 2, 1);
    let rust = DafYomiYerushalmi.on(hebrew_date(before_start));
    assert!(
        rust.is_none(),
        "Daf Yomi Yerushalmi should not exist before the first cycle for {before_start:?}: rust={rust:?}"
    );
    let java = java_yerushalmi_daf(before_start)?;
    assert_eq!(
        rust, java,
        "Daf Yomi Yerushalmi should not exist before the first cycle for {before_start:?}: rust={rust:?} java={java:?}"
    );

    for date in [
        date(1980, 2, 2),
        date(1980, 9, 20),
        date(1984, 5, 12),
        date(1984, 5, 13),
        date(2005, 10, 2),
        date(2005, 10, 3),
        date(2017, 12, 28),
        date(2025, 10, 2),
        // A cycle extended for Tisha B'Av 5932, ending with Niddah 13 on 7 August 2172.
        date(2172, 8, 6),
        date(2172, 8, 7),
        date(2172, 8, 8),
        // A cycle with Tisha B'Av 6240 the day before its last daf, ending on 16 August 2480.
        date(2480, 8, 14),
        date(2480, 8, 15),
        date(2480, 8, 16),
        date(2480, 8, 17),
    ] {
        assert_yerushalmi_matches(date, None)?;
    }

    for year in 1981..=2100 {
        for &(month, day) in &[(1, 1), (4, 15), (7, 1), (10, 1)] {
            assert_yerushalmi_matches(date(year, month, day), None)?;
        }
    }

    Ok(())
}

#[test]
fn test_random_daf_yomi_yerushalmi_against_kosher_java() -> Result<(), Box<dyn Error>> {
    let seed = test_seed();
    let iterations = test_iterations();
    let mut rng = StdRng::seed_from_u64(seed);

    for iteration in 0..iterations {
        let date = random_supported_date(&mut rng, date(1960, 2, 2), 2500);
        assert_yerushalmi_matches(date, Some((seed, iteration)))?;
    }

    Ok(())
}

fn assert_bavli_matches(date: Date, random_case: Option<(u64, u64)>) -> Result<(), Box<dyn Error>> {
    let rust = DafYomiBavli.on(hebrew_date(date));
    let context = random_case
        .map(|(seed, iteration)| format!(" seed={seed} iteration={iteration}"))
        .unwrap_or_default();

    let java = java_bavli_daf(date)?;
    assert_eq!(
        rust, java,
        "Daf Yomi Bavli mismatch for {date:?}{context}: rust={rust:?} java={java:?}"
    );
    Ok(())
}

fn assert_yerushalmi_matches(date: Date, random_case: Option<(u64, u64)>) -> Result<(), Box<dyn Error>> {
    let rust = DafYomiYerushalmi.on(hebrew_date(date));
    let context = random_case
        .map(|(seed, iteration)| format!(" seed={seed} iteration={iteration}"))
        .unwrap_or_default();

    let java = java_yerushalmi_daf(date)?;
    assert_eq!(
        rust, java,
        "Daf Yomi Yerushalmi mismatch for {date:?}{context}: rust={rust:?} java={java:?}"
    );
    Ok(())
}

fn hebrew_date(date: Date) -> IcuDate<Hebrew> {
    IcuDate::<Gregorian>::try_new_gregorian(i32::from(date.year()), date.month() as u8, date.day() as u8)
        .expect("valid Gregorian date")
        .to_calendar(Hebrew)
}

fn java_bavli_daf(date: Date) -> Result<Option<Daf<BavliTractate>>, Box<dyn Error>> {
    java_vm().attach_current_thread(|env| {
        let calendar = java_jewish_calendar(env, date)?;
        let daf = match YomiCalculator::get_daf_yomi_bavli(env, &calendar) {
            Ok(daf) => daf,
            Err(err) => {
                let err = caught_java_exception(env).unwrap_or(err);
                if is_before_first_cycle_exception(&err, JAVA_BAVLI_BEFORE_START_MESSAGE) {
                    return Ok(None);
                }
                return Err(err.into());
            }
        };
        if daf.is_null() {
            return Ok(None);
        }

        let (tractate, page) = java_daf_to_rust(env, &daf, Talmud::Bavli)?;
        let tractate = BavliTractate::from_name(tractate.name()).ok_or("not a Bavli tractate")?;
        Ok(Some(Daf { tractate, daf: page }))
    })
}

fn java_yerushalmi_daf(date: Date) -> Result<Option<Daf<YerushalmiTractate>>, Box<dyn Error>> {
    java_vm().attach_current_thread(|env| {
        let calendar = java_jewish_calendar(env, date)?;
        let daf = match YerushalmiYomiCalculator::get_daf_yomi_yerushalmi(env, &calendar) {
            Ok(daf) => daf,
            Err(err) => {
                let err = caught_java_exception(env).unwrap_or(err);
                if is_before_first_cycle_exception(&err, JAVA_YERUSHALMI_BEFORE_START_MESSAGE) {
                    return Ok(None);
                }
                return Err(err.into());
            }
        };
        if daf.is_null() {
            return Ok(None);
        }

        let (tractate, page) = java_daf_to_rust(env, &daf, Talmud::Yerushalmi)?;
        let tractate = YerushalmiTractate::from_name(tractate.name()).ok_or("not a Yerushalmi tractate")?;
        Ok(Some(Daf { tractate, daf: page }))
    })
}

fn caught_java_exception(env: &mut jni::Env<'_>) -> Option<jni::errors::Error> {
    if !env.exception_check() {
        return None;
    }

    match env.exception_catch() {
        Err(err) => Some(err),
        Ok(()) => None,
    }
}

fn is_before_first_cycle_exception(err: &jni::errors::Error, expected_message: &str) -> bool {
    matches!(
        err,
        jni::errors::Error::CaughtJavaException { name, msg, .. }
            if name == "java.lang.IllegalArgumentException" && msg.contains(expected_message)
    )
}

fn java_jewish_calendar<'local>(env: &mut jni::Env<'local>, date: Date) -> jni::errors::Result<JewishCalendar<'local>> {
    let local_date = new_local_date(
        env,
        i32::from(date.year()),
        i32::from(date.month() as u8),
        i32::from(date.day()),
    )?;
    JewishCalendar::new_local_date(env, &local_date)
}

/// The tractate and page of a KosherJava `Daf`.
fn java_daf_to_rust(
    env: &mut jni::Env<'_>,
    daf: &JavaDaf<'_>,
    talmud: Talmud,
) -> Result<(MishnaTractate, u16), Box<dyn Error>> {
    let page = env.call_method(daf, jni_str!("getDaf"), jni_sig!("()I"), &[])?.i()? as u16;
    let name = match talmud {
        Talmud::Bavli => env.call_method(
            daf,
            jni_str!("getMasechtaTransliterated"),
            jni_sig!("()Ljava/lang/String;"),
            &[],
        )?,
        Talmud::Yerushalmi => env.call_method(
            daf,
            jni_str!("getYerushalmiMasechtaTransliterated"),
            jni_sig!("()Ljava/lang/String;"),
            &[],
        )?,
    }
    .l()?;
    let name = env.cast_local::<JString>(name)?;
    let tractate = tractate_from_java_name(&name.to_string())?;

    Ok((tractate, page))
}

fn tractate_from_java_name(name: &str) -> Result<MishnaTractate, Box<dyn Error>> {
    match name {
        "Berachos" => Ok(MishnaTractate::Berachos),
        "Pe'ah" => Ok(MishnaTractate::Peah),
        "Demai" => Ok(MishnaTractate::Demai),
        "Kilayim" => Ok(MishnaTractate::Kilayim),
        "Shevi'is" => Ok(MishnaTractate::Sheviis),
        "Terumos" => Ok(MishnaTractate::Terumos),
        "Ma'asros" => Ok(MishnaTractate::Maasros),
        "Ma'aser Sheni" => Ok(MishnaTractate::MaaserSheni),
        "Chalah" => Ok(MishnaTractate::Chalah),
        "Orlah" => Ok(MishnaTractate::Orlah),
        "Bikurim" => Ok(MishnaTractate::Bikurim),
        "Shabbos" => Ok(MishnaTractate::Shabbos),
        "Eruvin" => Ok(MishnaTractate::Eruvin),
        "Pesachim" => Ok(MishnaTractate::Pesachim),
        "Shekalim" => Ok(MishnaTractate::Shekalim),
        "Yoma" => Ok(MishnaTractate::Yoma),
        "Sukkah" | "Sukah" => Ok(MishnaTractate::Sukkah),
        "Beitzah" => Ok(MishnaTractate::Beitzah),
        "Rosh Hashana" | "Rosh Hashanah" => Ok(MishnaTractate::RoshHashanah),
        "Taanis" | "Ta'anis" => Ok(MishnaTractate::Taanis),
        "Megillah" | "Megilah" => Ok(MishnaTractate::Megillah),
        "Moed Katan" => Ok(MishnaTractate::MoedKatan),
        "Chagigah" => Ok(MishnaTractate::Chagigah),
        "Yevamos" => Ok(MishnaTractate::Yevamos),
        "Kesubos" | "Kesuvos" => Ok(MishnaTractate::Kesubos),
        "Nedarim" => Ok(MishnaTractate::Nedarim),
        "Nazir" => Ok(MishnaTractate::Nazir),
        "Sotah" => Ok(MishnaTractate::Sotah),
        "Gitin" => Ok(MishnaTractate::Gitin),
        "Kiddushin" | "Kidushin" => Ok(MishnaTractate::Kiddushin),
        "Bava Kamma" | "Bava Kama" => Ok(MishnaTractate::BavaKamma),
        "Bava Metzia" => Ok(MishnaTractate::BavaMetzia),
        "Bava Basra" => Ok(MishnaTractate::BavaBasra),
        "Sanhedrin" => Ok(MishnaTractate::Sanhedrin),
        "Makkos" | "Makos" => Ok(MishnaTractate::Makkos),
        "Shevuos" => Ok(MishnaTractate::Shevuos),
        "Avodah Zarah" => Ok(MishnaTractate::AvodahZarah),
        "Horiyos" | "Horayos" => Ok(MishnaTractate::Horiyos),
        "Zevachim" => Ok(MishnaTractate::Zevachim),
        "Menachos" => Ok(MishnaTractate::Menachos),
        "Chullin" => Ok(MishnaTractate::Chullin),
        "Bechoros" => Ok(MishnaTractate::Bechoros),
        "Arachin" => Ok(MishnaTractate::Arachin),
        "Temurah" => Ok(MishnaTractate::Temurah),
        "Kerisos" => Ok(MishnaTractate::Kerisos),
        "Meilah" => Ok(MishnaTractate::Meilah),
        "Kinnim" => Ok(MishnaTractate::Kinnim),
        "Tamid" => Ok(MishnaTractate::Tamid),
        "Midos" => Ok(MishnaTractate::Midos),
        "Niddah" | "Nidah" => Ok(MishnaTractate::Niddah),
        _ => Err(format!("unknown KosherJava masechta name: {name}").into()),
    }
}
