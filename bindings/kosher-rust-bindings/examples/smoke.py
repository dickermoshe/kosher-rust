"""Calls kosher-rust from Python through the generated WeaveFFI bindings.

Run from bindings/kosher-ffi after `weaveffi generate`:
    KOSHER_FFI_LIBRARY=../../target/debug/kosher_ffi.dll PYTHONPATH=generated/python python examples/smoke.py
"""

from datetime import datetime, timedelta, timezone

import kosher_ffi as k

tz = "America/New_York"  # EDT, UTC-4, on this date
lakewood = k.Location(latitude=40.09, longitude=-74.22, timezone=tz, elevation=0.0,
                      eastern_horizon_elevation=0.0, western_horizon_elevation=0.0)
zmanim = k.Zmanim(lakewood, 2024, 4, 23, k.default_config())

for java_name in ["getAlos72Minutes", "getSunrise", "getSofZmanShmaGRA", "getSunset", "getTzais72Minutes"]:
    ms = zmanim.get(java_name)
    print(f"{java_name:<22} {datetime.fromtimestamp(ms / 1000, timezone(timedelta(hours=-4))):%H:%M:%S}")

print(len(k.zmanim_catalog()), "zmanim in the catalog")

day = k.HebrewDate.from_gregorian(2024, 4, 23)
print(day.display(), day.month().name, day.holidays(k.Region.Diaspora))

try:
    zmanim.get("getNoSuchZman")
except k.UnknownZman as e:
    print("error ok:", e)
try:
    k.Zmanim(k.Location(91.0, 0.0, None, 0.0, 0.0, 0.0), 2024, 1, 1, k.default_config())
except k.InvalidLatitude as e:
    print("error ok:", e)
