// Calls the generated C++ bindings: sunrise in Lakewood through a time zone callback, Kiddush
// Levana, which needs the right offset, a failing callback, the daf yomi, and the typed error for an
// invalid latitude. Build after `cargo build -p kosherz`,
// from the repository root:
//
//     c++ -std=c++17 -Wall -I kosherz/generated/cpp kosherz/tests/cpp/smoke.cpp \
//         -L target/debug -lkosherz -Wl,-rpath,"$PWD/target/debug" -o target/smoke-cpp
//     ./target/smoke-cpp
#include <cstdio>
#include <memory>
#include <stdexcept>
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

// UTC plus `hours`, every day of the year.
class Fixed : public kosherz::TimeZoneOffset {
public:
    explicit Fixed(int hours) : hours_(hours) {}
    int32_t offset_seconds(int64_t) override { return hours_ * 3600; }
    int64_t local_to_unix_seconds(int32_t year, int32_t month, int32_t day, int32_t hour, int32_t minute,
                                  int32_t second) override {
        return days_from_civil(year, month, day) * 86400 + hour * 3600 + minute * 60 + second - hours_ * 3600;
    }

private:
    int hours_;
};

// A time zone whose callbacks fail.
class Broken : public kosherz::TimeZoneOffset {
public:
    int32_t offset_seconds(int64_t) override { throw std::runtime_error("no time zone data"); }
    int64_t local_to_unix_seconds(int32_t, int32_t, int32_t, int32_t, int32_t, int32_t) override {
        throw std::runtime_error("no time zone data");
    }
};

const kosherz::GregorianDate april_23{2024, 4, 23};
const kosherz::GregorianDate december_3{2024, 12, 3};

kosherz::Zmanim new_zmanim(double latitude, double longitude) {
    return kosherz::Zmanim(kosherz::times::sea_level_location(latitude, longitude), april_23,
                           kosherz::times::default_config(), std::make_shared<Fixed>(-4));
}

kosherz::Zmanim lakewood_on(const kosherz::GregorianDate& date, std::shared_ptr<kosherz::TimeZoneOffset> zone) {
    return kosherz::Zmanim(kosherz::times::sea_level_location(40.09, -74.22), date, kosherz::times::default_config(),
                           std::move(zone));
}

// Kiddush Levana is the one time whose day depends on the offset: with UTC-4 it falls on
// 3 December 2024, and with UTC+4 it does not.
void check_kiddush_levana() {
    int64_t ms = lakewood_on(december_3, std::make_shared<Fixed>(-4)).get(kosherz::Zman::TchilasZmanKidushLevana3Days);
    check(ms == 1733279333504LL, "UTC-4: kiddush levana is " + std::to_string(ms) + ", want 1733279333504");
    try {
        lakewood_on(december_3, std::make_shared<Fixed>(4)).get(kosherz::Zman::TchilasZmanKidushLevana3Days);
        check(false, "UTC+4 gave a kiddush levana time");
    } catch (const kosherz::NotApplicableError&) {
    }
    try {
        lakewood_on(december_3, nullptr).get(kosherz::Zman::TchilasZmanKidushLevana3Days);
        check(false, "no time zone gave a kiddush levana time");
    } catch (const kosherz::TimeZoneRequiredError&) {
    }
}

void check_failing_callback() {
    try {
        lakewood_on(april_23, std::make_shared<Broken>()).get(kosherz::Zman::Sunrise);
        check(false, "a throwing callback gave a sunrise");
    } catch (const kosherz::TimeZoneCallbackFailedError&) {
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
        check_failing_callback();
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
