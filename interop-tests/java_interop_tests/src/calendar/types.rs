//! Shared types for Hebrew-date Java parity tests.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct DateTuple {
    pub(super) year: i32,
    pub(super) month: u8,
    pub(super) day: u8,
}

impl DateTuple {
    pub(super) const fn new(year: i32, month: u8, day: u8) -> Self {
        Self { year, month, day }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct JewishCalendarSnapshot {
    pub(super) is_yom_tov: bool,
    pub(super) is_yom_tov_assur_bemelacha: bool,
    pub(super) is_erev_yom_tov: bool,
    pub(super) is_erev_yom_tov_sheni: bool,
    pub(super) is_chol_hamoed: bool,
    pub(super) is_chol_hamoed_pesach: bool,
    pub(super) is_chol_hamoed_succos: bool,
    pub(super) is_pesach: bool,
    pub(super) is_shavuos: bool,
    pub(super) is_succos: bool,
    pub(super) is_shmini_atzeres: bool,
    pub(super) is_simchas_torah: bool,
    pub(super) is_rosh_hashana: bool,
    pub(super) is_yom_kippur: bool,
    pub(super) is_taanis: bool,
    pub(super) is_tisha_bav: bool,
    pub(super) is_chanukah: bool,
    pub(super) is_purim: bool,
    pub(super) is_isru_chag: bool,
    pub(super) is_erev_rosh_chodesh: bool,
    pub(super) is_tomorrow_shabbos_or_yom_tov: bool,
    pub(super) is_birkas_hachamah: bool,
    pub(super) parshah: Option<i32>,
    pub(super) upcoming_parshah: i32,
    pub(super) special_shabbos: Option<i32>,
    pub(super) yom_tov_index: i32,
    pub(super) is_assur_bemelacha: bool,
    pub(super) has_candle_lighting: bool,
    pub(super) is_aseres_yemei_teshuva: bool,
    pub(super) is_yom_kippur_katan: bool,
    pub(super) is_be_ha_b: bool,
    pub(super) is_taanis_bechoros: bool,
    pub(super) day_of_chanukah: Option<u8>,
    pub(super) is_rosh_chodesh: bool,
    pub(super) is_machar_chodesh: bool,
    pub(super) is_shabbos_mevorchim: bool,
    pub(super) day_of_omer: Option<u8>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct JewishDateSnapshot {
    pub(super) day_of_week: i32,
    pub(super) abs_date: i32,
    pub(super) days_in_jewish_month: i32,
    pub(super) days_in_jewish_year: i32,
    pub(super) static_days_in_jewish_year: i32,
    pub(super) days_since_start_of_jewish_year: i32,
    pub(super) jewish_calendar_elapsed_days: i32,
    pub(super) is_jewish_leap_year: bool,
    pub(super) is_cheshvan_long: bool,
    pub(super) is_kislev_short: bool,
    pub(super) cheshvan_kislev_kviah: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct JewishYearSnapshot {
    pub(super) days_in_jewish_year: i32,
    pub(super) jewish_calendar_elapsed_days: i32,
    pub(super) is_jewish_leap_year: bool,
    pub(super) is_cheshvan_long: bool,
    pub(super) is_kislev_short: bool,
}
