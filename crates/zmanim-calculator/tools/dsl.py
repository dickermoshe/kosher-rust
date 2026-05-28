from __future__ import annotations

from typing import Annotated, Literal, Union, cast

from pydantic import BaseModel, Field

ZmanimMethods = Literal[
    "getSunTransit",
    "getSolarMidnight",
    "getBeginCivilTwilight",
    "getEndCivilTwilight",
    "getBeginNauticalTwilight",
    "getEndNauticalTwilight",
    "getBeginAstronomicalTwilight",
    "getEndAstronomicalTwilight",
    "getSunsetOrWesternmostSolarAzimuth",
    "getSunriseOrEasternmostSolarAzimuth",
    "getSunrise",
    "getSeaLevelSunrise",
    "getSunset",
    "getSeaLevelSunset",
    "getAlos60Minutes",
    "getAlos72Minutes",
    "getAlos72Zmanis",
    "getAlos90Minutes",
    "getAlos90Zmanis",
    "getAlos96Minutes",
    "getAlos96Zmanis",
    "getAlos120Minutes",
    "getAlos120Zmanis",
    "getAlos16Point1Degrees",
    "getAlos18Degrees",
    "getAlos19Degrees",
    "getAlos19Point8Degrees",
    "getAlos26Degrees",
    "getAlosBaalHatanya",
    "getBainHashmashosRT13Point24Degrees",
    "getBainHashmashosRT58Point5Minutes",
    "getBainHashmashosRT13Point5MinutesBefore7Point083Degrees",
    "getBainHashmashosRT2Stars",
    "getBainHashmashosYereim18Minutes",
    "getBainHashmashosYereim16Point875Minutes",
    "getBainHashmashosYereim13Point5Minutes",
    "getBainHashmashosYereim2Point1Degrees",
    "getBainHashmashosYereim2Point8Degrees",
    "getBainHashmashosYereim3Point05Degrees",
    "getCandleLighting",
    "getChatzosHayom",
    "getChatzosHalayla",
    "getChatzosHayomAsHalfDay",
    "getFixedLocalChatzosHayom",
    "getMinchaGedolaGRA",
    "getMinchaGedola16Point1Degrees",
    "getMinchaGedola30Minutes",
    "getMinchaGedola72Minutes",
    "getMinchaGedolaAhavatShalom",
    "getMinchaGedolaGRAGreaterThan30",
    "getMinchaGedolaAteretTorah",
    "getMinchaGedolaBaalHatanya",
    "getMinchaGedolaGRAFixedLocalChatzos30Minutes",
    "getMinchaKetanaGRA",
    "getMinchaKetana16Point1Degrees",
    "getMinchaKetana72Minutes",
    "getMinchaKetanaAhavatShalom",
    "getMinchaKetanaAteretTorah",
    "getMinchaKetanaBaalHatanya",
    "getMinchaKetanaGRAFixedLocalChatzosToSunset",
    "getMisheyakir10Point2Degrees",
    "getMisheyakir11Degrees",
    "getMisheyakir11Point5Degrees",
    "getMisheyakir12Point85Degrees",
    "getMisheyakir7Point65Degrees",
    "getMisheyakir9Point5Degrees",
    "getPlagHaminchaGRA",
    "getPlagAhavatShalom",
    "getPlagAlos16Point1ToTzaisGeonim7Point083Degrees",
    "getPlagAlosToSunset",
    "getPlagHamincha60Minutes",
    "getPlagHamincha72Minutes",
    "getPlagHamincha72MinutesZmanis",
    "getPlagHamincha90Minutes",
    "getPlagHamincha90MinutesZmanis",
    "getPlagHamincha96Minutes",
    "getPlagHamincha96MinutesZmanis",
    "getPlagHamincha120Minutes",
    "getPlagHamincha120MinutesZmanis",
    "getPlagHamincha16Point1Degrees",
    "getPlagHamincha18Degrees",
    "getPlagHamincha19Point8Degrees",
    "getPlagHamincha26Degrees",
    "getPlagHaminchaAteretTorah",
    "getPlagHaminchaBaalHatanya",
    "getPlagHaminchaGRAFixedLocalChatzosToSunset",
    "getSamuchLeMinchaKetanaGRA",
    "getSamuchLeMinchaKetana16Point1Degrees",
    "getSamuchLeMinchaKetana72Minutes",
    "getSofZmanAchilasChametzGRA",
    "getSofZmanAchilasChametzMGA72Minutes",
    "getSofZmanAchilasChametzMGA72MinutesZmanis",
    "getSofZmanAchilasChametzMGA16Point1Degrees",
    "getSofZmanAchilasChametzBaalHatanya",
    "getSofZmanBiurChametzGRA",
    "getSofZmanBiurChametzMGA72Minutes",
    "getSofZmanBiurChametzMGA72MinutesZmanis",
    "getSofZmanBiurChametzMGA16Point1Degrees",
    "getSofZmanBiurChametzBaalHatanya",
    "getSofZmanShmaGRA",
    "getSofZmanShmaMGA19Point8Degrees",
    "getSofZmanShmaMGA16Point1Degrees",
    "getSofZmanShmaMGA18Degrees",
    "getSofZmanShmaMGA72Minutes",
    "getSofZmanShmaMGA72MinutesZmanis",
    "getSofZmanShmaMGA90Minutes",
    "getSofZmanShmaMGA90MinutesZmanis",
    "getSofZmanShmaMGA96Minutes",
    "getSofZmanShmaMGA96MinutesZmanis",
    "getSofZmanShma3HoursBeforeChatzos",
    "getSofZmanShmaMGA120Minutes",
    "getSofZmanShmaAlos16Point1ToSunset",
    "getSofZmanShmaAlos16Point1ToTzaisGeonim7Point083Degrees",
    "getSofZmanShmaAteretTorah",
    "getSofZmanShmaBaalHatanya",
    "getSofZmanShmaGRASunriseToFixedLocalChatzos",
    "getSofZmanShmaMGA18DegreesToFixedLocalChatzos",
    "getSofZmanShmaMGA16Point1DegreesToFixedLocalChatzos",
    "getSofZmanShmaMGA90MinutesToFixedLocalChatzos",
    "getSofZmanShmaMGA72MinutesToFixedLocalChatzos",
    "getSofZmanTfilaGRA",
    "getSofZmanTfilaMGA72Minutes",
    "getSofZmanTfilaMGA19Point8Degrees",
    "getSofZmanTfilaMGA16Point1Degrees",
    "getSofZmanTfilaMGA18Degrees",
    "getSofZmanTfilaMGA72MinutesZmanis",
    "getSofZmanTfilaMGA90Minutes",
    "getSofZmanTfilaMGA90MinutesZmanis",
    "getSofZmanTfilaMGA96Minutes",
    "getSofZmanTfilaMGA96MinutesZmanis",
    "getSofZmanTfila2HoursBeforeChatzos",
    "getSofZmanTfilaMGA120Minutes",
    "getSofZmanTfilaAteretTorah",
    "getSofZmanTfilaBaalHatanya",
    "getSofZmanTfilaGRASunriseToFixedLocalChatzos",
    "getTzaisGeonim8Point5Degrees",
    "getTzais50Minutes",
    "getTzais60Minutes",
    "getTzais72Minutes",
    "getTzais72Zmanis",
    "getTzais90Minutes",
    "getTzais90Zmanis",
    "getTzais96Minutes",
    "getTzais96Zmanis",
    "getTzais120Minutes",
    "getTzais120Zmanis",
    "getTzais16Point1Degrees",
    "getTzais18Degrees",
    "getTzais19Point8Degrees",
    "getTzais26Degrees",
    "getTzaisAteretTorah",
    "getTzaisBaalHatanya",
    "getTzaisGeonim3Point7Degrees",
    "getTzaisGeonim3Point8Degrees",
    "getTzaisGeonim5Point95Degrees",
    "getTzaisGeonim4Point66Degrees",
    "getTzaisGeonim4Point42Degrees",
    "getTzaisGeonim4Point8Degrees",
    "getTzaisGeonim6Point45Degrees",
    "getTzaisGeonim7Point083Degrees",
    "getTzaisGeonim7Point67Degrees",
    "getTzaisGeonim9Point3Degrees",
    "getTzaisGeonim9Point75Degrees",
    "getSofZmanKidushLevana15Days",
    "getSofZmanKidushLevanaBetweenMoldos",
    "getTchilasZmanKidushLevana3Days",
    "getTchilasZmanKidushLevana7Days",
    "getZmanMolad",
]

ZmanimTypes = Literal[
    "twilight",  # Start and End of Astronomical, Civil, and Nautical Twilight
    "alos",
    "misheyakir",
    "netz",
    "sof_zman_shema",
    "sof_zman_shma",
    "sof_zman_tefila",
    "sof_zman_achilas_chametz",
    "sof_zman_biur_chametz",
    "chatzos_hayom",
    "mincha_gedola",
    "plag_hamincha",
    "samuch_le_mincha_ketana",
    "mincha_ketana",
    "bein_hashmashos",
    "candle_lighting",
    "shkiya",
    "tzais",
    "kidush_levana",
    "molad",
    "chatzos_halayla",
]


class ElevationAdjustedSunrise(BaseModel):
    """Elevation-adjusted sunrise."""

    type_: Literal["elevation_adjusted_sunrise"] = "elevation_adjusted_sunrise"


class SeaLevelSunrise(BaseModel):
    """Sea level sunrise."""

    type_: Literal["sea_level_sunrise"] = "sea_level_sunrise"


class ConfiguredSunrise(BaseModel):
    """Sunrise using the configured elevation mode."""

    type_: Literal["configured_sunrise"] = "configured_sunrise"


class ConfiguredSunset(BaseModel):
    """Sunset using the configured elevation mode."""

    type_: Literal["configured_sunset"] = "configured_sunset"


class SolarTransit(BaseModel):
    """Solar transit (local apparent noon / astronomical chatzos)."""

    type_: Literal["solar_transit"] = "solar_transit"


class SolarMidnight(BaseModel):
    """Solar anti-transit (local apparent midnight / astronomical chatzos halayla)."""

    type_: Literal["solar_midnight"] = "solar_midnight"


class ElevationAdjustedSunset(BaseModel):
    """Elevation-adjusted sunset."""

    type_: Literal["elevation_adjusted_sunset"] = "elevation_adjusted_sunset"


class SeaLevelSunset(BaseModel):
    """Sea level sunset."""

    type_: Literal["sea_level_sunset"] = "sea_level_sunset"


class SunriseOffsetByDegrees(BaseModel):
    """Time before sunrise when the sun is `degrees` below the geometric horizon."""

    type_: Literal["sunrise_offset_by_degrees"] = "sunrise_offset_by_degrees"
    degrees: float


class SunsetOffsetByDegrees(BaseModel):
    """Time after sunset when the sun is `degrees` below the geometric horizon."""

    type_: Literal["sunset_offset_by_degrees"] = "sunset_offset_by_degrees"
    degrees: float


class LocalMeanTime(BaseModel):
    """Local mean time at the given hour (0.0-24.0)."""

    type_: Literal["local_mean_time"] = "local_mean_time"
    hour: float


class CandleLighting(BaseModel):
    """Shabbos/Yom Tov candle lighting time based on configuration."""

    type_: Literal["candle_lighting"] = "candle_lighting"


class Offset(BaseModel):
    """A fixed time offset from another primitive."""

    type_: Literal["offset"] = "offset"
    base: ZmanPrimitive
    duration_secs: float


class ZmanisOffset(BaseModel):
    """An offset in shaos zmaniyos (GRA) from another primitive."""

    type_: Literal["zmanis_offset"] = "zmanis_offset"
    base: ZmanPrimitive
    hours: float


class ShaahZmanisBasedOffset(BaseModel):
    """Temporal-hour offset between two primitives."""

    type_: Literal["shaah_zmanis_based_offset"] = "shaah_zmanis_based_offset"
    start: ZmanPrimitive
    end: ZmanPrimitive
    hours: float


class HalfDayBasedOffset(BaseModel):
    """Offset as a fraction of the half-day between two primitives."""

    type_: Literal["half_day_based_offset"] = "half_day_based_offset"
    start: ZmanPrimitive
    end: ZmanPrimitive
    fraction: float


class Shema(BaseModel):
    """Sof zman shma derived from two bounding primitives."""

    type_: Literal["shema"] = "shema"
    start: ZmanPrimitive
    end: ZmanPrimitive
    synchronous: bool


class MinchaGedola(BaseModel):
    """Mincha gedola derived from two bounding primitives."""

    type_: Literal["mincha_gedola"] = "mincha_gedola"
    start: ZmanPrimitive
    end: ZmanPrimitive
    synchronous: bool


class SamuchLeMinchaKetana(BaseModel):
    """Samuch le-mincha ketana derived from two bounding primitives."""

    type_: Literal["samuch_le_mincha_ketana"] = "samuch_le_mincha_ketana"
    start: ZmanPrimitive
    end: ZmanPrimitive
    synchronous: bool


class MinchaKetana(BaseModel):
    """Mincha ketana derived from two bounding primitives."""

    type_: Literal["mincha_ketana"] = "mincha_ketana"
    start: ZmanPrimitive
    end: ZmanPrimitive
    synchronous: bool


class Tefila(BaseModel):
    """Sof zman tefila derived from two bounding primitives."""

    type_: Literal["tefila"] = "tefila"
    start: ZmanPrimitive
    end: ZmanPrimitive
    synchronous: bool


class PlagHamincha(BaseModel):
    """Plag hamincha derived from two bounding primitives."""

    type_: Literal["plag_hamincha"] = "plag_hamincha"
    start: ZmanPrimitive
    end: ZmanPrimitive
    synchronous: bool


class SofZmanBiurChametz(BaseModel):
    """Sof zman biur chametz derived from two bounding primitives."""

    type_: Literal["sof_zman_biur_chametz"] = "sof_zman_biur_chametz"
    start: ZmanPrimitive
    end: ZmanPrimitive
    synchronous: bool


class TzaisAteretTorah(BaseModel):
    """Tzais according to the shita of Yeshivas Ateret Torah."""

    type_: Literal["tzais_ateret_torah"] = "tzais_ateret_torah"


class SofZmanKidushLevana15Days(BaseModel):
    """Latest Kiddush Levana: 15 days after the molad."""

    type_: Literal["sof_zman_kidush_levana_15_days"] = "sof_zman_kidush_levana_15_days"


class SofZmanKidushLevanaBetweenMoldos(BaseModel):
    """Latest Kiddush Levana: halfway between molad and molad (Maharil)."""

    type_: Literal["sof_zman_kidush_levana_between_moldos"] = (
        "sof_zman_kidush_levana_between_moldos"
    )


class TchilasZmanKidushLevana3Days(BaseModel):
    """Earliest Kiddush Levana: 3 days after the molad (Rabbeinu Yonah)."""

    type_: Literal["tchilas_zman_kidush_levana_3_days"] = (
        "tchilas_zman_kidush_levana_3_days"
    )


class TchilasZmanKidushLevana7Days(BaseModel):
    """Earliest Kiddush Levana: 7 days after the molad."""

    type_: Literal["tchilas_zman_kidush_levana_7_days"] = (
        "tchilas_zman_kidush_levana_7_days"
    )


class BainHashmashosRt2Stars(BaseModel):
    """Bain hashmashos (Rabbeinu Tam, 2-stars)."""

    type_: Literal["bain_hashmashos_rt2_stars"] = "bain_hashmashos_rt2_stars"


class MinchaGedolaAhavatShalom(BaseModel):
    """Mincha gedola (Ahavat Shalom)."""

    type_: Literal["mincha_gedola_ahavat_shalom"] = "mincha_gedola_ahavat_shalom"


class MinchaGedolaGraGreaterThan30(BaseModel):
    """Mincha gedola GRA, but no earlier than 30 minutes after chatzos."""

    type_: Literal["mincha_gedola_gra_greater_than_30"] = (
        "mincha_gedola_gra_greater_than_30"
    )


class MinchaKetanaAhavatShalom(BaseModel):
    """Mincha ketana (Ahavat Shalom)."""

    type_: Literal["mincha_ketana_ahavat_shalom"] = "mincha_ketana_ahavat_shalom"


class PlagAhavatShalom(BaseModel):
    """Plag hamincha (Ahavat Shalom)."""

    type_: Literal["plag_ahavat_shalom"] = "plag_ahavat_shalom"


class Molad(BaseModel):
    """The time of the molad, or null if it does not fall out on this day."""

    type_: Literal["molad"] = "molad"


class BeginCivilTwilight(BaseModel):
    """Beginning of civil twilight (sun 6° below horizon)."""

    type_: Literal["begin_civil_twilight"] = "begin_civil_twilight"


class EndCivilTwilight(BaseModel):
    """End of civil twilight (sun 6° below horizon)."""

    type_: Literal["end_civil_twilight"] = "end_civil_twilight"


class BeginNauticalTwilight(BaseModel):
    """Beginning of nautical twilight (sun 12° below horizon)."""

    type_: Literal["begin_nautical_twilight"] = "begin_nautical_twilight"


class EndNauticalTwilight(BaseModel):
    """End of nautical twilight (sun 12° below horizon)."""

    type_: Literal["end_nautical_twilight"] = "end_nautical_twilight"


class BeginAstronomicalTwilight(BaseModel):
    """Beginning of astronomical twilight (sun 18° below horizon)."""

    type_: Literal["begin_astronomical_twilight"] = "begin_astronomical_twilight"


class EndAstronomicalTwilight(BaseModel):
    """End of astronomical twilight (sun 18° below horizon)."""

    type_: Literal["end_astronomical_twilight"] = "end_astronomical_twilight"


class SunsetOrWesternmostSolarAzimuth(BaseModel):
    """Configured sunset, or westernmost solar azimuth when sunset does not occur."""

    type_: Literal["sunset_or_westernmost_solar_azimuth"] = (
        "sunset_or_westernmost_solar_azimuth"
    )


class SunriseOrEasternmostSolarAzimuth(BaseModel):
    """Configured sunrise, or easternmost solar azimuth when sunrise does not occur."""

    type_: Literal["sunrise_or_easternmost_solar_azimuth"] = (
        "sunrise_or_easternmost_solar_azimuth"
    )


ZmanPrimitive = Annotated[
    Union[
        ElevationAdjustedSunrise,
        SeaLevelSunrise,
        ConfiguredSunrise,
        ConfiguredSunset,
        SolarTransit,
        SolarMidnight,
        ElevationAdjustedSunset,
        SeaLevelSunset,
        SunriseOffsetByDegrees,
        SunsetOffsetByDegrees,
        LocalMeanTime,
        CandleLighting,
        Offset,
        ZmanisOffset,
        ShaahZmanisBasedOffset,
        HalfDayBasedOffset,
        Shema,
        MinchaGedola,
        SamuchLeMinchaKetana,
        MinchaKetana,
        Tefila,
        PlagHamincha,
        SofZmanBiurChametz,
        TzaisAteretTorah,
        SofZmanKidushLevana15Days,
        SofZmanKidushLevanaBetweenMoldos,
        TchilasZmanKidushLevana3Days,
        TchilasZmanKidushLevana7Days,
        BainHashmashosRt2Stars,
        MinchaGedolaAhavatShalom,
        MinchaGedolaGraGreaterThan30,
        MinchaKetanaAhavatShalom,
        PlagAhavatShalom,
        Molad,
        BeginCivilTwilight,
        EndCivilTwilight,
        BeginNauticalTwilight,
        EndNauticalTwilight,
        BeginAstronomicalTwilight,
        EndAstronomicalTwilight,
        SunsetOrWesternmostSolarAzimuth,
        SunriseOrEasternmostSolarAzimuth,
    ],
    Field(discriminator="type_"),
]


class Zman(BaseModel):
    type_: ZmanimTypes
    name: str
    zman: ZmanPrimitive | None = None
    deprecated: bool = False

    def description(self) -> str:
        try:
            from docs import DOCS
        except ImportError:
            raise RuntimeError(
                "docs.py is missing; run generate-docs first "
                "(e.g. uv run python tools/generate-docs.py)"
            )
        desc: str | None = DOCS.get(self.name)
        if desc is None:
            raise RuntimeError(f"No description found for {self.name}")
        return desc


ZMAN_NAMES: dict[ZmanimMethods, Zman] = {
    # Identical to `getChatzosHayom` with `setUseAstronomicalChatzos(true)` which is rusts only implementation
    "getSunTransit": Zman(
        type_="chatzos_hayom", name="Chatzos Hayom", zman=SolarTransit()
    ),
    # Identical to `getChatzosHalayla` with `setUseAstronomicalChatzos(true)` which is rusts only implementation
    "getSolarMidnight": Zman(
        type_="chatzos_halayla",
        name="Chatzos Halayla",
        zman=SolarMidnight(),
    ),
    "getBeginCivilTwilight": Zman(
        type_="twilight", name="Begin Civil Twilight", zman=BeginCivilTwilight()
    ),
    "getEndCivilTwilight": Zman(
        type_="twilight", name="End Civil Twilight", zman=EndCivilTwilight()
    ),
    "getBeginNauticalTwilight": Zman(
        type_="twilight", name="Begin Nautical Twilight", zman=BeginNauticalTwilight()
    ),
    "getEndNauticalTwilight": Zman(
        type_="twilight", name="End Nautical Twilight", zman=EndNauticalTwilight()
    ),
    "getBeginAstronomicalTwilight": Zman(
        type_="twilight",
        name="Begin Astronomical Twilight",
        zman=BeginAstronomicalTwilight(),
    ),
    "getEndAstronomicalTwilight": Zman(
        type_="twilight",
        name="End Astronomical Twilight",
        zman=EndAstronomicalTwilight(),
    ),
    "getSunsetOrWesternmostSolarAzimuth": Zman(
        type_="shkiya",
        name="Sunset Or Westernmost Solar Azimuth",
        zman=SunsetOrWesternmostSolarAzimuth(),
    ),
    "getSunriseOrEasternmostSolarAzimuth": Zman(
        type_="netz",
        name="Sunrise Or Easternmost Solar Azimuth",
        zman=SunriseOrEasternmostSolarAzimuth(),
    ),
    "getSunrise": Zman(type_="netz", name="Sunrise", zman=ElevationAdjustedSunrise()),
    "getSeaLevelSunrise": Zman(
        type_="netz", name="Sea Level Sunrise", zman=SeaLevelSunrise()
    ),
    "getSunset": Zman(type_="shkiya", name="Sunset", zman=ElevationAdjustedSunset()),
    "getSeaLevelSunset": Zman(
        type_="shkiya", name="Sea Level Sunset", zman=SeaLevelSunset()
    ),
    "getAlos60Minutes": Zman(
        type_="alos",
        name="Alos (60 Minutes)",
        zman=Offset(
            base=ConfiguredSunrise(),
            duration_secs=-3600.0,
        ),
    ),
    "getAlos72Minutes": Zman(
        type_="alos",
        name="Alos (72 Minutes)",
        zman=Offset(
            base=ConfiguredSunrise(),
            duration_secs=-4320.0,
        ),
    ),
    "getAlos72Zmanis": Zman(
        type_="alos",
        name="Alos (72 Minutes in Shaos Zmanios)",
        zman=ZmanisOffset(
            base=ConfiguredSunrise(),
            hours=-1.2,
        ),
    ),
    "getAlos90Minutes": Zman(
        type_="alos",
        name="Alos (90 Minutes)",
        zman=Offset(
            base=ConfiguredSunrise(),
            duration_secs=-5400.0,
        ),
    ),
    "getAlos90Zmanis": Zman(
        type_="alos",
        name="Alos (90 Minutes in Shaos Zmanios)",
        zman=ZmanisOffset(
            base=ConfiguredSunrise(),
            hours=-1.5,
        ),
    ),
    "getAlos96Minutes": Zman(
        type_="alos",
        name="Alos (96 Minutes)",
        zman=Offset(
            base=ConfiguredSunrise(),
            duration_secs=-5760.0,
        ),
    ),
    "getAlos96Zmanis": Zman(
        type_="alos",
        name="Alos (96 Minutes in Shaos Zmanios)",
        zman=ZmanisOffset(
            base=ConfiguredSunrise(),
            hours=-1.6,
        ),
    ),
    "getAlos120Minutes": Zman(
        type_="alos",
        name="Alos (120 Minutes)",
        zman=Offset(
            base=ConfiguredSunrise(),
            duration_secs=-7200.0,
        ),
    ),
    "getAlos120Zmanis": Zman(
        type_="alos",
        name="Alos (120 Minutes in Shaos Zmanios)",
        zman=ZmanisOffset(
            base=ConfiguredSunrise(),
            hours=-2.0,
        ),
    ),
    "getAlos16Point1Degrees": Zman(
        type_="alos",
        name="Alos (16.1 Degrees)",
        zman=SunriseOffsetByDegrees(degrees=16.1),
    ),
    "getAlos18Degrees": Zman(
        type_="alos",
        name="Alos (18 Degrees)",
        zman=SunriseOffsetByDegrees(degrees=18.0),
    ),
    "getAlos19Degrees": Zman(
        type_="alos",
        name="Alos (19 Degrees)",
        zman=SunriseOffsetByDegrees(degrees=19.0),
    ),
    "getAlos19Point8Degrees": Zman(
        type_="alos",
        name="Alos (19.8 Degrees)",
        zman=SunriseOffsetByDegrees(degrees=19.8),
    ),
    "getAlos26Degrees": Zman(
        type_="alos",
        name="Alos (26 Degrees)",
        zman=SunriseOffsetByDegrees(degrees=26.0),
    ),
    "getAlosBaalHatanya": Zman(
        type_="alos",
        name="Alos (Baal Hatanya)",
        zman=SunriseOffsetByDegrees(degrees=16.9),
    ),
    "getBainHashmashosRT13Point24Degrees": Zman(
        type_="bein_hashmashos",
        name="Bain Hashmashos (Rabbeinu Tam, 13.24 Degrees)",
        zman=SunsetOffsetByDegrees(degrees=13.24),
    ),
    "getBainHashmashosRT58Point5Minutes": Zman(
        type_="bein_hashmashos",
        name="Bain Hashmashos (Rabbeinu Tam, 58.5 Minutes)",
        zman=Offset(
            base=ConfiguredSunset(),
            duration_secs=3510.0,
        ),
    ),
    "getBainHashmashosRT13Point5MinutesBefore7Point083Degrees": Zman(
        type_="bein_hashmashos",
        name="Bain Hashmashos (Rabbeinu Tam, 13.5 Minutes Before 7.083 Degrees)",
        zman=Offset(
            base=SunsetOffsetByDegrees(degrees=7.083333333333333),
            duration_secs=-810.0,
        ),
    ),
    "getBainHashmashosRT2Stars": Zman(
        type_="bein_hashmashos",
        name="Bain Hashmashos (Rabbeinu Tam, 2 Stars)",
        zman=BainHashmashosRt2Stars(),
    ),
    "getBainHashmashosYereim18Minutes": Zman(
        type_="bein_hashmashos",
        name="Bain Hashmashos (Yereim, 18 Minutes)",
        zman=Offset(
            base=ConfiguredSunset(),
            duration_secs=-1080.0,
        ),
    ),
    "getBainHashmashosYereim16Point875Minutes": Zman(
        type_="bein_hashmashos",
        name="Bain Hashmashos (Yereim, 16.875 Minutes)",
        zman=Offset(
            base=ConfiguredSunset(),
            duration_secs=-1012.5,
        ),
    ),
    "getBainHashmashosYereim13Point5Minutes": Zman(
        type_="bein_hashmashos",
        name="Bain Hashmashos (Yereim, 13.5 Minutes)",
        zman=Offset(
            base=ConfiguredSunset(),
            duration_secs=-810.0,
        ),
    ),
    "getBainHashmashosYereim2Point1Degrees": Zman(
        type_="bein_hashmashos",
        name="Bain Hashmashos (Yereim, 2.1 Degrees)",
        zman=SunsetOffsetByDegrees(degrees=-2.1),
    ),
    "getBainHashmashosYereim2Point8Degrees": Zman(
        type_="bein_hashmashos",
        name="Bain Hashmashos (Yereim, 2.8 Degrees)",
        zman=SunsetOffsetByDegrees(degrees=-2.8),
    ),
    "getBainHashmashosYereim3Point05Degrees": Zman(
        type_="bein_hashmashos",
        name="Bain Hashmashos (Yereim, 3.05 Degrees)",
        zman=SunsetOffsetByDegrees(degrees=-3.05),
    ),
    "getCandleLighting": Zman(
        type_="candle_lighting", name="Candle Lighting", zman=CandleLighting()
    ),
    "getChatzosHayom": Zman(
        type_="chatzos_hayom", name="Chatzos Hayom (Solar Noon)", zman=SolarTransit()
    ),
    "getChatzosHalayla": Zman(
        type_="chatzos_halayla",
        name="Chatzos Halayla (Solar Midnight)",
        zman=SolarMidnight(),
    ),
    # This zman is functionally irrelevant now that we can get true solar transit
    "getChatzosHayomAsHalfDay": Zman(
        type_="chatzos_hayom",
        name="Chatzos Hayom (Half Day)",
        zman=HalfDayBasedOffset(
            start=SeaLevelSunrise(),
            end=SeaLevelSunset(),
            fraction=3.0,
        ),
    ),
    "getFixedLocalChatzosHayom": Zman(
        type_="chatzos_hayom",
        name="Chatzos Hayom (Fixed Local Chatzos)",
        zman=LocalMeanTime(hour=12.0),
    ),
    "getMinchaGedolaGRA": Zman(
        type_="mincha_gedola",
        name="Mincha Gedola (GR'A)",
        zman=MinchaGedola(
            start=ConfiguredSunrise(),
            end=ConfiguredSunset(),
            synchronous=True,
        ),
    ),
    "getMinchaGedola16Point1Degrees": Zman(
        type_="mincha_gedola",
        name="Mincha Gedola (16.1 Degrees)",
        zman=MinchaGedola(
            start=SunriseOffsetByDegrees(degrees=16.1),
            end=SunsetOffsetByDegrees(degrees=16.1),
            synchronous=True,
        ),
    ),
    "getMinchaGedola30Minutes": Zman(
        type_="mincha_gedola",
        name="Mincha Gedola (30 Minutes)",
        zman=Offset(
            base=SolarTransit(),
            duration_secs=1800.0,
        ),
    ),
    "getMinchaGedola72Minutes": Zman(
        type_="mincha_gedola",
        name="Mincha Gedola (72 Minutes)",
        zman=MinchaGedola(
            start=Offset(
                base=ConfiguredSunrise(),
                duration_secs=-4320.0,
            ),
            end=Offset(
                base=ConfiguredSunset(),
                duration_secs=4320.0,
            ),
            synchronous=True,
        ),
    ),
    "getMinchaGedolaAhavatShalom": Zman(
        type_="mincha_gedola",
        name="Mincha Gedola (Ahavat Shalom)",
        zman=MinchaGedolaAhavatShalom(),
    ),
    "getMinchaGedolaGRAGreaterThan30": Zman(
        type_="mincha_gedola",
        name="Mincha Gedola (GR'A, Greater Than 30)",
        zman=MinchaGedolaGraGreaterThan30(),
    ),
    "getMinchaGedolaAteretTorah": Zman(
        type_="mincha_gedola",
        name="Mincha Gedola (Ateret Torah)",
        zman=MinchaGedola(
            start=ZmanisOffset(
                base=ConfiguredSunrise(),
                hours=-1.2,
            ),
            end=TzaisAteretTorah(),
            synchronous=False,
        ),
    ),
    "getMinchaGedolaBaalHatanya": Zman(
        type_="mincha_gedola",
        name="Mincha Gedola (Baal Hatanya)",
        zman=MinchaGedola(
            start=SunriseOffsetByDegrees(degrees=1.583),
            end=SunsetOffsetByDegrees(degrees=1.583),
            synchronous=True,
        ),
    ),
    "getMinchaGedolaGRAFixedLocalChatzos30Minutes": Zman(
        type_="mincha_gedola",
        name="Mincha Gedola (GR'A, Fixed Local Chatzos, 30 Minutes)",
        zman=Offset(
            base=LocalMeanTime(hour=12.0),
            duration_secs=1800.0,
        ),
    ),
    "getMinchaKetanaGRA": Zman(
        type_="mincha_ketana",
        name="Mincha Ketana (GR'A)",
        zman=MinchaKetana(
            start=ConfiguredSunrise(),
            end=ConfiguredSunset(),
            synchronous=True,
        ),
    ),
    "getMinchaKetana16Point1Degrees": Zman(
        type_="mincha_ketana",
        name="Mincha Ketana (16.1 Degrees)",
        zman=MinchaKetana(
            start=SunriseOffsetByDegrees(degrees=16.1),
            end=SunsetOffsetByDegrees(degrees=16.1),
            synchronous=True,
        ),
    ),
    "getMinchaKetana72Minutes": Zman(
        type_="mincha_ketana",
        name="Mincha Ketana (72 Minutes)",
        zman=MinchaKetana(
            start=Offset(
                base=ConfiguredSunrise(),
                duration_secs=-4320.0,
            ),
            end=Offset(
                base=ConfiguredSunset(),
                duration_secs=4320.0,
            ),
            synchronous=True,
        ),
    ),
    "getMinchaKetanaAhavatShalom": Zman(
        type_="mincha_ketana",
        name="Mincha Ketana (Ahavat Shalom)",
        zman=MinchaKetanaAhavatShalom(),
    ),
    "getMinchaKetanaAteretTorah": Zman(
        type_="mincha_ketana",
        name="Mincha Ketana (Ateret Torah)",
        zman=MinchaKetana(
            start=ZmanisOffset(
                base=ConfiguredSunrise(),
                hours=-1.2,
            ),
            end=TzaisAteretTorah(),
            synchronous=False,
        ),
    ),
    "getMinchaKetanaBaalHatanya": Zman(
        type_="mincha_ketana",
        name="Mincha Ketana (Baal Hatanya)",
        zman=MinchaKetana(
            start=SunriseOffsetByDegrees(degrees=1.583),
            end=SunsetOffsetByDegrees(degrees=1.583),
            synchronous=True,
        ),
    ),
    "getMinchaKetanaGRAFixedLocalChatzosToSunset": Zman(
        type_="mincha_ketana",
        name="Mincha Ketana (GR'A, Fixed Local Chatzos to Sunset)",
        zman=HalfDayBasedOffset(
            start=LocalMeanTime(hour=12.0),
            end=ConfiguredSunset(),
            fraction=3.5,
        ),
    ),
    "getMisheyakir10Point2Degrees": Zman(
        type_="misheyakir",
        name="Misheyakir (10.2 Degrees)",
        zman=SunriseOffsetByDegrees(degrees=10.2),
    ),
    "getMisheyakir11Degrees": Zman(
        type_="misheyakir",
        name="Misheyakir (11 Degrees)",
        zman=SunriseOffsetByDegrees(degrees=11.0),
    ),
    "getMisheyakir11Point5Degrees": Zman(
        type_="misheyakir",
        name="Misheyakir (11.5 Degrees)",
        zman=SunriseOffsetByDegrees(degrees=11.5),
    ),
    "getMisheyakir12Point85Degrees": Zman(
        type_="misheyakir",
        name="Misheyakir (12.85 Degrees)",
        zman=SunriseOffsetByDegrees(degrees=12.85),
    ),
    "getMisheyakir7Point65Degrees": Zman(
        type_="misheyakir",
        name="Misheyakir (7.65 Degrees)",
        zman=SunriseOffsetByDegrees(degrees=7.65),
    ),
    "getMisheyakir9Point5Degrees": Zman(
        type_="misheyakir",
        name="Misheyakir (9.5 Degrees)",
        zman=SunriseOffsetByDegrees(degrees=9.5),
    ),
    "getPlagHaminchaGRA": Zman(
        type_="plag_hamincha",
        name="Plag Hamincha (GR'A)",
        zman=PlagHamincha(
            start=ConfiguredSunrise(),
            end=ConfiguredSunset(),
            synchronous=True,
        ),
    ),
    "getPlagAhavatShalom": Zman(
        type_="plag_hamincha",
        name="Plag Hamincha (Ahavat Shalom)",
        zman=PlagAhavatShalom(),
    ),
    "getPlagAlos16Point1ToTzaisGeonim7Point083Degrees": Zman(
        type_="plag_hamincha",
        name="Plag Hamincha (Alos 16.1 to Tzais Geonim 7.083 Degrees)",
        zman=PlagHamincha(
            start=SunriseOffsetByDegrees(degrees=16.1),
            end=SunsetOffsetByDegrees(degrees=7.083333333333333),
            synchronous=False,
        ),
    ),
    "getPlagAlosToSunset": Zman(
        type_="plag_hamincha",
        name="Plag Hamincha (Alos 16.1 to Sunset)",
        zman=PlagHamincha(
            start=SunriseOffsetByDegrees(degrees=16.1),
            end=ConfiguredSunset(),
            synchronous=False,
        ),
    ),
    "getPlagHamincha60Minutes": Zman(
        type_="plag_hamincha",
        name="Plag Hamincha (60 Minutes)",
        zman=PlagHamincha(
            start=Offset(
                base=ConfiguredSunrise(),
                duration_secs=-3600.0,
            ),
            end=Offset(
                base=ConfiguredSunset(),
                duration_secs=3600.0,
            ),
            synchronous=True,
        ),
    ),
    "getPlagHamincha72Minutes": Zman(
        type_="plag_hamincha",
        name="Plag Hamincha (72 Minutes)",
        zman=PlagHamincha(
            start=Offset(
                base=ConfiguredSunrise(),
                duration_secs=-4320.0,
            ),
            end=Offset(
                base=ConfiguredSunset(),
                duration_secs=4320.0,
            ),
            synchronous=True,
        ),
    ),
    "getPlagHamincha72MinutesZmanis": Zman(
        type_="plag_hamincha",
        name="Plag Hamincha (72 Minutes in Shaos Zmanios)",
        zman=PlagHamincha(
            start=ZmanisOffset(
                base=ConfiguredSunrise(),
                hours=-1.2,
            ),
            end=ZmanisOffset(
                base=ConfiguredSunset(),
                hours=1.2,
            ),
            synchronous=True,
        ),
    ),
    "getPlagHamincha90Minutes": Zman(
        type_="plag_hamincha",
        name="Plag Hamincha (90 Minutes)",
        zman=PlagHamincha(
            start=Offset(
                base=ConfiguredSunrise(),
                duration_secs=-5400.0,
            ),
            end=Offset(
                base=ConfiguredSunset(),
                duration_secs=5400.0,
            ),
            synchronous=True,
        ),
    ),
    "getPlagHamincha90MinutesZmanis": Zman(
        type_="plag_hamincha",
        name="Plag Hamincha (90 Minutes in Shaos Zmanios)",
        zman=PlagHamincha(
            start=ZmanisOffset(
                base=ConfiguredSunrise(),
                hours=-1.5,
            ),
            end=ZmanisOffset(
                base=ConfiguredSunset(),
                hours=1.5,
            ),
            synchronous=True,
        ),
    ),
    "getPlagHamincha96Minutes": Zman(
        type_="plag_hamincha",
        name="Plag Hamincha (96 Minutes)",
        zman=PlagHamincha(
            start=Offset(
                base=ConfiguredSunrise(),
                duration_secs=-5760.0,
            ),
            end=Offset(
                base=ConfiguredSunset(),
                duration_secs=5760.0,
            ),
            synchronous=True,
        ),
    ),
    "getPlagHamincha96MinutesZmanis": Zman(
        type_="plag_hamincha",
        name="Plag Hamincha (96 Minutes in Shaos Zmanios)",
        zman=PlagHamincha(
            start=ZmanisOffset(
                base=ConfiguredSunrise(),
                hours=-1.6,
            ),
            end=ZmanisOffset(
                base=ConfiguredSunset(),
                hours=1.6,
            ),
            synchronous=True,
        ),
    ),
    "getPlagHamincha120Minutes": Zman(
        type_="plag_hamincha",
        name="Plag Hamincha (120 Minutes)",
        zman=PlagHamincha(
            start=Offset(
                base=ConfiguredSunrise(),
                duration_secs=-7200.0,
            ),
            end=Offset(
                base=ConfiguredSunset(),
                duration_secs=7200.0,
            ),
            synchronous=True,
        ),
    ),
    "getPlagHamincha120MinutesZmanis": Zman(
        type_="plag_hamincha",
        name="Plag Hamincha (120 Minutes in Shaos Zmanios)",
        zman=PlagHamincha(
            start=ZmanisOffset(
                base=ConfiguredSunrise(),
                hours=-2.0,
            ),
            end=ZmanisOffset(
                base=ConfiguredSunset(),
                hours=2.0,
            ),
            synchronous=True,
        ),
    ),
    "getPlagHamincha16Point1Degrees": Zman(
        type_="plag_hamincha",
        name="Plag Hamincha (16.1 Degrees)",
        zman=PlagHamincha(
            start=SunriseOffsetByDegrees(degrees=16.1),
            end=SunsetOffsetByDegrees(degrees=16.1),
            synchronous=True,
        ),
    ),
    "getPlagHamincha18Degrees": Zman(
        type_="plag_hamincha",
        name="Plag Hamincha (18 Degrees)",
        zman=PlagHamincha(
            start=SunriseOffsetByDegrees(degrees=18.0),
            end=SunsetOffsetByDegrees(degrees=18.0),
            synchronous=True,
        ),
    ),
    "getPlagHamincha19Point8Degrees": Zman(
        type_="plag_hamincha",
        name="Plag Hamincha (19.8 Degrees)",
        zman=PlagHamincha(
            start=SunriseOffsetByDegrees(degrees=19.8),
            end=SunsetOffsetByDegrees(degrees=19.8),
            synchronous=True,
        ),
    ),
    "getPlagHamincha26Degrees": Zman(
        type_="plag_hamincha",
        name="Plag Hamincha (26 Degrees)",
        zman=PlagHamincha(
            start=SunriseOffsetByDegrees(degrees=26.0),
            end=SunsetOffsetByDegrees(degrees=26.0),
            synchronous=True,
        ),
    ),
    "getPlagHaminchaAteretTorah": Zman(
        type_="plag_hamincha",
        name="Plag Hamincha (Ateret Torah)",
        zman=PlagHamincha(
            start=ZmanisOffset(
                base=ConfiguredSunrise(),
                hours=-1.2,
            ),
            end=TzaisAteretTorah(),
            synchronous=False,
        ),
    ),
    "getPlagHaminchaBaalHatanya": Zman(
        type_="plag_hamincha",
        name="Plag Hamincha (Baal Hatanya)",
        zman=PlagHamincha(
            start=SunriseOffsetByDegrees(degrees=1.583),
            end=SunsetOffsetByDegrees(degrees=1.583),
            synchronous=True,
        ),
    ),
    "getPlagHaminchaGRAFixedLocalChatzosToSunset": Zman(
        type_="plag_hamincha",
        name="Plag Hamincha (GR'A, Fixed Local Chatzos to Sunset)",
        zman=HalfDayBasedOffset(
            start=LocalMeanTime(hour=12.0),
            end=ConfiguredSunset(),
            fraction=4.75,
        ),
    ),
    "getSamuchLeMinchaKetanaGRA": Zman(
        type_="samuch_le_mincha_ketana",
        name="Samuch Le Mincha Ketana (GR'A)",
        zman=SamuchLeMinchaKetana(
            start=ConfiguredSunrise(),
            end=ConfiguredSunset(),
            synchronous=True,
        ),
    ),
    "getSamuchLeMinchaKetana16Point1Degrees": Zman(
        type_="samuch_le_mincha_ketana",
        name="Samuch Le Mincha Ketana (16.1 Degrees)",
        zman=SamuchLeMinchaKetana(
            start=SunriseOffsetByDegrees(degrees=16.1),
            end=SunsetOffsetByDegrees(degrees=16.1),
            synchronous=True,
        ),
    ),
    "getSamuchLeMinchaKetana72Minutes": Zman(
        type_="samuch_le_mincha_ketana",
        name="Samuch Le Mincha Ketana (72 Minutes)",
        zman=SamuchLeMinchaKetana(
            start=Offset(
                base=ConfiguredSunrise(),
                duration_secs=-4320.0,
            ),
            end=Offset(
                base=ConfiguredSunset(),
                duration_secs=4320.0,
            ),
            synchronous=True,
        ),
    ),
    "getSofZmanAchilasChametzGRA": Zman(
        type_="sof_zman_achilas_chametz",
        name="Sof Zman Achilas Chametz (GR'A)",
        zman=Tefila(
            start=ConfiguredSunrise(),
            end=ConfiguredSunset(),
            synchronous=True,
        ),
    ),
    "getSofZmanAchilasChametzMGA72Minutes": Zman(
        type_="sof_zman_achilas_chametz",
        name="Sof Zman Achilas Chametz (MGA, 72 Minutes)",
        zman=Tefila(
            start=Offset(
                base=ConfiguredSunrise(),
                duration_secs=-4320.0,
            ),
            end=Offset(
                base=ConfiguredSunset(),
                duration_secs=4320.0,
            ),
            synchronous=True,
        ),
    ),
    "getSofZmanAchilasChametzMGA72MinutesZmanis": Zman(
        type_="sof_zman_achilas_chametz",
        name="Sof Zman Achilas Chametz (MGA, 72 Minutes in Shaos Zmanios)",
        zman=Tefila(
            start=ZmanisOffset(
                base=ConfiguredSunrise(),
                hours=-1.2,
            ),
            end=ZmanisOffset(
                base=ConfiguredSunset(),
                hours=1.2,
            ),
            synchronous=True,
        ),
    ),
    "getSofZmanAchilasChametzMGA16Point1Degrees": Zman(
        type_="sof_zman_achilas_chametz",
        name="Sof Zman Achilas Chametz (MGA, 16.1 Degrees)",
        zman=Tefila(
            start=SunriseOffsetByDegrees(degrees=16.1),
            end=SunsetOffsetByDegrees(degrees=16.1),
            synchronous=True,
        ),
    ),
    "getSofZmanAchilasChametzBaalHatanya": Zman(
        type_="sof_zman_achilas_chametz",
        name="Sof Zman Achilas Chametz (Baal Hatanya)",
        zman=Tefila(
            start=SunriseOffsetByDegrees(degrees=1.583),
            end=SunsetOffsetByDegrees(degrees=1.583),
            synchronous=True,
        ),
    ),
    "getSofZmanBiurChametzGRA": Zman(
        type_="sof_zman_biur_chametz",
        name="Sof Zman Biur Chametz (GR'A)",
        zman=SofZmanBiurChametz(
            start=ConfiguredSunrise(),
            end=ConfiguredSunset(),
            synchronous=True,
        ),
    ),
    "getSofZmanBiurChametzMGA72Minutes": Zman(
        type_="sof_zman_biur_chametz",
        name="Sof Zman Biur Chametz (MGA, 72 Minutes)",
        zman=SofZmanBiurChametz(
            start=Offset(
                base=ConfiguredSunrise(),
                duration_secs=-4320.0,
            ),
            end=Offset(
                base=ConfiguredSunset(),
                duration_secs=4320.0,
            ),
            synchronous=True,
        ),
    ),
    "getSofZmanBiurChametzMGA72MinutesZmanis": Zman(
        type_="sof_zman_biur_chametz",
        name="Sof Zman Biur Chametz (MGA, 72 Minutes in Shaos Zmanios)",
        zman=SofZmanBiurChametz(
            start=ZmanisOffset(
                base=ConfiguredSunrise(),
                hours=-1.2,
            ),
            end=ZmanisOffset(
                base=ConfiguredSunset(),
                hours=1.2,
            ),
            synchronous=True,
        ),
    ),
    "getSofZmanBiurChametzMGA16Point1Degrees": Zman(
        type_="sof_zman_biur_chametz",
        name="Sof Zman Biur Chametz (MGA, 16.1 Degrees)",
        zman=SofZmanBiurChametz(
            start=SunriseOffsetByDegrees(degrees=16.1),
            end=SunsetOffsetByDegrees(degrees=16.1),
            synchronous=True,
        ),
    ),
    "getSofZmanBiurChametzBaalHatanya": Zman(
        type_="sof_zman_biur_chametz",
        name="Sof Zman Biur Chametz (Baal Hatanya)",
        zman=SofZmanBiurChametz(
            start=SunriseOffsetByDegrees(degrees=1.583),
            end=SunsetOffsetByDegrees(degrees=1.583),
            synchronous=True,
        ),
    ),
    "getSofZmanShmaGRA": Zman(
        type_="sof_zman_shma",
        name="Sof Zman Shma (GR'A)",
        zman=Shema(
            start=ConfiguredSunrise(),
            end=ConfiguredSunset(),
            synchronous=True,
        ),
    ),
    "getSofZmanShmaMGA19Point8Degrees": Zman(
        type_="sof_zman_shma",
        name="Sof Zman Shma (MGA, 19.8 Degrees)",
        zman=Shema(
            start=SunriseOffsetByDegrees(degrees=19.8),
            end=SunsetOffsetByDegrees(degrees=19.8),
            synchronous=True,
        ),
    ),
    "getSofZmanShmaMGA16Point1Degrees": Zman(
        type_="sof_zman_shma",
        name="Sof Zman Shma (MGA, 16.1 Degrees)",
        zman=Shema(
            start=SunriseOffsetByDegrees(degrees=16.1),
            end=SunsetOffsetByDegrees(degrees=16.1),
            synchronous=True,
        ),
    ),
    "getSofZmanShmaMGA18Degrees": Zman(
        type_="sof_zman_shma",
        name="Sof Zman Shma (MGA, 18 Degrees)",
        zman=Shema(
            start=SunriseOffsetByDegrees(degrees=18.0),
            end=SunsetOffsetByDegrees(degrees=18.0),
            synchronous=True,
        ),
    ),
    "getSofZmanShmaMGA72Minutes": Zman(
        type_="sof_zman_shma",
        name="Sof Zman Shma (MGA, 72 Minutes)",
        zman=Shema(
            start=Offset(
                base=ConfiguredSunrise(),
                duration_secs=-4320.0,
            ),
            end=Offset(
                base=ConfiguredSunset(),
                duration_secs=4320.0,
            ),
            synchronous=True,
        ),
    ),
    "getSofZmanShmaMGA72MinutesZmanis": Zman(
        type_="sof_zman_shma",
        name="Sof Zman Shma (MGA, 72 Minutes in Shaos Zmanios)",
        zman=Shema(
            start=ZmanisOffset(
                base=ConfiguredSunrise(),
                hours=-1.2,
            ),
            end=ZmanisOffset(
                base=ConfiguredSunset(),
                hours=1.2,
            ),
            synchronous=True,
        ),
    ),
    "getSofZmanShmaMGA90Minutes": Zman(
        type_="sof_zman_shma",
        name="Sof Zman Shma (MGA, 90 Minutes)",
        zman=Shema(
            start=Offset(
                base=ConfiguredSunrise(),
                duration_secs=-5400.0,
            ),
            end=Offset(
                base=ConfiguredSunset(),
                duration_secs=5400.0,
            ),
            synchronous=True,
        ),
    ),
    "getSofZmanShmaMGA90MinutesZmanis": Zman(
        type_="sof_zman_shma",
        name="Sof Zman Shma (MGA, 90 Minutes in Shaos Zmanios)",
        zman=Shema(
            start=ZmanisOffset(
                base=ConfiguredSunrise(),
                hours=-1.5,
            ),
            end=ZmanisOffset(
                base=ConfiguredSunset(),
                hours=1.5,
            ),
            synchronous=True,
        ),
    ),
    "getSofZmanShmaMGA96Minutes": Zman(
        type_="sof_zman_shma",
        name="Sof Zman Shma (MGA, 96 Minutes)",
        zman=Shema(
            start=Offset(
                base=ConfiguredSunrise(),
                duration_secs=-5760.0,
            ),
            end=Offset(
                base=ConfiguredSunset(),
                duration_secs=5760.0,
            ),
            synchronous=True,
        ),
    ),
    "getSofZmanShmaMGA96MinutesZmanis": Zman(
        type_="sof_zman_shma",
        name="Sof Zman Shma (MGA, 96 Minutes in Shaos Zmanios)",
        zman=Shema(
            start=ZmanisOffset(
                base=ConfiguredSunrise(),
                hours=-1.6,
            ),
            end=ZmanisOffset(
                base=ConfiguredSunset(),
                hours=1.6,
            ),
            synchronous=True,
        ),
    ),
    "getSofZmanShma3HoursBeforeChatzos": Zman(
        type_="sof_zman_shma",
        name="Sof Zman Shma (3 Hours Before Chatzos)",
        zman=Offset(
            base=SolarTransit(),
            duration_secs=-10800.0,
        ),
    ),
    "getSofZmanShmaMGA120Minutes": Zman(
        type_="sof_zman_shma",
        name="Sof Zman Shma (MGA, 120 Minutes)",
        zman=Shema(
            start=Offset(
                base=ConfiguredSunrise(),
                duration_secs=-7200.0,
            ),
            end=Offset(
                base=ConfiguredSunset(),
                duration_secs=7200.0,
            ),
            synchronous=True,
        ),
    ),
    "getSofZmanShmaAlos16Point1ToSunset": Zman(
        type_="sof_zman_shma",
        name="Sof Zman Shma (Alos 16.1 to Sunset)",
        zman=Shema(
            start=SunriseOffsetByDegrees(degrees=16.1),
            end=ConfiguredSunset(),
            synchronous=False,
        ),
    ),
    "getSofZmanShmaAlos16Point1ToTzaisGeonim7Point083Degrees": Zman(
        type_="sof_zman_shma",
        name="Sof Zman Shma (Alos 16.1 to Tzais Geonim 7.083 Degrees)",
        zman=Shema(
            start=SunriseOffsetByDegrees(degrees=16.1),
            end=SunsetOffsetByDegrees(degrees=7.083333333333333),
            synchronous=False,
        ),
    ),
    "getSofZmanShmaAteretTorah": Zman(
        type_="sof_zman_shma",
        name="Sof Zman Shma (Ateret Torah)",
        zman=Shema(
            start=ZmanisOffset(
                base=ConfiguredSunrise(),
                hours=-1.2,
            ),
            end=TzaisAteretTorah(),
            synchronous=False,
        ),
    ),
    "getSofZmanShmaBaalHatanya": Zman(
        type_="sof_zman_shma",
        name="Sof Zman Shma (Baal Hatanya)",
        zman=Shema(
            start=SunriseOffsetByDegrees(degrees=1.583),
            end=SunsetOffsetByDegrees(degrees=1.583),
            synchronous=True,
        ),
    ),
    "getSofZmanShmaGRASunriseToFixedLocalChatzos": Zman(
        type_="sof_zman_shma",
        name="Sof Zman Shma (GR'A, Sunrise to Fixed Local Chatzos)",
        zman=HalfDayBasedOffset(
            start=ConfiguredSunrise(),
            end=LocalMeanTime(hour=12.0),
            fraction=3.0,
        ),
    ),
    "getSofZmanShmaMGA18DegreesToFixedLocalChatzos": Zman(
        type_="sof_zman_shma",
        name="Sof Zman Shma (MGA, 18 Degrees to Fixed Local Chatzos)",
        zman=HalfDayBasedOffset(
            start=SunriseOffsetByDegrees(degrees=18.0),
            end=LocalMeanTime(hour=12.0),
            fraction=3.0,
        ),
    ),
    "getSofZmanShmaMGA16Point1DegreesToFixedLocalChatzos": Zman(
        type_="sof_zman_shma",
        name="Sof Zman Shma (MGA, 16.1 Degrees to Fixed Local Chatzos)",
        zman=HalfDayBasedOffset(
            start=SunriseOffsetByDegrees(degrees=16.1),
            end=LocalMeanTime(hour=12.0),
            fraction=3.0,
        ),
    ),
    "getSofZmanShmaMGA90MinutesToFixedLocalChatzos": Zman(
        type_="sof_zman_shma",
        name="Sof Zman Shma (MGA, 90 Minutes to Fixed Local Chatzos)",
        zman=HalfDayBasedOffset(
            start=Offset(
                base=ConfiguredSunrise(),
                duration_secs=-5400.0,
            ),
            end=LocalMeanTime(hour=12.0),
            fraction=3.0,
        ),
    ),
    "getSofZmanShmaMGA72MinutesToFixedLocalChatzos": Zman(
        type_="sof_zman_shma",
        name="Sof Zman Shma (MGA, 72 Minutes to Fixed Local Chatzos)",
        zman=HalfDayBasedOffset(
            start=Offset(
                base=ConfiguredSunrise(),
                duration_secs=-4320.0,
            ),
            end=LocalMeanTime(hour=12.0),
            fraction=3.0,
        ),
    ),
    "getSofZmanTfilaGRA": Zman(
        type_="sof_zman_tefila",
        name="Sof Zman Tfila (GR'A)",
        zman=Tefila(
            start=ConfiguredSunrise(),
            end=ConfiguredSunset(),
            synchronous=True,
        ),
    ),
    "getSofZmanTfilaMGA72Minutes": Zman(
        type_="sof_zman_tefila",
        name="Sof Zman Tfila (MGA, 72 Minutes)",
        zman=Tefila(
            start=Offset(
                base=ConfiguredSunrise(),
                duration_secs=-4320.0,
            ),
            end=Offset(
                base=ConfiguredSunset(),
                duration_secs=4320.0,
            ),
            synchronous=True,
        ),
    ),
    "getSofZmanTfilaMGA19Point8Degrees": Zman(
        type_="sof_zman_tefila",
        name="Sof Zman Tfila (MGA, 19.8 Degrees)",
        zman=Tefila(
            start=SunriseOffsetByDegrees(degrees=19.8),
            end=SunsetOffsetByDegrees(degrees=19.8),
            synchronous=True,
        ),
    ),
    "getSofZmanTfilaMGA16Point1Degrees": Zman(
        type_="sof_zman_tefila",
        name="Sof Zman Tfila (MGA, 16.1 Degrees)",
        zman=Tefila(
            start=SunriseOffsetByDegrees(degrees=16.1),
            end=SunsetOffsetByDegrees(degrees=16.1),
            synchronous=True,
        ),
    ),
    "getSofZmanTfilaMGA18Degrees": Zman(
        type_="sof_zman_tefila",
        name="Sof Zman Tfila (MGA, 18 Degrees)",
        zman=Tefila(
            start=SunriseOffsetByDegrees(degrees=18.0),
            end=SunsetOffsetByDegrees(degrees=18.0),
            synchronous=True,
        ),
    ),
    "getSofZmanTfilaMGA72MinutesZmanis": Zman(
        type_="sof_zman_tefila",
        name="Sof Zman Tfila (MGA, 72 Minutes in Shaos Zmanios)",
        zman=Tefila(
            start=ZmanisOffset(
                base=ConfiguredSunrise(),
                hours=-1.2,
            ),
            end=ZmanisOffset(
                base=ConfiguredSunset(),
                hours=1.2,
            ),
            synchronous=True,
        ),
    ),
    "getSofZmanTfilaMGA90Minutes": Zman(
        type_="sof_zman_tefila",
        name="Sof Zman Tfila (MGA, 90 Minutes)",
        zman=Tefila(
            start=Offset(
                base=ConfiguredSunrise(),
                duration_secs=-5400.0,
            ),
            end=Offset(
                base=ConfiguredSunset(),
                duration_secs=5400.0,
            ),
            synchronous=True,
        ),
    ),
    "getSofZmanTfilaMGA90MinutesZmanis": Zman(
        type_="sof_zman_tefila",
        name="Sof Zman Tfila (MGA, 90 Minutes in Shaos Zmanios)",
        zman=Tefila(
            start=ZmanisOffset(
                base=ConfiguredSunrise(),
                hours=-1.5,
            ),
            end=ZmanisOffset(
                base=ConfiguredSunset(),
                hours=1.5,
            ),
            synchronous=True,
        ),
    ),
    "getSofZmanTfilaMGA96Minutes": Zman(
        type_="sof_zman_tefila",
        name="Sof Zman Tfila (MGA, 96 Minutes)",
        zman=Tefila(
            start=Offset(
                base=ConfiguredSunrise(),
                duration_secs=-5760.0,
            ),
            end=Offset(
                base=ConfiguredSunset(),
                duration_secs=5760.0,
            ),
            synchronous=True,
        ),
    ),
    "getSofZmanTfilaMGA96MinutesZmanis": Zman(
        type_="sof_zman_tefila",
        name="Sof Zman Tfila (MGA, 96 Minutes in Shaos Zmanios)",
        zman=Tefila(
            start=ZmanisOffset(
                base=ConfiguredSunrise(),
                hours=-1.6,
            ),
            end=ZmanisOffset(
                base=ConfiguredSunset(),
                hours=1.6,
            ),
            synchronous=True,
        ),
    ),
    "getSofZmanTfila2HoursBeforeChatzos": Zman(
        type_="sof_zman_tefila",
        name="Sof Zman Tfila (2 Hours Before Chatzos)",
        zman=Offset(
            base=SolarTransit(),
            duration_secs=-7200.0,
        ),
    ),
    "getSofZmanTfilaMGA120Minutes": Zman(
        type_="sof_zman_tefila",
        name="Sof Zman Tfila (MGA, 120 Minutes)",
        zman=Tefila(
            start=Offset(
                base=ConfiguredSunrise(),
                duration_secs=-7200.0,
            ),
            end=Offset(
                base=ConfiguredSunset(),
                duration_secs=7200.0,
            ),
            synchronous=True,
        ),
    ),
    "getSofZmanTfilaAteretTorah": Zman(
        type_="sof_zman_tefila",
        name="Sof Zman Tfila (Ateret Torah)",
        zman=Tefila(
            start=ZmanisOffset(
                base=ConfiguredSunrise(),
                hours=-1.2,
            ),
            end=TzaisAteretTorah(),
            synchronous=False,
        ),
    ),
    "getSofZmanTfilaBaalHatanya": Zman(
        type_="sof_zman_tefila",
        name="Sof Zman Tfila (Baal Hatanya)",
        zman=Tefila(
            start=SunriseOffsetByDegrees(degrees=1.583),
            end=SunsetOffsetByDegrees(degrees=1.583),
            synchronous=True,
        ),
    ),
    "getSofZmanTfilaGRASunriseToFixedLocalChatzos": Zman(
        type_="sof_zman_tefila",
        name="Sof Zman Tfila (GR'A, Sunrise to Fixed Local Chatzos)",
        zman=HalfDayBasedOffset(
            start=ConfiguredSunrise(),
            end=LocalMeanTime(hour=12.0),
            fraction=4.0,
        ),
    ),
    "getTzaisGeonim8Point5Degrees": Zman(
        type_="tzais",
        name="Tzais (Geonim, 8.5 Degrees)",
        zman=SunsetOffsetByDegrees(degrees=8.5),
    ),
    "getTzais50Minutes": Zman(
        type_="tzais",
        name="Tzais (50 Minutes)",
        zman=Offset(
            base=ConfiguredSunset(),
            duration_secs=3000.0,
        ),
    ),
    "getTzais60Minutes": Zman(
        type_="tzais",
        name="Tzais (60 Minutes)",
        zman=Offset(
            base=ConfiguredSunset(),
            duration_secs=3600.0,
        ),
    ),
    "getTzais72Minutes": Zman(
        type_="tzais",
        name="Tzais (72 Minutes)",
        zman=Offset(
            base=ConfiguredSunset(),
            duration_secs=4320.0,
        ),
    ),
    "getTzais72Zmanis": Zman(
        type_="tzais",
        name="Tzais (72 Minutes in Shaos Zmanios)",
        zman=ZmanisOffset(
            base=ConfiguredSunset(),
            hours=1.2,
        ),
    ),
    "getTzais90Minutes": Zman(
        type_="tzais",
        name="Tzais (90 Minutes)",
        zman=Offset(
            base=ConfiguredSunset(),
            duration_secs=5400.0,
        ),
    ),
    "getTzais90Zmanis": Zman(
        type_="tzais",
        name="Tzais (90 Minutes in Shaos Zmanios)",
        zman=ZmanisOffset(
            base=ConfiguredSunset(),
            hours=1.5,
        ),
    ),
    "getTzais96Minutes": Zman(
        type_="tzais",
        name="Tzais (96 Minutes)",
        zman=Offset(
            base=ConfiguredSunset(),
            duration_secs=5760.0,
        ),
    ),
    "getTzais96Zmanis": Zman(
        type_="tzais",
        name="Tzais (96 Minutes in Shaos Zmanios)",
        zman=ZmanisOffset(
            base=ConfiguredSunset(),
            hours=1.6,
        ),
    ),
    "getTzais120Minutes": Zman(
        type_="tzais",
        name="Tzais (120 Minutes)",
        zman=Offset(
            base=ConfiguredSunset(),
            duration_secs=7200.0,
        ),
    ),
    "getTzais120Zmanis": Zman(
        type_="tzais",
        name="Tzais (120 Minutes in Shaos Zmanios)",
        zman=ZmanisOffset(
            base=ConfiguredSunset(),
            hours=2.0,
        ),
    ),
    "getTzais16Point1Degrees": Zman(
        type_="tzais",
        name="Tzais (16.1 Degrees)",
        zman=SunsetOffsetByDegrees(degrees=16.1),
    ),
    "getTzais18Degrees": Zman(
        type_="tzais",
        name="Tzais (18 Degrees)",
        zman=SunsetOffsetByDegrees(degrees=18.0),
    ),
    "getTzais19Point8Degrees": Zman(
        type_="tzais",
        name="Tzais (19.8 Degrees)",
        zman=SunsetOffsetByDegrees(degrees=19.8),
    ),
    "getTzais26Degrees": Zman(
        type_="tzais",
        name="Tzais (26 Degrees)",
        zman=SunsetOffsetByDegrees(degrees=26.0),
    ),
    "getTzaisAteretTorah": Zman(
        type_="tzais", name="Tzais (Ateret Torah)", zman=TzaisAteretTorah()
    ),
    "getTzaisBaalHatanya": Zman(
        type_="tzais",
        name="Tzais (Baal Hatanya)",
        zman=SunsetOffsetByDegrees(degrees=6.0),
    ),
    "getTzaisGeonim3Point7Degrees": Zman(
        type_="tzais",
        name="Tzais (Geonim, 3.7 Degrees)",
        zman=SunsetOffsetByDegrees(degrees=3.7),
    ),
    "getTzaisGeonim3Point8Degrees": Zman(
        type_="tzais",
        name="Tzais (Geonim, 3.8 Degrees)",
        zman=SunsetOffsetByDegrees(degrees=3.8),
    ),
    "getTzaisGeonim5Point95Degrees": Zman(
        type_="tzais",
        name="Tzais (Geonim, 5.95 Degrees)",
        zman=SunsetOffsetByDegrees(degrees=5.95),
    ),
    "getTzaisGeonim4Point66Degrees": Zman(
        type_="tzais",
        name="Tzais (Geonim, 4.66 Degrees)",
        zman=SunsetOffsetByDegrees(degrees=4.66),
    ),
    "getTzaisGeonim4Point42Degrees": Zman(
        type_="tzais",
        name="Tzais (Geonim, 4.42 Degrees)",
        zman=SunsetOffsetByDegrees(degrees=4.42),
    ),
    "getTzaisGeonim4Point8Degrees": Zman(
        type_="tzais",
        name="Tzais (Geonim, 4.8 Degrees)",
        zman=SunsetOffsetByDegrees(degrees=4.8),
    ),
    "getTzaisGeonim6Point45Degrees": Zman(
        type_="tzais",
        name="Tzais (Geonim, 6.45 Degrees)",
        zman=SunsetOffsetByDegrees(degrees=6.45),
    ),
    "getTzaisGeonim7Point083Degrees": Zman(
        type_="tzais",
        name="Tzais (Geonim, 7.083 Degrees)",
        zman=SunsetOffsetByDegrees(degrees=7.083333333333333),
    ),
    "getTzaisGeonim7Point67Degrees": Zman(
        type_="tzais",
        name="Tzais (Geonim, 7.67 Degrees)",
        zman=SunsetOffsetByDegrees(degrees=7.67),
    ),
    "getTzaisGeonim9Point3Degrees": Zman(
        type_="tzais",
        name="Tzais (Geonim, 9.3 Degrees)",
        zman=SunsetOffsetByDegrees(degrees=9.3),
    ),
    "getTzaisGeonim9Point75Degrees": Zman(
        type_="tzais",
        name="Tzais (Geonim, 9.75 Degrees)",
        zman=SunsetOffsetByDegrees(degrees=9.75),
    ),
    "getSofZmanKidushLevana15Days": Zman(
        type_="kidush_levana",
        name="Sof Zman Kidush Levana (15 Days)",
        zman=SofZmanKidushLevana15Days(),
    ),
    "getSofZmanKidushLevanaBetweenMoldos": Zman(
        type_="kidush_levana",
        name="Sof Zman Kidush Levana (Between Moldos)",
        zman=SofZmanKidushLevanaBetweenMoldos(),
    ),
    "getTchilasZmanKidushLevana3Days": Zman(
        type_="kidush_levana",
        name="Tchilas Zman Kidush Levana (3 Days)",
        zman=TchilasZmanKidushLevana3Days(),
    ),
    "getTchilasZmanKidushLevana7Days": Zman(
        type_="kidush_levana",
        name="Tchilas Zman Kidush Levana (7 Days)",
        zman=TchilasZmanKidushLevana7Days(),
    ),
    "getZmanMolad": Zman(type_="molad", name="Molad", zman=Molad()),
}


for _model in (
    Offset,
    ZmanisOffset,
    ShaahZmanisBasedOffset,
    HalfDayBasedOffset,
    Shema,
    MinchaGedola,
    SamuchLeMinchaKetana,
    MinchaKetana,
    Tefila,
    PlagHamincha,
    SofZmanBiurChametz,
):
    _model.model_rebuild()

Zman.model_rebuild()

try:
    from deprecated import DEPRECATED_METHODS

    for method in DEPRECATED_METHODS:
        ZMAN_NAMES[cast(ZmanimMethods, method)].deprecated = True
except ImportError:
    raise RuntimeError(
        "tools/deprecated.py is missing; run uv run python tools/generate-docs.py"
    )
