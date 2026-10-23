// Calls the generated C bindings: sunrise in Lakewood on a local day, Kiddush Levana, which needs
// the right offset, an impossible local day, the daf yomi, and the typed error for an invalid
// latitude. Build after `cargo build -p kosherz`, from the repository root:
//
//     cc -std=c11 -Wall -I kosherz/generated/c kosherz/tests/c/smoke.c \
//         -L target/debug -lkosherz -Wl,-rpath,"$PWD/target/debug" -o target/smoke-c
//     ./target/smoke-c
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "kosherz_buffer.h"

static int failures = 0;

#define CHECK(cond, ...)                                     \
    do {                                                     \
        if (!(cond)) {                                       \
            fprintf(stderr, "FAIL %s:%d: ", __FILE__, __LINE__); \
            fprintf(stderr, __VA_ARGS__);                    \
            fprintf(stderr, "\n");                           \
            failures++;                                      \
        }                                                    \
    } while (0)

// Days from 1970-01-01 to the given date on the Gregorian calendar.
static int64_t days_from_civil(int64_t year, int64_t month, int64_t day) {
    year -= month <= 2;
    int64_t era = (year >= 0 ? year : year - 399) / 400;
    int64_t year_of_era = year - era * 400;
    int64_t day_of_year = (153 * (month + (month > 2 ? -3 : 9)) + 2) / 5 + day - 1;
    int64_t day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;
    return era * 146097 + day_of_era - 719468;
}

// `date` at UTC plus `hours` all year: its two local midnights and the offsets at them.
static kosherz_times_LocalDay fixed_day(int hours, kosherz_calendar_GregorianDate date) {
    int64_t start = (days_from_civil(date.year, date.month, date.day) * 86400 - hours * 3600) * 1000;
    kosherz_times_LocalDay day = {start, hours * 3600, start + 86400 * 1000, hours * 3600};
    return day;
}

// Builds Zmanim for `date` at sea level on `local_day`, which may be NULL, leaving any error in
// `err`.
static kosherz_times_Zmanim* new_zmanim_on(double latitude, double longitude, kosherz_calendar_GregorianDate date,
                                           const kosherz_times_LocalDay* local_day, kosherz_error* err) {
    size_t location_len = 0, config_len = 0;
    const uint8_t* location = kosherz_times_sea_level_location(latitude, longitude, &location_len, err);
    if (err->code != 0) return NULL;
    const uint8_t* config = kosherz_times_default_config(&config_len, err);
    if (err->code != 0) {
        kosherz_free_bytes((uint8_t*)location, location_len);
        return NULL;
    }
    kosherz_writer date_buf = {0};
    kosherz_calendar_GregorianDate_write(&date_buf, &date);
    kosherz_writer day_buf = {0};
    kosherz_opt_times_LocalDay_write(&day_buf, local_day);

    kosherz_times_Zmanim* zmanim = kosherz_times_Zmanim_new(location, location_len, date_buf.ptr, date_buf.len, config,
                                                            config_len, day_buf.ptr, day_buf.len, err);

    kosherz_writer_free(&day_buf);
    kosherz_writer_free(&date_buf);
    kosherz_free_bytes((uint8_t*)config, config_len);
    kosherz_free_bytes((uint8_t*)location, location_len);
    return zmanim;
}

static const kosherz_calendar_GregorianDate april_23 = {2024, 4, 23};

// Builds Zmanim for 2024-04-23 at sea level in UTC-4, leaving any error in `err`.
static kosherz_times_Zmanim* new_zmanim(double latitude, double longitude, kosherz_error* err) {
    kosherz_times_LocalDay edt = fixed_day(-4, april_23);
    return new_zmanim_on(latitude, longitude, april_23, &edt, err);
}

static void check_sunrise(void) {
    kosherz_error err = {0};
    kosherz_times_Zmanim* day = new_zmanim(40.09, -74.22, &err);
    CHECK(err.code == 0 && day != NULL, "Zmanim_new failed: %d %s", err.code, err.message);
    if (day == NULL) {
        kosherz_error_clear(&err);
        return;
    }
    int64_t ms = kosherz_times_Zmanim_get(day, kosherz_times_Zman_Sunrise, &err);
    CHECK(err.code == 0, "Zmanim_get failed: %d %s", err.code, err.message);
    int64_t seconds = ms >= 0 ? ms / 1000 : -((-ms + 999) / 1000);
    CHECK(seconds == 1713866795, "sunrise is %lld, want 1713866795", (long long)seconds);
    kosherz_error_clear(&err);
    kosherz_times_Zmanim_destroy(day);
}

// The error code of Tchilas Zman Kiddush Levana (3 days) in Lakewood on 2024-12-03 on
// `local_day`, and its time.
static int32_t kiddush_levana(const kosherz_times_LocalDay* local_day, int64_t* ms) {
    kosherz_error err = {0};
    kosherz_calendar_GregorianDate date = {2024, 12, 3};
    kosherz_times_Zmanim* day = new_zmanim_on(40.09, -74.22, date, local_day, &err);
    CHECK(err.code == 0 && day != NULL, "Zmanim_new failed: %d %s", err.code, err.message);
    if (day == NULL) {
        kosherz_error_clear(&err);
        return -1;
    }
    *ms = kosherz_times_Zmanim_get(day, kosherz_times_Zman_TchilasZmanKidushLevana3Days, &err);
    int32_t code = err.code;
    kosherz_error_clear(&err);
    kosherz_times_Zmanim_destroy(day);
    return code;
}

// Kiddush Levana is the one time whose day depends on the offset: with UTC-4 it falls on
// 3 December 2024, and with UTC+4 it does not.
static void check_kiddush_levana(void) {
    kosherz_calendar_GregorianDate december_3 = {2024, 12, 3};
    kosherz_times_LocalDay edt = fixed_day(-4, december_3);
    kosherz_times_LocalDay wrong_sign = fixed_day(4, december_3);
    int64_t ms = 0;
    int32_t code = kiddush_levana(&edt, &ms);
    CHECK(code == 0 && ms == 1733279333504LL, "UTC-4: code %d, time %lld, want 1733279333504", code,
          (long long)ms);
    code = kiddush_levana(&wrong_sign, &ms);
    CHECK(code == kosherz_times_ZmanimError_NotApplicable, "UTC+4: code %d, want NotApplicable", code);
    code = kiddush_levana(NULL, &ms);
    CHECK(code == kosherz_times_ZmanimError_TimeZoneRequired, "no local day: code %d, want TimeZoneRequired", code);
}

static void check_impossible_local_day(void) {
    kosherz_error err = {0};
    kosherz_times_LocalDay backwards = {1, 0, 0, 0};
    kosherz_times_Zmanim* day = new_zmanim_on(40.09, -74.22, april_23, &backwards, &err);
    CHECK(day == NULL, "Zmanim_new accepted a local day that ends before it starts");
    CHECK(err.code == kosherz_times_ZmanimError_InvalidLocalDay, "error code is %d, want %d", err.code,
          kosherz_times_ZmanimError_InvalidLocalDay);
    if (day != NULL) kosherz_times_Zmanim_destroy(day);
    kosherz_error_clear(&err);
}

static void check_daf_yomi(void) {
    kosherz_error err = {0};
    kosherz_writer date_buf = {0};
    kosherz_calendar_GregorianDate_write(&date_buf, &april_23);
    kosherz_calendar_HebrewDate* hebrew =
        kosherz_calendar_HebrewDate_from_gregorian(date_buf.ptr, date_buf.len, &err);
    kosherz_writer_free(&date_buf);
    CHECK(err.code == 0 && hebrew != NULL, "HebrewDate_from_gregorian failed: %d %s", err.code, err.message);
    if (hebrew == NULL) {
        kosherz_error_clear(&err);
        return;
    }

    size_t len = 0;
    const uint8_t* buf = kosherz_calendar_limudim_daf_yomi_bavli(hebrew, &len, &err);
    CHECK(err.code == 0, "daf_yomi_bavli failed: %d %s", err.code, err.message);
    kosherz_calendar_limudim_Daf* daf = NULL;
    CHECK(kosherz_opt_calendar_limudim_Daf_decode(buf, len, &daf), "could not decode the daf");
    CHECK(daf != NULL, "no daf yomi");
    if (daf != NULL) {
        const char* want = "Bava Metzia 55";
        CHECK(daf->display.len == strlen(want) && memcmp(daf->display.ptr, want, daf->display.len) == 0,
              "daf yomi is \"%.*s\", want \"%s\"", (int)daf->display.len, daf->display.ptr, want);
    }
    kosherz_opt_calendar_limudim_Daf_free(&daf);
    kosherz_free_bytes((uint8_t*)buf, len);
    kosherz_error_clear(&err);
    kosherz_calendar_HebrewDate_destroy(hebrew);
}

static void check_invalid_latitude(void) {
    kosherz_error err = {0};
    kosherz_times_Zmanim* day = new_zmanim(91, 0, &err);
    CHECK(day == NULL, "Zmanim_new accepted latitude 91");
    CHECK(err.code == kosherz_times_ZmanimError_InvalidLatitude, "error code is %d (%s), want %d", err.code,
          err.message ? err.message : "", kosherz_times_ZmanimError_InvalidLatitude);
    if (day != NULL) kosherz_times_Zmanim_destroy(day);
    kosherz_error_clear(&err);
}

int main(void) {
    check_sunrise();
    check_kiddush_levana();
    check_impossible_local_day();
    check_daf_yomi();
    check_invalid_latitude();
    if (failures != 0) {
        fprintf(stderr, "%d check(s) failed\n", failures);
        return 1;
    }
    printf("C smoke test passed\n");
    return 0;
}
