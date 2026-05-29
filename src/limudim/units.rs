/// A tractate of the Talmud (Bavli or Yerushalmi).
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[allow(missing_docs)]
pub enum Tractate {
    Berachos,
    Peah,
    Demai,
    Kilayim,
    Sheviis,
    Terumos,
    Maasros,
    MaaserSheni,
    Chalah,
    Orlah,
    Bikurim,
    Shabbos,
    Eruvin,
    Pesachim,
    Shekalim,
    Yoma,
    Sukkah,
    Beitzah,
    RoshHashanah,
    Taanis,
    Megillah,
    MoedKatan,
    Chagigah,
    Yevamos,
    Kesubos,
    Nedarim,
    Nazir,
    Sotah,
    Gitin,
    Kiddushin,
    BavaKamma,
    BavaMetzia,
    BavaBasra,
    Sanhedrin,
    Makkos,
    Shevuos,
    Eduyos,
    AvodahZarah,
    Avos,
    Horiyos,
    Zevachim,
    Menachos,
    Chullin,
    Bechoros,
    Arachin,
    Temurah,
    Kerisos,
    Meilah,
    Tamid,
    Midos,
    Kinnim,
    Keilim,
    Ohalos,
    Negaim,
    Parah,
    Taharos,
    Mikvaos,
    Niddah,
    Machshirin,
    Zavim,
    TevulYom,
    Yadayim,
    Uktzin,
}
/// A list of all the tractates in Talmud Bavli.
pub const TRACTATES: [Tractate; 63] = [
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
pub const BAVLI_TRACTATES: [Tractate; 40] = [
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
pub const YERUSHALMI_TRACTATES: [Tractate; 39] = [
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[allow(missing_docs)]
pub struct Daf {
    pub tractate: Tractate,
    /// The page number of the amud. Starts at 2 for Talmud Bavli and 1 for Talmud Yerushalmi.
    pub page: u16,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[allow(missing_docs)]
pub enum Side {
    Aleph,
    Bet,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[allow(missing_docs)]
pub struct Amud {
    pub tractate: Tractate,
    /// The page number of the amud. Starts at 2 for Talmud Bavli and 1 for Talmud Yerushalmi.
    pub page: u16,
    pub side: Side,
}
impl Amud {
    pub(crate) const fn new(tractate: Tractate, page: u16, side: Side) -> Self {
        Self { tractate, page, side }
    }

    fn page_side_index(self) -> usize {
        self.page as usize * 2
            + match self.side {
                Side::Aleph => 0,
                Side::Bet => 1,
            }
    }

    /// Number of amudim from `self` through `end`, inclusive.
    ///
    /// Returns `None` if the tractates differ or if this tractate starts after `end`.
    /// e.g. Kiddushin 2a through 2b → `2`.
    pub fn amudim_to(self, end: Self) -> Option<i32> {
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
    pub fn at_offset(self, offset: i32) -> Option<Self> {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[allow(missing_docs)]
pub struct Mishna {
    pub tractate: Tractate,
    pub chapter: usize,
    pub mishna: u16,
}
