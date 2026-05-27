from __future__ import annotations
from typing import cast

import json
import re
from pathlib import Path

SCRIPT_DIR = Path(__file__).parent
INPUT = SCRIPT_DIR / "output" / "methods-with-documentation.json"
OUTPUT = SCRIPT_DIR.parent / "src" / "presets_gen.rs"

PRIMITIVES: dict[str, str] = {
    "getSunTransit": "ZmanPrimitive::SolarTransit",
    "getSolarMidnight": "ZmanPrimitive::SolarMidnight",
    "getBeginCivilTwilight": "ZmanPrimitive::BeginCivilTwilight",
    "getEndCivilTwilight": "ZmanPrimitive::EndCivilTwilight",
    "getBeginNauticalTwilight": "ZmanPrimitive::BeginNauticalTwilight",
    "getEndNauticalTwilight": "ZmanPrimitive::EndNauticalTwilight",
    "getBeginAstronomicalTwilight": "ZmanPrimitive::BeginAstronomicalTwilight",
    "getEndAstronomicalTwilight": "ZmanPrimitive::EndAstronomicalTwilight",
    "getSunsetOrWesternmostSolarAzimuth": "ZmanPrimitive::SunsetOrWesternmostSolarAzimuth",
    "getSunriseOrEasternmostSolarAzimuth": "ZmanPrimitive::SunriseOrEasternmostSolarAzimuth",
    "getSunrise": "ZmanPrimitive::ElevationAdjustedSunrise",
    "getSeaLevelSunrise": "ZmanPrimitive::SeaLevelSunrise",
    "getSunset": "ZmanPrimitive::ElevationAdjustedSunset",
    "getSeaLevelSunset": "ZmanPrimitive::SeaLevelSunset",
    "getAlos60Minutes": "ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunrise, Duration::from_mins(-60))",
    "getAlos72Minutes": "ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunrise, Duration::from_mins(-72))",
    "getAlos72Zmanis": "ZmanPrimitive::ZmanisOffset(&ZmanPrimitive::ConfiguredSunrise, -1.2)",
    "getAlos90Minutes": "ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunrise, Duration::from_mins(-90))",
    "getAlos90Zmanis": "ZmanPrimitive::ZmanisOffset(&ZmanPrimitive::ConfiguredSunrise, -1.5)",
    "getAlos96Minutes": "ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunrise, Duration::from_mins(-96))",
    "getAlos96Zmanis": "ZmanPrimitive::ZmanisOffset(&ZmanPrimitive::ConfiguredSunrise, -1.6)",
    "getAlos120Minutes": "ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunrise, Duration::from_mins(-120))",
    "getAlos120Zmanis": "ZmanPrimitive::ZmanisOffset(&ZmanPrimitive::ConfiguredSunrise, -2.0)",
    "getAlos16Point1Degrees": "ZmanPrimitive::SunriseOffsetByDegrees(16.1)",
    "getAlos18Degrees": "ZmanPrimitive::SunriseOffsetByDegrees(18.0)",
    "getAlos19Degrees": "ZmanPrimitive::SunriseOffsetByDegrees(19.0)",
    "getAlos19Point8Degrees": "ZmanPrimitive::SunriseOffsetByDegrees(19.8)",
    "getAlos26Degrees": "ZmanPrimitive::SunriseOffsetByDegrees(26.0)",
    "getAlosBaalHatanya": "ZmanPrimitive::SunriseOffsetByDegrees(16.9)",
    "getBainHashmashosRT13Point24Degrees": "ZmanPrimitive::SunsetOffsetByDegrees(13.24)",
    "getBainHashmashosRT58Point5Minutes": "ZmanPrimitive::Offset( &ZmanPrimitive::ConfiguredSunset, Duration::from_millis((58.5 * 60.0 * 1000.0) as i64), )",
    "getBainHashmashosRT13Point5MinutesBefore7Point083Degrees": "ZmanPrimitive::Offset( &ZmanPrimitive::SunsetOffsetByDegrees(7.0 + (5.0 / 60.0)), Duration::from_millis((-13.5 * 60.0 * 1000.0) as i64), )",
    "getBainHashmashosRT2Stars": "ZmanPrimitive::BainHashmashosRt2Stars",
    "getBainHashmashosYereim18Minutes": "ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunset, Duration::from_mins(-18))",
    "getBainHashmashosYereim16Point875Minutes": "ZmanPrimitive::Offset( &ZmanPrimitive::ConfiguredSunset, Duration::from_millis((-16.875 * 60.0 * 1000.0) as i64), )",
    "getBainHashmashosYereim13Point5Minutes": "ZmanPrimitive::Offset( &ZmanPrimitive::ConfiguredSunset, Duration::from_millis((-13.5 * 60.0 * 1000.0) as i64), )",
    "getBainHashmashosYereim2Point1Degrees": "ZmanPrimitive::SunsetOffsetByDegrees(-2.1)",
    "getBainHashmashosYereim2Point8Degrees": "ZmanPrimitive::SunsetOffsetByDegrees(-2.8)",
    "getBainHashmashosYereim3Point05Degrees": "ZmanPrimitive::SunsetOffsetByDegrees(-3.05)",
    "getCandleLighting": "ZmanPrimitive::CandleLighting",
    "getChatzosHayom": "ZmanPrimitive::SolarTransit",
    "getChatzosHalayla": "ZmanPrimitive::SolarMidnight",
    "getChatzosHayomAsHalfDay": "ZmanPrimitive::HalfDayBasedOffset( &ZmanPrimitive::SeaLevelSunrise, &ZmanPrimitive::SeaLevelSunset, 3.0, )",
    "getFixedLocalChatzosHayom": "ZmanPrimitive::LocalMeanTime(12.0)",
    "getMinchaGedolaGRA": "ZmanPrimitive::MinchaGedola( &ZmanPrimitive::ConfiguredSunrise, &ZmanPrimitive::ConfiguredSunset, true, )",
    "getMinchaGedola16Point1Degrees": "ZmanPrimitive::MinchaGedola( &ZmanPrimitive::SunriseOffsetByDegrees(16.1), &ZmanPrimitive::SunsetOffsetByDegrees(16.1), true, )",
    "getMinchaGedola30Minutes": "ZmanPrimitive::Offset(&ZmanPrimitive::SolarTransit, Duration::from_mins(30))",
    "getMinchaGedola72Minutes": "ZmanPrimitive::MinchaGedola( &ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunrise, Duration::from_mins(-72)), &ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunset, Duration::from_mins(72)), true, )",
    "getMinchaGedolaAhavatShalom": "ZmanPrimitive::MinchaGedolaAhavatShalom",
    "getMinchaGedolaGRAGreaterThan30": "ZmanPrimitive::MinchaGedolaGraGreaterThan30",
    "getMinchaGedolaAteretTorah": "ZmanPrimitive::MinchaGedola( &ZmanPrimitive::ZmanisOffset(&ZmanPrimitive::ConfiguredSunrise, -1.2), &ZmanPrimitive::TzaisAteretTorah, false, )",
    "getMinchaGedolaBaalHatanya": "ZmanPrimitive::MinchaGedola( &ZmanPrimitive::SunriseOffsetByDegrees(1.583), &ZmanPrimitive::SunsetOffsetByDegrees(1.583), true, )",
    "getMinchaGedolaGRAFixedLocalChatzos30Minutes": "ZmanPrimitive::Offset(&ZmanPrimitive::LocalMeanTime(12.0), Duration::from_mins(30))",
    "getMinchaKetanaGRA": "ZmanPrimitive::MinchaKetana( &ZmanPrimitive::ConfiguredSunrise, &ZmanPrimitive::ConfiguredSunset, true, )",
    "getMinchaKetana16Point1Degrees": "ZmanPrimitive::MinchaKetana( &ZmanPrimitive::SunriseOffsetByDegrees(16.1), &ZmanPrimitive::SunsetOffsetByDegrees(16.1), true, )",
    "getMinchaKetana72Minutes": "ZmanPrimitive::MinchaKetana( &ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunrise, Duration::from_mins(-72)), &ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunset, Duration::from_mins(72)), true, )",
    "getMinchaKetanaAhavatShalom": "ZmanPrimitive::MinchaKetanaAhavatShalom",
    "getMinchaKetanaAteretTorah": "ZmanPrimitive::MinchaKetana( &ZmanPrimitive::ZmanisOffset(&ZmanPrimitive::ConfiguredSunrise, -1.2), &ZmanPrimitive::TzaisAteretTorah, false, )",
    "getMinchaKetanaBaalHatanya": "ZmanPrimitive::MinchaKetana( &ZmanPrimitive::SunriseOffsetByDegrees(1.583), &ZmanPrimitive::SunsetOffsetByDegrees(1.583), true, )",
    "getMinchaKetanaGRAFixedLocalChatzosToSunset": "ZmanPrimitive::HalfDayBasedOffset( &ZmanPrimitive::LocalMeanTime(12.0), &ZmanPrimitive::ConfiguredSunset, 3.5, )",
    "getMisheyakir10Point2Degrees": "ZmanPrimitive::SunriseOffsetByDegrees(10.2)",
    "getMisheyakir11Degrees": "ZmanPrimitive::SunriseOffsetByDegrees(11.0)",
    "getMisheyakir11Point5Degrees": "ZmanPrimitive::SunriseOffsetByDegrees(11.5)",
    "getMisheyakir12Point85Degrees": "ZmanPrimitive::SunriseOffsetByDegrees(12.85)",
    "getMisheyakir7Point65Degrees": "ZmanPrimitive::SunriseOffsetByDegrees(7.65)",
    "getMisheyakir9Point5Degrees": "ZmanPrimitive::SunriseOffsetByDegrees(9.5)",
    "getPlagHaminchaGRA": "ZmanPrimitive::PlagHamincha( &ZmanPrimitive::ConfiguredSunrise, &ZmanPrimitive::ConfiguredSunset, true, )",
    "getPlagAhavatShalom": "ZmanPrimitive::PlagAhavatShalom",
    "getPlagAlos16Point1ToTzaisGeonim7Point083Degrees": "ZmanPrimitive::PlagHamincha( &ZmanPrimitive::SunriseOffsetByDegrees(16.1), &ZmanPrimitive::SunsetOffsetByDegrees(7.0 + (5.0 / 60.0)), false, )",
    "getPlagAlosToSunset": "ZmanPrimitive::PlagHamincha( &ZmanPrimitive::SunriseOffsetByDegrees(16.1), &ZmanPrimitive::ConfiguredSunset, false, )",
    "getPlagHamincha60Minutes": "ZmanPrimitive::PlagHamincha( &ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunrise, Duration::from_mins(-60)), &ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunset, Duration::from_mins(60)), true, )",
    "getPlagHamincha72Minutes": "ZmanPrimitive::PlagHamincha( &ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunrise, Duration::from_mins(-72)), &ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunset, Duration::from_mins(72)), true, )",
    "getPlagHamincha72MinutesZmanis": "ZmanPrimitive::PlagHamincha( &ZmanPrimitive::ZmanisOffset(&ZmanPrimitive::ConfiguredSunrise, -1.2), &ZmanPrimitive::ZmanisOffset(&ZmanPrimitive::ConfiguredSunset, 1.2), true, )",
    "getPlagHamincha90Minutes": "ZmanPrimitive::PlagHamincha( &ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunrise, Duration::from_mins(-90)), &ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunset, Duration::from_mins(90)), true, )",
    "getPlagHamincha90MinutesZmanis": "ZmanPrimitive::PlagHamincha( &ZmanPrimitive::ZmanisOffset(&ZmanPrimitive::ConfiguredSunrise, -1.5),&ZmanPrimitive::ZmanisOffset(&ZmanPrimitive::ConfiguredSunset, 1.5), true, )",
    "getPlagHamincha96Minutes": "ZmanPrimitive::PlagHamincha( &ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunrise, Duration::from_mins(-96)), &ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunset, Duration::from_mins(96)), true, )",
    "getPlagHamincha96MinutesZmanis": "ZmanPrimitive::PlagHamincha( &ZmanPrimitive::ZmanisOffset(&ZmanPrimitive::ConfiguredSunrise, -1.6), &ZmanPrimitive::ZmanisOffset(&ZmanPrimitive::ConfiguredSunset, 1.6), true, )",
    "getPlagHamincha120Minutes": "ZmanPrimitive::PlagHamincha( &ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunrise, Duration::from_mins(-120)), &ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunset, Duration::from_mins(120)), true, )",
    "getPlagHamincha120MinutesZmanis": "ZmanPrimitive::PlagHamincha( &ZmanPrimitive::ZmanisOffset(&ZmanPrimitive::ConfiguredSunrise, -2.0), &ZmanPrimitive::ZmanisOffset(&ZmanPrimitive::ConfiguredSunset, 2.0), true, )",
    "getPlagHamincha16Point1Degrees": "ZmanPrimitive::PlagHamincha( &ZmanPrimitive::SunriseOffsetByDegrees(16.1), &ZmanPrimitive::SunsetOffsetByDegrees(16.1), true, )",
    "getPlagHamincha18Degrees": "ZmanPrimitive::PlagHamincha( &ZmanPrimitive::SunriseOffsetByDegrees(18.0), &ZmanPrimitive::SunsetOffsetByDegrees(18.0), true, )",
    "getPlagHamincha19Point8Degrees": "ZmanPrimitive::PlagHamincha( &ZmanPrimitive::SunriseOffsetByDegrees(19.8), &ZmanPrimitive::SunsetOffsetByDegrees(19.8), true, )",
    "getPlagHamincha26Degrees": "ZmanPrimitive::PlagHamincha( &ZmanPrimitive::SunriseOffsetByDegrees(26.0), &ZmanPrimitive::SunsetOffsetByDegrees(26.0), true, )",
    "getPlagHaminchaAteretTorah": "ZmanPrimitive::PlagHamincha( &ZmanPrimitive::ZmanisOffset(&ZmanPrimitive::ConfiguredSunrise, -1.2), &ZmanPrimitive::TzaisAteretTorah, false, )",
    "getPlagHaminchaBaalHatanya": "ZmanPrimitive::PlagHamincha( &ZmanPrimitive::SunriseOffsetByDegrees(1.583), &ZmanPrimitive::SunsetOffsetByDegrees(1.583), true, )",
    "getPlagHaminchaGRAFixedLocalChatzosToSunset": "ZmanPrimitive::HalfDayBasedOffset( &ZmanPrimitive::LocalMeanTime(12.0), &ZmanPrimitive::ConfiguredSunset, 4.75, )",
    "getSamuchLeMinchaKetanaGRA": "ZmanPrimitive::SamuchLeMinchaKetana( &ZmanPrimitive::ConfiguredSunrise, &ZmanPrimitive::ConfiguredSunset, true, )",
    "getSamuchLeMinchaKetana16Point1Degrees": "ZmanPrimitive::SamuchLeMinchaKetana( &ZmanPrimitive::SunriseOffsetByDegrees(16.1), &ZmanPrimitive::SunsetOffsetByDegrees(16.1), true, )",
    "getSamuchLeMinchaKetana72Minutes": "ZmanPrimitive::SamuchLeMinchaKetana( &ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunrise, Duration::from_mins(-72)), &ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunset, Duration::from_mins(72)), true, )",
    "getSofZmanAchilasChametzGRA": "ZmanPrimitive::Tefila( &ZmanPrimitive::ConfiguredSunrise, &ZmanPrimitive::ConfiguredSunset, true, )",
    "getSofZmanAchilasChametzMGA72Minutes": "ZmanPrimitive::Tefila( &ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunrise, Duration::from_mins(-72)), &ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunset, Duration::from_mins(72)), true, )",
    "getSofZmanAchilasChametzMGA72MinutesZmanis": "ZmanPrimitive::Tefila( &ZmanPrimitive::ZmanisOffset(&ZmanPrimitive::ConfiguredSunrise, -1.2), &ZmanPrimitive::ZmanisOffset(&ZmanPrimitive::ConfiguredSunset, 1.2), true, )",
    "getSofZmanAchilasChametzMGA16Point1Degrees": "ZmanPrimitive::Tefila( &ZmanPrimitive::SunriseOffsetByDegrees(16.1), &ZmanPrimitive::SunsetOffsetByDegrees(16.1), true, )",
    "getSofZmanAchilasChametzBaalHatanya": "ZmanPrimitive::Tefila( &ZmanPrimitive::SunriseOffsetByDegrees(1.583), &ZmanPrimitive::SunsetOffsetByDegrees(1.583), true, )",
    "getSofZmanBiurChametzGRA": "ZmanPrimitive::SofZmanBiurChametz( &ZmanPrimitive::ConfiguredSunrise, &ZmanPrimitive::ConfiguredSunset, true, )",
    "getSofZmanBiurChametzMGA72Minutes": "ZmanPrimitive::SofZmanBiurChametz( &ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunrise, Duration::from_mins(-72)), &ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunset, Duration::from_mins(72)), true, )",
    "getSofZmanBiurChametzMGA72MinutesZmanis": "ZmanPrimitive::SofZmanBiurChametz( &ZmanPrimitive::ZmanisOffset(&ZmanPrimitive::ConfiguredSunrise, -1.2), &ZmanPrimitive::ZmanisOffset(&ZmanPrimitive::ConfiguredSunset, 1.2), true, )",
    "getSofZmanBiurChametzMGA16Point1Degrees": "ZmanPrimitive::SofZmanBiurChametz( &ZmanPrimitive::SunriseOffsetByDegrees(16.1), &ZmanPrimitive::SunsetOffsetByDegrees(16.1), true, )",
    "getSofZmanBiurChametzBaalHatanya": "ZmanPrimitive::SofZmanBiurChametz( &ZmanPrimitive::SunriseOffsetByDegrees(1.583), &ZmanPrimitive::SunsetOffsetByDegrees(1.583), true, )",
    "getSofZmanShmaGRA": "ZmanPrimitive::Shema( &ZmanPrimitive::ConfiguredSunrise, &ZmanPrimitive::ConfiguredSunset, true, )",
    "getSofZmanShmaMGA19Point8Degrees": "ZmanPrimitive::Shema( &ZmanPrimitive::SunriseOffsetByDegrees(19.8), &ZmanPrimitive::SunsetOffsetByDegrees(19.8), true, )",
    "getSofZmanShmaMGA16Point1Degrees": "ZmanPrimitive::Shema( &ZmanPrimitive::SunriseOffsetByDegrees(16.1), &ZmanPrimitive::SunsetOffsetByDegrees(16.1), true, )",
    "getSofZmanShmaMGA18Degrees": "ZmanPrimitive::Shema( &ZmanPrimitive::SunriseOffsetByDegrees(18.0), &ZmanPrimitive::SunsetOffsetByDegrees(18.0), true, )",
    "getSofZmanShmaMGA72Minutes": "ZmanPrimitive::Shema( &ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunrise, Duration::from_mins(-72)), &ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunset, Duration::from_mins(72)), true, )",
    "getSofZmanShmaMGA72MinutesZmanis": "ZmanPrimitive::Shema( &ZmanPrimitive::ZmanisOffset(&ZmanPrimitive::ConfiguredSunrise, -1.2), &ZmanPrimitive::ZmanisOffset(&ZmanPrimitive::ConfiguredSunset, 1.2), true, )",
    "getSofZmanShmaMGA90Minutes": "ZmanPrimitive::Shema( &ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunrise, Duration::from_mins(-90)), &ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunset, Duration::from_mins(90)), true, )",
    "getSofZmanShmaMGA90MinutesZmanis": "ZmanPrimitive::Shema( &ZmanPrimitive::ZmanisOffset(&ZmanPrimitive::ConfiguredSunrise, -1.5), &ZmanPrimitive::ZmanisOffset(&ZmanPrimitive::ConfiguredSunset, 1.5), true, )",
    "getSofZmanShmaMGA96Minutes": "ZmanPrimitive::Shema( &ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunrise, Duration::from_mins(-96)), &ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunset, Duration::from_mins(96)), true, )",
    "getSofZmanShmaMGA96MinutesZmanis": "ZmanPrimitive::Shema( &ZmanPrimitive::ZmanisOffset(&ZmanPrimitive::ConfiguredSunrise, -1.6), &ZmanPrimitive::ZmanisOffset(&ZmanPrimitive::ConfiguredSunset, 1.6), true, )",
    "getSofZmanShma3HoursBeforeChatzos": "ZmanPrimitive::Offset(&ZmanPrimitive::SolarTransit, Duration::from_mins(-180))",
    "getSofZmanShmaMGA120Minutes": "ZmanPrimitive::Shema( &ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunrise, Duration::from_mins(-120)), &ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunset, Duration::from_mins(120)), true, )",
    "getSofZmanShmaAlos16Point1ToSunset": "ZmanPrimitive::Shema( &ZmanPrimitive::SunriseOffsetByDegrees(16.1), &ZmanPrimitive::ConfiguredSunset, false, )",
    "getSofZmanShmaAlos16Point1ToTzaisGeonim7Point083Degrees": "ZmanPrimitive::Shema( &ZmanPrimitive::SunriseOffsetByDegrees(16.1), &ZmanPrimitive::SunsetOffsetByDegrees(7.0 + (5.0 / 60.0)), false, )",
    "getSofZmanShmaAteretTorah": "ZmanPrimitive::Shema( &ZmanPrimitive::ZmanisOffset(&ZmanPrimitive::ConfiguredSunrise, -1.2), &ZmanPrimitive::TzaisAteretTorah, false, )",
    "getSofZmanShmaBaalHatanya": "ZmanPrimitive::Shema( &ZmanPrimitive::SunriseOffsetByDegrees(1.583), &ZmanPrimitive::SunsetOffsetByDegrees(1.583), true, )",
    "getSofZmanShmaGRASunriseToFixedLocalChatzos": "ZmanPrimitive::HalfDayBasedOffset( &ZmanPrimitive::ConfiguredSunrise, &ZmanPrimitive::LocalMeanTime(12.0), 3.0, )",
    "getSofZmanShmaMGA18DegreesToFixedLocalChatzos": "ZmanPrimitive::HalfDayBasedOffset( &ZmanPrimitive::SunriseOffsetByDegrees(18.0), &ZmanPrimitive::LocalMeanTime(12.0), 3.0, )",
    "getSofZmanShmaMGA16Point1DegreesToFixedLocalChatzos": "ZmanPrimitive::HalfDayBasedOffset( &ZmanPrimitive::SunriseOffsetByDegrees(16.1), &ZmanPrimitive::LocalMeanTime(12.0), 3.0, )",
    "getSofZmanShmaMGA90MinutesToFixedLocalChatzos": "ZmanPrimitive::HalfDayBasedOffset( &ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunrise, Duration::from_mins(-90)), &ZmanPrimitive::LocalMeanTime(12.0), 3.0, )",
    "getSofZmanShmaMGA72MinutesToFixedLocalChatzos": "ZmanPrimitive::HalfDayBasedOffset( &ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunrise, Duration::from_mins(-72)), &ZmanPrimitive::LocalMeanTime(12.0), 3.0, )",
    "getSofZmanTfilaGRA": "ZmanPrimitive::Tefila( &ZmanPrimitive::ConfiguredSunrise, &ZmanPrimitive::ConfiguredSunset, true, )",
    "getSofZmanTfilaMGA72Minutes": "ZmanPrimitive::Tefila( &ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunrise, Duration::from_mins(-72)), &ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunset, Duration::from_mins(72)), true, )",
    "getSofZmanTfilaMGA19Point8Degrees": "ZmanPrimitive::Tefila( &ZmanPrimitive::SunriseOffsetByDegrees(19.8), &ZmanPrimitive::SunsetOffsetByDegrees(19.8), true, )",
    "getSofZmanTfilaMGA16Point1Degrees": "ZmanPrimitive::Tefila( &ZmanPrimitive::SunriseOffsetByDegrees(16.1), &ZmanPrimitive::SunsetOffsetByDegrees(16.1), true, )",
    "getSofZmanTfilaMGA18Degrees": "ZmanPrimitive::Tefila( &ZmanPrimitive::SunriseOffsetByDegrees(18.0), &ZmanPrimitive::SunsetOffsetByDegrees(18.0), true, )",
    "getSofZmanTfilaMGA72MinutesZmanis": "ZmanPrimitive::Tefila( &ZmanPrimitive::ZmanisOffset(&ZmanPrimitive::ConfiguredSunrise, -1.2), &ZmanPrimitive::ZmanisOffset(&ZmanPrimitive::ConfiguredSunset, 1.2), true, )",
    "getSofZmanTfilaMGA90Minutes": "ZmanPrimitive::Tefila( &ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunrise, Duration::from_mins(-90)), &ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunset, Duration::from_mins(90)), true, )",
    "getSofZmanTfilaMGA90MinutesZmanis": "ZmanPrimitive::Tefila( &ZmanPrimitive::ZmanisOffset(&ZmanPrimitive::ConfiguredSunrise, -1.5), &ZmanPrimitive::ZmanisOffset(&ZmanPrimitive::ConfiguredSunset, 1.5), true, )",
    "getSofZmanTfilaMGA96Minutes": "ZmanPrimitive::Tefila( &ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunrise, Duration::from_mins(-96)), &ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunset, Duration::from_mins(96)), true, )",
    "getSofZmanTfilaMGA96MinutesZmanis": "ZmanPrimitive::Tefila( &ZmanPrimitive::ZmanisOffset(&ZmanPrimitive::ConfiguredSunrise, -1.6), &ZmanPrimitive::ZmanisOffset(&ZmanPrimitive::ConfiguredSunset, 1.6), true, )",
    "getSofZmanTfila2HoursBeforeChatzos": "ZmanPrimitive::Offset(&ZmanPrimitive::SolarTransit, Duration::from_mins(-120))",
    "getSofZmanTfilaMGA120Minutes": "ZmanPrimitive::Tefila( &ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunrise, Duration::from_mins(-120)), &ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunset, Duration::from_mins(120)), true, )",
    "getSofZmanTfilaAteretTorah": "ZmanPrimitive::Tefila( &ZmanPrimitive::ZmanisOffset(&ZmanPrimitive::ConfiguredSunrise, -1.2), &ZmanPrimitive::TzaisAteretTorah, false, )",
    "getSofZmanTfilaBaalHatanya": "ZmanPrimitive::Tefila( &ZmanPrimitive::SunriseOffsetByDegrees(1.583), &ZmanPrimitive::SunsetOffsetByDegrees(1.583), true, )",
    "getSofZmanTfilaGRASunriseToFixedLocalChatzos": "ZmanPrimitive::HalfDayBasedOffset( &ZmanPrimitive::ConfiguredSunrise, &ZmanPrimitive::LocalMeanTime(12.0), 4.0, )",
    "getTzaisGeonim8Point5Degrees": "ZmanPrimitive::SunsetOffsetByDegrees(8.5)",
    "getTzais50Minutes": "ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunset, Duration::from_mins(50))",
    "getTzais60Minutes": "ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunset, Duration::from_mins(60))",
    "getTzais72Minutes": "ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunset, Duration::from_mins(72))",
    "getTzais72Zmanis": "ZmanPrimitive::ZmanisOffset(&ZmanPrimitive::ConfiguredSunset, 1.2)",
    "getTzais90Minutes": "ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunset, Duration::from_mins(90))",
    "getTzais90Zmanis": "ZmanPrimitive::ZmanisOffset(&ZmanPrimitive::ConfiguredSunset, 1.5)",
    "getTzais96Minutes": "ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunset, Duration::from_mins(96))",
    "getTzais96Zmanis": "ZmanPrimitive::ZmanisOffset(&ZmanPrimitive::ConfiguredSunset, 1.6)",
    "getTzais120Minutes": "ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunset, Duration::from_mins(120))",
    "getTzais120Zmanis": "ZmanPrimitive::ZmanisOffset(&ZmanPrimitive::ConfiguredSunset, 2.0)",
    "getTzais16Point1Degrees": "ZmanPrimitive::SunsetOffsetByDegrees(16.1)",
    "getTzais18Degrees": "ZmanPrimitive::SunsetOffsetByDegrees(18.0)",
    "getTzais19Point8Degrees": "ZmanPrimitive::SunsetOffsetByDegrees(19.8)",
    "getTzais26Degrees": "ZmanPrimitive::SunsetOffsetByDegrees(26.0)",
    "getTzaisAteretTorah": "ZmanPrimitive::TzaisAteretTorah",
    "getTzaisBaalHatanya": "ZmanPrimitive::SunsetOffsetByDegrees(6.0)",
    "getTzaisGeonim3Point7Degrees": "ZmanPrimitive::SunsetOffsetByDegrees(3.7)",
    "getTzaisGeonim3Point8Degrees": "ZmanPrimitive::SunsetOffsetByDegrees(3.8)",
    "getTzaisGeonim5Point95Degrees": "ZmanPrimitive::SunsetOffsetByDegrees(5.95)",
    "getTzaisGeonim4Point66Degrees": "ZmanPrimitive::SunsetOffsetByDegrees(4.66)",
    "getTzaisGeonim4Point42Degrees": "ZmanPrimitive::SunsetOffsetByDegrees(4.42)",
    "getTzaisGeonim4Point8Degrees": "ZmanPrimitive::SunsetOffsetByDegrees(4.8)",
    "getTzaisGeonim6Point45Degrees": "ZmanPrimitive::SunsetOffsetByDegrees(6.45)",
    "getTzaisGeonim7Point083Degrees": "ZmanPrimitive::SunsetOffsetByDegrees(7.0 + (5.0 / 60.0))",
    "getTzaisGeonim7Point67Degrees": "ZmanPrimitive::SunsetOffsetByDegrees(7.67)",
    "getTzaisGeonim9Point3Degrees": "ZmanPrimitive::SunsetOffsetByDegrees(9.3)",
    "getTzaisGeonim9Point75Degrees": "ZmanPrimitive::SunsetOffsetByDegrees(9.75)",
    "getSofZmanKidushLevana15Days": "ZmanPrimitive::SofZmanKidushLevana15Days",
    "getSofZmanKidushLevanaBetweenMoldos": "ZmanPrimitive::SofZmanKidushLevanaBetweenMoldos",
    "getTchilasZmanKidushLevana3Days": "ZmanPrimitive::TchilasZmanKidushLevana3Days",
    "getTchilasZmanKidushLevana7Days": "ZmanPrimitive::TchilasZmanKidushLevana7Days",
    "getZmanMolad": "ZmanPrimitive::Molad",
}


def method_to_const(method_name: str) -> str:
    if not method_name.startswith("get"):
        raise ValueError(f"Expected Java getter name, got {method_name!r}")

    name = method_name[3:]
    with_underscores = re.sub(r"([a-z0-9])([A-Z])", r"\1_\2", name)
    with_underscores = re.sub(r"([a-zA-Z])(\d)", r"\1_\2", with_underscores)
    with_underscores = re.sub(r"(\d)([a-zA-Z])", r"\1_\2", with_underscores)
    return with_underscores.upper()


def load_methods(path: Path) -> list[dict[str, object]]:
    data = json.loads(path.read_text(encoding="utf-8-sig"))
    if not isinstance(data, list):
        raise ValueError(f"Expected a JSON list in {path}")
    if not data:
        raise ValueError(f"No methods found in {path}")
    return data


def rust_doc_comment(text: str) -> str:
    """Format user-facing text as Rust line doc comments."""
    lines = text.strip().splitlines() or ["Generated zman preset."]
    return "\n".join(f"/// {line}" if line else "///" for line in lines)


def preset_block(
    const_name: str, method_name: str, event: str, display_name: str
) -> str:
    literal = json.dumps(display_name, ensure_ascii=False)
    doc = rust_doc_comment(display_name)
    return f"""{doc}
pub static {const_name}: ZmanPreset = ZmanPreset {{
    event: {event},
    #[cfg(test)]
    method_name: {json.dumps(method_name)},
    name: {literal},
    #[cfg(feature = "alloc")]
    description: |_| {literal}.to_string(),
}};
"""


def all_presets_array(const_names: list[str]) -> str:
    entries = ",\n".join(f"    &{const_name}" for const_name in const_names)
    return f"""/// Every generated zman preset.
pub static ALL: &[&ZmanPreset] = &[
{entries},
];
"""


def generate(methods: list[dict[str, object]]) -> str:
    presets: list[tuple[str, str]] = []
    seen_consts: set[str] = set()
    skipped: list[str] = []

    for item in methods:
        if not isinstance(item, dict):
            raise ValueError("Each method entry must be an object")

        method_name = item.get("name")
        if not isinstance(method_name, str):
            raise ValueError("Each method entry must have a string name")

        event = PRIMITIVES.get(method_name)
        if event is None:
            skipped.append(method_name)
            continue

        const_name = method_to_const(method_name)
        if const_name in seen_consts:
            raise ValueError(f"Duplicate preset constant {const_name}")
        seen_consts.add(const_name)

        user_docs: dict[str, str] = cast(dict[str, str], item.get("user_docs"))
        if not isinstance(user_docs, dict):
            raise ValueError(f"{method_name} is missing user_docs")

        meaning = user_docs.get("meaning")
        if not isinstance(meaning, str) or not meaning.strip():
            raise ValueError(f"{method_name} is missing user_docs.meaning")

        presets.append(
            (const_name, preset_block(const_name, method_name, event, meaning))
        )

    if not presets:
        raise RuntimeError(
            "No presets were generated; add primitive mappings for JSON methods."
        )

    if skipped:
        raise ValueError(
            f"Skipped {len(skipped)} JSON methods without primitive mappings: {skipped}"
        )

    presets.sort(key=lambda preset: preset[0])
    const_names = [const_name for const_name, _ in presets]
    blocks = [block for _, block in presets]

    header = """//! Generated by tools/generate-rust.py. Do not edit by hand.

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::string::ToString;

use crate::presets::ZmanPreset;
use crate::primitive_zman::ZmanPrimitive;
use jiff::SignedDuration as Duration;

"""
    return header + "\n".join(blocks) + "\n" + all_presets_array(const_names) + "\n"


def main() -> None:
    methods = load_methods(INPUT)
    OUTPUT.write_text(generate(methods), encoding="utf-8", newline="\n")
    print(f"Wrote presets to {OUTPUT}.")


if __name__ == "__main__":
    main()
