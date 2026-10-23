"""Calls kosher-rust from Python through the generated bindings and checks the results. The README
says how to run it."""

import calendar
from datetime import datetime
from zoneinfo import ZoneInfo

import kosherz as k


class IanaZone(k.TimeZoneOffset):
    """An IANA time zone, through Python's zoneinfo."""

    def __init__(self, name: str) -> None:
        self.zone = ZoneInfo(name)

    def offset_seconds(self, unix_seconds: int) -> int:
        return int(datetime.fromtimestamp(unix_seconds, self.zone).utcoffset().total_seconds())

    def local_to_unix_seconds(self, year: int, month: int, day: int, hour: int, minute: int, second: int) -> int:
        # fold=0 resolves a skipped or repeated local time the way Java's ZonedDateTime.of does.
        return int(datetime(year, month, day, hour, minute, second, tzinfo=self.zone, fold=0).timestamp())


class FixedZone(k.TimeZoneOffset):
    """UTC plus `hours` all year."""

    def __init__(self, hours: int) -> None:
        self.seconds = hours * 3600

    def offset_seconds(self, unix_seconds: int) -> int:
        return self.seconds

    def local_to_unix_seconds(self, year: int, month: int, day: int, hour: int, minute: int, second: int) -> int:
        return calendar.timegm((year, month, day, hour, minute, second)) - self.seconds


class BrokenZone(k.TimeZoneOffset):
    """A time zone whose callbacks fail."""

    def offset_seconds(self, unix_seconds: int) -> int:
        raise RuntimeError("no time zone data")

    def local_to_unix_seconds(self, year: int, month: int, day: int, hour: int, minute: int, second: int) -> int:
        raise RuntimeError("no time zone data")


new_york = IanaZone("America/New_York")


def local(ms: int) -> str:
    return f"{datetime.fromtimestamp(ms / 1000, new_york.zone):%H:%M:%S}"


def raises(error: type, call) -> None:
    try:
        call()
    except error:
        return
    raise AssertionError(f"expected {error.__name__}")


lakewood = k.sea_level_location(40.09, -74.22)
day = k.Zmanim(lakewood, k.GregorianDate(2024, 4, 23), k.default_config(), new_york)
assert local(day.get(k.Zman.Sunrise)) == "06:06:35"
assert local(day.get(k.Zman.SofZmanShmaGra)) == "09:31:00"
assert len(day.all()) == len(k.all_zmanim())
raises(k.NotApplicable, lambda: day.get(k.Zman.SofZmanAchilasChametzGra))

# Kiddush Levana is the one time whose day depends on the offset: with UTC-4 it falls on 3 December
# 2024, and with UTC+4 or UTC it does not.
kiddush_levana = k.Zmanim(lakewood, k.GregorianDate(2024, 12, 3), k.default_config(), FixedZone(-4))
assert kiddush_levana.get(k.Zman.TchilasZmanKidushLevana3Days) == 1733279333504
wrong_sign = k.Zmanim(lakewood, k.GregorianDate(2024, 12, 3), k.default_config(), FixedZone(4))
raises(k.NotApplicable, lambda: wrong_sign.get(k.Zman.TchilasZmanKidushLevana3Days))
no_zone = k.Zmanim(lakewood, k.GregorianDate(2024, 12, 3), k.default_config(), None)
raises(k.TimeZoneRequired, lambda: no_zone.get(k.Zman.TchilasZmanKidushLevana3Days))

broken = k.Zmanim(lakewood, k.GregorianDate(2024, 4, 23), k.default_config(), BrokenZone())
raises(k.TimeZoneCallbackFailed, lambda: broken.get(k.Zman.Sunrise))

date = k.HebrewDate.from_gregorian(k.GregorianDate(2024, 4, 23))
assert date.display() == "15 Nisan 5784"
assert [k.special_day_name(d) for d in date.special_days(k.Region.Diaspora)] == ["Pesach"]
assert k.daf_yomi_bavli(date).display == "Bava Metzia 55"

print("Python smoke test passed")
