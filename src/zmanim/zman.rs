use core::fmt;

#[cfg(feature = "alloc")]
use alloc::{format, string::String};

#[cfg(feature = "alloc")]
use super::Config;

/// What kind of time a [`Zman`] is: dawn, sunrise, the latest time for Shema, and so on.
///
/// Use it to group or filter [`Zman::ALL`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Category {
    /// Start and end of astronomical, civil and nautical twilight.
    Twilight,
    /// Dawn (*alos*).
    Alos,
    /// Earliest time for tallis and tefillin (*misheyakir*).
    Misheyakir,
    /// Sunrise (*netz*).
    Netz,
    /// Latest time for the morning Shema (*sof zman shma*).
    SofZmanShma,
    /// Latest time for the morning prayer (*sof zman tefila*).
    SofZmanTefila,
    /// Latest time to eat chametz on Erev Pesach.
    SofZmanAchilasChametz,
    /// Latest time to burn chametz on Erev Pesach.
    SofZmanBiurChametz,
    /// Midday (*chatzos hayom*).
    ChatzosHayom,
    /// Earliest Mincha (*mincha gedola*).
    MinchaGedola,
    /// *Plag hamincha*.
    PlagHamincha,
    /// Shortly before *mincha ketana* (*samuch le-mincha ketana*).
    SamuchLeMinchaKetana,
    /// Preferred Mincha time (*mincha ketana*).
    MinchaKetana,
    /// Twilight between sunset and nightfall (*bein hashmashos*).
    BeinHashmashos,
    /// Candle lighting before Shabbos or Yom Tov.
    CandleLighting,
    /// Sunset (*shkiya*).
    Shkiya,
    /// Nightfall (*tzais*).
    Tzais,
    /// The window for Kiddush Levana.
    KidushLevana,
    /// Midnight (*chatzos halayla*).
    ChatzosHalayla,
}

/// Declares `enum Zman` and its metadata from the entries in `catalog.rs`.
///
/// Each entry is a variant with its doc comment and optional `#[deprecated]`, followed by its
/// KosherJava getter, display name, category, formula and end-user description.
macro_rules! zman_catalog {
    ($(
        $(#[doc = $doc:literal])*
        $(#[deprecated $($deprecation:tt)*])?
        $variant:ident {
            java: $java:literal,
            name: $name:literal,
            category: $category:ident,
            formula: $formula:expr,
            about: $about:expr $(,)?
        }
    ),* $(,)?) => {
        /// A halachic time this crate can calculate. Pass one to
        /// [`Zmanim::get`](crate::zmanim::Zmanim::get).
        ///
        /// Each variant is named after the KosherJava getter that calculates it, without `get`:
        /// `getSofZmanShmaGRA` is [`Zman::SofZmanShmaGra`]. [`Zman::ALL`] lists every one, and
        /// [`category`](Zman::category) groups them.
        ///
        /// ```
        /// use kosher_rust::zmanim::{Category, Zman};
        ///
        /// let shema: Vec<Zman> = Zman::ALL
        ///     .into_iter()
        ///     .filter(|zman| zman.category() == Category::SofZmanShma)
        ///     .collect();
        /// assert!(shema.contains(&Zman::SofZmanShmaGra));
        /// assert_eq!(Zman::SofZmanShmaGra.java_name(), "getSofZmanShmaGRA");
        /// ```
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum Zman {
            $(
                $(#[doc = $doc])*
                $(#[deprecated $($deprecation)*])?
                $variant,
            )*
        }

        #[allow(deprecated)]
        impl Zman {
            /// Every zman, grouped by [`Category`].
            pub const ALL: [Zman; [$(stringify!($variant)),*].len()] = [$(Zman::$variant),*];

            /// The display name, for example "Sof Zman Shma (GR'A)".
            pub const fn name(self) -> &'static str {
                match self {
                    $(Zman::$variant => $name,)*
                }
            }

            /// The KosherJava method that calculates this time, for example
            /// `"getSofZmanShmaGRA"`.
            pub const fn java_name(self) -> &'static str {
                match self {
                    $(Zman::$variant => $java,)*
                }
            }

            /// What kind of time this is.
            pub const fn category(self) -> Category {
                match self {
                    $(Zman::$variant => Category::$category,)*
                }
            }

            /// Whether KosherJava deprecates this time. Deprecated variants are also marked
            /// `#[deprecated]`.
            pub const fn is_deprecated(self) -> bool {
                match self {
                    $(Zman::$variant => false $(|| { let _ = stringify!($($deprecation)*); true })?,)*
                }
            }

            /// The end-user description, with `{...}` placeholders for config values.
            #[cfg(feature = "alloc")]
            pub(crate) const fn about(self) -> &'static str {
                match self {
                    $(Zman::$variant => $about,)*
                }
            }

            pub(crate) fn formula(self) -> &'static Formula {
                match self {
                    $(Zman::$variant => {
                        static FORMULA: Formula = $formula;
                        &FORMULA
                    })*
                }
            }
        }

        #[cfg(feature = "defmt")]
        impl defmt::Format for Zman {
            fn format(&self, fmt: defmt::Formatter) {
                defmt::write!(fmt, "{=str}", self.name())
            }
        }
    };
}

pub(crate) use zman_catalog;

use super::Zman;
#[cfg(feature = "alloc")]
use jiff::SignedDuration;

impl Zman {
    /// The zman KosherJava calculates with the method `name`, for example
    /// `Zman::from_java_name("getSofZmanShmaGRA")`.
    pub fn from_java_name(name: &str) -> Option<Zman> {
        Zman::ALL.into_iter().find(|zman| zman.java_name() == name)
    }

    /// A paragraph or two explaining this time to your users: what it is, its source, and where
    /// it may be unavailable, with the offsets and options from `config` filled in.
    ///
    /// Needs the `alloc` feature.
    #[cfg(feature = "alloc")]
    pub fn description(self, config: &Config) -> String {
        let mut text = self
            .about()
            .replace("{candle_lighting_offset}", &minutes(config.candle_lighting_offset))
            .replace("{ateret_torah_offset}", &minutes(config.ateret_torah_sunset_offset));

        let formula = self.formula();
        if formula.uses_elevation_from_config() {
            text.push_str(if config.use_elevation {
                "\n\nThis zman takes the configured location's elevation into account when it is calculated."
            } else {
                "\n\nThis zman is calculated as though the configured location were at sea level."
            });
        }
        if formula.uses_astronomical_chatzos_from_config() {
            text.push_str(if config.use_astronomical_chatzos {
                "\n\nThis zman uses astronomical chatzos (solar transit) as the midpoint of the day."
            } else {
                "\n\nThis zman uses the midpoint between sunrise and sunset for chatzos."
            });
        }
        if formula.uses_astronomical_chatzos_for_other_zmanim_from_config() {
            text.push_str(if config.use_astronomical_chatzos_for_other_zmanim {
                "\n\nThis zman divides the afternoon from astronomical chatzos when calculating shaos zmaniyos."
            } else {
                "\n\nThis zman uses a fixed fraction of the sunrise-to-sunset interval for shaos zmaniyos, without dividing the day at astronomical chatzos."
            });
        }
        text
    }
}

/// The display name.
impl fmt::Display for Zman {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

#[cfg(feature = "alloc")]
fn minutes(duration: SignedDuration) -> String {
    format!("{} minutes", duration.as_mins().unsigned_abs())
}
