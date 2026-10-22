"""Calls kosher-rust from Python through the generated bindings. The README says how to run it."""

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


new_york = IanaZone("America/New_York")


def local(ms: int) -> str:
    return f"{datetime.fromtimestamp(ms / 1000, new_york.zone):%H:%M:%S}"


lakewood = k.sea_level_location(40.09, -74.22)
day = k.Zmanim(lakewood, k.GregorianDate(2024, 4, 23), k.default_config(), new_york)
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
