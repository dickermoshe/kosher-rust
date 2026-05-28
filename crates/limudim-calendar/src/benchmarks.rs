//! Release-mode performance guardrails for every limud calculator.
//!
//! ```bash
//! cargo test -p limudim-calendar benchmark_all_limud_calculators --release -- --nocapture
//! ```

extern crate std;

use std::{time::Instant, vec::Vec};

use icu_calendar::{cal::Hebrew, Date, Gregorian};
use rand::{rngs::StdRng, RngExt, SeedableRng};

use crate::{
    AmudYomiBavliDirshu, DafHashavuaBavli, DafYomiBavli, DafYomiYerushalmiVilna, LimudCalendar,
    MishnaYomis, PirkeiAvos, TehillimMonthly,
};

const DEFAULT_ITERATIONS: u64 = 250;
const DEFAULT_SEED: u64 = 42;
const MAX_MS_PER_CALL: f64 = 1.0;

type HebrewDate = Date<Hebrew>;

struct BenchmarkCase {
    name: &'static str,
    iterations: u64,
}

struct BenchmarkResult {
    name: &'static str,
    iterations: u64,
    null_results: u64,
    checksum: u64,
    total_ms: f64,
    per_call_ms: f64,
}

fn benchmark_iterations() -> u64 {
    match option_env!("LIMUDIM_BENCHMARK_ITERATIONS") {
        Some(value) => value.parse().expect("invalid LIMUDIM_BENCHMARK_ITERATIONS"),
        None => DEFAULT_ITERATIONS,
    }
}

fn benchmark_seed() -> u64 {
    match option_env!("LIMUDIM_BENCHMARK_SEED") {
        Some(value) => value.parse().expect("invalid LIMUDIM_BENCHMARK_SEED"),
        None => DEFAULT_SEED,
    }
}

fn gregorian_to_hebrew(year: i32, month: u8, day: u8) -> HebrewDate {
    Date::<Gregorian>::try_new_gregorian(year, month, day)
        .expect("valid gregorian date")
        .to_calendar(Hebrew)
}

fn random_hebrew_dates(rng: &mut StdRng, count: u64, min_year: i32, max_year: i32) -> Vec<HebrewDate> {
    (0..count)
        .map(|_| {
            let year = rng.random_range(min_year..=max_year);
            let month = rng.random_range(1..=12);
            let day = rng.random_range(
                1..=Date::<Gregorian>::try_new_gregorian(year, month, 1)
                    .expect("valid month")
                    .days_in_month(),
            );
            gregorian_to_hebrew(year, month, day)
        })
        .collect()
}

fn run_benchmark<T, F>(case: BenchmarkCase, dates: &[HebrewDate], mut calculate: F) -> BenchmarkResult
where
    F: FnMut(HebrewDate) -> Option<T>,
{
    let mut checksum = 0u64;
    let mut null_results = 0u64;

    let start = Instant::now();
    for &date in dates {
        match calculate(date) {
            None => null_results += 1,
            Some(_) => checksum = checksum.wrapping_add(1),
        }
    }
    let elapsed = start.elapsed();

    let total_ms = elapsed.as_secs_f64() * 1000.0;
    let per_call_ms = total_ms / case.iterations as f64;

    BenchmarkResult {
        name: case.name,
        iterations: case.iterations,
        null_results,
        checksum,
        total_ms,
        per_call_ms,
    }
}

fn report_and_assert(result: &BenchmarkResult) {
    std::eprintln!(
        "{}: iterations={} nullResults={} checksum={} total={:.2} ms per_call={:.3} ms",
        result.name,
        result.iterations,
        result.null_results,
        result.checksum,
        result.total_ms,
        result.per_call_ms
    );

    if cfg!(debug_assertions) {
        std::eprintln!(
            "  (debug build: skipping {MAX_MS_PER_CALL} ms/call assertion; run with --release)"
        );
        return;
    }

    assert!(
        result.per_call_ms <= MAX_MS_PER_CALL,
        "{} averaged {:.3} ms/call (limit {MAX_MS_PER_CALL} ms/call). \
         Run `cargo test -p limudim-calendar benchmark_all_limud_calculators --release -- --nocapture`",
        result.name,
        result.per_call_ms
    );
}

#[test]
fn benchmark_all_limud_calculators() {
    let iterations = benchmark_iterations();
    let seed = benchmark_seed();
    let mut rng = StdRng::seed_from_u64(seed);

    let cases = [
        run_benchmark(
            BenchmarkCase {
                name: "DafYomiBavli",
                iterations,
            },
            &random_hebrew_dates(&mut rng, iterations, 1924, 2100),
            |date| date.limud(DafYomiBavli::default()),
        ),
        run_benchmark(
            BenchmarkCase {
                name: "DafYomiYerushalmiVilna",
                iterations,
            },
            &random_hebrew_dates(&mut rng, iterations, 1980, 2100),
            |date| date.limud(DafYomiYerushalmiVilna::default()),
        ),
        run_benchmark(
            BenchmarkCase {
                name: "AmudYomiBavliDirshu",
                iterations,
            },
            &random_hebrew_dates(&mut rng, iterations, 2024, 2100),
            |date| date.limud(AmudYomiBavliDirshu::default()),
        ),
        run_benchmark(
            BenchmarkCase {
                name: "DafHashavuaBavli",
                iterations,
            },
            &random_hebrew_dates(&mut rng, iterations, 2006, 2100),
            |date| date.limud(DafHashavuaBavli::default()),
        ),
        run_benchmark(
            BenchmarkCase {
                name: "MishnaYomis",
                iterations,
            },
            &random_hebrew_dates(&mut rng, iterations, 1950, 2100),
            |date| date.limud(MishnaYomis),
        ),
        run_benchmark(
            BenchmarkCase {
                name: "PirkeiAvosDiaspora",
                iterations,
            },
            &random_hebrew_dates(&mut rng, iterations, 1980, 2100),
            |date| date.limud(PirkeiAvos::new(false)),
        ),
        run_benchmark(
            BenchmarkCase {
                name: "PirkeiAvosIsrael",
                iterations,
            },
            &random_hebrew_dates(&mut rng, iterations, 1980, 2100),
            |date| date.limud(PirkeiAvos::new(true)),
        ),
        run_benchmark(
            BenchmarkCase {
                name: "TehillimMonthly",
                iterations,
            },
            &random_hebrew_dates(&mut rng, iterations, 1980, 2100),
            |date| date.limud(TehillimMonthly),
        ),
    ];

    std::eprintln!("limudim-calendar benchmarks: seed={seed} iterations={iterations}");
    for result in &cases {
        report_and_assert(result);
    }
}
