"""Calls kosher-rust from Python through the generated bindings and checks the results. The README
says how to run it."""

import calendar
from datetime import datetime, timedelta
from zoneinfo import ZoneInfo

import kosherz as k


def local_day(zone: ZoneInfo, year: int, month: int, day: int) -> k.LocalDay:
    """The date in `zone`: its two local midnights and the offsets at them."""
    start = datetime(year, month, day, tzinfo=zone)
    end = datetime.combine(start.date() + timedelta(days=1), datetime.min.time(), tzinfo=zone)
    return k.LocalDay(
        start_ms=int(start.timestamp()) * 1000,
        start_offset_seconds=int(start.utcoffset().total_seconds()),
        end_ms=int(end.timestamp()) * 1000,
        end_offset_seconds=int(end.utcoffset().total_seconds()),
    )


def fixed_day(hours: int, year: int, month: int, day: int) -> k.LocalDay:
    """The date at UTC plus `hours` all year."""
    start = (calendar.timegm((year, month, day, 0, 0, 0)) - hours * 3600) * 1000
    return k.LocalDay(
        start_ms=start,
        start_offset_seconds=hours * 3600,
        end_ms=start + 86_400_000,
        end_offset_seconds=hours * 3600,
    )


new_york = ZoneInfo("America/New_York")


def local(ms: int) -> str:
    return f"{datetime.fromtimestamp(ms / 1000, new_york):%H:%M:%S}"


def raises(error: type, call) -> None:
    try:
        call()
    except error:
        return
    raise AssertionError(f"expected {error.__name__}")


lakewood = k.sea_level_location(40.09, -74.22)
april = k.GregorianDate(2024, 4, 23)
day = k.Zmanim(lakewood, april, k.default_config(), local_day(new_york, 2024, 4, 23))
assert local(day.get(k.Zman.Sunrise)) == "06:06:35"
assert local(day.get(k.Zman.SofZmanShmaGra)) == "09:31:00"
assert len(day.all()) == len(k.all_zmanim())
raises(k.NotApplicable, lambda: day.get(k.Zman.SofZmanAchilasChametzGra))

# Kiddush Levana is the one time whose day depends on the offset: with UTC-4 it falls on 3 December
# 2024, and with UTC+4 it does not.
december = k.GregorianDate(2024, 12, 3)
kiddush_levana = k.Zmanim(lakewood, december, k.default_config(), fixed_day(-4, 2024, 12, 3))
assert kiddush_levana.get(k.Zman.TchilasZmanKidushLevana3Days) == 1733279333504
wrong_sign = k.Zmanim(lakewood, december, k.default_config(), fixed_day(4, 2024, 12, 3))
raises(k.NotApplicable, lambda: wrong_sign.get(k.Zman.TchilasZmanKidushLevana3Days))
no_zone = k.Zmanim(lakewood, december, k.default_config(), None)
raises(k.TimeZoneRequired, lambda: no_zone.get(k.Zman.TchilasZmanKidushLevana3Days))

backwards = k.LocalDay(start_ms=1, start_offset_seconds=0, end_ms=0, end_offset_seconds=0)
raises(k.InvalidLocalDay, lambda: k.Zmanim(lakewood, april, k.default_config(), backwards))

date = k.HebrewDate.from_gregorian(april)
assert date.display() == "15 Nisan 5784"
assert [k.special_day_name(d) for d in date.special_days(k.Region.Diaspora)] == ["Pesach"]
assert k.daf_yomi_bavli(date).display == "Bava Metzia 55"

print("Python smoke test passed")
