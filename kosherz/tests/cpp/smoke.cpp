// Calls the generated C++ bindings: sunrise in Lakewood on a local day, Kiddush Levana, which
// needs the right offset, an impossible local day, the daf yomi, and the typed error for an invalid
// latitude. Build after `cargo build -p kosherz`,
// from the repository root:
//
//     c++ -std=c++17 -Wall -I kosherz/generated/cpp kosherz/tests/cpp/smoke.cpp \
//         -L target/debug -lkosherz -Wl,-rpath,"$PWD/target/debug" -o target/smoke-cpp
//     ./target/smoke-cpp
#include <cstdio>
#include <optional>
#include <string>

#include "kosherz.hpp"

namespace {

int failures = 0;

void check(bool ok, const std::string& message) {
    if (!ok) {
        std::fprintf(stderr, "FAIL: %s\n", message.c_str());
        failures++;
    }
}

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
kosherz::LocalDay fixed_day(int hours, const kosherz::GregorianDate& date) {
    int64_t start = (days_from_civil(date.year, date.month, date.day) * 86400 - hours * 3600) * 1000;
    return kosherz::LocalDay{start, hours * 3600, start + 86400 * 1000, hours * 3600};
}

const kosherz::GregorianDate april_23{2024, 4, 23};
const kosherz::GregorianDate december_3{2024, 12, 3};

kosherz::Zmanim new_zmanim(double latitude, double longitude) {
    return kosherz::Zmanim(kosherz::times::sea_level_location(latitude, longitude), april_23,
                           kosherz::times::default_config(), fixed_day(-4, april_23));
}

kosherz::Zmanim lakewood_on(const kosherz::GregorianDate& date, const std::optional<kosherz::LocalDay>& local_day) {
    return kosherz::Zmanim(kosherz::times::sea_level_location(40.09, -74.22), date, kosherz::times::default_config(),
                           local_day);
}

// Kiddush Levana is the one time whose day depends on the offset: with UTC-4 it falls on
// 3 December 2024, and with UTC+4 it does not.
void check_kiddush_levana() {
    int64_t ms = lakewood_on(december_3, fixed_day(-4, december_3)).get(kosherz::Zman::TchilasZmanKidushLevana3Days);
    check(ms == 1733279333504LL, "UTC-4: kiddush levana is " + std::to_string(ms) + ", want 1733279333504");
    try {
        lakewood_on(december_3, fixed_day(4, december_3)).get(kosherz::Zman::TchilasZmanKidushLevana3Days);
        check(false, "UTC+4 gave a kiddush levana time");
    } catch (const kosherz::NotApplicableError&) {
    }
    try {
        lakewood_on(december_3, std::nullopt).get(kosherz::Zman::TchilasZmanKidushLevana3Days);
        check(false, "no local day gave a kiddush levana time");
    } catch (const kosherz::TimeZoneRequiredError&) {
    }
}

void check_impossible_local_day() {
    try {
        lakewood_on(april_23, kosherz::LocalDay{1, 0, 0, 0});
        check(false, "Zmanim accepted a local day that ends before it starts");
    } catch (const kosherz::InvalidLocalDayError&) {
    }
}

void check_sunrise() {
    int64_t ms = new_zmanim(40.09, -74.22).get(kosherz::Zman::Sunrise);
    int64_t seconds = ms >= 0 ? ms / 1000 : -((-ms + 999) / 1000);
    check(seconds == 1713866795, "sunrise is " + std::to_string(seconds) + ", want 1713866795");
}

void check_daf_yomi() {
    auto daf = kosherz::calendar::limudim::daf_yomi_bavli(kosherz::HebrewDate::from_gregorian(april_23));
    check(daf.has_value(), "no daf yomi");
    if (daf) check(daf->display == "Bava Metzia 55", "daf yomi is \"" + daf->display + "\", want \"Bava Metzia 55\"");
}

void check_invalid_latitude() {
    try {
        new_zmanim(91, 0);
        check(false, "Zmanim accepted latitude 91");
    } catch (const kosherz::InvalidLatitudeError&) {
    }
}

}  // namespace

int main() {
    try {
        check_sunrise();
        check_kiddush_levana();
        check_impossible_local_day();
        check_daf_yomi();
        check_invalid_latitude();
    } catch (const std::exception& e) {
        std::fprintf(stderr, "FAIL: unexpected exception: %s\n", e.what());
        return 1;
    }
    if (failures != 0) {
        std::fprintf(stderr, "%d check(s) failed\n", failures);
        return 1;
    }
    std::printf("C++ smoke test passed\n");
    return 0;
}
