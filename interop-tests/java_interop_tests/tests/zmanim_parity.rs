//! Java parity for every zman: a Jerusalem case per solar algorithm, the recorded regressions,
//! and randomized cases. Each is its own test, named like `getSunrise::random`, so they run in
//! parallel and can be filtered: `cargo test --test zmanim_parity -- getSunrise`.

use java_interop_tests::zmanim::{test_preset, test_preset_in_jerusalem, test_regressions};
use kosher_rust::Zman;
use libtest_mimic::{Arguments, Failed, Trial};

fn main() {
    let args = Arguments::from_args();
    let mut trials = Vec::new();
    for zman in Zman::ALL {
        let name = zman.java_name();
        trials.push(Trial::test(format!("{name}::jerusalem"), move || {
            test_preset_in_jerusalem(zman).map_err(|error| Failed::from(error.to_string()))
        }));
        trials.push(Trial::test(format!("{name}::regressions"), move || {
            test_regressions(zman);
            Ok(())
        }));
        trials.push(Trial::test(format!("{name}::random"), move || {
            test_preset(zman).map_err(|error| Failed::from(error.to_string()))
        }));
    }
    libtest_mimic::run(&args, trials).exit();
}
