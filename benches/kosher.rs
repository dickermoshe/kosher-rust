//! How long each kind of calculation takes. Run with `cargo bench`.

#![allow(clippy::unwrap_used, missing_docs, deprecated)]

use divan::{Bencher, black_box};
use jiff::{civil::date, tz::TimeZone};
use kosher_rust::Location;
use kosher_rust::calendar::{HebrewDate, HebrewMonth, HebrewYear, Region};
use kosher_rust::limudim::{
    AmudYomiDirshu, DafHashavuaBavli, DafYomiBavli, DafYomiYerushalmi, MishnaYomis, PirkeiAvos, Schedule,
    TehillimMonthly,
};
use kosher_rust::myzmanim::{MyZman, MyZmanim};
use kosher_rust::zmanim::{AstronomicalCalculator, Config, Zman, Zmanim};

fn main() {
    divan::main();
}

fn pesach() -> HebrewDate {
    HebrewDate::new(5784, HebrewMonth::Nisan, 15).unwrap()
}

fn jerusalem() -> Location {
    Location::new(31.778, 35.235, Some(TimeZone::get("Asia/Jerusalem").unwrap()))
        .unwrap()
        .with_elevation(754.0)
        .unwrap()
        .with_horizons(754.0, 0.0)
        .unwrap()
}

mod calendar {
    use super::*;

    #[divan::bench]
    fn convert_from_gregorian() -> HebrewDate {
        HebrewDate::from(black_box(date(2024, 4, 23)))
    }

    #[divan::bench]
    fn holidays() -> usize {
        black_box(pesach()).holidays(Region::Diaspora).count()
    }

    #[divan::bench]
    fn parsha() {
        black_box(HebrewDate::from(date(2024, 5, 4)).parsha(Region::Diaspora));
    }

    #[divan::bench]
    fn next_parsha() {
        black_box(pesach().next_parsha(Region::Diaspora));
    }

    #[divan::bench]
    fn year_length() -> u16 {
        black_box(HebrewYear::new(5784)).days()
    }

    #[divan::bench]
    fn molad() {
        black_box(pesach().molad());
    }
}

mod limudim {
    use super::*;

    #[divan::bench]
    fn daf_yomi_bavli() {
        black_box(DafYomiBavli.on(black_box(pesach())));
    }

    #[divan::bench]
    fn daf_yomi_yerushalmi() {
        black_box(DafYomiYerushalmi.on(black_box(pesach())));
    }

    #[divan::bench]
    fn daf_hashavua_bavli() {
        black_box(DafHashavuaBavli.on(black_box(pesach())));
    }

    #[divan::bench]
    fn amud_yomi_dirshu() {
        black_box(AmudYomiDirshu.on(black_box(pesach())));
    }

    #[divan::bench]
    fn mishna_yomis() {
        black_box(MishnaYomis.on(black_box(pesach())));
    }

    #[divan::bench]
    fn pirkei_avos() {
        black_box(PirkeiAvos(Region::Diaspora).on(black_box(pesach())));
    }

    #[divan::bench]
    fn tehillim_monthly() {
        black_box(TehillimMonthly.on(black_box(pesach())));
    }
}

mod zmanim {
    use super::*;

    const ALGORITHMS: [AstronomicalCalculator; 3] = [
        AstronomicalCalculator::Noaa,
        AstronomicalCalculator::Meeus { apply_delta_t: true },
        AstronomicalCalculator::Spa { apply_delta_t: true },
    ];

    fn zmanim(algorithm: AstronomicalCalculator) -> Zmanim {
        let config = Config {
            astronomical_calculator: algorithm,
            ..Config::default()
        };
        Zmanim::with_config(jerusalem(), date(2024, 4, 23), config)
    }

    #[divan::bench(args = ALGORITHMS)]
    fn sunrise(bencher: Bencher, algorithm: AstronomicalCalculator) {
        let zmanim = zmanim(algorithm);
        bencher.bench(|| black_box(&zmanim).get(Zman::Sunrise));
    }

    #[divan::bench(args = ALGORITHMS)]
    fn sof_zman_shma_gra(bencher: Bencher, algorithm: AstronomicalCalculator) {
        let zmanim = zmanim(algorithm);
        bencher.bench(|| black_box(&zmanim).get(Zman::SofZmanShmaGra));
    }

    #[divan::bench(args = ALGORITHMS)]
    fn every_zman(bencher: Bencher, algorithm: AstronomicalCalculator) {
        let zmanim = zmanim(algorithm);
        bencher.bench(|| Zman::ALL.map(|zman| black_box(&zmanim).get(zman)));
    }
}

mod myzmanim {
    use super::*;

    #[divan::bench]
    fn every_row(bencher: Bencher) {
        let times = MyZmanim::new(jerusalem(), date(2024, 4, 23));
        bencher.bench(|| MyZman::ALL.map(|zman| black_box(&times).get(zman)));
    }
}
