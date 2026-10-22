use core::fmt;

use num_enum::{IntoPrimitive, TryFromPrimitive};

/// A Torah reading for Shabbos: one of the 54 weekly *parshiyos*, a pair read together in a
/// common year, or a special Shabbos.
///
/// Get one from [`HebrewDate::parsha`](super::HebrewDate::parsha),
/// [`next_parsha`](super::HebrewDate::next_parsha) or
/// [`special_parsha`](super::HebrewDate::special_parsha). The numbering matches KosherJava's
/// `JewishCalendar.Parsha`.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum Parsha {
    /// Bereshis (בראשית).
    Bereshis = 0,
    /// Noach (נח).
    Noach = 1,
    /// Lech Lecha (לך לך).
    LechLecha = 2,
    /// Vayera (וירא).
    Vayera = 3,
    /// Chayei Sara (חיי שרה).
    ChayeiSara = 4,
    /// Toldos (תולדות).
    Toldos = 5,
    /// Vayetzei (ויצא).
    Vayetzei = 6,
    /// Vayishlach (וישלח).
    Vayishlach = 7,
    /// Vayeshev (וישב).
    Vayeshev = 8,
    /// Miketz (מקץ).
    Miketz = 9,
    /// Vayigash (ויגש).
    Vayigash = 10,
    /// Vayechi (ויחי).
    Vayechi = 11,
    /// Shemos (שמות).
    Shemos = 12,
    /// Vaera (וארא).
    Vaera = 13,
    /// Bo (בא).
    Bo = 14,
    /// Beshalach (בשלח).
    Beshalach = 15,
    /// Yisro (יתרו).
    Yisro = 16,
    /// Mishpatim (משפטים).
    Mishpatim = 17,
    /// Terumah (תרומה).
    Terumah = 18,
    /// Tetzaveh (תצוה).
    Tetzaveh = 19,
    /// Ki Sisa (כי תשא).
    KiSisa = 20,
    /// Vayakhel (ויקהל).
    Vayakhel = 21,
    /// Pekudei (פקודי).
    Pekudei = 22,
    /// Vayikra (ויקרא).
    Vayikra = 23,
    /// Tzav (צו).
    Tzav = 24,
    /// Shmini (שמיני).
    Shmini = 25,
    /// Tazria (תזריע).
    Tazria = 26,
    /// Metzora (מצרע).
    Metzora = 27,
    /// Achrei Mos (אחרי מות).
    AchreiMos = 28,
    /// Kedoshim (קדושים).
    Kedoshim = 29,
    /// Emor (אמור).
    Emor = 30,
    /// Behar (בהר).
    Behar = 31,
    /// Bechukosai (בחקתי).
    Bechukosai = 32,
    /// Bamidbar (במדבר).
    Bamidbar = 33,
    /// Nasso (נשא).
    Nasso = 34,
    /// Behaaloscha (בהעלתך).
    Behaaloscha = 35,
    /// Shlach (שלח לך).
    Shlach = 36,
    /// Korach (קרח).
    Korach = 37,
    /// Chukas (חוקת).
    Chukas = 38,
    /// Balak (בלק).
    Balak = 39,
    /// Pinchas (פינחס).
    Pinchas = 40,
    /// Matos (מטות).
    Matos = 41,
    /// Masei (מסעי).
    Masei = 42,
    /// Devarim (דברים).
    Devarim = 43,
    /// Vaeschanan (ואתחנן).
    Vaeschanan = 44,
    /// Eikev (עקב).
    Eikev = 45,
    /// Reeh (ראה).
    Reeh = 46,
    /// Shoftim (שופטים).
    Shoftim = 47,
    /// Ki Seitzei (כי תצא).
    KiSeitzei = 48,
    /// Ki Savo (כי תבוא).
    KiSavo = 49,
    /// Nitzavim (נצבים).
    Nitzavim = 50,
    /// Vayeilech (וילך).
    Vayeilech = 51,
    /// Haazinu (האזינו).
    HaAzinu = 52,
    /// Vezos Habracha (וזאת הברכה).
    VezosHabracha = 53,
    /// Vayakhel and Pekudei, read together (ויקהל - פקודי).
    VayakhelPekudei = 54,
    /// Tazria and Metzora, read together (תזריע - מצרע).
    TazriaMetzora = 55,
    /// Achrei Mos and Kedoshim, read together (אחרי מות - קדושים).
    AchreiMosKedoshim = 56,
    /// Behar and Bechukosai, read together (בהר - בחקתי).
    BeharBechukosai = 57,
    /// Chukas and Balak, read together (חוקת - בלק).
    ChukasBalak = 58,
    /// Matos and Masei, read together (מטות - מסעי).
    MatosMasei = 59,
    /// Nitzavim and Vayeilech, read together (נצבים - וילך).
    NitzavimVayeilech = 60,
    /// Shabbos Shekalim (שקלים), on or before Rosh Chodesh Adar.
    Shekalim = 61,
    /// Shabbos Zachor (זכור), the Shabbos before Purim.
    Zachor = 62,
    /// Shabbos Parah (פרה), the Shabbos before Hachodesh.
    Parah = 63,
    /// Shabbos Hachodesh (החדש), on or before Rosh Chodesh Nisan.
    Hachodesh = 64,
    /// Shabbos Shuva (שובה), between Rosh Hashana and Yom Kippur.
    Shuva = 65,
    /// Shabbos Shira (שירה), the Shabbos of Beshalach.
    Shira = 66,
    /// Shabbos Hagadol (הגדול), the Shabbos before Pesach.
    Hagadol = 67,
    /// Shabbos Chazon (חזון), the Shabbos before Tishah B'Av.
    Chazon = 68,
    /// Shabbos Nachamu (נחמו), the Shabbos after Tishah B'Av.
    Nachamu = 69,
}

impl Parsha {
    /// The name in Hebrew, for example `"בראשית"`.
    pub const fn hebrew(self) -> &'static str {
        match self {
            Parsha::Bereshis => "בראשית",
            Parsha::Noach => "נח",
            Parsha::LechLecha => "לך לך",
            Parsha::Vayera => "וירא",
            Parsha::ChayeiSara => "חיי שרה",
            Parsha::Toldos => "תולדות",
            Parsha::Vayetzei => "ויצא",
            Parsha::Vayishlach => "וישלח",
            Parsha::Vayeshev => "וישב",
            Parsha::Miketz => "מקץ",
            Parsha::Vayigash => "ויגש",
            Parsha::Vayechi => "ויחי",
            Parsha::Shemos => "שמות",
            Parsha::Vaera => "וארא",
            Parsha::Bo => "בא",
            Parsha::Beshalach => "בשלח",
            Parsha::Yisro => "יתרו",
            Parsha::Mishpatim => "משפטים",
            Parsha::Terumah => "תרומה",
            Parsha::Tetzaveh => "תצוה",
            Parsha::KiSisa => "כי תשא",
            Parsha::Vayakhel => "ויקהל",
            Parsha::Pekudei => "פקודי",
            Parsha::Vayikra => "ויקרא",
            Parsha::Tzav => "צו",
            Parsha::Shmini => "שמיני",
            Parsha::Tazria => "תזריע",
            Parsha::Metzora => "מצרע",
            Parsha::AchreiMos => "אחרי מות",
            Parsha::Kedoshim => "קדושים",
            Parsha::Emor => "אמור",
            Parsha::Behar => "בהר",
            Parsha::Bechukosai => "בחקתי",
            Parsha::Bamidbar => "במדבר",
            Parsha::Nasso => "נשא",
            Parsha::Behaaloscha => "בהעלתך",
            Parsha::Shlach => "שלח לך",
            Parsha::Korach => "קרח",
            Parsha::Chukas => "חוקת",
            Parsha::Balak => "בלק",
            Parsha::Pinchas => "פינחס",
            Parsha::Matos => "מטות",
            Parsha::Masei => "מסעי",
            Parsha::Devarim => "דברים",
            Parsha::Vaeschanan => "ואתחנן",
            Parsha::Eikev => "עקב",
            Parsha::Reeh => "ראה",
            Parsha::Shoftim => "שופטים",
            Parsha::KiSeitzei => "כי תצא",
            Parsha::KiSavo => "כי תבוא",
            Parsha::Nitzavim => "נצבים",
            Parsha::Vayeilech => "וילך",
            Parsha::HaAzinu => "האזינו",
            Parsha::VezosHabracha => "וזאת הברכה",
            Parsha::VayakhelPekudei => "ויקהל - פקודי",
            Parsha::TazriaMetzora => "תזריע - מצרע",
            Parsha::AchreiMosKedoshim => "אחרי מות - קדושים",
            Parsha::BeharBechukosai => "בהר - בחקתי",
            Parsha::ChukasBalak => "חוקת - בלק",
            Parsha::MatosMasei => "מטות - מסעי",
            Parsha::NitzavimVayeilech => "נצבים - וילך",
            Parsha::Shekalim => "שקלים",
            Parsha::Zachor => "זכור",
            Parsha::Parah => "פרה",
            Parsha::Hachodesh => "החדש",
            Parsha::Shuva => "שובה",
            Parsha::Shira => "שירה",
            Parsha::Hagadol => "הגדול",
            Parsha::Chazon => "חזון",
            Parsha::Nachamu => "נחמו",
        }
    }
}

/// The name in English, for example "Bereshis" or "Vayakhel-Pekudei".
impl fmt::Display for Parsha {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Parsha::Bereshis => "Bereshis",
            Parsha::Noach => "Noach",
            Parsha::LechLecha => "Lech Lecha",
            Parsha::Vayera => "Vayera",
            Parsha::ChayeiSara => "Chayei Sara",
            Parsha::Toldos => "Toldos",
            Parsha::Vayetzei => "Vayetzei",
            Parsha::Vayishlach => "Vayishlach",
            Parsha::Vayeshev => "Vayeshev",
            Parsha::Miketz => "Miketz",
            Parsha::Vayigash => "Vayigash",
            Parsha::Vayechi => "Vayechi",
            Parsha::Shemos => "Shemos",
            Parsha::Vaera => "Vaera",
            Parsha::Bo => "Bo",
            Parsha::Beshalach => "Beshalach",
            Parsha::Yisro => "Yisro",
            Parsha::Mishpatim => "Mishpatim",
            Parsha::Terumah => "Terumah",
            Parsha::Tetzaveh => "Tetzaveh",
            Parsha::KiSisa => "Ki Sisa",
            Parsha::Vayakhel => "Vayakhel",
            Parsha::Pekudei => "Pekudei",
            Parsha::Vayikra => "Vayikra",
            Parsha::Tzav => "Tzav",
            Parsha::Shmini => "Shmini",
            Parsha::Tazria => "Tazria",
            Parsha::Metzora => "Metzora",
            Parsha::AchreiMos => "Achrei Mos",
            Parsha::Kedoshim => "Kedoshim",
            Parsha::Emor => "Emor",
            Parsha::Behar => "Behar",
            Parsha::Bechukosai => "Bechukosai",
            Parsha::Bamidbar => "Bamidbar",
            Parsha::Nasso => "Nasso",
            Parsha::Behaaloscha => "Behaaloscha",
            Parsha::Shlach => "Shlach",
            Parsha::Korach => "Korach",
            Parsha::Chukas => "Chukas",
            Parsha::Balak => "Balak",
            Parsha::Pinchas => "Pinchas",
            Parsha::Matos => "Matos",
            Parsha::Masei => "Masei",
            Parsha::Devarim => "Devarim",
            Parsha::Vaeschanan => "Vaeschanan",
            Parsha::Eikev => "Eikev",
            Parsha::Reeh => "Reeh",
            Parsha::Shoftim => "Shoftim",
            Parsha::KiSeitzei => "Ki Seitzei",
            Parsha::KiSavo => "Ki Savo",
            Parsha::Nitzavim => "Nitzavim",
            Parsha::Vayeilech => "Vayeilech",
            Parsha::HaAzinu => "Haazinu",
            Parsha::VezosHabracha => "Vezos Habracha",
            Parsha::VayakhelPekudei => "Vayakhel-Pekudei",
            Parsha::TazriaMetzora => "Tazria-Metzora",
            Parsha::AchreiMosKedoshim => "Achrei Mos-Kedoshim",
            Parsha::BeharBechukosai => "Behar-Bechukosai",
            Parsha::ChukasBalak => "Chukas-Balak",
            Parsha::MatosMasei => "Matos-Masei",
            Parsha::NitzavimVayeilech => "Nitzavim-Vayeilech",
            Parsha::Shekalim => "Shekalim",
            Parsha::Zachor => "Zachor",
            Parsha::Parah => "Parah",
            Parsha::Hachodesh => "Hachodesh",
            Parsha::Shuva => "Shuva",
            Parsha::Shira => "Shira",
            Parsha::Hagadol => "Hagadol",
            Parsha::Chazon => "Chazon",
            Parsha::Nachamu => "Nachamu",
        })
    }
}
