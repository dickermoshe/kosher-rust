"""Calls kosher-rust from Python through the generated bindings. The README says how to run it."""

from datetime import datetime, timedelta, timezone

import kosherz as k


class UsEastern(k.TimeZoneOffset):
    """US Eastern time by the post-2007 rules, without a time zone database."""

    def offset_seconds(self, unix_seconds: int) -> int:
        instant = datetime.fromtimestamp(unix_seconds, timezone.utc)
        year = instant.year
        # Daylight time runs from 2:00 EST on the second Sunday of March to 2:00 EDT on the
        # first Sunday of November.
        march = datetime(year, 3, 8 + (6 - datetime(year, 3, 8).weekday()) % 7, 7, tzinfo=timezone.utc)
        november = datetime(year, 11, 1 + (6 - datetime(year, 11, 1).weekday()) % 7, 6, tzinfo=timezone.utc)
        return -4 * 3600 if march <= instant < november else -5 * 3600

    def local_to_unix_seconds(self, year: int, month: int, day: int, hour: int, minute: int, second: int) -> int:
        as_utc = int(datetime(year, month, day, hour, minute, second, tzinfo=timezone.utc).timestamp())
        # Daylight time gives the earlier instant, so it is tried first.
        for offset in (-4 * 3600, -5 * 3600):
            if self.offset_seconds(as_utc - offset) == offset:
                return as_utc - offset
        # The clocks skipped the time in spring. Java moves it later by the gap.
        return as_utc + 5 * 3600


eastern = UsEastern()


def local(ms: int) -> str:
    instant = datetime.fromtimestamp(ms / 1000, timezone.utc)
    return f"{instant + timedelta(seconds=eastern.offset_seconds(ms // 1000)):%H:%M:%S}"


lakewood = k.sea_level_location(40.09, -74.22)
day = k.Zmanim(lakewood, k.GregorianDate(2024, 4, 23), k.default_config(), eastern)
for zman in [k.Zman.Alos72Minutes, k.Zman.Sunrise, k.Zman.SofZmanShmaGra, k.Zman.Sunset, k.Zman.Tzais72Minutes]:
    print(f"{k.zman_name(zman):<28} {local(day.get(zman))}")
print(len(k.all_zmanim()), "zmanim in the catalog")

date = k.HebrewDate.from_gregorian(k.GregorianDate(2024, 4, 23))
print(date.display(), [k.holiday_name(h) for h in date.holidays(k.Region.Diaspora)])
print("Daf Yomi:", k.daf_yomi_bavli(date).display)

try:
    day.get(k.Zman.SofZmanAchilasChametzGra)
except k.NotApplicable as error:
    print("as expected:", error)
