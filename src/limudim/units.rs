use core::fmt;

/// A tractate of the Mishnah. Tractates sort in the order of the Mishnah.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(
    feature = "wasm",
    derive(serde::Serialize, serde::Deserialize, tsify::Tsify),
    serde(rename_all = "camelCase")
)]
pub enum MishnaTractate {
    /// Berachos (ברכות).
    Berachos,
    /// Peah (פאה).
    Peah,
    /// Demai (דמאי).
    Demai,
    /// Kilayim (כלאים).
    Kilayim,
    /// Sheviis (שביעית).
    Sheviis,
    /// Terumos (תרומות).
    Terumos,
    /// Maasros (מעשרות).
    Maasros,
    /// Maaser Sheni (מעשר שני).
    MaaserSheni,
    /// Chalah (חלה).
    Chalah,
    /// Orlah (ערלה).
    Orlah,
    /// Bikurim (ביכורים).
    Bikurim,
    /// Shabbos (שבת).
    Shabbos,
    /// Eruvin (עירובין).
    Eruvin,
    /// Pesachim (פסחים).
    Pesachim,
    /// Shekalim (שקלים).
    Shekalim,
    /// Yoma (יומא).
    Yoma,
    /// Sukkah (סוכה).
    Sukkah,
    /// Beitzah (ביצה).
    Beitzah,
    /// Rosh Hashanah (ראש השנה).
    RoshHashanah,
    /// Taanis (תענית).
    Taanis,
    /// Megillah (מגילה).
    Megillah,
    /// Moed Katan (מועד קטן).
    MoedKatan,
    /// Chagigah (חגיגה).
    Chagigah,
    /// Yevamos (יבמות).
    Yevamos,
    /// Kesubos (כתובות).
    Kesubos,
    /// Nedarim (נדרים).
    Nedarim,
    /// Nazir (נזיר).
    Nazir,
    /// Sotah (סוטה).
    Sotah,
    /// Gitin (גיטין).
    Gitin,
    /// Kiddushin (קידושין).
    Kiddushin,
    /// Bava Kamma (בבא קמא).
    BavaKamma,
    /// Bava Metzia (בבא מציעא).
    BavaMetzia,
    /// Bava Basra (בבא בתרא).
    BavaBasra,
    /// Sanhedrin (סנהדרין).
    Sanhedrin,
    /// Makkos (מכות).
    Makkos,
    /// Shevuos (שבועות).
    Shevuos,
    /// Eduyos (עדיות).
    Eduyos,
    /// Avodah Zarah (עבודה זרה).
    AvodahZarah,
    /// Avos (אבות).
    Avos,
    /// Horiyos (הוריות).
    Horiyos,
    /// Zevachim (זבחים).
    Zevachim,
    /// Menachos (מנחות).
    Menachos,
    /// Chullin (חולין).
    Chullin,
    /// Bechoros (בכורות).
    Bechoros,
    /// Arachin (ערכין).
    Arachin,
    /// Temurah (תמורה).
    Temurah,
    /// Kerisos (כריתות).
    Kerisos,
    /// Meilah (מעילה).
    Meilah,
    /// Tamid (תמיד).
    Tamid,
    /// Midos (מידות).
    Midos,
    /// Kinnim (קינים).
    Kinnim,
    /// Keilim (כלים).
    Keilim,
    /// Ohalos (אהלות).
    Ohalos,
    /// Negaim (נגעים).
    Negaim,
    /// Parah (פרה).
    Parah,
    /// Taharos (טהרות).
    Taharos,
    /// Mikvaos (מקואות).
    Mikvaos,
    /// Niddah (נדה).
    Niddah,
    /// Machshirin (מכשירין).
    Machshirin,
    /// Zavim (זבים).
    Zavim,
    /// Tevul Yom (טבול יום).
    TevulYom,
    /// Yadayim (ידים).
    Yadayim,
    /// Uktzin (עוקצין).
    Uktzin,
}

impl MishnaTractate {
    /// Every tractate, in the order of the Mishnah.
    pub const ALL: [MishnaTractate; 63] = [
        MishnaTractate::Berachos,
        MishnaTractate::Peah,
        MishnaTractate::Demai,
        MishnaTractate::Kilayim,
        MishnaTractate::Sheviis,
        MishnaTractate::Terumos,
        MishnaTractate::Maasros,
        MishnaTractate::MaaserSheni,
        MishnaTractate::Chalah,
        MishnaTractate::Orlah,
        MishnaTractate::Bikurim,
        MishnaTractate::Shabbos,
        MishnaTractate::Eruvin,
        MishnaTractate::Pesachim,
        MishnaTractate::Shekalim,
        MishnaTractate::Yoma,
        MishnaTractate::Sukkah,
        MishnaTractate::Beitzah,
        MishnaTractate::RoshHashanah,
        MishnaTractate::Taanis,
        MishnaTractate::Megillah,
        MishnaTractate::MoedKatan,
        MishnaTractate::Chagigah,
        MishnaTractate::Yevamos,
        MishnaTractate::Kesubos,
        MishnaTractate::Nedarim,
        MishnaTractate::Nazir,
        MishnaTractate::Sotah,
        MishnaTractate::Gitin,
        MishnaTractate::Kiddushin,
        MishnaTractate::BavaKamma,
        MishnaTractate::BavaMetzia,
        MishnaTractate::BavaBasra,
        MishnaTractate::Sanhedrin,
        MishnaTractate::Makkos,
        MishnaTractate::Shevuos,
        MishnaTractate::Eduyos,
        MishnaTractate::AvodahZarah,
        MishnaTractate::Avos,
        MishnaTractate::Horiyos,
        MishnaTractate::Zevachim,
        MishnaTractate::Menachos,
        MishnaTractate::Chullin,
        MishnaTractate::Bechoros,
        MishnaTractate::Arachin,
        MishnaTractate::Temurah,
        MishnaTractate::Kerisos,
        MishnaTractate::Meilah,
        MishnaTractate::Tamid,
        MishnaTractate::Midos,
        MishnaTractate::Kinnim,
        MishnaTractate::Keilim,
        MishnaTractate::Ohalos,
        MishnaTractate::Negaim,
        MishnaTractate::Parah,
        MishnaTractate::Taharos,
        MishnaTractate::Mikvaos,
        MishnaTractate::Niddah,
        MishnaTractate::Machshirin,
        MishnaTractate::Zavim,
        MishnaTractate::TevulYom,
        MishnaTractate::Yadayim,
        MishnaTractate::Uktzin,
    ];

    /// The name in Hebrew, for example `"בבא מציעא"`.
    pub const fn hebrew(self) -> &'static str {
        match self {
            MishnaTractate::Berachos => "ברכות",
            MishnaTractate::Peah => "פאה",
            MishnaTractate::Demai => "דמאי",
            MishnaTractate::Kilayim => "כלאים",
            MishnaTractate::Sheviis => "שביעית",
            MishnaTractate::Terumos => "תרומות",
            MishnaTractate::Maasros => "מעשרות",
            MishnaTractate::MaaserSheni => "מעשר שני",
            MishnaTractate::Chalah => "חלה",
            MishnaTractate::Orlah => "ערלה",
            MishnaTractate::Bikurim => "ביכורים",
            MishnaTractate::Shabbos => "שבת",
            MishnaTractate::Eruvin => "עירובין",
            MishnaTractate::Pesachim => "פסחים",
            MishnaTractate::Shekalim => "שקלים",
            MishnaTractate::Yoma => "יומא",
            MishnaTractate::Sukkah => "סוכה",
            MishnaTractate::Beitzah => "ביצה",
            MishnaTractate::RoshHashanah => "ראש השנה",
            MishnaTractate::Taanis => "תענית",
            MishnaTractate::Megillah => "מגילה",
            MishnaTractate::MoedKatan => "מועד קטן",
            MishnaTractate::Chagigah => "חגיגה",
            MishnaTractate::Yevamos => "יבמות",
            MishnaTractate::Kesubos => "כתובות",
            MishnaTractate::Nedarim => "נדרים",
            MishnaTractate::Nazir => "נזיר",
            MishnaTractate::Sotah => "סוטה",
            MishnaTractate::Gitin => "גיטין",
            MishnaTractate::Kiddushin => "קידושין",
            MishnaTractate::BavaKamma => "בבא קמא",
            MishnaTractate::BavaMetzia => "בבא מציעא",
            MishnaTractate::BavaBasra => "בבא בתרא",
            MishnaTractate::Sanhedrin => "סנהדרין",
            MishnaTractate::Makkos => "מכות",
            MishnaTractate::Shevuos => "שבועות",
            MishnaTractate::Eduyos => "עדיות",
            MishnaTractate::AvodahZarah => "עבודה זרה",
            MishnaTractate::Avos => "אבות",
            MishnaTractate::Horiyos => "הוריות",
            MishnaTractate::Zevachim => "זבחים",
            MishnaTractate::Menachos => "מנחות",
            MishnaTractate::Chullin => "חולין",
            MishnaTractate::Bechoros => "בכורות",
            MishnaTractate::Arachin => "ערכין",
            MishnaTractate::Temurah => "תמורה",
            MishnaTractate::Kerisos => "כריתות",
            MishnaTractate::Meilah => "מעילה",
            MishnaTractate::Tamid => "תמיד",
            MishnaTractate::Midos => "מידות",
            MishnaTractate::Kinnim => "קינים",
            MishnaTractate::Keilim => "כלים",
            MishnaTractate::Ohalos => "אהלות",
            MishnaTractate::Negaim => "נגעים",
            MishnaTractate::Parah => "פרה",
            MishnaTractate::Taharos => "טהרות",
            MishnaTractate::Mikvaos => "מקואות",
            MishnaTractate::Niddah => "נדה",
            MishnaTractate::Machshirin => "מכשירין",
            MishnaTractate::Zavim => "זבים",
            MishnaTractate::TevulYom => "טבול יום",
            MishnaTractate::Yadayim => "ידים",
            MishnaTractate::Uktzin => "עוקצין",
        }
    }
}

impl MishnaTractate {
    /// The variant whose [`name`](Self::name) is exactly `name`. Returns `None` when no
    /// variant has that name.
    pub fn from_name(name: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|candidate| candidate.name() == name)
    }

    /// The name in English, for example `"Bava Metzia"`.
    pub const fn name(self) -> &'static str {
        match self {
            MishnaTractate::Berachos => "Berachos",
            MishnaTractate::Peah => "Peah",
            MishnaTractate::Demai => "Demai",
            MishnaTractate::Kilayim => "Kilayim",
            MishnaTractate::Sheviis => "Sheviis",
            MishnaTractate::Terumos => "Terumos",
            MishnaTractate::Maasros => "Maasros",
            MishnaTractate::MaaserSheni => "Maaser Sheni",
            MishnaTractate::Chalah => "Chalah",
            MishnaTractate::Orlah => "Orlah",
            MishnaTractate::Bikurim => "Bikurim",
            MishnaTractate::Shabbos => "Shabbos",
            MishnaTractate::Eruvin => "Eruvin",
            MishnaTractate::Pesachim => "Pesachim",
            MishnaTractate::Shekalim => "Shekalim",
            MishnaTractate::Yoma => "Yoma",
            MishnaTractate::Sukkah => "Sukkah",
            MishnaTractate::Beitzah => "Beitzah",
            MishnaTractate::RoshHashanah => "Rosh Hashanah",
            MishnaTractate::Taanis => "Taanis",
            MishnaTractate::Megillah => "Megillah",
            MishnaTractate::MoedKatan => "Moed Katan",
            MishnaTractate::Chagigah => "Chagigah",
            MishnaTractate::Yevamos => "Yevamos",
            MishnaTractate::Kesubos => "Kesubos",
            MishnaTractate::Nedarim => "Nedarim",
            MishnaTractate::Nazir => "Nazir",
            MishnaTractate::Sotah => "Sotah",
            MishnaTractate::Gitin => "Gitin",
            MishnaTractate::Kiddushin => "Kiddushin",
            MishnaTractate::BavaKamma => "Bava Kamma",
            MishnaTractate::BavaMetzia => "Bava Metzia",
            MishnaTractate::BavaBasra => "Bava Basra",
            MishnaTractate::Sanhedrin => "Sanhedrin",
            MishnaTractate::Makkos => "Makkos",
            MishnaTractate::Shevuos => "Shevuos",
            MishnaTractate::Eduyos => "Eduyos",
            MishnaTractate::AvodahZarah => "Avodah Zarah",
            MishnaTractate::Avos => "Avos",
            MishnaTractate::Horiyos => "Horiyos",
            MishnaTractate::Zevachim => "Zevachim",
            MishnaTractate::Menachos => "Menachos",
            MishnaTractate::Chullin => "Chullin",
            MishnaTractate::Bechoros => "Bechoros",
            MishnaTractate::Arachin => "Arachin",
            MishnaTractate::Temurah => "Temurah",
            MishnaTractate::Kerisos => "Kerisos",
            MishnaTractate::Meilah => "Meilah",
            MishnaTractate::Tamid => "Tamid",
            MishnaTractate::Midos => "Midos",
            MishnaTractate::Kinnim => "Kinnim",
            MishnaTractate::Keilim => "Keilim",
            MishnaTractate::Ohalos => "Ohalos",
            MishnaTractate::Negaim => "Negaim",
            MishnaTractate::Parah => "Parah",
            MishnaTractate::Taharos => "Taharos",
            MishnaTractate::Mikvaos => "Mikvaos",
            MishnaTractate::Niddah => "Niddah",
            MishnaTractate::Machshirin => "Machshirin",
            MishnaTractate::Zavim => "Zavim",
            MishnaTractate::TevulYom => "Tevul Yom",
            MishnaTractate::Yadayim => "Yadayim",
            MishnaTractate::Uktzin => "Uktzin",
        }
    }
}

/// The name in English, for example "Bava Metzia".
impl fmt::Display for MishnaTractate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}
/// Declares a Talmud's tractates, in its own order, from the [`MishnaTractate`]s they share names
/// with.
macro_rules! talmud_tractates {
    ($(#[doc = $enum_doc:literal])* $talmud:ident, $count:literal { $($(#[doc = $doc:literal])* $variant:ident,)* }) => {
        $(#[doc = $enum_doc])*
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum $talmud {
            $($(#[doc = $doc])* $variant,)*
        }

        impl $talmud {
            /// Every tractate, in this Talmud's order.
            pub const ALL: [$talmud; $count] = [$($talmud::$variant),*];

            /// The variant whose [`name`](Self::name) is exactly `name`. Returns `None` when no
            /// variant has that name.
            pub fn from_name(name: &str) -> Option<Self> {
                Self::ALL.into_iter().find(|candidate| candidate.name() == name)
            }

            /// The name in English, for example `"Bava Metzia"`.
            pub const fn name(self) -> &'static str {
                self.mishna().name()
            }

            /// The name in Hebrew, for example `"בבא מציעא"`.
            pub const fn hebrew(self) -> &'static str {
                self.mishna().hebrew()
            }

            /// The tractate of the Mishnah with the same name.
            pub const fn mishna(self) -> MishnaTractate {
                match self {
                    $($talmud::$variant => MishnaTractate::$variant,)*
                }
            }
        }

        impl From<$talmud> for MishnaTractate {
            fn from(tractate: $talmud) -> Self {
                tractate.mishna()
            }
        }

        /// The name in English, for example "Bava Metzia".
        impl fmt::Display for $talmud {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(self.name())
            }
        }
    };
}

talmud_tractates! {
    /// A tractate of Talmud Bavli. Tractates sort in the order Daf Yomi learns them.
    ///
    /// It includes Shekalim, which is from the Yerushalmi but printed and learned with the Bavli.
    BavliTractate, 40 {
        /// Berachos (ברכות).
        Berachos,
        /// Shabbos (שבת).
        Shabbos,
        /// Eruvin (עירובין).
        Eruvin,
        /// Pesachim (פסחים).
        Pesachim,
        /// Shekalim (שקלים).
        Shekalim,
        /// Yoma (יומא).
        Yoma,
        /// Sukkah (סוכה).
        Sukkah,
        /// Beitzah (ביצה).
        Beitzah,
        /// Rosh Hashanah (ראש השנה).
        RoshHashanah,
        /// Taanis (תענית).
        Taanis,
        /// Megillah (מגילה).
        Megillah,
        /// Moed Katan (מועד קטן).
        MoedKatan,
        /// Chagigah (חגיגה).
        Chagigah,
        /// Yevamos (יבמות).
        Yevamos,
        /// Kesubos (כתובות).
        Kesubos,
        /// Nedarim (נדרים).
        Nedarim,
        /// Nazir (נזיר).
        Nazir,
        /// Sotah (סוטה).
        Sotah,
        /// Gitin (גיטין).
        Gitin,
        /// Kiddushin (קידושין).
        Kiddushin,
        /// Bava Kamma (בבא קמא).
        BavaKamma,
        /// Bava Metzia (בבא מציעא).
        BavaMetzia,
        /// Bava Basra (בבא בתרא).
        BavaBasra,
        /// Sanhedrin (סנהדרין).
        Sanhedrin,
        /// Makkos (מכות).
        Makkos,
        /// Shevuos (שבועות).
        Shevuos,
        /// Avodah Zarah (עבודה זרה).
        AvodahZarah,
        /// Horiyos (הוריות).
        Horiyos,
        /// Zevachim (זבחים).
        Zevachim,
        /// Menachos (מנחות).
        Menachos,
        /// Chullin (חולין).
        Chullin,
        /// Bechoros (בכורות).
        Bechoros,
        /// Arachin (ערכין).
        Arachin,
        /// Temurah (תמורה).
        Temurah,
        /// Kerisos (כריתות).
        Kerisos,
        /// Meilah (מעילה).
        Meilah,
        /// Kinnim (קינים).
        Kinnim,
        /// Tamid (תמיד).
        Tamid,
        /// Midos (מידות).
        Midos,
        /// Niddah (נדה).
        Niddah,
    }
}

talmud_tractates! {
    /// A tractate of Talmud Yerushalmi. Tractates sort in the order of the Vilna edition.
    YerushalmiTractate, 39 {
        /// Berachos (ברכות).
        Berachos,
        /// Peah (פאה).
        Peah,
        /// Demai (דמאי).
        Demai,
        /// Kilayim (כלאים).
        Kilayim,
        /// Sheviis (שביעית).
        Sheviis,
        /// Terumos (תרומות).
        Terumos,
        /// Maasros (מעשרות).
        Maasros,
        /// Maaser Sheni (מעשר שני).
        MaaserSheni,
        /// Chalah (חלה).
        Chalah,
        /// Orlah (ערלה).
        Orlah,
        /// Bikurim (ביכורים).
        Bikurim,
        /// Shabbos (שבת).
        Shabbos,
        /// Eruvin (עירובין).
        Eruvin,
        /// Pesachim (פסחים).
        Pesachim,
        /// Beitzah (ביצה).
        Beitzah,
        /// Rosh Hashanah (ראש השנה).
        RoshHashanah,
        /// Yoma (יומא).
        Yoma,
        /// Sukkah (סוכה).
        Sukkah,
        /// Taanis (תענית).
        Taanis,
        /// Shekalim (שקלים).
        Shekalim,
        /// Megillah (מגילה).
        Megillah,
        /// Chagigah (חגיגה).
        Chagigah,
        /// Moed Katan (מועד קטן).
        MoedKatan,
        /// Yevamos (יבמות).
        Yevamos,
        /// Kesubos (כתובות).
        Kesubos,
        /// Sotah (סוטה).
        Sotah,
        /// Nedarim (נדרים).
        Nedarim,
        /// Nazir (נזיר).
        Nazir,
        /// Gitin (גיטין).
        Gitin,
        /// Kiddushin (קידושין).
        Kiddushin,
        /// Bava Kamma (בבא קמא).
        BavaKamma,
        /// Bava Metzia (בבא מציעא).
        BavaMetzia,
        /// Bava Basra (בבא בתרא).
        BavaBasra,
        /// Shevuos (שבועות).
        Shevuos,
        /// Makkos (מכות).
        Makkos,
        /// Sanhedrin (סנהדרין).
        Sanhedrin,
        /// Avodah Zarah (עבודה זרה).
        AvodahZarah,
        /// Horiyos (הוריות).
        Horiyos,
        /// Niddah (נדה).
        Niddah,
    }
}

/// A page (*daf*) of the Talmud, both sides of one leaf. Prints as "Bava Metzia 55".
///
/// `T` is the Talmud's tractate type, [`BavliTractate`] or [`YerushalmiTractate`]. Pages sort in
/// that Talmud's order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Daf<T> {
    /// The tractate.
    pub tractate: T,
    /// The page number. The Bavli starts at 2, the Yerushalmi at 1.
    pub daf: u16,
}

impl<T: fmt::Display> fmt::Display for Daf<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.tractate, self.daf)
    }
}

/// A side of a page, either *amud aleph* (a) or *amud beis* (b).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "wasm", derive(serde::Serialize, serde::Deserialize, tsify::Tsify))]
pub enum Side {
    /// The front of the page, "a".
    #[cfg_attr(feature = "wasm", serde(rename = "a"))]
    Aleph,
    /// The back of the page, "b".
    #[cfg_attr(feature = "wasm", serde(rename = "b"))]
    Bet,
}

/// One side of a page (*amud*) of the Bavli. Prints as "Bava Metzia 55b".
///
/// Sides sort in the order Daf Yomi learns them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Amud {
    /// The tractate.
    pub tractate: BavliTractate,
    /// The page number. The Bavli starts at 2.
    pub daf: u16,
    /// Which side of the page.
    pub side: Side,
}

impl fmt::Display for Amud {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let side = match self.side {
            Side::Aleph => "a",
            Side::Bet => "b",
        };
        write!(f, "{} {}{}", self.tractate, self.daf, side)
    }
}

impl Amud {
    pub(crate) const fn new(tractate: BavliTractate, daf: u16, side: Side) -> Self {
        Self { tractate, daf, side }
    }

    fn page_side_index(self) -> usize {
        self.daf as usize * 2
            + match self.side {
                Side::Aleph => 0,
                Side::Bet => 1,
            }
    }

    /// Number of amudim from `self` through `end`, inclusive.
    ///
    /// Returns `None` if the tractates differ or if this tractate starts after `end`.
    /// e.g. Kiddushin 2a through 2b → `2`.
    pub(crate) fn amudim_to(self, end: Self) -> Option<i32> {
        if self.tractate != end.tractate {
            return None;
        }
        let start = self.page_side_index();
        let end_index = end.page_side_index();
        if start > end_index {
            return None;
        }
        Some((end_index - start + 1) as i32)
    }

    /// Amud at a 0-based offset from `self` (offset 0 returns `self`).
    pub(crate) fn at_offset(self, offset: i32) -> Option<Self> {
        if offset < 0 {
            return None;
        }
        let index = self.page_side_index() + offset as usize;
        Some(Self::new(
            self.tractate,
            (index / 2) as u16,
            if index.is_multiple_of(2) {
                Side::Aleph
            } else {
                Side::Bet
            },
        ))
    }
}

/// One *mishnah*. Prints as "Nazir 5:5".
///
/// *Mishnayos* sort in the order of the Mishnah.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Mishna {
    /// The tractate.
    pub tractate: MishnaTractate,
    /// The chapter (*perek*), from 1.
    pub chapter: u16,
    /// The *mishnah* within the chapter, from 1.
    pub mishna: u16,
}

impl fmt::Display for Mishna {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}:{}", self.tractate, self.chapter, self.mishna)
    }
}
