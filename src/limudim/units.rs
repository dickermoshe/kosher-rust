use core::fmt;

/// A tractate of the Mishnah and Talmud, in the order of the Mishnah.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tractate {
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

impl Tractate {
    /// The name in Hebrew, for example `"בבא מציעא"`.
    pub const fn hebrew(self) -> &'static str {
        match self {
            Tractate::Berachos => "ברכות",
            Tractate::Peah => "פאה",
            Tractate::Demai => "דמאי",
            Tractate::Kilayim => "כלאים",
            Tractate::Sheviis => "שביעית",
            Tractate::Terumos => "תרומות",
            Tractate::Maasros => "מעשרות",
            Tractate::MaaserSheni => "מעשר שני",
            Tractate::Chalah => "חלה",
            Tractate::Orlah => "ערלה",
            Tractate::Bikurim => "ביכורים",
            Tractate::Shabbos => "שבת",
            Tractate::Eruvin => "עירובין",
            Tractate::Pesachim => "פסחים",
            Tractate::Shekalim => "שקלים",
            Tractate::Yoma => "יומא",
            Tractate::Sukkah => "סוכה",
            Tractate::Beitzah => "ביצה",
            Tractate::RoshHashanah => "ראש השנה",
            Tractate::Taanis => "תענית",
            Tractate::Megillah => "מגילה",
            Tractate::MoedKatan => "מועד קטן",
            Tractate::Chagigah => "חגיגה",
            Tractate::Yevamos => "יבמות",
            Tractate::Kesubos => "כתובות",
            Tractate::Nedarim => "נדרים",
            Tractate::Nazir => "נזיר",
            Tractate::Sotah => "סוטה",
            Tractate::Gitin => "גיטין",
            Tractate::Kiddushin => "קידושין",
            Tractate::BavaKamma => "בבא קמא",
            Tractate::BavaMetzia => "בבא מציעא",
            Tractate::BavaBasra => "בבא בתרא",
            Tractate::Sanhedrin => "סנהדרין",
            Tractate::Makkos => "מכות",
            Tractate::Shevuos => "שבועות",
            Tractate::Eduyos => "עדיות",
            Tractate::AvodahZarah => "עבודה זרה",
            Tractate::Avos => "אבות",
            Tractate::Horiyos => "הוריות",
            Tractate::Zevachim => "זבחים",
            Tractate::Menachos => "מנחות",
            Tractate::Chullin => "חולין",
            Tractate::Bechoros => "בכורות",
            Tractate::Arachin => "ערכין",
            Tractate::Temurah => "תמורה",
            Tractate::Kerisos => "כריתות",
            Tractate::Meilah => "מעילה",
            Tractate::Tamid => "תמיד",
            Tractate::Midos => "מידות",
            Tractate::Kinnim => "קינים",
            Tractate::Keilim => "כלים",
            Tractate::Ohalos => "אהלות",
            Tractate::Negaim => "נגעים",
            Tractate::Parah => "פרה",
            Tractate::Taharos => "טהרות",
            Tractate::Mikvaos => "מקואות",
            Tractate::Niddah => "נדה",
            Tractate::Machshirin => "מכשירין",
            Tractate::Zavim => "זבים",
            Tractate::TevulYom => "טבול יום",
            Tractate::Yadayim => "ידים",
            Tractate::Uktzin => "עוקצין",
        }
    }
}

/// The name in English, for example "Bava Metzia".
impl fmt::Display for Tractate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Tractate::Berachos => "Berachos",
            Tractate::Peah => "Peah",
            Tractate::Demai => "Demai",
            Tractate::Kilayim => "Kilayim",
            Tractate::Sheviis => "Sheviis",
            Tractate::Terumos => "Terumos",
            Tractate::Maasros => "Maasros",
            Tractate::MaaserSheni => "Maaser Sheni",
            Tractate::Chalah => "Chalah",
            Tractate::Orlah => "Orlah",
            Tractate::Bikurim => "Bikurim",
            Tractate::Shabbos => "Shabbos",
            Tractate::Eruvin => "Eruvin",
            Tractate::Pesachim => "Pesachim",
            Tractate::Shekalim => "Shekalim",
            Tractate::Yoma => "Yoma",
            Tractate::Sukkah => "Sukkah",
            Tractate::Beitzah => "Beitzah",
            Tractate::RoshHashanah => "Rosh Hashanah",
            Tractate::Taanis => "Taanis",
            Tractate::Megillah => "Megillah",
            Tractate::MoedKatan => "Moed Katan",
            Tractate::Chagigah => "Chagigah",
            Tractate::Yevamos => "Yevamos",
            Tractate::Kesubos => "Kesubos",
            Tractate::Nedarim => "Nedarim",
            Tractate::Nazir => "Nazir",
            Tractate::Sotah => "Sotah",
            Tractate::Gitin => "Gitin",
            Tractate::Kiddushin => "Kiddushin",
            Tractate::BavaKamma => "Bava Kamma",
            Tractate::BavaMetzia => "Bava Metzia",
            Tractate::BavaBasra => "Bava Basra",
            Tractate::Sanhedrin => "Sanhedrin",
            Tractate::Makkos => "Makkos",
            Tractate::Shevuos => "Shevuos",
            Tractate::Eduyos => "Eduyos",
            Tractate::AvodahZarah => "Avodah Zarah",
            Tractate::Avos => "Avos",
            Tractate::Horiyos => "Horiyos",
            Tractate::Zevachim => "Zevachim",
            Tractate::Menachos => "Menachos",
            Tractate::Chullin => "Chullin",
            Tractate::Bechoros => "Bechoros",
            Tractate::Arachin => "Arachin",
            Tractate::Temurah => "Temurah",
            Tractate::Kerisos => "Kerisos",
            Tractate::Meilah => "Meilah",
            Tractate::Tamid => "Tamid",
            Tractate::Midos => "Midos",
            Tractate::Kinnim => "Kinnim",
            Tractate::Keilim => "Keilim",
            Tractate::Ohalos => "Ohalos",
            Tractate::Negaim => "Negaim",
            Tractate::Parah => "Parah",
            Tractate::Taharos => "Taharos",
            Tractate::Mikvaos => "Mikvaos",
            Tractate::Niddah => "Niddah",
            Tractate::Machshirin => "Machshirin",
            Tractate::Zavim => "Zavim",
            Tractate::TevulYom => "Tevul Yom",
            Tractate::Yadayim => "Yadayim",
            Tractate::Uktzin => "Uktzin",
        })
    }
}
/// Every tractate, in the order of the Mishnah.
#[allow(dead_code)]
pub(crate) const TRACTATES: [Tractate; 63] = [
    Tractate::Berachos,
    Tractate::Peah,
    Tractate::Demai,
    Tractate::Kilayim,
    Tractate::Sheviis,
    Tractate::Terumos,
    Tractate::Maasros,
    Tractate::MaaserSheni,
    Tractate::Chalah,
    Tractate::Orlah,
    Tractate::Bikurim,
    Tractate::Shabbos,
    Tractate::Eruvin,
    Tractate::Pesachim,
    Tractate::Shekalim,
    Tractate::Yoma,
    Tractate::Sukkah,
    Tractate::Beitzah,
    Tractate::RoshHashanah,
    Tractate::Taanis,
    Tractate::Megillah,
    Tractate::MoedKatan,
    Tractate::Chagigah,
    Tractate::Yevamos,
    Tractate::Kesubos,
    Tractate::Nedarim,
    Tractate::Nazir,
    Tractate::Sotah,
    Tractate::Gitin,
    Tractate::Kiddushin,
    Tractate::BavaKamma,
    Tractate::BavaMetzia,
    Tractate::BavaBasra,
    Tractate::Sanhedrin,
    Tractate::Makkos,
    Tractate::Shevuos,
    Tractate::Eduyos,
    Tractate::AvodahZarah,
    Tractate::Avos,
    Tractate::Horiyos,
    Tractate::Zevachim,
    Tractate::Menachos,
    Tractate::Chullin,
    Tractate::Bechoros,
    Tractate::Arachin,
    Tractate::Temurah,
    Tractate::Kerisos,
    Tractate::Meilah,
    Tractate::Tamid,
    Tractate::Midos,
    Tractate::Kinnim,
    Tractate::Keilim,
    Tractate::Ohalos,
    Tractate::Negaim,
    Tractate::Parah,
    Tractate::Taharos,
    Tractate::Mikvaos,
    Tractate::Niddah,
    Tractate::Machshirin,
    Tractate::Zavim,
    Tractate::TevulYom,
    Tractate::Yadayim,
    Tractate::Uktzin,
];
/// A list of all the tractates in Talmud Bavli.
pub(crate) const BAVLI_TRACTATES: [Tractate; 40] = [
    Tractate::Berachos,
    Tractate::Shabbos,
    Tractate::Eruvin,
    Tractate::Pesachim,
    Tractate::Shekalim,
    Tractate::Yoma,
    Tractate::Sukkah,
    Tractate::Beitzah,
    Tractate::RoshHashanah,
    Tractate::Taanis,
    Tractate::Megillah,
    Tractate::MoedKatan,
    Tractate::Chagigah,
    Tractate::Yevamos,
    Tractate::Kesubos,
    Tractate::Nedarim,
    Tractate::Nazir,
    Tractate::Sotah,
    Tractate::Gitin,
    Tractate::Kiddushin,
    Tractate::BavaKamma,
    Tractate::BavaMetzia,
    Tractate::BavaBasra,
    Tractate::Sanhedrin,
    Tractate::Makkos,
    Tractate::Shevuos,
    Tractate::AvodahZarah,
    Tractate::Horiyos,
    Tractate::Zevachim,
    Tractate::Menachos,
    Tractate::Chullin,
    Tractate::Bechoros,
    Tractate::Arachin,
    Tractate::Temurah,
    Tractate::Kerisos,
    Tractate::Meilah,
    Tractate::Kinnim,
    Tractate::Tamid,
    Tractate::Midos,
    Tractate::Niddah,
];

/// A list of all the tractates in Talmud Yerushalmi.
pub(crate) const YERUSHALMI_TRACTATES: [Tractate; 39] = [
    Tractate::Berachos,
    Tractate::Peah,
    Tractate::Demai,
    Tractate::Kilayim,
    Tractate::Sheviis,
    Tractate::Terumos,
    Tractate::Maasros,
    Tractate::MaaserSheni,
    Tractate::Chalah,
    Tractate::Orlah,
    Tractate::Bikurim,
    Tractate::Shabbos,
    Tractate::Eruvin,
    Tractate::Pesachim,
    Tractate::Beitzah,
    Tractate::RoshHashanah,
    Tractate::Yoma,
    Tractate::Sukkah,
    Tractate::Taanis,
    Tractate::Shekalim,
    Tractate::Megillah,
    Tractate::Chagigah,
    Tractate::MoedKatan,
    Tractate::Yevamos,
    Tractate::Kesubos,
    Tractate::Sotah,
    Tractate::Nedarim,
    Tractate::Nazir,
    Tractate::Gitin,
    Tractate::Kiddushin,
    Tractate::BavaKamma,
    Tractate::BavaMetzia,
    Tractate::BavaBasra,
    Tractate::Shevuos,
    Tractate::Makkos,
    Tractate::Sanhedrin,
    Tractate::AvodahZarah,
    Tractate::Horiyos,
    Tractate::Niddah,
];

/// A page (*daf*) of the Talmud: both sides of one leaf. Prints as "Bava Metzia 55".
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Daf {
    /// The tractate.
    pub tractate: Tractate,
    /// The page number. The Bavli starts at 2, the Yerushalmi at 1.
    pub daf: u16,
}

impl fmt::Display for Daf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.tractate, self.daf)
    }
}

/// A side of a page: *amud aleph* (a) or *amud beis* (b).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Side {
    /// The front of the page, "a".
    Aleph,
    /// The back of the page, "b".
    Bet,
}

/// One side of a page (*amud*). Prints as "Bava Metzia 55b".
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Amud {
    /// The tractate.
    pub tractate: Tractate,
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
    pub(crate) const fn new(tractate: Tractate, daf: u16, side: Side) -> Self {
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Mishna {
    /// The tractate.
    pub tractate: Tractate,
    /// The chapter (*perek*), from 1.
    pub chapter: usize,
    /// The *mishnah* within the chapter, from 1.
    pub mishna: u16,
}

impl fmt::Display for Mishna {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}:{}", self.tractate, self.chapter, self.mishna)
    }
}
