//! Everything the bindings export: the `calendar` module, with `limudim` inside it, and the
//! `times` module, with `myzmanim` inside it. Each has its own error type.
//!
//! WeaveFFI reads this module's source to generate the bindings, so every exported declaration is
//! written out here, inline. Method bodies are one-liners into the private `convert` module, which holds
//! the conversions to and from kosher-rust's types. Instants are milliseconds since the Unix epoch
//! and durations are milliseconds.

/// The Hebrew calendar: dates, holidays, the weekly *parsha*, years and the molad.
#[weaveffi::module]
pub mod calendar {
    use crate::convert;

    /// Why a calendar call failed.
    #[weaveffi::error]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(i32)]
    pub enum CalendarError {
        /// The date does not exist, or is outside Hebrew years 1 to 9999.
        InvalidDate = 1,
        /// A day of Chanukah outside 1 to 8, or of the Omer outside 1 to 49.
        InvalidSpecialDay = 2,
    }

    impl std::fmt::Display for CalendarError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(convert::error::message(*self))
        }
    }

    /// A day on the Gregorian calendar.
    #[weaveffi::record]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct GregorianDate {
        /// The year, for example 2024.
        pub year: i32,
        /// The month, 1 to 12.
        pub month: i32,
        /// The day of the month, from 1.
        pub day: i32,
    }

    /// Where the calendar is kept. Outside Israel, Yom Tov has a second day.
    #[weaveffi::enumeration]
    #[repr(i32)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum Region {
        /// Eretz Yisrael: one day of Yom Tov.
        Israel = 0,
        /// Everywhere else: two days of Yom Tov.
        Diaspora = 1,
    }

    /// A month of the Hebrew calendar, numbered from Tishrei. A leap year has two Adars:
    /// `AdarI` is the added month, and `Adar` is Adar II in a leap year.
    #[weaveffi::enumeration]
    #[repr(i32)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum HebrewMonth {
        /// Tishrei.
        Tishrei = 1,
        /// Cheshvan.
        Cheshvan = 2,
        /// Kislev.
        Kislev = 3,
        /// Teves.
        Teves = 4,
        /// Shevat.
        Shevat = 5,
        /// Adar I, only in a leap year.
        AdarI = 6,
        /// Adar, or Adar II in a leap year.
        Adar = 7,
        /// Nisan.
        Nisan = 8,
        /// Iyar.
        Iyar = 9,
        /// Sivan.
        Sivan = 10,
        /// Tammuz.
        Tammuz = 11,
        /// Av.
        Av = 12,
        /// Elul.
        Elul = 13,
    }

    /// A day of the week, numbered from Sunday as the Hebrew calendar counts.
    #[weaveffi::enumeration]
    #[repr(i32)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum Weekday {
        /// Sunday.
        Sunday = 1,
        /// Monday.
        Monday = 2,
        /// Tuesday.
        Tuesday = 3,
        /// Wednesday.
        Wednesday = 4,
        /// Thursday.
        Thursday = 5,
        /// Friday.
        Friday = 6,
        /// Shabbos.
        Shabbos = 7,
    }

    /// Whether Cheshvan and Kislev are long or short in a year.
    #[weaveffi::enumeration]
    #[repr(i32)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum YearLength {
        /// Deficient: Cheshvan and Kislev both have 29 days (353 or 383 days).
        Chaserim = 0,
        /// Regular: Cheshvan has 29 days and Kislev 30 (354 or 384 days).
        Kesidran = 1,
        /// Complete: Cheshvan and Kislev both have 30 days (355 or 385 days).
        Shelaimim = 2,
    }

    /// A weekly Torah portion, a double portion, or one of the special Shabbosos.
    #[weaveffi::enumeration]
    #[repr(i32)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum Parsha {
        /// Bereshis.
        Bereshis = 0,
        /// Noach.
        Noach = 1,
        /// Lech Lecha.
        LechLecha = 2,
        /// Vayera.
        Vayera = 3,
        /// Chayei Sara.
        ChayeiSara = 4,
        /// Toldos.
        Toldos = 5,
        /// Vayetzei.
        Vayetzei = 6,
        /// Vayishlach.
        Vayishlach = 7,
        /// Vayeshev.
        Vayeshev = 8,
        /// Miketz.
        Miketz = 9,
        /// Vayigash.
        Vayigash = 10,
        /// Vayechi.
        Vayechi = 11,
        /// Shemos.
        Shemos = 12,
        /// Vaera.
        Vaera = 13,
        /// Bo.
        Bo = 14,
        /// Beshalach.
        Beshalach = 15,
        /// Yisro.
        Yisro = 16,
        /// Mishpatim.
        Mishpatim = 17,
        /// Terumah.
        Terumah = 18,
        /// Tetzaveh.
        Tetzaveh = 19,
        /// Ki Sisa.
        KiSisa = 20,
        /// Vayakhel.
        Vayakhel = 21,
        /// Pekudei.
        Pekudei = 22,
        /// Vayikra.
        Vayikra = 23,
        /// Tzav.
        Tzav = 24,
        /// Shmini.
        Shmini = 25,
        /// Tazria.
        Tazria = 26,
        /// Metzora.
        Metzora = 27,
        /// Achrei Mos.
        AchreiMos = 28,
        /// Kedoshim.
        Kedoshim = 29,
        /// Emor.
        Emor = 30,
        /// Behar.
        Behar = 31,
        /// Bechukosai.
        Bechukosai = 32,
        /// Bamidbar.
        Bamidbar = 33,
        /// Nasso.
        Nasso = 34,
        /// Behaaloscha.
        Behaaloscha = 35,
        /// Shlach.
        Shlach = 36,
        /// Korach.
        Korach = 37,
        /// Chukas.
        Chukas = 38,
        /// Balak.
        Balak = 39,
        /// Pinchas.
        Pinchas = 40,
        /// Matos.
        Matos = 41,
        /// Masei.
        Masei = 42,
        /// Devarim.
        Devarim = 43,
        /// Vaeschanan.
        Vaeschanan = 44,
        /// Eikev.
        Eikev = 45,
        /// Re'eh.
        Reeh = 46,
        /// Shoftim.
        Shoftim = 47,
        /// Ki Seitzei.
        KiSeitzei = 48,
        /// Ki Savo.
        KiSavo = 49,
        /// Nitzavim.
        Nitzavim = 50,
        /// Vayeilech.
        Vayeilech = 51,
        /// Ha'Azinu.
        HaAzinu = 52,
        /// Vezos Habracha.
        VezosHabracha = 53,
        /// Vayakhel-Pekudei.
        VayakhelPekudei = 54,
        /// Tazria-Metzora.
        TazriaMetzora = 55,
        /// Achrei Mos-Kedoshim.
        AchreiMosKedoshim = 56,
        /// Behar-Bechukosai.
        BeharBechukosai = 57,
        /// Chukas-Balak.
        ChukasBalak = 58,
        /// Matos-Masei.
        MatosMasei = 59,
        /// Nitzavim-Vayeilech.
        NitzavimVayeilech = 60,
        /// Shabbos Shekalim.
        Shekalim = 61,
        /// Shabbos Zachor.
        Zachor = 62,
        /// Shabbos Parah.
        Parah = 63,
        /// Shabbos Hachodesh.
        Hachodesh = 64,
        /// Shabbos Shuva.
        Shuva = 65,
        /// Shabbos Shira.
        Shira = 66,
        /// Shabbos Hagadol.
        Hagadol = 67,
        /// Shabbos Chazon.
        Chazon = 68,
        /// Shabbos Nachamu.
        Nachamu = 69,
    }

    /// A holiday, fast, or other day the calendar marks. Some are not holidays in the halachic
    /// sense (Rosh Chodesh, Shabbos Mevarchim, the Omer count) but are listed because calendars
    /// show them.
    #[weaveffi::enumeration]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum SpecialDay {
        /// Erev Pesach, 14 Nisan.
        ErevPesach,
        /// Pesach.
        Pesach,
        /// Chol Hamoed Pesach.
        CholHamoedPesach,
        /// Pesach Sheni, 14 Iyar.
        PesachSheni,
        /// Erev Shavuos, 5 Sivan.
        ErevShavuos,
        /// Shavuos.
        Shavuos,
        /// The fast of 17 Tammuz.
        SeventeenthOfTammuz,
        /// Tishah B'Av.
        TishahBav,
        /// Tu B'Av, 15 Av.
        TuBav,
        /// Erev Rosh Hashana, 29 Elul.
        ErevRoshHashana,
        /// Rosh Hashana.
        RoshHashana,
        /// The Fast of Gedalyah.
        FastOfGedalyah,
        /// Erev Yom Kippur, 9 Tishrei.
        ErevYomKippur,
        /// Yom Kippur, 10 Tishrei.
        YomKippur,
        /// Erev Succos, 14 Tishrei.
        ErevSuccos,
        /// Succos.
        Succos,
        /// Chol Hamoed Succos.
        CholHamoedSuccos,
        /// Hoshana Rabbah, 21 Tishrei.
        HoshanaRabbah,
        /// Shemini Atzeres.
        SheminiAtzeres,
        /// Simchas Torah.
        SimchasTorah,
        /// Chanukah.
        Chanukah {
            /// The day of Chanukah, 1 to 8.
            day: i32,
        },
        /// The fast of 10 Teves.
        TenthOfTeves,
        /// Tu B'Shvat, 15 Shevat.
        TuBshvat,
        /// The Fast of Esther.
        FastOfEsther,
        /// Purim.
        Purim,
        /// Shushan Purim.
        ShushanPurim,
        /// Purim Katan, 14 Adar I.
        PurimKatan,
        /// Rosh Chodesh.
        RoshChodesh,
        /// Yom HaShoah.
        YomHaShoah,
        /// Yom Hazikaron.
        YomHazikaron,
        /// Yom Ha'atzmaut.
        YomHaatzmaut,
        /// Yom Yerushalayim.
        YomYerushalayim,
        /// Lag B'Omer, 18 Iyar.
        LagBomer,
        /// Shushan Purim Katan, 15 Adar I.
        ShushanPurimKatan,
        /// Isru Chag, the day after a festival.
        IsruChag,
        /// Yom Kippur Katan, the eve of Rosh Chodesh.
        YomKippurKatan,
        /// Behab, the Monday-Thursday-Monday fasts after Pesach and Succos.
        Behab,
        /// The Fast of the Firstborn.
        FastOfTheFirstborn,
        /// The Omer count.
        CountOfTheOmer {
            /// The day of the count, 1 to 49.
            day: i32,
        },
        /// Birchas Hachamah, every 28 years.
        BirchasHachamah,
        /// Shabbos Machar Hachodesh, the Shabbos before Rosh Chodesh.
        MacharHachodesh,
        /// Shabbos Mevarchim, the Shabbos when the coming month is announced.
        ShabbosMevarchim,
    }

    /// Facts about a Hebrew year.
    #[weaveffi::record]
    #[derive(Debug, Clone, PartialEq)]
    pub struct YearInfo {
        /// The year, for example 5785.
        pub year: i32,
        /// Whether it has 13 months.
        pub is_leap: bool,
        /// The number of days, 353 to 385.
        pub days: i32,
        /// Whether Cheshvan and Kislev are long or short.
        pub length: YearLength,
        /// Whether Cheshvan has 30 days.
        pub is_cheshvan_long: bool,
        /// Whether Kislev has 29 days.
        pub is_kislev_short: bool,
        /// The months in order, from Tishrei.
        pub months: Vec<HebrewMonth>,
    }

    /// A day on the Hebrew calendar: the civil day it mostly overlaps, since a Hebrew day
    /// begins at nightfall. Years 1 to 9999.
    #[weaveffi::interface]
    pub struct HebrewDate {
        pub(crate) inner: kosher_rust::HebrewDate,
    }

    impl HebrewDate {
        /// The date `day` `month` `year`, for example 15 Nisan 5785.
        ///
        /// # Errors
        ///
        /// `CalendarError::InvalidDate` when there is no such day: day 0, day 30 of a 29-day
        /// month, Adar I in a common year, or a year outside the supported range.
        pub fn new(year: i32, month: HebrewMonth, day: i32) -> Result<Self, CalendarError> {
            convert::calendar::hebrew_date(year, month, day).map(|inner| Self { inner })
        }

        /// The Hebrew date of a Gregorian date.
        ///
        /// # Errors
        ///
        /// `CalendarError::InvalidDate` when `date` does not exist, or falls outside Hebrew years 1
        /// to 9999.
        pub fn from_gregorian(date: GregorianDate) -> Result<Self, CalendarError> {
            convert::calendar::from_gregorian(&date).map(|inner| Self { inner })
        }

        /// The year, for example 5785.
        pub fn year(&self) -> i32 {
            self.inner.year().get()
        }

        /// The month.
        pub fn month(&self) -> HebrewMonth {
            self.inner.month().into()
        }

        /// The day of the month, from 1.
        pub fn day(&self) -> i32 {
            i32::from(self.inner.day())
        }

        /// The day of the year, from 1 on 1 Tishrei.
        pub fn day_of_year(&self) -> i32 {
            i32::from(self.inner.day_of_year())
        }

        /// The day of the week.
        pub fn weekday(&self) -> Weekday {
            self.inner.weekday().into()
        }

        /// The Gregorian date, if it is in the Gregorian calendar's supported range.
        pub fn to_gregorian(&self) -> Option<GregorianDate> {
            self.inner.to_gregorian().map(Into::into)
        }

        /// The date `days` later, or earlier when negative.
        ///
        /// # Errors
        ///
        /// `CalendarError::InvalidDate` outside Hebrew years 1 to 9999.
        pub fn add_days(&self, days: i32) -> Result<Self, CalendarError> {
            convert::calendar::add_days(self.inner, days).map(|inner| Self { inner })
        }

        /// The holidays, fasts and other marked days on this day.
        pub fn special_days(&self, region: Region) -> Vec<SpecialDay> {
            self.inner
                .special_days(region.into())
                .filter_map(|special_day| special_day.try_into().ok())
                .collect()
        }

        /// Whether melacha is forbidden: Shabbos or Yom Tov.
        pub fn is_assur_bemelacha(&self, region: Region) -> bool {
            self.inner.is_assur_bemelacha(region.into())
        }

        /// Whether candles are lit tonight for Shabbos or Yom Tov.
        pub fn has_candle_lighting(&self, region: Region) -> bool {
            self.inner.has_candle_lighting(region.into())
        }

        /// Whether this is one of the ten days from Rosh Hashana to Yom Kippur.
        pub fn is_aseres_yemei_teshuva(&self) -> bool {
            self.inner.is_aseres_yemei_teshuva()
        }

        /// The weekly portion read on this day, when it is a Shabbos with one.
        pub fn parsha(&self, region: Region) -> Option<Parsha> {
            self.inner.parsha(region.into()).map(Into::into)
        }

        /// The special Shabbos this is, such as Shabbos Zachor, if any.
        pub fn special_parsha(&self, region: Region) -> Option<Parsha> {
            self.inner.special_parsha(region.into()).map(Into::into)
        }

        /// The weekly portion of the coming Shabbos.
        pub fn next_parsha(&self, region: Region) -> Option<Parsha> {
            self.inner.next_parsha(region.into()).map(Into::into)
        }

        /// The molad of this month, in Unix milliseconds.
        pub fn molad(&self) -> Option<i64> {
            self.inner.molad().map(|molad| molad.timestamp().as_millisecond())
        }

        /// The date as text, for example "15 Nisan 5784".
        pub fn display(&self) -> String {
            self.inner.to_string()
        }

        /// Whether `other` is the same day.
        pub fn equals(&self, other: &HebrewDate) -> bool {
            self.inner == other.inner
        }

        /// -1, 0 or 1 as this date is before, the same as, or after `other`.
        pub fn compare(&self, other: &HebrewDate) -> i32 {
            self.inner.cmp(&other.inner) as i32
        }
    }

    /// Facts about the Hebrew year `year`.
    ///
    /// # Errors
    ///
    /// `CalendarError::InvalidDate` for a year outside 1 to 9999.
    #[weaveffi::export]
    pub fn year_info(year: i32) -> Result<YearInfo, CalendarError> {
        convert::calendar::year_info(year)
    }

    /// The number of days in `month` of `year`, or none when the year has no such month or is
    /// outside 1 to 9999.
    #[weaveffi::export]
    pub fn days_in_month(year: i32, month: HebrewMonth) -> Option<i32> {
        convert::calendar::days_in_month(year, month)
    }

    /// 1 Tishrei of `year`.
    ///
    /// # Errors
    ///
    /// `CalendarError::InvalidDate` for a year outside 1 to 9999.
    #[weaveffi::export]
    pub fn rosh_hashana(year: i32) -> Result<HebrewDate, CalendarError> {
        convert::calendar::rosh_hashana(year).map(|inner| HebrewDate { inner })
    }

    /// The day's English name, for example "Chanukah".
    ///
    /// # Errors
    ///
    /// `CalendarError::InvalidSpecialDay` for a day of Chanukah outside 1 to 8, or of the Omer
    /// outside 1 to 49.
    #[weaveffi::export]
    pub fn special_day_name(special_day: SpecialDay) -> Result<String, CalendarError> {
        kosher_rust::calendar::SpecialDay::try_from(special_day).map(|special_day| special_day.to_string())
    }

    /// The day's Hebrew name.
    ///
    /// # Errors
    ///
    /// `CalendarError::InvalidSpecialDay` for a day of Chanukah outside 1 to 8, or of the Omer
    /// outside 1 to 49.
    #[weaveffi::export]
    pub fn special_day_hebrew(special_day: SpecialDay) -> Result<String, CalendarError> {
        kosher_rust::calendar::SpecialDay::try_from(special_day).map(|special_day| special_day.hebrew().into())
    }

    /// Whether melacha is forbidden on the holiday.
    ///
    /// # Errors
    ///
    /// `CalendarError::InvalidSpecialDay` for a day of Chanukah outside 1 to 8, or of the Omer
    /// outside 1 to 49.
    #[weaveffi::export]
    pub fn special_day_is_assur_bemelacha(special_day: SpecialDay) -> Result<bool, CalendarError> {
        kosher_rust::calendar::SpecialDay::try_from(special_day).map(|special_day| special_day.is_assur_bemelacha())
    }

    /// Whether the holiday is a modern Israeli one, such as Yom Ha'atzmaut.
    ///
    /// # Errors
    ///
    /// `CalendarError::InvalidSpecialDay` for a day of Chanukah outside 1 to 8, or of the Omer
    /// outside 1 to 49.
    #[weaveffi::export]
    pub fn special_day_is_modern(special_day: SpecialDay) -> Result<bool, CalendarError> {
        kosher_rust::calendar::SpecialDay::try_from(special_day).map(|special_day| special_day.is_modern())
    }

    /// Whether the holiday is a fast day.
    ///
    /// # Errors
    ///
    /// `CalendarError::InvalidSpecialDay` for a day of Chanukah outside 1 to 8, or of the Omer
    /// outside 1 to 49.
    #[weaveffi::export]
    pub fn special_day_is_fast_day(special_day: SpecialDay) -> Result<bool, CalendarError> {
        kosher_rust::calendar::SpecialDay::try_from(special_day).map(|special_day| special_day.is_fast_day())
    }

    /// The portion's English name, for example "Lech Lecha".
    #[weaveffi::export]
    pub fn parsha_name(parsha: Parsha) -> String {
        kosher_rust::calendar::Parsha::from(parsha).to_string()
    }

    /// The portion's Hebrew name.
    #[weaveffi::export]
    pub fn parsha_hebrew(parsha: Parsha) -> String {
        kosher_rust::calendar::Parsha::from(parsha).hebrew().into()
    }

    /// The month's English name, for example "Nisan".
    #[weaveffi::export]
    pub fn month_name(month: HebrewMonth) -> String {
        kosher_rust::calendar::HebrewMonth::from(month).to_string()
    }

    /// The month's Hebrew name.
    #[weaveffi::export]
    pub fn month_hebrew(month: HebrewMonth) -> String {
        kosher_rust::calendar::HebrewMonth::from(month).hebrew().into()
    }

    /// Daily learning schedules: Daf Yomi, Mishna Yomis, Tehillim and the others.
    ///
    /// Each function gives the portion for a date, or none when nothing is scheduled (before the
    /// program began, on Shabbos for a weekly program, outside the Pirkei Avos season).
    #[weaveffi::module]
    pub mod limudim {
        use super::{HebrewDate, Region};
        use crate::convert;

        /// Why a learning-schedule call failed.
        #[weaveffi::error]
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        #[repr(i32)]
        pub enum LimudimError {
            /// A psalm outside 1 to 150, a verse outside 1 to 176, or a chapter of Pirkei Avos outside 1
            /// to 6.
            InvalidUnit = 1,
        }

        impl std::fmt::Display for LimudimError {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(convert::error::message(*self))
            }
        }

        /// A tractate of the Mishnah and Talmud, in the order of the Mishnah.
        #[weaveffi::enumeration]
        #[repr(i32)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum Tractate {
            /// Berachos.
            Berachos = 0,
            /// Peah.
            Peah = 1,
            /// Demai.
            Demai = 2,
            /// Kilayim.
            Kilayim = 3,
            /// Sheviis.
            Sheviis = 4,
            /// Terumos.
            Terumos = 5,
            /// Maasros.
            Maasros = 6,
            /// Maaser Sheni.
            MaaserSheni = 7,
            /// Chalah.
            Chalah = 8,
            /// Orlah.
            Orlah = 9,
            /// Bikurim.
            Bikurim = 10,
            /// Shabbos.
            Shabbos = 11,
            /// Eruvin.
            Eruvin = 12,
            /// Pesachim.
            Pesachim = 13,
            /// Shekalim.
            Shekalim = 14,
            /// Yoma.
            Yoma = 15,
            /// Sukkah.
            Sukkah = 16,
            /// Beitzah.
            Beitzah = 17,
            /// Rosh Hashanah.
            RoshHashanah = 18,
            /// Taanis.
            Taanis = 19,
            /// Megillah.
            Megillah = 20,
            /// Moed Katan.
            MoedKatan = 21,
            /// Chagigah.
            Chagigah = 22,
            /// Yevamos.
            Yevamos = 23,
            /// Kesubos.
            Kesubos = 24,
            /// Nedarim.
            Nedarim = 25,
            /// Nazir.
            Nazir = 26,
            /// Sotah.
            Sotah = 27,
            /// Gitin.
            Gitin = 28,
            /// Kiddushin.
            Kiddushin = 29,
            /// Bava Kamma.
            BavaKamma = 30,
            /// Bava Metzia.
            BavaMetzia = 31,
            /// Bava Basra.
            BavaBasra = 32,
            /// Sanhedrin.
            Sanhedrin = 33,
            /// Makkos.
            Makkos = 34,
            /// Shevuos.
            Shevuos = 35,
            /// Eduyos.
            Eduyos = 36,
            /// Avodah Zarah.
            AvodahZarah = 37,
            /// Avos.
            Avos = 38,
            /// Horiyos.
            Horiyos = 39,
            /// Zevachim.
            Zevachim = 40,
            /// Menachos.
            Menachos = 41,
            /// Chullin.
            Chullin = 42,
            /// Bechoros.
            Bechoros = 43,
            /// Arachin.
            Arachin = 44,
            /// Temurah.
            Temurah = 45,
            /// Kerisos.
            Kerisos = 46,
            /// Meilah.
            Meilah = 47,
            /// Tamid.
            Tamid = 48,
            /// Midos.
            Midos = 49,
            /// Kinnim.
            Kinnim = 50,
            /// Keilim.
            Keilim = 51,
            /// Ohalos.
            Ohalos = 52,
            /// Negaim.
            Negaim = 53,
            /// Parah.
            Parah = 54,
            /// Taharos.
            Taharos = 55,
            /// Mikvaos.
            Mikvaos = 56,
            /// Niddah.
            Niddah = 57,
            /// Machshirin.
            Machshirin = 58,
            /// Zavim.
            Zavim = 59,
            /// Tevul Yom.
            TevulYom = 60,
            /// Yadayim.
            Yadayim = 61,
            /// Uktzin.
            Uktzin = 62,
        }

        /// A side of a page: *amud aleph* (a) or *amud beis* (b).
        #[weaveffi::enumeration]
        #[repr(i32)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum Side {
            /// The front of the page, "a".
            Aleph = 0,
            /// The back of the page, "b".
            Bet = 1,
        }

        /// A page (*daf*) of the Talmud: both sides of one leaf.
        #[weaveffi::record]
        #[derive(Debug, Clone, PartialEq)]
        pub struct Daf {
            /// The tractate.
            pub tractate: Tractate,
            /// The page number. The Bavli starts at 2, the Yerushalmi at 1.
            pub number: i32,
            /// As text, for example "Bava Metzia 55".
            pub display: String,
        }

        /// One side of a page (*amud*).
        #[weaveffi::record]
        #[derive(Debug, Clone, PartialEq)]
        pub struct Amud {
            /// The tractate.
            pub tractate: Tractate,
            /// The page number.
            pub number: i32,
            /// Which side of the page.
            pub side: Side,
            /// As text, for example "Bava Metzia 55b".
            pub display: String,
        }

        /// One *mishnah*.
        #[weaveffi::record]
        #[derive(Debug, Clone, PartialEq)]
        pub struct Mishna {
            /// The tractate.
            pub tractate: Tractate,
            /// The chapter (*perek*), from 1.
            pub chapter: i32,
            /// The *mishnah* within the chapter, from 1.
            pub number: i32,
            /// As text, for example "Nazir 5:5".
            pub display: String,
        }

        /// The two *mishnayos* of a day of Mishna Yomis.
        #[weaveffi::record]
        #[derive(Debug, Clone, PartialEq)]
        pub struct MishnaPair {
            /// The first *mishnah*.
            pub first: Mishna,
            /// The second, the next *mishnah* in order.
            pub second: Mishna,
            /// As text, for example "Nazir 5:5-6".
            pub display: String,
        }

        /// A day's portion of Tehillim.
        #[weaveffi::enumeration]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum TehillimUnit {
            /// Whole chapters, `start` to `end` inclusive.
            Psalms {
                /// The first chapter.
                start: i32,
                /// The last chapter.
                end: i32,
            },
            /// Part of one long chapter (119, split over two days).
            PsalmVerses {
                /// The chapter.
                psalm: i32,
                /// The first verse.
                start_verse: i32,
                /// The last verse.
                end_verse: i32,
            },
        }

        /// The Pirkei Avos read on a Shabbos afternoon.
        #[weaveffi::enumeration]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum PirkeiAvosUnit {
            /// One chapter.
            Single {
                /// The chapter, 1 to 6.
                chapter: i32,
            },
            /// Two chapters read together, at the end of the summer.
            Combined {
                /// The first chapter.
                first: i32,
                /// The second chapter.
                second: i32,
            },
        }

        /// The tractate's English name, for example "Bava Metzia".
        #[weaveffi::export]
        pub fn tractate_name(tractate: Tractate) -> String {
            kosher_rust::limudim::Tractate::from(tractate).to_string()
        }

        /// The tractate's Hebrew name.
        #[weaveffi::export]
        pub fn tractate_hebrew(tractate: Tractate) -> String {
            kosher_rust::limudim::Tractate::from(tractate).hebrew().into()
        }

        /// The portion as text, for example "Tehillim 1-9".
        ///
        /// # Errors
        ///
        /// `LimudimError::InvalidUnit` for a psalm outside 1 to 150 or a verse outside 1 to 176.
        #[weaveffi::export]
        pub fn tehillim_display(unit: TehillimUnit) -> Result<String, LimudimError> {
            kosher_rust::limudim::TehillimUnit::try_from(unit).map(|unit| unit.to_string())
        }

        /// The portion as text, for example "Pirkei Avos 5-6".
        ///
        /// # Errors
        ///
        /// `LimudimError::InvalidUnit` for a chapter outside 1 to 6.
        #[weaveffi::export]
        pub fn pirkei_avos_display(unit: PirkeiAvosUnit) -> Result<String, LimudimError> {
            kosher_rust::limudim::PirkeiAvosUnit::try_from(unit).map(|unit| unit.to_string())
        }

        /// Daf Yomi: a page of the Babylonian Talmud a day, from 11 September 1923.
        #[weaveffi::export]
        pub fn daf_yomi_bavli(date: &HebrewDate) -> Option<Daf> {
            convert::limudim::on(kosher_rust::limudim::DafYomiBavli, date)
        }

        /// Daf Hashavua: a page of the Babylonian Talmud a week.
        #[weaveffi::export]
        pub fn daf_hashavua_bavli(date: &HebrewDate) -> Option<Daf> {
            convert::limudim::on(kosher_rust::limudim::DafHashavuaBavli, date)
        }

        /// Daf Yomi Yerushalmi: a page of the Jerusalem Talmud a day.
        #[weaveffi::export]
        pub fn daf_yomi_yerushalmi(date: &HebrewDate) -> Option<Daf> {
            convert::limudim::on(kosher_rust::limudim::DafYomiYerushalmi, date)
        }

        /// Dirshu's Amud Yomi: one side of a page of the Babylonian Talmud a day.
        #[weaveffi::export]
        pub fn amud_yomi_dirshu(date: &HebrewDate) -> Option<Amud> {
            convert::limudim::on(kosher_rust::limudim::AmudYomiDirshu, date)
        }

        /// Mishna Yomis: two *mishnayos* a day, from 20 May 1947.
        #[weaveffi::export]
        pub fn mishna_yomis(date: &HebrewDate) -> Option<MishnaPair> {
            convert::limudim::on(kosher_rust::limudim::MishnaYomis, date)
        }

        /// Tehillim divided over the days of the Hebrew month.
        #[weaveffi::export]
        pub fn tehillim_monthly(date: &HebrewDate) -> Option<TehillimUnit> {
            convert::limudim::on(kosher_rust::limudim::TehillimMonthly, date)
        }

        /// Pirkei Avos: a chapter each Shabbos afternoon from Pesach to Rosh Hashana.
        #[weaveffi::export]
        pub fn pirkei_avos(date: &HebrewDate, region: Region) -> Option<PirkeiAvosUnit> {
            convert::limudim::on(kosher_rust::limudim::PirkeiAvos(region.into()), date)
        }
    }
}

/// Halachic times (*zmanim*) for a place and a day.
#[weaveffi::module]
pub mod times {
    use std::sync::Arc;

    use crate::api::calendar::GregorianDate;
    use crate::convert;

    /// Why a *zmanim* call failed.
    #[weaveffi::error]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(i32)]
    pub enum ZmanimError {
        /// The latitude is not between -90 and 90 degrees.
        InvalidLatitude = 1,
        /// The longitude is not between -180 and 180 degrees.
        InvalidLongitude = 2,
        /// An elevation is negative or not a finite number.
        InvalidElevation = 3,
        /// The Gregorian date does not exist, or is outside the supported range.
        InvalidGregorianDate = 4,
        /// A time zone is needed and none was given. The four Kiddush Levana times need one.
        /// `Zmanim` and `MyZmanim` also need one at a location more than 150 degrees from
        /// Greenwich.
        TimeZoneRequired = 5,
        /// The time zone callback failed, or gave an offset of a day or more.
        TimeZoneCallbackFailed = 6,
        /// A number of minutes, for candle lighting or Tzais Ateret Torah, is not a finite number
        /// within a day (1440 minutes) either way.
        InvalidMinutes = 7,
        /// An angle in degrees is not a finite number.
        InvalidDegrees = 8,
        /// The sun stays above the angle the time needs all day, as in the far north in summer.
        AllDay = 10,
        /// The sun stays below the angle the time needs all day, as in the far north in winter.
        AllNight = 11,
        /// The time only exists on certain dates, and this is not one.
        NotApplicable = 12,
        /// A calculation failed in a way that should not happen; please report the location and
        /// date.
        Unexpected = 13,
    }

    impl std::fmt::Display for ZmanimError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(convert::error::message(*self))
        }
    }

    /// A place on earth: where to calculate *zmanim* for. Heights are in meters above sea level;
    /// leave them 0 for sea level.
    ///
    /// The time zone is not part of the location. You pass a `TimeZoneOffset` to `Zmanim` and
    /// `MyZmanim` when you create them.
    #[weaveffi::record]
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Location {
        /// Latitude in degrees, north positive.
        pub latitude: f64,
        /// Longitude in degrees, east positive.
        pub longitude: f64,
        /// The observer's height. Elevation makes sunrise earlier and sunset later.
        pub elevation: f64,
        /// The height of the eastern horizon, over which the sun rises.
        pub eastern_horizon_elevation: f64,
        /// The height of the western horizon, behind which the sun sets.
        pub western_horizon_elevation: f64,
    }

    /// A location at sea level, with flat horizons.
    #[weaveffi::export]
    pub fn sea_level_location(latitude: f64, longitude: f64) -> Location {
        Location {
            latitude,
            longitude,
            elevation: 0.0,
            eastern_horizon_elevation: 0.0,
            western_horizon_elevation: 0.0,
        }
    }

    /// Checks a location the way `Zmanim` will.
    ///
    /// # Errors
    ///
    /// `ZmanimError::InvalidLatitude`, `ZmanimError::InvalidLongitude` or
    /// `ZmanimError::InvalidElevation`; `ZmanimError::TimeZoneRequired` beyond 150 degrees
    /// from Greenwich when `has_time_zone` is false.
    #[weaveffi::export]
    pub fn validate_location(location: Location, has_time_zone: bool) -> Result<(), ZmanimError> {
        convert::validate_location(&location, has_time_zone)
    }

    /// The time zone. You supply it as a callback with two methods. One gives the offset from UTC
    /// at an instant. The other gives the instant of a local date and time. Java's `ZoneId`, .NET's
    /// `TimeZoneInfo`, Python's `zoneinfo` and JavaScript's `Temporal` can answer both.
    ///
    /// The time zone is optional. With a time zone every time works. You can also leave it out.
    /// These do not work without a time zone:
    ///
    /// - The four Kiddush Levana times return `ZmanimError::TimeZoneRequired`.
    /// - `Zmanim` and `MyZmanim` cannot be created for a location more than 150 degrees from
    ///   Greenwich. Creating them returns `ZmanimError::TimeZoneRequired`.
    ///
    /// The callback is called synchronously, once for each sunrise, sunset or other event that a
    /// time is built from.
    #[weaveffi::callback_interface]
    pub trait TimeZoneOffset: Send + Sync {
        /// The offset from UTC, in seconds east, in effect at `unix_seconds`.
        ///
        /// # Errors
        ///
        /// Fail when the offset is unknown. The calculation then reports
        /// `ZmanimError::TimeZoneCallbackFailed`.
        fn offset_seconds(&self, unix_seconds: i64) -> Result<i32, weaveffi::ForeignError>;

        /// The Unix time, in seconds, at which the clocks in this time zone show the local date
        /// and time given.
        ///
        /// Resolve it the way Java's `ZonedDateTime.of` does. When the clocks skip over the time,
        /// return the instant one gap length later. When the clocks show the time twice, return
        /// the earlier instant. JavaScript's `Temporal` calls this the `compatible`
        /// disambiguation, and it is Temporal's default.
        ///
        /// # Errors
        ///
        /// Fail when the instant is unknown. The calculation then reports
        /// `ZmanimError::TimeZoneCallbackFailed`.
        fn local_to_unix_seconds(
            &self,
            year: i32,
            month: i32,
            day: i32,
            hour: i32,
            minute: i32,
            second: i32,
        ) -> Result<i64, weaveffi::ForeignError>;
    }

    // BEGIN GENERATED: Zman
    /// A halachic time this library can calculate, named after the KosherJava getter that
    /// calculates it, without `get`. Its value is its position in `all_zmanim`.
    #[weaveffi::enumeration]
    #[repr(i32)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum Zman {
        /// Begin Civil Twilight (KosherJava `getBeginCivilTwilight`).
        BeginCivilTwilight = 0,
        /// End Civil Twilight (KosherJava `getEndCivilTwilight`).
        EndCivilTwilight = 1,
        /// Begin Nautical Twilight (KosherJava `getBeginNauticalTwilight`).
        BeginNauticalTwilight = 2,
        /// End Nautical Twilight (KosherJava `getEndNauticalTwilight`).
        EndNauticalTwilight = 3,
        /// Begin Astronomical Twilight (KosherJava `getBeginAstronomicalTwilight`).
        BeginAstronomicalTwilight = 4,
        /// End Astronomical Twilight (KosherJava `getEndAstronomicalTwilight`).
        EndAstronomicalTwilight = 5,
        /// Alos (60 Minutes) (KosherJava `getAlos60Minutes`).
        Alos60Minutes = 6,
        /// Alos (72 Minutes) (KosherJava `getAlos72Minutes`).
        Alos72Minutes = 7,
        /// Alos (72 Minutes in Shaos Zmanios) (KosherJava `getAlos72Zmanis`).
        Alos72Zmanis = 8,
        /// Alos (90 Minutes) (KosherJava `getAlos90Minutes`).
        Alos90Minutes = 9,
        /// Alos (90 Minutes in Shaos Zmanios) (KosherJava `getAlos90Zmanis`).
        Alos90Zmanis = 10,
        /// Alos (96 Minutes) (KosherJava `getAlos96Minutes`).
        Alos96Minutes = 11,
        /// Alos (96 Minutes in Shaos Zmanios) (KosherJava `getAlos96Zmanis`).
        Alos96Zmanis = 12,
        /// Alos (120 Minutes) (KosherJava `getAlos120Minutes`).
        ///
        /// Deprecated in KosherJava.
        Alos120Minutes = 13,
        /// Alos (120 Minutes in Shaos Zmanios) (KosherJava `getAlos120Zmanis`).
        ///
        /// Deprecated in KosherJava.
        Alos120Zmanis = 14,
        /// Alos (16.1 Degrees) (KosherJava `getAlos16Point1Degrees`).
        Alos16Point1Degrees = 15,
        /// Alos (18 Degrees) (KosherJava `getAlos18Degrees`).
        Alos18Degrees = 16,
        /// Alos (19 Degrees) (KosherJava `getAlos19Degrees`).
        Alos19Degrees = 17,
        /// Alos (19.8 Degrees) (KosherJava `getAlos19Point8Degrees`).
        Alos19Point8Degrees = 18,
        /// Alos (26 Degrees) (KosherJava `getAlos26Degrees`).
        ///
        /// Deprecated in KosherJava.
        Alos26Degrees = 19,
        /// Alos (Baal Hatanya) (KosherJava `getAlosBaalHatanya`).
        AlosBaalHatanya = 20,
        /// Misheyakir (10.2 Degrees) (KosherJava `getMisheyakir10Point2Degrees`).
        Misheyakir10Point2Degrees = 21,
        /// Misheyakir (11 Degrees) (KosherJava `getMisheyakir11Degrees`).
        Misheyakir11Degrees = 22,
        /// Misheyakir (11.5 Degrees) (KosherJava `getMisheyakir11Point5Degrees`).
        Misheyakir11Point5Degrees = 23,
        /// Misheyakir (12.85 Degrees) (KosherJava `getMisheyakir12Point85Degrees`).
        ///
        /// Deprecated in KosherJava.
        Misheyakir12Point85Degrees = 24,
        /// Misheyakir (7.65 Degrees) (KosherJava `getMisheyakir7Point65Degrees`).
        Misheyakir7Point65Degrees = 25,
        /// Misheyakir (9.5 Degrees) (KosherJava `getMisheyakir9Point5Degrees`).
        Misheyakir9Point5Degrees = 26,
        /// Polar Sunrise Ben Ish Chai (KosherJava `getPolarSunriseBenIshChai`).
        PolarSunriseBenIshChai = 27,
        /// Sunrise (KosherJava `getSunrise`).
        Sunrise = 28,
        /// Sea Level Sunrise (KosherJava `getSeaLevelSunrise`).
        SeaLevelSunrise = 29,
        /// Sof Zman Shma (GR'A) (KosherJava `getSofZmanShmaGRA`).
        SofZmanShmaGra = 30,
        /// Sof Zman Shma (MGA, 19.8 Degrees) (KosherJava `getSofZmanShmaMGA19Point8Degrees`).
        SofZmanShmaMga19Point8Degrees = 31,
        /// Sof Zman Shma (MGA, 16.1 Degrees) (KosherJava `getSofZmanShmaMGA16Point1Degrees`).
        SofZmanShmaMga16Point1Degrees = 32,
        /// Sof Zman Shma (MGA, 18 Degrees) (KosherJava `getSofZmanShmaMGA18Degrees`).
        SofZmanShmaMga18Degrees = 33,
        /// Sof Zman Shma (MGA, 72 Minutes) (KosherJava `getSofZmanShmaMGA72Minutes`).
        SofZmanShmaMga72Minutes = 34,
        /// Sof Zman Shma (MGA, 72 Minutes in Shaos Zmanios) (KosherJava `getSofZmanShmaMGA72MinutesZmanis`).
        SofZmanShmaMga72MinutesZmanis = 35,
        /// Sof Zman Shma (MGA, 90 Minutes) (KosherJava `getSofZmanShmaMGA90Minutes`).
        SofZmanShmaMga90Minutes = 36,
        /// Sof Zman Shma (MGA, 90 Minutes in Shaos Zmanios) (KosherJava `getSofZmanShmaMGA90MinutesZmanis`).
        SofZmanShmaMga90MinutesZmanis = 37,
        /// Sof Zman Shma (MGA, 96 Minutes) (KosherJava `getSofZmanShmaMGA96Minutes`).
        SofZmanShmaMga96Minutes = 38,
        /// Sof Zman Shma (MGA, 96 Minutes in Shaos Zmanios) (KosherJava `getSofZmanShmaMGA96MinutesZmanis`).
        SofZmanShmaMga96MinutesZmanis = 39,
        /// Sof Zman Shma (3 Hours Before Chatzos) (KosherJava `getSofZmanShma3HoursBeforeChatzos`).
        SofZmanShma3HoursBeforeChatzos = 40,
        /// Sof Zman Shma (MGA, 120 Minutes) (KosherJava `getSofZmanShmaMGA120Minutes`).
        SofZmanShmaMga120Minutes = 41,
        /// Sof Zman Shma (Alos 16.1 to Sunset) (KosherJava `getSofZmanShmaAlos16Point1ToSunset`).
        SofZmanShmaAlos16Point1ToSunset = 42,
        /// Sof Zman Shma (Alos 16.1 to Tzais Geonim 7.083 Degrees) (KosherJava `getSofZmanShmaAlos16Point1DegreesToTzaisGeonim7Point083Degrees`).
        SofZmanShmaAlos16Point1DegreesToTzaisGeonim7Point083Degrees = 43,
        /// Sof Zman Shma (Ateret Torah) (KosherJava `getSofZmanShmaAteretTorah`).
        SofZmanShmaAteretTorah = 44,
        /// Sof Zman Shma (Baal Hatanya) (KosherJava `getSofZmanShmaBaalHatanya`).
        SofZmanShmaBaalHatanya = 45,
        /// Sof Zman Shma (GR'A, Sunrise to Fixed Local Chatzos) (KosherJava `getSofZmanShmaGRASunriseToFixedLocalChatzos`).
        SofZmanShmaGraSunriseToFixedLocalChatzos = 46,
        /// Sof Zman Shma (MGA, 18 Degrees to Fixed Local Chatzos) (KosherJava `getSofZmanShmaMGA18DegreesToFixedLocalChatzos`).
        SofZmanShmaMga18DegreesToFixedLocalChatzos = 47,
        /// Sof Zman Shma (MGA, 16.1 Degrees to Fixed Local Chatzos) (KosherJava `getSofZmanShmaMGA16Point1DegreesToFixedLocalChatzos`).
        SofZmanShmaMga16Point1DegreesToFixedLocalChatzos = 48,
        /// Sof Zman Shma (MGA, 90 Minutes to Fixed Local Chatzos) (KosherJava `getSofZmanShmaMGA90MinutesToFixedLocalChatzos`).
        SofZmanShmaMga90MinutesToFixedLocalChatzos = 49,
        /// Sof Zman Shma (MGA, 72 Minutes to Fixed Local Chatzos) (KosherJava `getSofZmanShmaMGA72MinutesToFixedLocalChatzos`).
        SofZmanShmaMga72MinutesToFixedLocalChatzos = 50,
        /// Sof Zman Tfila (GR'A) (KosherJava `getSofZmanTfilaGRA`).
        SofZmanTfilaGra = 51,
        /// Sof Zman Tfila (MGA, 72 Minutes) (KosherJava `getSofZmanTfilaMGA72Minutes`).
        SofZmanTfilaMga72Minutes = 52,
        /// Sof Zman Tfila (MGA, 19.8 Degrees) (KosherJava `getSofZmanTfilaMGA19Point8Degrees`).
        SofZmanTfilaMga19Point8Degrees = 53,
        /// Sof Zman Tfila (MGA, 16.1 Degrees) (KosherJava `getSofZmanTfilaMGA16Point1Degrees`).
        SofZmanTfilaMga16Point1Degrees = 54,
        /// Sof Zman Tfila (MGA, 18 Degrees) (KosherJava `getSofZmanTfilaMGA18Degrees`).
        SofZmanTfilaMga18Degrees = 55,
        /// Sof Zman Tfila (MGA, 72 Minutes in Shaos Zmanios) (KosherJava `getSofZmanTfilaMGA72MinutesZmanis`).
        SofZmanTfilaMga72MinutesZmanis = 56,
        /// Sof Zman Tfila (MGA, 90 Minutes) (KosherJava `getSofZmanTfilaMGA90Minutes`).
        SofZmanTfilaMga90Minutes = 57,
        /// Sof Zman Tfila (MGA, 90 Minutes in Shaos Zmanios) (KosherJava `getSofZmanTfilaMGA90MinutesZmanis`).
        SofZmanTfilaMga90MinutesZmanis = 58,
        /// Sof Zman Tfila (MGA, 96 Minutes) (KosherJava `getSofZmanTfilaMGA96Minutes`).
        SofZmanTfilaMga96Minutes = 59,
        /// Sof Zman Tfila (MGA, 96 Minutes in Shaos Zmanios) (KosherJava `getSofZmanTfilaMGA96MinutesZmanis`).
        SofZmanTfilaMga96MinutesZmanis = 60,
        /// Sof Zman Tfila (2 Hours Before Chatzos) (KosherJava `getSofZmanTfila2HoursBeforeChatzos`).
        SofZmanTfila2HoursBeforeChatzos = 61,
        /// Sof Zman Tfila (MGA, 120 Minutes) (KosherJava `getSofZmanTfilaMGA120Minutes`).
        SofZmanTfilaMga120Minutes = 62,
        /// Sof Zman Tfila (Ateret Torah) (KosherJava `getSofZmanTfilaAteretTorah`).
        SofZmanTfilaAteretTorah = 63,
        /// Sof Zman Tfila (Baal Hatanya) (KosherJava `getSofZmanTfilaBaalHatanya`).
        SofZmanTfilaBaalHatanya = 64,
        /// Sof Zman Tfila (GR'A, Sunrise to Fixed Local Chatzos) (KosherJava `getSofZmanTfilaGRASunriseToFixedLocalChatzos`).
        SofZmanTfilaGraSunriseToFixedLocalChatzos = 65,
        /// Sof Zman Achilas Chametz (GR'A) (KosherJava `getSofZmanAchilasChametzGRA`).
        SofZmanAchilasChametzGra = 66,
        /// Sof Zman Achilas Chametz (MGA, 72 Minutes) (KosherJava `getSofZmanAchilasChametzMGA72Minutes`).
        SofZmanAchilasChametzMga72Minutes = 67,
        /// Sof Zman Achilas Chametz (MGA, 72 Minutes in Shaos Zmanios) (KosherJava `getSofZmanAchilasChametzMGA72MinutesZmanis`).
        SofZmanAchilasChametzMga72MinutesZmanis = 68,
        /// Sof Zman Achilas Chametz (MGA, 16.1 Degrees) (KosherJava `getSofZmanAchilasChametzMGA16Point1Degrees`).
        SofZmanAchilasChametzMga16Point1Degrees = 69,
        /// Sof Zman Achilas Chametz (Baal Hatanya) (KosherJava `getSofZmanAchilasChametzBaalHatanya`).
        SofZmanAchilasChametzBaalHatanya = 70,
        /// Sof Zman Biur Chametz (GR'A) (KosherJava `getSofZmanBiurChametzGRA`).
        SofZmanBiurChametzGra = 71,
        /// Sof Zman Biur Chametz (MGA, 72 Minutes) (KosherJava `getSofZmanBiurChametzMGA72Minutes`).
        SofZmanBiurChametzMga72Minutes = 72,
        /// Sof Zman Biur Chametz (MGA, 72 Minutes in Shaos Zmanios) (KosherJava `getSofZmanBiurChametzMGA72MinutesZmanis`).
        SofZmanBiurChametzMga72MinutesZmanis = 73,
        /// Sof Zman Biur Chametz (MGA, 16.1 Degrees) (KosherJava `getSofZmanBiurChametzMGA16Point1Degrees`).
        SofZmanBiurChametzMga16Point1Degrees = 74,
        /// Sof Zman Biur Chametz (Baal Hatanya) (KosherJava `getSofZmanBiurChametzBaalHatanya`).
        SofZmanBiurChametzBaalHatanya = 75,
        /// Solar Transit (KosherJava `getSunTransit`).
        SunTransit = 76,
        /// Chatzos Hayom (KosherJava `getChatzosHayom`).
        ChatzosHayom = 77,
        /// Chatzos Hayom (Midpoint of Sunrise and Sunset) (KosherJava `getChatzosHayomAsHalfDay`).
        ChatzosHayomAsHalfDay = 78,
        /// Chatzos Hayom (Fixed Local Chatzos) (KosherJava `getFixedLocalChatzosHayom`).
        FixedLocalChatzosHayom = 79,
        /// Mincha Gedola (GR'A) (KosherJava `getMinchaGedolaGRA`).
        MinchaGedolaGra = 80,
        /// Mincha Gedola (16.1 Degrees) (KosherJava `getMinchaGedola16Point1Degrees`).
        MinchaGedola16Point1Degrees = 81,
        /// Mincha Gedola (30 Minutes) (KosherJava `getMinchaGedola30Minutes`).
        MinchaGedola30Minutes = 82,
        /// Mincha Gedola (72 Minutes) (KosherJava `getMinchaGedola72Minutes`).
        MinchaGedola72Minutes = 83,
        /// Mincha Gedola (Ahavat Shalom) (KosherJava `getMinchaGedolaAhavatShalom`).
        MinchaGedolaAhavatShalom = 84,
        /// Mincha Gedola (GR'A, Greater Than 30) (KosherJava `getMinchaGedolaGRAGreaterThan30`).
        MinchaGedolaGraGreaterThan30 = 85,
        /// Mincha Gedola (Ateret Torah) (KosherJava `getMinchaGedolaAteretTorah`).
        MinchaGedolaAteretTorah = 86,
        /// Mincha Gedola (Baal Hatanya) (KosherJava `getMinchaGedolaBaalHatanya`).
        MinchaGedolaBaalHatanya = 87,
        /// Mincha Gedola (GR'A, Fixed Local Chatzos, 30 Minutes) (KosherJava `getMinchaGedolaGRAFixedLocalChatzos30Minutes`).
        MinchaGedolaGraFixedLocalChatzos30Minutes = 88,
        /// Plag Hamincha (GR'A) (KosherJava `getPlagHaminchaGRA`).
        PlagHaminchaGra = 89,
        /// Plag Hamincha (Ahavat Shalom) (KosherJava `getPlagAhavatShalom`).
        PlagAhavatShalom = 90,
        /// Plag Hamincha (Alos 16.1 to Tzais Geonim 7.083 Degrees) (KosherJava `getPlagAlos16Point1DegreesToTzaisGeonim7Point083Degrees`).
        PlagAlos16Point1DegreesToTzaisGeonim7Point083Degrees = 91,
        /// Plag Hamincha (Alos 16.1 to Sunset) (KosherJava `getPlagAlosToSunset`).
        ///
        /// Deprecated in KosherJava.
        PlagAlosToSunset = 92,
        /// Plag Hamincha (60 Minutes) (KosherJava `getPlagHamincha60Minutes`).
        PlagHamincha60Minutes = 93,
        /// Plag Hamincha (72 Minutes) (KosherJava `getPlagHamincha72Minutes`).
        ///
        /// Deprecated in KosherJava.
        PlagHamincha72Minutes = 94,
        /// Plag Hamincha (72 Minutes in Shaos Zmanios) (KosherJava `getPlagHamincha72MinutesZmanis`).
        ///
        /// Deprecated in KosherJava.
        PlagHamincha72MinutesZmanis = 95,
        /// Plag Hamincha (90 Minutes) (KosherJava `getPlagHamincha90Minutes`).
        ///
        /// Deprecated in KosherJava.
        PlagHamincha90Minutes = 96,
        /// Plag Hamincha (90 Minutes in Shaos Zmanios) (KosherJava `getPlagHamincha90MinutesZmanis`).
        ///
        /// Deprecated in KosherJava.
        PlagHamincha90MinutesZmanis = 97,
        /// Plag Hamincha (96 Minutes) (KosherJava `getPlagHamincha96Minutes`).
        ///
        /// Deprecated in KosherJava.
        PlagHamincha96Minutes = 98,
        /// Plag Hamincha (96 Minutes in Shaos Zmanios) (KosherJava `getPlagHamincha96MinutesZmanis`).
        ///
        /// Deprecated in KosherJava.
        PlagHamincha96MinutesZmanis = 99,
        /// Plag Hamincha (120 Minutes) (KosherJava `getPlagHamincha120Minutes`).
        ///
        /// Deprecated in KosherJava.
        PlagHamincha120Minutes = 100,
        /// Plag Hamincha (120 Minutes in Shaos Zmanios) (KosherJava `getPlagHamincha120MinutesZmanis`).
        ///
        /// Deprecated in KosherJava.
        PlagHamincha120MinutesZmanis = 101,
        /// Plag Hamincha (16.1 Degrees) (KosherJava `getPlagHamincha16Point1Degrees`).
        ///
        /// Deprecated in KosherJava.
        PlagHamincha16Point1Degrees = 102,
        /// Plag Hamincha (18 Degrees) (KosherJava `getPlagHamincha18Degrees`).
        ///
        /// Deprecated in KosherJava.
        PlagHamincha18Degrees = 103,
        /// Plag Hamincha (19.8 Degrees) (KosherJava `getPlagHamincha19Point8Degrees`).
        ///
        /// Deprecated in KosherJava.
        PlagHamincha19Point8Degrees = 104,
        /// Plag Hamincha (26 Degrees) (KosherJava `getPlagHamincha26Degrees`).
        ///
        /// Deprecated in KosherJava.
        PlagHamincha26Degrees = 105,
        /// Plag Hamincha (Ateret Torah) (KosherJava `getPlagHaminchaAteretTorah`).
        PlagHaminchaAteretTorah = 106,
        /// Plag Hamincha (Baal Hatanya) (KosherJava `getPlagHaminchaBaalHatanya`).
        PlagHaminchaBaalHatanya = 107,
        /// Plag Hamincha (GR'A, Fixed Local Chatzos to Sunset) (KosherJava `getPlagHaminchaGRAFixedLocalChatzosToSunset`).
        PlagHaminchaGraFixedLocalChatzosToSunset = 108,
        /// Samuch Le Mincha Ketana (GR'A) (KosherJava `getSamuchLeMinchaKetanaGRA`).
        SamuchLeMinchaKetanaGra = 109,
        /// Samuch Le Mincha Ketana (16.1 Degrees) (KosherJava `getSamuchLeMinchaKetana16Point1Degrees`).
        SamuchLeMinchaKetana16Point1Degrees = 110,
        /// Samuch Le Mincha Ketana (72 Minutes) (KosherJava `getSamuchLeMinchaKetana72Minutes`).
        SamuchLeMinchaKetana72Minutes = 111,
        /// Mincha Ketana (GR'A) (KosherJava `getMinchaKetanaGRA`).
        MinchaKetanaGra = 112,
        /// Mincha Ketana (16.1 Degrees) (KosherJava `getMinchaKetana16Point1Degrees`).
        MinchaKetana16Point1Degrees = 113,
        /// Mincha Ketana (72 Minutes) (KosherJava `getMinchaKetana72Minutes`).
        MinchaKetana72Minutes = 114,
        /// Mincha Ketana (Ahavat Shalom) (KosherJava `getMinchaKetanaAhavatShalom`).
        MinchaKetanaAhavatShalom = 115,
        /// Mincha Ketana (Ateret Torah) (KosherJava `getMinchaKetanaAteretTorah`).
        MinchaKetanaAteretTorah = 116,
        /// Mincha Ketana (Baal Hatanya) (KosherJava `getMinchaKetanaBaalHatanya`).
        MinchaKetanaBaalHatanya = 117,
        /// Mincha Ketana (GR'A, Fixed Local Chatzos to Sunset) (KosherJava `getMinchaKetanaGRAFixedLocalChatzosToSunset`).
        MinchaKetanaGraFixedLocalChatzosToSunset = 118,
        /// Bain Hashmashos (Rabbeinu Tam, 13.24 Degrees) (KosherJava `getBainHashmashosRT13Point24Degrees`).
        BainHashmashosRt13Point24Degrees = 119,
        /// Bain Hashmashos (Rabbeinu Tam, 58.5 Minutes) (KosherJava `getBainHashmashosRT58Point5Minutes`).
        BainHashmashosRt58Point5Minutes = 120,
        /// Bain Hashmashos (Rabbeinu Tam, 13.5 Minutes Before 7.083 Degrees) (KosherJava `getBainHashmashosRT13Point5MinutesBefore7Point083Degrees`).
        BainHashmashosRt13Point5MinutesBefore7Point083Degrees = 121,
        /// Bain Hashmashos (Rabbeinu Tam, 2 Stars) (KosherJava `getBainHashmashosRT2Stars`).
        BainHashmashosRt2Stars = 122,
        /// Bain Hashmashos (Yereim, 18 Minutes) (KosherJava `getBainHashmashosYereim18Minutes`).
        BainHashmashosYereim18Minutes = 123,
        /// Bain Hashmashos (Yereim, 16.875 Minutes) (KosherJava `getBainHashmashosYereim16Point875Minutes`).
        BainHashmashosYereim16Point875Minutes = 124,
        /// Bain Hashmashos (Yereim, 13.5 Minutes) (KosherJava `getBainHashmashosYereim13Point5Minutes`).
        BainHashmashosYereim13Point5Minutes = 125,
        /// Bain Hashmashos (Yereim, 2.1 Degrees) (KosherJava `getBainHashmashosYereim2Point1Degrees`).
        BainHashmashosYereim2Point1Degrees = 126,
        /// Bain Hashmashos (Yereim, 2.8 Degrees) (KosherJava `getBainHashmashosYereim2Point8Degrees`).
        BainHashmashosYereim2Point8Degrees = 127,
        /// Bain Hashmashos (Yereim, 3.05 Degrees) (KosherJava `getBainHashmashosYereim3Point05Degrees`).
        BainHashmashosYereim3Point05Degrees = 128,
        /// Candle Lighting (KosherJava `getCandleLighting`).
        CandleLighting = 129,
        /// Polar Sunset Ben Ish Chai (KosherJava `getPolarSunsetBenIshChai`).
        PolarSunsetBenIshChai = 130,
        /// Sunset (KosherJava `getSunset`).
        Sunset = 131,
        /// Sea Level Sunset (KosherJava `getSeaLevelSunset`).
        SeaLevelSunset = 132,
        /// Tzais (Geonim, 8.5 Degrees) (KosherJava `getTzaisGeonim8Point5Degrees`).
        TzaisGeonim8Point5Degrees = 133,
        /// Tzais (50 Minutes) (KosherJava `getTzais50Minutes`).
        Tzais50Minutes = 134,
        /// Tzais (60 Minutes) (KosherJava `getTzais60Minutes`).
        Tzais60Minutes = 135,
        /// Tzais (72 Minutes) (KosherJava `getTzais72Minutes`).
        Tzais72Minutes = 136,
        /// Tzais (72 Minutes in Shaos Zmanios) (KosherJava `getTzais72Zmanis`).
        Tzais72Zmanis = 137,
        /// Tzais (90 Minutes) (KosherJava `getTzais90Minutes`).
        Tzais90Minutes = 138,
        /// Tzais (90 Minutes in Shaos Zmanios) (KosherJava `getTzais90Zmanis`).
        Tzais90Zmanis = 139,
        /// Tzais (96 Minutes) (KosherJava `getTzais96Minutes`).
        Tzais96Minutes = 140,
        /// Tzais (96 Minutes in Shaos Zmanios) (KosherJava `getTzais96Zmanis`).
        Tzais96Zmanis = 141,
        /// Tzais (120 Minutes) (KosherJava `getTzais120Minutes`).
        ///
        /// Deprecated in KosherJava.
        Tzais120Minutes = 142,
        /// Tzais (120 Minutes in Shaos Zmanios) (KosherJava `getTzais120Zmanis`).
        ///
        /// Deprecated in KosherJava.
        Tzais120Zmanis = 143,
        /// Tzais (16.1 Degrees) (KosherJava `getTzais16Point1Degrees`).
        Tzais16Point1Degrees = 144,
        /// Tzais (18 Degrees) (KosherJava `getTzais18Degrees`).
        Tzais18Degrees = 145,
        /// Tzais (19.8 Degrees) (KosherJava `getTzais19Point8Degrees`).
        Tzais19Point8Degrees = 146,
        /// Tzais (26 Degrees) (KosherJava `getTzais26Degrees`).
        ///
        /// Deprecated in KosherJava.
        Tzais26Degrees = 147,
        /// Tzais (Ateret Torah) (KosherJava `getTzaisAteretTorah`).
        TzaisAteretTorah = 148,
        /// Tzais (Baal Hatanya) (KosherJava `getTzaisBaalHatanya`).
        TzaisBaalHatanya = 149,
        /// Tzais (Geonim, 3.7 Degrees) (KosherJava `getTzaisGeonim3Point7Degrees`).
        ///
        /// Deprecated in KosherJava.
        TzaisGeonim3Point7Degrees = 150,
        /// Tzais (Geonim, 3.8 Degrees) (KosherJava `getTzaisGeonim3Point8Degrees`).
        ///
        /// Deprecated in KosherJava.
        TzaisGeonim3Point8Degrees = 151,
        /// Tzais (Geonim, 5.95 Degrees) (KosherJava `getTzaisGeonim5Point95Degrees`).
        TzaisGeonim5Point95Degrees = 152,
        /// Tzais (Geonim, 4.66 Degrees) (KosherJava `getTzaisGeonim4Point66Degrees`).
        ///
        /// Deprecated in KosherJava.
        TzaisGeonim4Point66Degrees = 153,
        /// Tzais (Geonim, 4.42 Degrees) (KosherJava `getTzaisGeonim4Point42Degrees`).
        ///
        /// Deprecated in KosherJava.
        TzaisGeonim4Point42Degrees = 154,
        /// Tzais (Geonim, 4.8 Degrees) (KosherJava `getTzaisGeonim4Point8Degrees`).
        TzaisGeonim4Point8Degrees = 155,
        /// Tzais (Geonim, 6.45 Degrees) (KosherJava `getTzaisGeonim6Point45Degrees`).
        TzaisGeonim6Point45Degrees = 156,
        /// Tzais (Geonim, 7.083 Degrees) (KosherJava `getTzaisGeonim7Point083Degrees`).
        TzaisGeonim7Point083Degrees = 157,
        /// Tzais (Geonim, 7.67 Degrees) (KosherJava `getTzaisGeonim7Point67Degrees`).
        TzaisGeonim7Point67Degrees = 158,
        /// Tzais (Geonim, 9.3 Degrees) (KosherJava `getTzaisGeonim9Point3Degrees`).
        TzaisGeonim9Point3Degrees = 159,
        /// Tzais (Geonim, 9.75 Degrees) (KosherJava `getTzaisGeonim9Point75Degrees`).
        TzaisGeonim9Point75Degrees = 160,
        /// Sof Zman Kidush Levana (15 Days) (KosherJava `getSofZmanKidushLevana15Days`).
        ///
        /// This time needs a time zone. Without one it fails with `ZmanimError::TimeZoneRequired`.
        SofZmanKidushLevana15Days = 161,
        /// Sof Zman Kidush Levana (Between Moldos) (KosherJava `getSofZmanKidushLevanaBetweenMoldos`).
        ///
        /// This time needs a time zone. Without one it fails with `ZmanimError::TimeZoneRequired`.
        SofZmanKidushLevanaBetweenMoldos = 162,
        /// Tchilas Zman Kidush Levana (3 Days) (KosherJava `getTchilasZmanKidushLevana3Days`).
        ///
        /// This time needs a time zone. Without one it fails with `ZmanimError::TimeZoneRequired`.
        TchilasZmanKidushLevana3Days = 163,
        /// Tchilas Zman Kidush Levana (7 Days) (KosherJava `getTchilasZmanKidushLevana7Days`).
        ///
        /// This time needs a time zone. Without one it fails with `ZmanimError::TimeZoneRequired`.
        TchilasZmanKidushLevana7Days = 164,
        /// Solar Midnight (KosherJava `getSolarMidnight`).
        SolarMidnight = 165,
        /// Chatzos Halayla (KosherJava `getChatzosHalayla`).
        ChatzosHalayla = 166,
    }
    // END GENERATED: Zman

    /// What kind of time a `Zman` is.
    #[weaveffi::enumeration]
    #[repr(i32)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum Category {
        /// Start and end of astronomical, civil and nautical twilight.
        Twilight = 0,
        /// Dawn (*alos*).
        Alos = 1,
        /// Earliest time for tallis and tefillin (*misheyakir*).
        Misheyakir = 2,
        /// Sunrise (*netz*).
        Netz = 3,
        /// Latest time for the morning Shema.
        SofZmanShma = 4,
        /// Latest time for the morning prayer.
        SofZmanTefila = 5,
        /// Latest time to eat chametz on Erev Pesach.
        SofZmanAchilasChametz = 6,
        /// Latest time to burn chametz on Erev Pesach.
        SofZmanBiurChametz = 7,
        /// Midday (*chatzos hayom*).
        ChatzosHayom = 8,
        /// Earliest Mincha (*mincha gedola*).
        MinchaGedola = 9,
        /// *Plag hamincha*.
        PlagHamincha = 10,
        /// Shortly before *mincha ketana*.
        SamuchLeMinchaKetana = 11,
        /// Preferred Mincha time (*mincha ketana*).
        MinchaKetana = 12,
        /// Twilight between sunset and nightfall (*bein hashmashos*).
        BeinHashmashos = 13,
        /// Candle lighting before Shabbos or Yom Tov.
        CandleLighting = 14,
        /// Sunset (*shkiya*).
        Shkiya = 15,
        /// Nightfall (*tzais*).
        Tzais = 16,
        /// The window for Kiddush Levana.
        KidushLevana = 17,
        /// Midnight (*chatzos halayla*).
        ChatzosHalayla = 18,
    }

    /// The solar position algorithm.
    #[weaveffi::enumeration]
    #[repr(i32)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Calculator {
        /// The NOAA algorithm, KosherJava's default. Good to about a minute.
        Noaa = 0,
        /// Jean Meeus's higher-accuracy VSOP87 algorithm.
        Meeus = 1,
        /// The NREL Solar Position Algorithm, solved on the topocentric zenith.
        Spa = 2,
    }

    /// How `Zmanim` calculates. `default_config` matches KosherJava's defaults.
    #[weaveffi::record]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Config {
        /// How long before sea-level sunset candle lighting is, in minutes, at most 1440 either
        /// way. Default 18.
        pub candle_lighting_minutes: i32,
        /// Whether proportional zmanim divide the day at astronomical *chatzos*. Default false.
        pub use_astronomical_chatzos_for_other_zmanim: bool,
        /// Whether elevation adjusts every zman built on sunrise and sunset, not only sunrise
        /// and sunset themselves. Default false.
        pub use_elevation: bool,
        /// How long after sunset Tzais Ateret Torah is, in minutes, at most 1440 either way.
        /// Default 40.
        pub ateret_torah_sunset_minutes: i32,
        /// Whether *chatzos* is the sun's transit rather than the midpoint of sunrise and
        /// sunset. Default true.
        pub use_astronomical_chatzos: bool,
        /// The solar position algorithm. Default NOAA.
        pub calculator: Calculator,
        /// For Meeus and SPA: whether to apply the delta-T (TT - UT) correction. Default true.
        pub apply_delta_t: bool,
    }

    /// KosherJava's default configuration.
    #[weaveffi::export]
    pub fn default_config() -> Config {
        kosher_rust::zmanim::Config::default().into()
    }

    /// Why a time in `ZmanTime` could not be calculated.
    #[weaveffi::enumeration]
    #[repr(i32)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum ZmanFailure {
        /// The sun stays above the angle the time needs all day.
        AllDay = 0,
        /// The sun stays below the angle the time needs all day.
        AllNight = 1,
        /// The time does not apply on this date.
        NotApplicable = 2,
        /// The time needs a time zone and none was given. Only the four Kiddush Levana times need
        /// one.
        TimeZoneRequired = 3,
        /// The time zone callback failed.
        TimeZoneCallbackFailed = 4,
        /// An internal calculation error.
        Unexpected = 5,
    }

    /// One zman's time, or why it has none.
    #[weaveffi::record]
    #[derive(Debug, Clone, PartialEq)]
    pub struct ZmanTime {
        /// The zman.
        pub zman: Zman,
        /// The time in Unix milliseconds, when it could be calculated.
        pub time: Option<i64>,
        /// Why it could not, when it could not.
        pub failure: Option<ZmanFailure>,
    }

    /// The *zmanim* for one place on one day.
    #[weaveffi::interface]
    pub struct Zmanim {
        pub(crate) inner: kosher_rust::Zmanim<convert::CallbackTimeZone>,
        pub(crate) config: Config,
    }

    impl Zmanim {
        /// The *zmanim* at `location` on `date` in `time_zone`, calculated with `config`.
        ///
        /// The time zone is optional. With a time zone every time works. Without one the four
        /// Kiddush Levana times return `ZmanimError::TimeZoneRequired`. A location more than 150
        /// degrees from Greenwich needs a time zone. Without one this returns
        /// `ZmanimError::TimeZoneRequired`.
        ///
        /// # Errors
        ///
        /// - `ZmanimError::InvalidGregorianDate` when the date does not exist.
        /// - `ZmanimError::InvalidMinutes` when the candle-lighting or Ateret Torah offset is more
        ///   than a day.
        /// - The errors of `validate_location`. This includes `ZmanimError::TimeZoneRequired` when
        ///   there is no time zone and the location is more than 150 degrees from Greenwich.
        pub fn new(
            location: Location,
            date: GregorianDate,
            config: Config,
            time_zone: Option<Arc<dyn TimeZoneOffset>>,
        ) -> Result<Self, ZmanimError> {
            convert::zmanim::zmanim(&location, &date, &config, time_zone).map(|inner| Self { inner, config })
        }

        /// The time of `zman`, in Unix milliseconds.
        ///
        /// # Errors
        ///
        /// `ZmanimError::AllDay` or `ZmanimError::AllNight` where the sun never reaches
        /// the angle the time needs; `ZmanimError::NotApplicable` for a time that only exists
        /// on some days; `ZmanimError::TimeZoneRequired` for the Kiddush Levana times without
        /// a time zone; `ZmanimError::TimeZoneCallbackFailed` when the callback failed.
        pub fn get(&self, zman: Zman) -> Result<i64, ZmanimError> {
            convert::zmanim::checked(&self.inner, |inner| inner.get(zman.into()))
        }

        /// Every zman, with its time or why it has none.
        pub fn all(&self) -> Vec<ZmanTime> {
            convert::zmanim::all(&self.inner)
        }

        /// When the sun is `degrees` below the geometric horizon before sunrise, in Unix
        /// milliseconds.
        ///
        /// # Errors
        ///
        /// `ZmanimError::InvalidDegrees` when `degrees` is not a finite number; otherwise the
        /// same as `get`.
        pub fn sunrise_offset_by_degrees(&self, degrees: f64) -> Result<i64, ZmanimError> {
            convert::zmanim::offset_by_degrees(&self.inner, degrees, kosher_rust::Zmanim::sunrise_offset_by_degrees)
        }

        /// When the sun is `degrees` below the geometric horizon after sunset, in Unix
        /// milliseconds.
        ///
        /// # Errors
        ///
        /// `ZmanimError::InvalidDegrees` when `degrees` is not a finite number; otherwise the
        /// same as `get`.
        pub fn sunset_offset_by_degrees(&self, degrees: f64) -> Result<i64, ZmanimError> {
            convert::zmanim::offset_by_degrees(&self.inner, degrees, kosher_rust::Zmanim::sunset_offset_by_degrees)
        }

        /// The location.
        pub fn location(&self) -> Location {
            self.inner.location().into()
        }

        /// The date.
        pub fn date(&self) -> GregorianDate {
            self.inner.date().into()
        }

        /// The configuration, as given to `new`.
        pub fn config(&self) -> Config {
            self.config
        }
    }

    /// Every zman, grouped by `Category`.
    #[weaveffi::export]
    pub fn all_zmanim() -> Vec<Zman> {
        kosher_rust::Zman::ALL
            .into_iter()
            .filter_map(|zman| zman.try_into().ok())
            .collect()
    }

    /// The zman's display name, for example "Sof Zman Shma (GR'A)".
    #[weaveffi::export]
    pub fn zman_name(zman: Zman) -> String {
        kosher_rust::Zman::from(zman).name().into()
    }

    /// The KosherJava method that calculates the zman, for example "getSofZmanShmaGRA".
    #[weaveffi::export]
    pub fn zman_java_name(zman: Zman) -> String {
        kosher_rust::Zman::from(zman).java_name().into()
    }

    /// What kind of time the zman is.
    #[weaveffi::export]
    pub fn zman_category(zman: Zman) -> Category {
        // Every zman the bindings have is in a category they have; `every_variant_converts` checks it.
        kosher_rust::Zman::from(zman)
            .category()
            .try_into()
            .unwrap_or(Category::Twilight)
    }

    /// Whether KosherJava deprecates the zman.
    #[weaveffi::export]
    pub fn zman_is_deprecated(zman: Zman) -> bool {
        kosher_rust::Zman::from(zman).is_deprecated()
    }

    /// A paragraph or two explaining the zman to users, with the offsets from `config` filled
    /// in. May contain Markdown links.
    #[weaveffi::export]
    pub fn zman_description(zman: Zman, config: Config) -> String {
        kosher_rust::Zman::from(zman).description(&kosher_rust::zmanim::Config::from(config))
    }

    /// The algorithm's descriptive name, as KosherJava's `getCalculatorName` gives it.
    #[weaveffi::export]
    pub fn calculator_name(calculator: Calculator) -> String {
        let config: kosher_rust::zmanim::Config = Config {
            calculator,
            ..default_config()
        }
        .into();
        config.astronomical_calculator.name().into()
    }

    /// A *shaah zmanis*: a twelfth of the day from `start` to `end` (Unix milliseconds), in
    /// milliseconds.
    #[weaveffi::export]
    pub fn shaah_zmanis(start: i64, end: i64) -> i64 {
        convert::zmanim::shaah_zmanis(start, end)
    }

    /// The *zmanim* of the MyZmanim website, reproduced to the second.
    #[weaveffi::module]
    pub mod myzmanim {
        use std::sync::Arc;

        use super::{Location, TimeZoneOffset, ZmanFailure, ZmanimError};
        use crate::api::calendar::GregorianDate;
        use crate::convert;

        // BEGIN GENERATED: MyZman
        /// One row of the MyZmanim website's "All zmanim" table, in the table's order.
        #[weaveffi::enumeration]
        #[repr(i32)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum MyZman {
            /// Dawn, 90 minutes as degrees.
            Dawn90Degrees = 0,
            /// Dawn, fixed 72 minutes.
            Dawn72Fixed = 1,
            /// Dawn, 72 minutes as 16.1 degrees.
            Dawn72Degrees = 2,
            /// Earliest talis & tefillin, 11.5 degrees.
            Talis11Point5Degrees = 3,
            /// Earliest talis & tefillin, 11 degrees.
            Talis11Degrees = 4,
            /// Earliest talis & tefillin, 10.2 degrees.
            Talis10Point2Degrees = 5,
            /// Sunrise, level region.
            SunriseLevel = 6,
            /// Sunrise, elevated observer/horizon.
            SunriseElevated = 7,
            /// Latest shema Magen Avraham, 90 minutes as degrees.
            ShemaMA90Degrees = 8,
            /// Latest shema Magen Avraham, fixed 72 minutes.
            ShemaMA72Fixed = 9,
            /// Latest shema Magen Avraham, 72 minutes as 16.1 degrees.
            ShemaMA72Degrees = 10,
            /// Latest shema Gra & Baal HaTanya.
            ShemaGra = 11,
            /// Latest shacharis Magen Avraham, 90 minutes as degrees.
            ShacharisMA90Degrees = 12,
            /// Latest shacharis Magen Avraham, fixed 72 minutes.
            ShacharisMA72Fixed = 13,
            /// Latest shacharis Magen Avraham, 72 minutes as 16.1 degrees.
            ShacharisMA72Degrees = 14,
            /// Latest shacharis Gra & Baal HaTanya.
            ShacharisGra = 15,
            /// Midday.
            Midday = 16,
            /// Earliest mincha, 30 fixed minutes after midday.
            Mincha30Fixed = 17,
            /// Earliest mincha, Gra & Baal HaTanya.
            MinchaGra = 18,
            /// Earliest mincha, lechumra.
            MinchaLechumra = 19,
            /// Earliest mincha, Magen Avraham fixed 72 minutes.
            MinchaMA72Fixed = 20,
            /// Mincha ketana, Gra & Baal HaTanya.
            KetanaGra = 21,
            /// Mincha ketana, Magen Avraham fixed 72 minutes.
            KetanaMA72Fixed = 22,
            /// Plag hamincha, Gra & Baal HaTanya.
            PlagGra = 23,
            /// Plag hamincha, Magen Avraham fixed 72 minutes.
            PlagMA72Fixed = 24,
            /// Candle lighting.
            CandleLighting = 25,
            /// Sunset, level region.
            SunsetLevel = 26,
            /// Sunset, elevated observer/horizon.
            SunsetElevated = 27,
            /// Gra's nightfall, 13 1/2 minutes as degrees.
            NightGra18Minutes = 28,
            /// Gra's nightfall, 16 7/8 minutes as degrees.
            NightGra22Point5Minutes = 29,
            /// Gra's nightfall, 18 minutes as degrees.
            NightGra24Minutes = 30,
            /// Gra's nightfall, 24 minutes as degrees, R' Schneur Zalman.
            NightZalman = 31,
            /// End of ordained fasts, R' Tukaccinsky.
            NightFastTukaccinsky = 32,
            /// End of ordained fasts, R' Moshe Feinstein.
            NightFastFeinstein = 33,
            /// 3 stars emerge, Sefer Moed Moadim.
            NightMoed = 34,
            /// 3 stars emerge, end of Shabbos and Yom Tov.
            NightShabbos = 35,
            /// 3 stars emerge, Chazon Ish.
            NightChazonIsh = 36,
            /// R' Tam's nightfall, fixed 72 minutes.
            NightRTam72Fixed = 37,
            /// R' Tam's nightfall, 72 minutes as 16.1 degrees.
            NightRTam72Degrees = 38,
            /// Midnight.
            Midnight = 39,
        }
        // END GENERATED: MyZman

        /// One MyZmanim time, or why it has none.
        #[weaveffi::record]
        #[derive(Debug, Clone, PartialEq)]
        pub struct MyZmanTime {
            /// The row.
            pub zman: MyZman,
            /// The time in Unix milliseconds, when it could be calculated.
            pub time: Option<i64>,
            /// Why it could not, when it could not.
            pub failure: Option<ZmanFailure>,
        }

        /// MyZmanim's *zmanim* for one place on one day. The location's elevation and horizons
        /// matter: MyZmanim gives both "level" and "elevated" sunrise and sunset.
        #[weaveffi::interface]
        pub struct MyZmanim {
            pub(crate) inner: kosher_rust::myzmanim::MyZmanim<convert::CallbackTimeZone>,
        }

        impl MyZmanim {
            /// The *zmanim* at `location` on `date` in `time_zone`. Candles are lit
            /// `candle_lighting_minutes` before sunset. The website uses 18 minutes by default.
            ///
            /// The time zone is optional. With a time zone every time works. A location more than
            /// 150 degrees from Greenwich needs a time zone. Without one this returns
            /// `ZmanimError::TimeZoneRequired`.
            ///
            /// # Errors
            ///
            /// - `ZmanimError::InvalidGregorianDate` when the date does not exist.
            /// - `ZmanimError::InvalidMinutes` when `candle_lighting_minutes` is not a number or
            ///   is more than 1440 minutes either way.
            /// - The errors of `validate_location`. This includes `ZmanimError::TimeZoneRequired`
            ///   when there is no time zone and the location is more than 150 degrees from
            ///   Greenwich.
            pub fn new(
                location: Location,
                date: GregorianDate,
                candle_lighting_minutes: f64,
                time_zone: Option<Arc<dyn TimeZoneOffset>>,
            ) -> Result<Self, ZmanimError> {
                convert::myzmanim::my_zmanim(&location, &date, candle_lighting_minutes, time_zone)
                    .map(|inner| Self { inner })
            }

            /// The time of `zman`, in Unix milliseconds.
            ///
            /// # Errors
            ///
            /// `ZmanimError::AllDay` or `ZmanimError::AllNight` where the website shows no
            /// time; `ZmanimError::TimeZoneCallbackFailed` when the callback failed.
            pub fn get(&self, zman: MyZman) -> Result<i64, ZmanimError> {
                convert::myzmanim::checked(&self.inner, |inner| inner.get(zman.into()))
            }

            /// Every row, with its time or why it has none.
            pub fn all(&self) -> Vec<MyZmanTime> {
                convert::myzmanim::all(&self.inner)
            }

            /// A twelfth of the day from sea-level sunrise to sea-level sunset, in milliseconds.
            ///
            /// # Errors
            ///
            /// The same as `get`.
            pub fn proportional_hour_gra(&self) -> Result<i64, ZmanimError> {
                convert::myzmanim::checked_duration(&self.inner)
            }

            /// The location.
            pub fn location(&self) -> Location {
                self.inner.location().into()
            }

            /// The date.
            pub fn date(&self) -> GregorianDate {
                self.inner.date().into()
            }

            /// The minutes before sunset of candle lighting.
            pub fn candle_lighting_minutes(&self) -> f64 {
                self.inner.candle_lighting_minutes()
            }
        }

        /// The row's name as the website shows it.
        #[weaveffi::export]
        pub fn my_zman_name(zman: MyZman) -> String {
            kosher_rust::myzmanim::MyZman::from(zman).name().into()
        }

        /// Every row, in the website's order.
        #[weaveffi::export]
        pub fn all_my_zmanim() -> Vec<MyZman> {
            kosher_rust::myzmanim::MyZman::ALL
                .into_iter()
                .filter_map(|zman| zman.try_into().ok())
                .collect()
        }

        /// The sun's depression for "90 minutes as degrees": 19.75 degrees.
        #[weaveffi::export]
        pub fn dawn_90_degrees() -> f64 {
            kosher_rust::myzmanim::DAWN_90_DEGREES
        }

        /// The sun's depression for "72 minutes as 16.1 degrees".
        #[weaveffi::export]
        pub fn dawn_72_degrees() -> f64 {
            kosher_rust::myzmanim::DAWN_72_DEGREES
        }

        /// Refraction at a sea-level horizon, in arcminutes.
        #[weaveffi::export]
        pub fn sea_level_refraction_arcminutes() -> f64 {
            kosher_rust::myzmanim::SEA_LEVEL_REFRACTION_ARCMINUTES
        }

        /// The dip of the horizon per square root of a meter of height, in degrees.
        #[weaveffi::export]
        pub fn dip_degrees_per_root_meter() -> f64 {
            kosher_rust::myzmanim::DIP_DEGREES_PER_ROOT_METER
        }

        /// The website's default minutes before sunset for candle lighting.
        #[weaveffi::export]
        pub fn default_candle_lighting_minutes() -> f64 {
            kosher_rust::myzmanim::DEFAULT_CANDLE_LIGHTING_MINUTES
        }
    }
}
