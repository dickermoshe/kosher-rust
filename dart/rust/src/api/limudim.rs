//! The learning schedules, as functions of a date. kosher-rust's `Schedule` trait and its `Daf` are
//! generic, which Dart can't call.

use extend::ext;
use flutter_rust_bridge::frb;
use kosher_rust::calendar::HebrewDate;
use kosher_rust::limudim::{
    self, Amud, AmudYomiDirshu, BavliTractate, DafHashavuaBavli, DafYomiBavli, DafYomiYerushalmi, Mishna, MishnaPair,
    MishnaTractate, MishnaYomis, PirkeiAvos, PirkeiAvosUnit, Schedule, TehillimMonthly, TehillimUnit,
    YerushalmiTractate,
};
use kosher_rust::Region;

/// A page (*daf*) of the Babylonian Talmud, both sides of one leaf. Prints as "Bava Metzia 55".
pub struct BavliDaf {
    /// The tractate.
    pub tractate: BavliTractate,
    /// The page number, from 2.
    pub daf: u16,
}

/// A page (*daf*) of the Jerusalem Talmud. Prints as "Bava Metzia 5".
pub struct YerushalmiDaf {
    /// The tractate.
    pub tractate: YerushalmiTractate,
    /// The page number, from 1.
    pub daf: u16,
}

impl From<limudim::Daf<BavliTractate>> for BavliDaf {
    fn from(daf: limudim::Daf<BavliTractate>) -> Self {
        Self {
            tractate: daf.tractate,
            daf: daf.daf,
        }
    }
}

impl From<limudim::Daf<YerushalmiTractate>> for YerushalmiDaf {
    fn from(daf: limudim::Daf<YerushalmiTractate>) -> Self {
        Self {
            tractate: daf.tractate,
            daf: daf.daf,
        }
    }
}

impl BavliDaf {
    /// The page written out, for example "Bava Metzia 55".
    #[frb(name = "toString")]
    pub fn display(&self) -> String {
        limudim::Daf {
            tractate: self.tractate,
            daf: self.daf,
        }
        .to_string()
    }
}

impl YerushalmiDaf {
    /// The page written out, for example "Bava Metzia 5".
    #[frb(name = "toString")]
    pub fn display(&self) -> String {
        limudim::Daf {
            tractate: self.tractate,
            daf: self.daf,
        }
        .to_string()
    }
}

/// The *daf* of Daf Yomi on `date`, one page of the Babylonian Talmud a day since 11 September
/// 1923.
///
/// Returns null before the program began.
pub fn daf_yomi_bavli(date: &HebrewDate) -> Option<BavliDaf> {
    DafYomiBavli.on(*date).map(Into::into)
}

/// The *daf* of Daf HaShavua on `date`, one page of the Babylonian Talmud a week from Sunday to
/// Shabbos, since 6 March 2005.
///
/// Returns null before the program began.
pub fn daf_hashavua_bavli(date: &HebrewDate) -> Option<BavliDaf> {
    DafHashavuaBavli.on(*date).map(Into::into)
}

/// The *amud* of Dirshu's Amud Yomi on `date`, one side of a page of the Babylonian Talmud a day
/// since 16 October 2023.
///
/// Returns null before the program began.
pub fn amud_yomi_dirshu(date: &HebrewDate) -> Option<Amud> {
    AmudYomiDirshu.on(*date)
}

/// The *daf* of Daf Yomi Yerushalmi on `date`, one page of the Vilna Yerushalmi a day since
/// 2 February 1980.
///
/// Returns null before the program began, and on Yom Kippur and Tishah B'Av, when no page is
/// learned.
pub fn daf_yomi_yerushalmi(date: &HebrewDate) -> Option<YerushalmiDaf> {
    DafYomiYerushalmi.on(*date).map(Into::into)
}

/// The two *mishnayos* of Mishna Yomis on `date`, two a day since 20 May 1947.
///
/// Returns null before the program began.
pub fn mishna_yomis(date: &HebrewDate) -> Option<MishnaPair> {
    MishnaYomis.on(*date)
}

/// The portion of Tehillim for `date`, the book divided over the days of the Hebrew month.
pub fn tehillim_monthly(date: &HebrewDate) -> Option<TehillimUnit> {
    TehillimMonthly.on(*date)
}

/// The Pirkei Avos read on the Shabbos of the week of `date`, counting the week from Sunday.
///
/// Returns null for a week whose Shabbos has no reading. The reading runs from Pesach to Rosh
/// Hashana.
pub fn pirkei_avos(date: &HebrewDate, region: Region) -> Option<PirkeiAvosUnit> {
    PirkeiAvos(region).on(*date)
}

#[ext]
pub impl Amud {
    /// The side written out, for example "Bava Metzia 55b".
    #[frb(name = "toString")]
    fn display(&self) -> String {
        self.to_string()
    }
}

#[ext]
pub impl Mishna {
    /// The *mishnah* written out, for example "Nazir 5:5".
    #[frb(name = "toString")]
    fn display(&self) -> String {
        self.to_string()
    }
}

#[ext]
pub impl MishnaPair {
    /// The pair written out, for example "Nazir 5:5-6", "Nazir 5:10-6:1" or "Nazir 9:5-Sotah 1:1".
    #[frb(name = "toString")]
    fn display(&self) -> String {
        self.to_string()
    }
}

#[ext]
pub impl TehillimUnit {
    /// The portion written out, for example "Tehillim 1-9" or "Tehillim 119:1-96".
    #[frb(name = "toString")]
    fn display(&self) -> String {
        self.to_string()
    }
}

#[ext]
pub impl PirkeiAvosUnit {
    /// The reading written out, for example "Pirkei Avos 3" or "Pirkei Avos 5-6".
    #[frb(name = "toString")]
    fn display(&self) -> String {
        self.to_string()
    }
}

#[ext]
pub impl MishnaTractate {
    /// The name in English, for example "Bava Metzia".
    #[frb(getter)]
    fn display_name(self) -> String {
        self.name().into()
    }

    /// The name in Hebrew, for example "בבא מציעא".
    #[frb(getter)]
    fn hebrew_name(self) -> String {
        self.hebrew().into()
    }
}

#[ext]
pub impl BavliTractate {
    /// The name in English, for example "Bava Metzia".
    #[frb(getter)]
    fn display_name(self) -> String {
        self.name().into()
    }

    /// The name in Hebrew, for example "בבא מציעא".
    #[frb(getter)]
    fn hebrew_name(self) -> String {
        self.hebrew().into()
    }
}

#[ext]
pub impl YerushalmiTractate {
    /// The name in English, for example "Bava Metzia".
    #[frb(getter)]
    fn display_name(self) -> String {
        self.name().into()
    }

    /// The name in Hebrew, for example "בבא מציעא".
    #[frb(getter)]
    fn hebrew_name(self) -> String {
        self.hebrew().into()
    }
}
