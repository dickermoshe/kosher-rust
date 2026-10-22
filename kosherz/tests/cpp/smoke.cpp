// Calls the generated C++ bindings: sunrise in Lakewood through a time zone callback, the daf
// yomi, and the typed error for an invalid latitude. Build after `cargo build -p kosherz`,
// from the repository root:
//
//     c++ -std=c++17 -Wall -I kosherz/generated/cpp kosherz/tests/cpp/smoke.cpp \
//         -L target/debug -lkosherz -Wl,-rpath,"$PWD/target/debug" -o target/smoke-cpp
//     ./target/smoke-cpp
#include <cstdio>
#include <memory>
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

// A fixed UTC-4 time zone.
// Days from 1970-01-01 to the given date on the Gregorian calendar.
static int64_t days_from_civil(int64_t year, int64_t month, int64_t day) {
    year -= month <= 2;
    int64_t era = (year >= 0 ? year : year - 399) / 400;
    int64_t year_of_era = year - era * 400;
    int64_t day_of_year = (153 * (month + (month > 2 ? -3 : 9)) + 2) / 5 + day - 1;
    int64_t day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;
    return era * 146097 + day_of_era - 719468;
}

class Edt : public kosherz::TimeZoneOffset {
public:
    int32_t offset_seconds(int64_t) override { return -4 * 3600; }
    int64_t local_to_unix_seconds(int32_t year, int32_t month, int32_t day, int32_t hour, int32_t minute,
                                  int32_t second) override {
        return days_from_civil(year, month, day) * 86400 + hour * 3600 + minute * 60 + second + 4 * 3600;
    }
};

const kosherz::GregorianDate april_23{2024, 4, 23};

kosherz::Zmanim new_zmanim(double latitude, double longitude) {
    return kosherz::Zmanim(kosherz::times::sea_level_location(latitude, longitude), april_23,
                           kosherz::times::default_config(), std::make_shared<Edt>());
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
