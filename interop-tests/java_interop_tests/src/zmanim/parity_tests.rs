//! One `#[test]` per preset so `cargo test` can run zmanim Java parity in parallel.
//!
//! Expands via [`kosher_rust::for_all_zman_presets`] instead of looping over
//! [`kosher_rust::zmanim::presets::ALL_ZMANIM`] inside a few large tests.

macro_rules! java_parity_tests_for_preset {
    ($preset:ident) => {
        #[allow(non_snake_case, deprecated)]
        mod $preset {
            use kosher_rust::zmanim::presets::$preset;

            use crate::zmanim::{test_preset, test_preset_in_jerusalem, test_regressions};

            #[test]
            fn jerusalem() -> Result<(), Box<dyn std::error::Error>> {
                test_preset_in_jerusalem(&$preset)
            }

            #[test]
            fn regressions() {
                test_regressions(&$preset);
            }

            #[test]
            fn random() -> Result<(), Box<dyn std::error::Error>> {
                test_preset(&$preset)
            }
        }
    };
}

kosher_rust::for_all_zman_presets!(java_parity_tests_for_preset);
