// Every zman this crate knows, one entry each. The entry's doc comment is what developers see in
// rustdoc; `about` is the explanation `Zman::description` gives to end users. Write `{candle_lighting_offset}`
// or `{ateret_torah_offset}` in `about` where the configured offset belongs.
//
// To add a zman: add an entry under its category, run `cargo test` (the catalog snapshot will
// fail), check the snapshot diff, and regenerate it with UPDATE_SNAPSHOT=1.

use jiff::SignedDuration as Duration;

use super::formula::Formula::{self, *};
use super::zman::Category;
use super::zman::zman_catalog;

zman_catalog! {
    // Twilight

    /// The beginning of [civil twilight](https://en.wikipedia.org/wiki/Twilight#Civil_twilight) at dawn.
    ///
    /// Calculated using a zenith of 96 degrees.
    BeginCivilTwilight {
        java: "getBeginCivilTwilight",
        name: "Begin Civil Twilight",
        category: Twilight,
        formula: BeginCivilTwilight,
        about: concat!(
            "The beginning of [civil twilight](https://en.wikipedia.org/wiki/Twilight#Civil_twilight) at dawn.\n\n",
            "Calculated using a zenith of 96 degrees.\n\n",
            "This zman may not be available or cannot be calculated when the computation cannot be performed.",
        ),
    },

    /// The end of [civil twilight](https://en.wikipedia.org/wiki/Twilight#Civil_twilight) in the evening.
    ///
    /// Calculated using a zenith of 96 degrees.
    EndCivilTwilight {
        java: "getEndCivilTwilight",
        name: "End Civil Twilight",
        category: Twilight,
        formula: EndCivilTwilight,
        about: concat!(
            "The end of [civil twilight](https://en.wikipedia.org/wiki/Twilight#Civil_twilight) in the evening.\n\n",
            "Calculated using a zenith of 96 degrees.\n\n",
            "This zman may not be available or cannot be calculated when the computation cannot be performed.",
        ),
    },

    /// The beginning of [nautical twilight](https://en.wikipedia.org/wiki/Twilight#Nautical_twilight) at dawn.
    ///
    /// Calculated using a zenith of 102 degrees.
    BeginNauticalTwilight {
        java: "getBeginNauticalTwilight",
        name: "Begin Nautical Twilight",
        category: Twilight,
        formula: BeginNauticalTwilight,
        about: concat!(
            "The beginning of [nautical twilight](https://en.wikipedia.org/wiki/Twilight#Nautical_twilight) at dawn.\n\n",
            "Calculated using a zenith of 102 degrees.\n\n",
            "This zman may not be available or cannot be calculated when the computation cannot be performed.",
        ),
    },

    /// The end of nautical twilight in the evening.
    ///
    /// Calculated using a zenith of 102 degrees.
    EndNauticalTwilight {
        java: "getEndNauticalTwilight",
        name: "End Nautical Twilight",
        category: Twilight,
        formula: EndNauticalTwilight,
        about: concat!(
            "The end of nautical twilight in the evening.\n\n",
            "Calculated using a zenith of 102 degrees.\n\n",
            "This zman may not be available or cannot be calculated when the computation cannot be performed.",
        ),
    },

    /// The beginning of [astronomical twilight](https://en.wikipedia.org/wiki/Twilight#Astronomical_twilight) at
    /// dawn.
    ///
    /// Calculated using a zenith of 108 degrees.
    BeginAstronomicalTwilight {
        java: "getBeginAstronomicalTwilight",
        name: "Begin Astronomical Twilight",
        category: Twilight,
        formula: BeginAstronomicalTwilight,
        about: concat!(
            "The beginning of [astronomical twilight](https://en.wikipedia.org/wiki/Twilight#Astronomical_twilight) at dawn.\n\n",
            "Calculated using a zenith of 108 degrees.\n\n",
            "This zman may not be available or cannot be calculated when the computation cannot be performed.",
        ),
    },

    /// The end of astronomical twilight in the evening.
    ///
    /// Calculated using a zenith of 108 degrees.
    EndAstronomicalTwilight {
        java: "getEndAstronomicalTwilight",
        name: "End Astronomical Twilight",
        category: Twilight,
        formula: EndAstronomicalTwilight,
        about: concat!(
            "The end of astronomical twilight in the evening.\n\n",
            "Calculated using a zenith of 108 degrees.\n\n",
            "This zman may not be available or cannot be calculated when the computation cannot be performed.",
        ),
    },

    // Alos

    /// Alos (dawn), the beginning of morning twilight before sunrise.
    ///
    /// 60 minutes before sunrise.
    Alos60Minutes {
        java: "getAlos60Minutes",
        name: "Alos (60 Minutes)",
        category: Alos,
        formula: Offset(&ConfiguredSunrise, Duration::from_mins(-60)),
        about: concat!(
            "Alos (dawn), the beginning of morning twilight before sunrise.\n\n",
            "60 minutes before sunrise. \n\n",
            "Based on the time to walk 4 mil at 15 minutes per mil.\n\n",
            "In places where sunrise cannot be calculated, this zman may not be available.",
        ),
    },

    /// Alos (dawn), the beginning of morning twilight before sunrise.
    ///
    /// 72 minutes before sunrise.
    Alos72Minutes {
        java: "getAlos72Minutes",
        name: "Alos (72 Minutes)",
        category: Alos,
        formula: Offset(&ConfiguredSunrise, Duration::from_mins(-72)),
        about: concat!(
            "Alos (dawn), the beginning of morning twilight before sunrise.\n\n",
            "72 minutes before sunrise. \n\n",
            "Based on the time to walk 4 mil at 18 minutes per mil.\n\n",
            "In places where sunrise cannot be calculated, this zman may not be available.",
        ),
    },

    /// Alos (dawn), the beginning of morning twilight before sunrise.
    ///
    /// 72 zmaniyos minutes before sunrise, or 1/10 of the day.
    Alos72Zmanis {
        java: "getAlos72Zmanis",
        name: "Alos (72 Minutes in Shaos Zmanios)",
        category: Alos,
        formula: ZmanisOffset(&ConfiguredSunrise, -1.2),
        about: concat!(
            "Alos (dawn), the beginning of morning twilight before sunrise.\n\n",
            "72 zmaniyos minutes before sunrise, or 1/10 of the day. \n\n",
            "Based on 4 mil at 18 minutes per mil, measured in shaos zmaniyos.\n\n",
            "In places where sunrise or sunset cannot be calculated, this zman may not be available.",
        ),
    },

    /// Alos (dawn), the beginning of morning twilight before sunrise.
    ///
    /// 90 minutes before sunrise.
    Alos90Minutes {
        java: "getAlos90Minutes",
        name: "Alos (90 Minutes)",
        category: Alos,
        formula: Offset(&ConfiguredSunrise, Duration::from_mins(-90)),
        about: concat!(
            "Alos (dawn), the beginning of morning twilight before sunrise.\n\n",
            "90 minutes before sunrise. \n\n",
            "Based on the time to walk 4 mil at 22.5 minutes per mil.\n\n",
            "In places where sunrise cannot be calculated, this zman may not be available.",
        ),
    },

    /// Alos (dawn), the beginning of morning twilight before sunrise.
    ///
    /// 90 zmaniyos minutes before sunrise, or 1/8 of the day.
    Alos90Zmanis {
        java: "getAlos90Zmanis",
        name: "Alos (90 Minutes in Shaos Zmanios)",
        category: Alos,
        formula: ZmanisOffset(&ConfiguredSunrise, -1.5),
        about: concat!(
            "Alos (dawn), the beginning of morning twilight before sunrise.\n\n",
            "90 zmaniyos minutes before sunrise, or 1/8 of the day. \n\n",
            "Based on 4 mil at 22.5 minutes per mil, measured in shaos zmaniyos.\n\n",
            "In places where sunrise or sunset cannot be calculated, this zman may not be available.",
        ),
    },

    /// Alos (dawn), the beginning of morning twilight before sunrise.
    ///
    /// 96 minutes before sunrise.
    Alos96Minutes {
        java: "getAlos96Minutes",
        name: "Alos (96 Minutes)",
        category: Alos,
        formula: Offset(&ConfiguredSunrise, Duration::from_mins(-96)),
        about: concat!(
            "Alos (dawn), the beginning of morning twilight before sunrise.\n\n",
            "96 minutes before sunrise. \n\n",
            "Based on the time to walk 4 mil at 24 minutes per mil.\n\n",
            "In places where sunrise cannot be calculated, this zman may not be available.",
        ),
    },

    /// Alos (dawn), the beginning of morning twilight before sunrise.
    ///
    /// 96 zmaniyos minutes before sunrise, or 1/7.5 of the day.
    Alos96Zmanis {
        java: "getAlos96Zmanis",
        name: "Alos (96 Minutes in Shaos Zmanios)",
        category: Alos,
        formula: ZmanisOffset(&ConfiguredSunrise, -1.6),
        about: concat!(
            "Alos (dawn), the beginning of morning twilight before sunrise.\n\n",
            "96 zmaniyos minutes before sunrise, or 1/7.5 of the day. \n\n",
            "Based on 4 mil at 24 minutes per mil, measured in shaos zmaniyos.\n\n",
            "In places where sunrise or sunset cannot be calculated, this zman may not be available.",
        ),
    },

    /// Alos (dawn), using an extremely early 120-minute calculation.
    ///
    /// 120 minutes before sunrise.
    #[deprecated(note = "KosherJava deprecates this zman")]
    Alos120Minutes {
        java: "getAlos120Minutes",
        name: "Alos (120 Minutes)",
        category: Alos,
        formula: Offset(&ConfiguredSunrise, Duration::from_mins(-120)),
        about: concat!(
            "Alos (dawn), using an extremely early 120-minute calculation.\n\n",
            "120 minutes before sunrise. \n\n",
            "Based on the time to walk 5 mil at 24 minutes per mil.\n\n",
            "This zman should be used lechumra only, such as stopping to eat on a fast day, and not as the start of daytime mitzvos.\n\n",
            "In places where sunrise cannot be calculated, this zman may not be available.",
        ),
    },

    /// Alos (dawn), using an extremely early 120-zmaniyos-minute calculation.
    ///
    /// 120 zmaniyos minutes before sunrise, or 1/6 of the day.
    #[deprecated(note = "KosherJava deprecates this zman")]
    Alos120Zmanis {
        java: "getAlos120Zmanis",
        name: "Alos (120 Minutes in Shaos Zmanios)",
        category: Alos,
        formula: ZmanisOffset(&ConfiguredSunrise, -2.0),
        about: concat!(
            "Alos (dawn), using an extremely early 120-zmaniyos-minute calculation.\n\n",
            "120 zmaniyos minutes before sunrise, or 1/6 of the day. \n\n",
            "Based on 5 mil at 24 minutes per mil, measured in shaos zmaniyos.\n\n",
            "This zman should be used lechumra only, such as stopping to eat on a fast day, and not as the start of daytime mitzvos.\n\n",
            "In places where sunrise or sunset cannot be calculated, this zman may not be available.",
        ),
    },

    /// Alos (dawn), the beginning of morning twilight before sunrise.
    ///
    /// The time when the sun is 16.1 degrees below the eastern horizon before sunrise.
    Alos16Point1Degrees {
        java: "getAlos16Point1Degrees",
        name: "Alos (16.1 Degrees)",
        category: Alos,
        formula: SunriseOffsetByDegrees(16.1),
        about: concat!(
            "Alos (dawn), the beginning of morning twilight before sunrise.\n\n",
            "The time when the sun is 16.1 degrees below the eastern horizon before sunrise.\n\n",
            "This reflects the 72-minute alos calculation, based on 4 mil at 18 minutes per mil.\n\n",
            "At some northern and southern locations, this zman may not be available if the sun does not reach low enough below the horizon.",
        ),
    },

    /// Alos (dawn), the beginning of morning twilight before sunrise.
    ///
    /// The time when the sun is 18 degrees below the eastern horizon before sunrise.
    Alos18Degrees {
        java: "getAlos18Degrees",
        name: "Alos (18 Degrees)",
        category: Alos,
        formula: SunriseOffsetByDegrees(18.0),
        about: concat!(
            "Alos (dawn), the beginning of morning twilight before sunrise.\n\n",
            "The time when the sun is 18 degrees below the eastern horizon before sunrise.\n\n",
            "At some northern and southern locations, this zman may not be available if the sun does not reach low enough below the horizon.",
        ),
    },

    /// Alos (dawn), associated with the Rambam's alos.
    ///
    /// The time when the sun is 19 degrees below the eastern horizon before sunrise.
    Alos19Degrees {
        java: "getAlos19Degrees",
        name: "Alos (19 Degrees)",
        category: Alos,
        formula: SunriseOffsetByDegrees(19.0),
        about: concat!(
            "Alos (dawn), associated with the Rambam's alos.\n\n",
            "The time when the sun is 19 degrees below the eastern horizon before sunrise.\n\n",
            "At some northern and southern locations, this zman may not be available if the sun does not reach low enough below the horizon.",
        ),
    },

    /// Alos (dawn), the beginning of morning twilight before sunrise.
    ///
    /// The time when the sun is 19.8 degrees below the eastern horizon before sunrise.
    Alos19Point8Degrees {
        java: "getAlos19Point8Degrees",
        name: "Alos (19.8 Degrees)",
        category: Alos,
        formula: SunriseOffsetByDegrees(19.8),
        about: concat!(
            "Alos (dawn), the beginning of morning twilight before sunrise.\n\n",
            "The time when the sun is 19.8 degrees below the eastern horizon before sunrise.\n\n",
            "This is the degree-based equivalent of alos 90 minutes before sunrise around the equinox in Jerusalem.\n\n",
            "At some northern and southern locations, this zman may not be available if the sun does not reach low enough below the horizon.",
        ),
    },

    /// Alos (dawn), using an extremely early degree-based calculation.
    ///
    /// The time when the sun is 26 degrees below the eastern horizon before sunrise.
    #[deprecated(note = "KosherJava deprecates this zman")]
    Alos26Degrees {
        java: "getAlos26Degrees",
        name: "Alos (26 Degrees)",
        category: Alos,
        formula: SunriseOffsetByDegrees(26.0),
        about: concat!(
            "Alos (dawn), using an extremely early degree-based calculation.\n\n",
            "The time when the sun is 26 degrees below the eastern horizon before sunrise.\n\n",
            "This is the degree-based equivalent of alos 120 minutes before sunrise around the equinox in Jerusalem.\n\n",
            "This zman should be used lechumra only, such as stopping to eat on a fast day, and not as the start of daytime mitzvos.\n\n",
            "At some northern and southern locations, this zman may not be available if the sun does not reach low enough below the horizon.",
        ),
    },

    /// Alos (dawn) according to the Baal Hatanya.
    ///
    /// The time when the sun is 16.9 degrees below the eastern horizon before sunrise.
    AlosBaalHatanya {
        java: "getAlosBaalHatanya",
        name: "Alos (Baal Hatanya)",
        category: Alos,
        formula: SunriseOffsetByDegrees(16.9),
        about: concat!(
            "Alos (dawn) according to the Baal Hatanya.\n\n",
            "The time when the sun is 16.9 degrees below the eastern horizon before sunrise.\n\n",
            "Based on the view that the interval from dawn to netz amiti is 72 minutes, or 4 mil at 18 minutes per mil.\n\n",
            "At some northern and southern locations, this zman may not be available if the sun does not reach low enough below the horizon.",
        ),
    },

    // Misheyakir

    /// Misheyakir according to some opinions.
    ///
    /// The time when the sun is 10.2 degrees below the horizon before sunrise.
    Misheyakir10Point2Degrees {
        java: "getMisheyakir10Point2Degrees",
        name: "Misheyakir (10.2 Degrees)",
        category: Misheyakir,
        formula: SunriseOffsetByDegrees(10.2),
        about: concat!(
            "Misheyakir according to some opinions.\n\n",
            "The time when the sun is 10.2 degrees below the horizon before sunrise.\n\n",
            "This is about 45 minutes before sunrise in Jerusalem around the equinox.\n\n",
            "At some northern and southern locations, this zman may not be available if the sun does not reach low enough below the horizon.",
        ),
    },

    /// Misheyakir according to some opinions.
    ///
    /// The time when the sun is 11 degrees below the horizon before sunrise.
    Misheyakir11Degrees {
        java: "getMisheyakir11Degrees",
        name: "Misheyakir (11 Degrees)",
        category: Misheyakir,
        formula: SunriseOffsetByDegrees(11.0),
        about: concat!(
            "Misheyakir according to some opinions.\n\n",
            "The time when the sun is 11 degrees below the horizon before sunrise.\n\n",
            "This is about 48 minutes before sunrise in Jerusalem around the equinox.\n\n",
            "At some northern and southern locations, this zman may not be available if the sun does not reach low enough below the horizon.",
        ),
    },

    /// Misheyakir according to some opinions.
    ///
    /// The time when the sun is 11.5 degrees below the horizon before sunrise.
    Misheyakir11Point5Degrees {
        java: "getMisheyakir11Point5Degrees",
        name: "Misheyakir (11.5 Degrees)",
        category: Misheyakir,
        formula: SunriseOffsetByDegrees(11.5),
        about: concat!(
            "Misheyakir according to some opinions.\n\n",
            "The time when the sun is 11.5 degrees below the horizon before sunrise.\n\n",
            "This is about 52 minutes before sunrise in Jerusalem around the equinox.\n\n",
            "At some northern and southern locations, this zman may not be available if the sun does not reach low enough below the horizon.",
        ),
    },

    /// Misheyakir using a very early calculation.
    ///
    /// The time when the sun is 12.85 degrees below the horizon before sunrise.
    #[deprecated(note = "KosherJava deprecates this zman")]
    Misheyakir12Point85Degrees {
        java: "getMisheyakir12Point85Degrees",
        name: "Misheyakir (12.85 Degrees)",
        category: Misheyakir,
        formula: SunriseOffsetByDegrees(12.85),
        about: concat!(
            "Misheyakir using a very early calculation.\n\n",
            "The time when the sun is 12.85 degrees below the horizon before sunrise.\n\n",
            "This is slightly later than 57 minutes before sunrise in Jerusalem around the equinox.\n\n",
            "This zman should be used only bish'as hadchak. A later zman should be used lechatchila.\n\n",
            "At some northern and southern locations, this zman may not be available if the sun does not reach low enough below the horizon.",
        ),
    },

    /// Misheyakir according to the 35-36 minute approach.
    ///
    /// The time when the sun is 7.65 degrees below the horizon before sunrise.
    Misheyakir7Point65Degrees {
        java: "getMisheyakir7Point65Degrees",
        name: "Misheyakir (7.65 Degrees)",
        category: Misheyakir,
        formula: SunriseOffsetByDegrees(7.65),
        about: concat!(
            "Misheyakir according to the 35-36 minute approach.\n\n",
            "The time when the sun is 7.65 degrees below the horizon before sunrise.\n\n",
            "This is based on a 35-36 minute misheyakir around the equinox, when twilight is shortest.\n\n",
            "At some northern and southern locations, this zman may not be available if the sun does not reach low enough below the horizon.",
        ),
    },

    /// Misheyakir according to the 45-minute approach used by some communities.
    ///
    /// The time when the sun is 9.5 degrees below the horizon before sunrise.
    Misheyakir9Point5Degrees {
        java: "getMisheyakir9Point5Degrees",
        name: "Misheyakir (9.5 Degrees)",
        category: Misheyakir,
        formula: SunriseOffsetByDegrees(9.5),
        about: concat!(
            "Misheyakir according to the 45-minute approach used by some communities.\n\n",
            "The time when the sun is 9.5 degrees below the horizon before sunrise.\n\n",
            "This is based on a 45-minute misheyakir calculation.\n\n",
            "At some northern and southern locations, this zman may not be available if the sun does not reach low enough below the horizon.",
        ),
    },

    // Netz

    /// Sunrise calculated as the time when the sun is directly due east (azimuth 90 degrees) in polar regions on days
    /// when there is no sunrise.
    ///
    /// If sunrise occurs that day, this zman is not available. In polar regions (the Arctic or Antarctic circles),
    /// there are days with no sunrise or sunset. Some opinions hold that during these periods, sunrise is when the
    /// sun is directly due east (azimuth 90 degrees), and sunset is when the sun is directly due west (azimuth 270
    /// degrees), given by [`Zman::PolarSunsetBenIshChai`]. This follows [Rabbi Yehosef
    /// Schwarz](https://en.wikipedia.org/wiki/Joseph_Schwarz_(geographer)) in [Devarim Yosef - Derech Mevo
    /// Hashemesh](https://hebrewbooks.org/pdfpager.aspx?req=31703&pgnum=134) and [Devarim Yosef - Teshuvot, She'elah
    /// 8](https://hebrewbooks.org/pdfpager.aspx?req=159&pgnum=83), brought lehalacha by the [Ben Ish
    /// Chai](https://en.wikipedia.org/wiki/Yosef_Hayyim) in [Rav Pe'alim, chelek 2, Sod Yesharim siman
    /// 4](https://hebrewbooks.org/pdfpager.aspx?req=1401&pgnum=461). This time is close to six hours before
    /// astronomical chatzos, but depending on the season and location in the Arctic or Antarctic, it can be up to 46
    /// minutes earlier or later.
    PolarSunriseBenIshChai {
        java: "getPolarSunriseBenIshChai",
        name: "Polar Sunrise Ben Ish Chai",
        category: Netz,
        formula: PolarSunriseBenIshChai,
        about: concat!(
            "Sunrise calculated as the time when the sun is directly due east (azimuth 90 degrees) in polar regions on days when there is no sunrise.\n\n",
            "If sunrise occurs that day, this zman is not available. In polar regions (the Arctic or Antarctic circles), there are days with no sunrise or sunset. Some opinions hold that during these periods, sunrise is when the sun is directly due east (azimuth 90 degrees), and sunset is when the sun is directly due west (azimuth 270 degrees). This follows [Rabbi Yehosef Schwarz](https://en.wikipedia.org/wiki/Joseph_Schwarz_(geographer)) in [Devarim Yosef - Derech Mevo Hashemesh](https://hebrewbooks.org/pdfpager.aspx?req=31703&pgnum=134) and [Devarim Yosef - Teshuvot, She'elah 8](https://hebrewbooks.org/pdfpager.aspx?req=159&pgnum=83), brought lehalacha by the [Ben Ish Chai](https://en.wikipedia.org/wiki/Yosef_Hayyim) in [Rav Pe'alim, chelek 2, Sod Yesharim siman 4](https://hebrewbooks.org/pdfpager.aspx?req=1401&pgnum=461). This time is close to six hours before astronomical chatzos, but depending on the season and location in the Arctic or Antarctic, it can be up to 46 minutes earlier or later.\n\n",
            "If there is no sunrise that day and the sun does not reach azimuth 90 degrees, this zman may not be available or cannot be calculated.",
        ),
    },

    /// Sunrise, adjusted for the location's elevation.
    ///
    /// The time when the upper edge of the sun appears above the horizon, accounting for atmospheric refraction and
    /// the sun's radius.
    Sunrise {
        java: "getSunrise",
        name: "Sunrise",
        category: Netz,
        formula: ElevationAdjustedSunrise,
        about: concat!(
            "Sunrise, adjusted for the location's elevation.\n\n",
            "The time when the upper edge of the sun appears above the horizon, accounting for atmospheric refraction and the sun's radius.\n\n",
            "In places such as the Arctic Circle, where there is at least one day a year when the sun does not rise and one when it does not set, this zman may not be available or cannot be calculated.",
        ),
    },

    /// Sunrise at sea level, without adjusting for the location's elevation.
    ///
    /// This is the astronomical sunrise used as the reference for dawn times measured as degrees below the horizon.
    /// Dawn and dusk depend on visible light, which is not affected by elevation the way sunrise and sunset at a
    /// raised location can be.
    SeaLevelSunrise {
        java: "getSeaLevelSunrise",
        name: "Sea Level Sunrise",
        category: Netz,
        formula: SeaLevelSunrise,
        about: concat!(
            "Sunrise at sea level, without adjusting for the location's elevation.\n\n",
            "This is the astronomical sunrise used as the reference for dawn times measured as degrees below the horizon. Dawn and dusk depend on visible light, which is not affected by elevation the way sunrise and sunset at a raised location can be.\n\n",
            "In places such as the Arctic Circle, where there is at least one day a year when the sun does not rise and one when it does not set, this zman may not be available or cannot be calculated.",
        ),
    },

    // SofZmanShma

    /// Sof zman krias shema - the latest time to recite morning Shema according to the
    /// [GRA](https://en.wikipedia.org/wiki/Vilna_Gaon).
    ///
    /// 3 shaos zmaniyos after sunrise.
    SofZmanShmaGra {
        java: "getSofZmanShmaGRA",
        name: "Sof Zman Shma (GR'A)",
        category: SofZmanShma,
        formula: Shema(&ConfiguredSunrise, &ConfiguredSunset, true),
        about: concat!(
            "Sof zman krias shema - the latest time to recite morning Shema according to the [GRA](https://en.wikipedia.org/wiki/Vilna_Gaon).\n\n",
            "3 shaos zmaniyos after sunrise. \n\n",
            "In places such as the Arctic Circle, where there is at least one day a year when the sun does not rise and one when it does not set, this zman may not be available or cannot be calculated.",
        ),
    },

    /// Sof zman krias shema - the latest time to recite morning Shema according to the [Magen Avraham
    /// (MGA)](https://en.wikipedia.org/wiki/Avraham_Gombiner), using the 19.8-degree day.
    ///
    /// 3 shaos zmaniyos after alos at 19.8 degrees, with the day measured from alos at 19.8 degrees to tzais at 19.8
    /// degrees.
    SofZmanShmaMga19Point8Degrees {
        java: "getSofZmanShmaMGA19Point8Degrees",
        name: "Sof Zman Shma (MGA, 19.8 Degrees)",
        category: SofZmanShma,
        formula: Shema(&SunriseOffsetByDegrees(19.8), &SunsetOffsetByDegrees(19.8), true),
        about: concat!(
            "Sof zman krias shema - the latest time to recite morning Shema according to the [Magen Avraham (MGA)](https://en.wikipedia.org/wiki/Avraham_Gombiner), using the 19.8-degree day.\n\n",
            "3 shaos zmaniyos after alos at 19.8 degrees, with the day measured from alos at 19.8 degrees to tzais at 19.8 degrees.\n\n",
            "At some northern and southern locations, this zman may not be available or cannot be calculated if the sun does not reach low enough below the horizon.",
        ),
    },

    /// Sof zman krias shema - the latest time to recite morning Shema according to the [Magen Avraham
    /// (MGA)](https://en.wikipedia.org/wiki/Avraham_Gombiner), using the 16.1-degree day.
    ///
    /// 3 shaos zmaniyos after alos at 16.1 degrees, with the day measured from alos at 16.1 degrees to tzais at 16.1
    /// degrees.
    SofZmanShmaMga16Point1Degrees {
        java: "getSofZmanShmaMGA16Point1Degrees",
        name: "Sof Zman Shma (MGA, 16.1 Degrees)",
        category: SofZmanShma,
        formula: Shema(&SunriseOffsetByDegrees(16.1), &SunsetOffsetByDegrees(16.1), true),
        about: concat!(
            "Sof zman krias shema - the latest time to recite morning Shema according to the [Magen Avraham (MGA)](https://en.wikipedia.org/wiki/Avraham_Gombiner), using the 16.1-degree day.\n\n",
            "3 shaos zmaniyos after alos at 16.1 degrees, with the day measured from alos at 16.1 degrees to tzais at 16.1 degrees.\n\n",
            "At some northern and southern locations, this zman may not be available or cannot be calculated if the sun does not reach low enough below the horizon.",
        ),
    },

    /// Sof zman krias shema - the latest time to recite morning Shema according to the [Magen Avraham
    /// (MGA)](https://en.wikipedia.org/wiki/Avraham_Gombiner), using the 18-degree day.
    ///
    /// 3 shaos zmaniyos after alos at 18 degrees, with the day measured from alos at 18 degrees to tzais at 18
    /// degrees.
    SofZmanShmaMga18Degrees {
        java: "getSofZmanShmaMGA18Degrees",
        name: "Sof Zman Shma (MGA, 18 Degrees)",
        category: SofZmanShma,
        formula: Shema(&SunriseOffsetByDegrees(18.0), &SunsetOffsetByDegrees(18.0), true),
        about: concat!(
            "Sof zman krias shema - the latest time to recite morning Shema according to the [Magen Avraham (MGA)](https://en.wikipedia.org/wiki/Avraham_Gombiner), using the 18-degree day.\n\n",
            "3 shaos zmaniyos after alos at 18 degrees, with the day measured from alos at 18 degrees to tzais at 18 degrees.\n\n",
            "At some northern and southern locations, this zman may not be available or cannot be calculated if the sun does not reach low enough below the horizon.",
        ),
    },

    /// Sof zman krias shema - the latest time to recite morning Shema according to the [Magen Avraham
    /// (MGA)](https://en.wikipedia.org/wiki/Avraham_Gombiner), using the 72-minute day.
    ///
    /// 3 shaos zmaniyos after alos 72 minutes before sunrise, with the day measured from alos 72 minutes before
    /// sunrise to tzais 72 minutes after sunset.
    SofZmanShmaMga72Minutes {
        java: "getSofZmanShmaMGA72Minutes",
        name: "Sof Zman Shma (MGA, 72 Minutes)",
        category: SofZmanShma,
        formula: Shema(&Offset(&ConfiguredSunrise, Duration::from_mins(-72)), &Offset(&ConfiguredSunset, Duration::from_mins(72)), true),
        about: concat!(
            "Sof zman krias shema - the latest time to recite morning Shema according to the [Magen Avraham (MGA)](https://en.wikipedia.org/wiki/Avraham_Gombiner), using the 72-minute day.\n\n",
            "3 shaos zmaniyos after alos 72 minutes before sunrise, with the day measured from alos 72 minutes before sunrise to tzais 72 minutes after sunset.\n\n",
            "In places such as the Arctic Circle, where there is at least one day a year when the sun does not rise and one when it does not set, this zman may not be available or cannot be calculated.",
        ),
    },

    /// Sof zman krias shema - the latest time to recite morning Shema according to the [Magen Avraham
    /// (MGA)](https://en.wikipedia.org/wiki/Avraham_Gombiner), using the 72 zmaniyos-minute day.
    ///
    /// 3 shaos zmaniyos after alos 72 zmaniyos minutes before sunrise, with the day measured from that alos to tzais
    /// 72 zmaniyos minutes after sunset.
    SofZmanShmaMga72MinutesZmanis {
        java: "getSofZmanShmaMGA72MinutesZmanis",
        name: "Sof Zman Shma (MGA, 72 Minutes in Shaos Zmanios)",
        category: SofZmanShma,
        formula: Shema(&ZmanisOffset(&ConfiguredSunrise, -1.2), &ZmanisOffset(&ConfiguredSunset, 1.2), true),
        about: concat!(
            "Sof zman krias shema - the latest time to recite morning Shema according to the [Magen Avraham (MGA)](https://en.wikipedia.org/wiki/Avraham_Gombiner), using the 72 zmaniyos-minute day.\n\n",
            "3 shaos zmaniyos after alos 72 zmaniyos minutes before sunrise, with the day measured from that alos to tzais 72 zmaniyos minutes after sunset.\n\n",
            "In places such as the Arctic Circle, where there is at least one day a year when the sun does not rise and one when it does not set, this zman may not be available or cannot be calculated.",
        ),
    },

    /// Sof zman krias shema - the latest time to recite morning Shema according to the [Magen Avraham
    /// (MGA)](https://en.wikipedia.org/wiki/Avraham_Gombiner), using the 90-minute day.
    ///
    /// 3 shaos zmaniyos after alos 90 minutes before sunrise, with the day measured from alos 90 minutes before
    /// sunrise to tzais 90 minutes after sunset.
    SofZmanShmaMga90Minutes {
        java: "getSofZmanShmaMGA90Minutes",
        name: "Sof Zman Shma (MGA, 90 Minutes)",
        category: SofZmanShma,
        formula: Shema(&Offset(&ConfiguredSunrise, Duration::from_mins(-90)), &Offset(&ConfiguredSunset, Duration::from_mins(90)), true),
        about: concat!(
            "Sof zman krias shema - the latest time to recite morning Shema according to the [Magen Avraham (MGA)](https://en.wikipedia.org/wiki/Avraham_Gombiner), using the 90-minute day.\n\n",
            "3 shaos zmaniyos after alos 90 minutes before sunrise, with the day measured from alos 90 minutes before sunrise to tzais 90 minutes after sunset.\n\n",
            "In places such as the Arctic Circle, where there is at least one day a year when the sun does not rise and one when it does not set, this zman may not be available or cannot be calculated.",
        ),
    },

    /// Sof zman krias shema - the latest time to recite morning Shema according to the [Magen Avraham
    /// (MGA)](https://en.wikipedia.org/wiki/Avraham_Gombiner), using the 90 zmaniyos-minute day.
    ///
    /// 3 shaos zmaniyos after alos 90 zmaniyos minutes before sunrise, with the day measured from that alos to tzais
    /// 90 zmaniyos minutes after sunset.
    SofZmanShmaMga90MinutesZmanis {
        java: "getSofZmanShmaMGA90MinutesZmanis",
        name: "Sof Zman Shma (MGA, 90 Minutes in Shaos Zmanios)",
        category: SofZmanShma,
        formula: Shema(&ZmanisOffset(&ConfiguredSunrise, -1.5), &ZmanisOffset(&ConfiguredSunset, 1.5), true),
        about: concat!(
            "Sof zman krias shema - the latest time to recite morning Shema according to the [Magen Avraham (MGA)](https://en.wikipedia.org/wiki/Avraham_Gombiner), using the 90 zmaniyos-minute day.\n\n",
            "3 shaos zmaniyos after alos 90 zmaniyos minutes before sunrise, with the day measured from that alos to tzais 90 zmaniyos minutes after sunset.\n\n",
            "In places such as the Arctic Circle, where there is at least one day a year when the sun does not rise and one when it does not set, this zman may not be available or cannot be calculated.",
        ),
    },

    /// Sof zman krias shema - the latest time to recite morning Shema according to the [Magen Avraham
    /// (MGA)](https://en.wikipedia.org/wiki/Avraham_Gombiner), using the 96-minute day.
    ///
    /// 3 shaos zmaniyos after alos 96 minutes before sunrise, with the day measured from alos 96 minutes before
    /// sunrise to tzais 96 minutes after sunset.
    SofZmanShmaMga96Minutes {
        java: "getSofZmanShmaMGA96Minutes",
        name: "Sof Zman Shma (MGA, 96 Minutes)",
        category: SofZmanShma,
        formula: Shema(&Offset(&ConfiguredSunrise, Duration::from_mins(-96)), &Offset(&ConfiguredSunset, Duration::from_mins(96)), true),
        about: concat!(
            "Sof zman krias shema - the latest time to recite morning Shema according to the [Magen Avraham (MGA)](https://en.wikipedia.org/wiki/Avraham_Gombiner), using the 96-minute day.\n\n",
            "3 shaos zmaniyos after alos 96 minutes before sunrise, with the day measured from alos 96 minutes before sunrise to tzais 96 minutes after sunset.\n\n",
            "In places such as the Arctic Circle, where there is at least one day a year when the sun does not rise and one when it does not set, this zman may not be available or cannot be calculated.",
        ),
    },

    /// Sof zman krias shema - the latest time to recite morning Shema according to the [Magen Avraham
    /// (MGA)](https://en.wikipedia.org/wiki/Avraham_Gombiner), using the 96 zmaniyos-minute day.
    ///
    /// 3 shaos zmaniyos after alos 96 zmaniyos minutes before sunrise, with the day measured from that alos to tzais
    /// 96 zmaniyos minutes after sunset.
    SofZmanShmaMga96MinutesZmanis {
        java: "getSofZmanShmaMGA96MinutesZmanis",
        name: "Sof Zman Shma (MGA, 96 Minutes in Shaos Zmanios)",
        category: SofZmanShma,
        formula: Shema(&ZmanisOffset(&ConfiguredSunrise, -1.6), &ZmanisOffset(&ConfiguredSunset, 1.6), true),
        about: concat!(
            "Sof zman krias shema - the latest time to recite morning Shema according to the [Magen Avraham (MGA)](https://en.wikipedia.org/wiki/Avraham_Gombiner), using the 96 zmaniyos-minute day.\n\n",
            "3 shaos zmaniyos after alos 96 zmaniyos minutes before sunrise, with the day measured from that alos to tzais 96 zmaniyos minutes after sunset.\n\n",
            "In places such as the Arctic Circle, where there is at least one day a year when the sun does not rise and one when it does not set, this zman may not be available or cannot be calculated.",
        ),
    },

    /// Sof zman krias shema - the latest time to recite morning Shema, calculated as 3 regular clock hours before
    /// chatzos hayom (not shaos zmaniyos). Often grouped with the "Komarno" zmanim after [Rav Yitzchak Eizik of
    /// Komarno](https://en.wikipedia.org/wiki/Komarno_(Hasidic_dynasty)#Rabbi_Yitzchak_Eisik_Safrin), though this
    /// calculation is much older.
    ///
    /// 3 clock hours before chatzos hayom.
    SofZmanShma3HoursBeforeChatzos {
        java: "getSofZmanShma3HoursBeforeChatzos",
        name: "Sof Zman Shma (3 Hours Before Chatzos)",
        category: SofZmanShma,
        formula: Offset(&ChatzosHayom, Duration::from_mins(-180)),
        about: concat!(
            "Sof zman krias shema - the latest time to recite morning Shema, calculated as 3 regular clock hours before chatzos hayom (not shaos zmaniyos). Often grouped with the \"Komarno\" zmanim after [Rav Yitzchak Eizik of Komarno](https://en.wikipedia.org/wiki/Komarno_(Hasidic_dynasty)#Rabbi_Yitzchak_Eisik_Safrin), though this calculation is much older.\n\n",
            "3 clock hours before chatzos hayom.\n\n",
            "This view is cited by the Shach in Nekudas Hakesef (Yoreh Deah 184), [Rav Moshe Lifshitz](https://hebrewbooks.org/pdfpager.aspx?req=21638&st=&pgnum=30) in [Lechem Mishneh on Brachos 1:2](https://hebrewbooks.org/pdfpager.aspx?req=21638&st=&pgnum=50), the [Yaavetz](https://en.wikipedia.org/wiki/Jacob_Emden), and later by Komarno, Shevus Yaakov, Chasan Sofer, and others. See also [Yisrael Vehazmanim vol. 1, 7:3](https://hebrewbooks.org/pdfpager.aspx?req=9765&st=&pgnum=83).\n\n",
            "In places such as the Arctic Circle, where there is at least one day a year when the sun does not rise and one when it does not set, this zman may not be available or cannot be calculated.",
        ),
    },

    /// Sof zman krias shema - the latest time to recite morning Shema according to the [Magen Avraham
    /// (MGA)](https://en.wikipedia.org/wiki/Avraham_Gombiner), using the 120-minute day. This is an extremely early
    /// time, used as a chumra.
    ///
    /// 3 shaos zmaniyos after alos 120 minutes before sunrise, with the day measured from alos 120 minutes before
    /// sunrise to tzais 120 minutes after sunset.
    SofZmanShmaMga120Minutes {
        java: "getSofZmanShmaMGA120Minutes",
        name: "Sof Zman Shma (MGA, 120 Minutes)",
        category: SofZmanShma,
        formula: Shema(&Offset(&ConfiguredSunrise, Duration::from_mins(-120)), &Offset(&ConfiguredSunset, Duration::from_mins(120)), true),
        about: concat!(
            "Sof zman krias shema - the latest time to recite morning Shema according to the [Magen Avraham (MGA)](https://en.wikipedia.org/wiki/Avraham_Gombiner), using the 120-minute day. This is an extremely early time, used as a chumra.\n\n",
            "3 shaos zmaniyos after alos 120 minutes before sunrise, with the day measured from alos 120 minutes before sunrise to tzais 120 minutes after sunset.\n\n",
            "In places such as the Arctic Circle, where there is at least one day a year when the sun does not rise and one when it does not set, this zman may not be available or cannot be calculated.",
        ),
    },

    /// Sof zman krias shema - the latest time to recite morning Shema according to the opinion of the [Chidushei
    /// V'Chlalot HaRazah](https://hebrewbooks.org/40357) and the [Menorah HaTehorah](https://hebrewbooks.org/14799),
    /// as cited in [Yisrael Vehazmanim vol. 1, sec. 7, ch. 3 no.
    /// 16](https://hebrewbooks.org/pdfpager.aspx?req=9765&pgnum=81).
    ///
    /// 3 shaos zmaniyos after alos at 16.1 degrees, with the day measured from alos at 16.1 degrees to
    /// sunset. By this calculation, chatzos is not at midday.
    SofZmanShmaAlos16Point1ToSunset {
        java: "getSofZmanShmaAlos16Point1ToSunset",
        name: "Sof Zman Shma (Alos 16.1 to Sunset)",
        category: SofZmanShma,
        formula: Shema(&SunriseOffsetByDegrees(16.1), &ConfiguredSunset, false),
        about: concat!(
            "Sof zman krias shema - the latest time to recite morning Shema according to the opinion of the [Chidushei V'Chlalot HaRazah](https://hebrewbooks.org/40357) and the [Menorah HaTehorah](https://hebrewbooks.org/14799), as cited in [Yisrael Vehazmanim vol. 1, sec. 7, ch. 3 no. 16](https://hebrewbooks.org/pdfpager.aspx?req=9765&pgnum=81).\n\n",
            "3 shaos zmaniyos after alos at 16.1 degrees, with the day measured from alos at 16.1 degrees to sunset. By this calculation, chatzos is not at midday.\n\n",
            "At some northern and southern locations, this zman may not be available or cannot be calculated if the sun does not reach low enough below the horizon.",
        ),
    },

    /// Sof zman krias shema - the latest time to recite morning Shema using a day from alos at 16.1 degrees to tzais
    /// at 7.083 degrees.
    ///
    /// 3 shaos zmaniyos after alos at 16.1 degrees, with the day measured from alos at 16.1 degrees to tzais at 7.083
    /// degrees. By this calculation, chatzos is not at midday.
    SofZmanShmaAlos16Point1DegreesToTzaisGeonim7Point083Degrees {
        java: "getSofZmanShmaAlos16Point1DegreesToTzaisGeonim7Point083Degrees",
        name: "Sof Zman Shma (Alos 16.1 to Tzais Geonim 7.083 Degrees)",
        category: SofZmanShma,
        formula: Shema(&SunriseOffsetByDegrees(16.1), &SunsetOffsetByDegrees(7.083333333333333), false),
        about: concat!(
            "Sof zman krias shema - the latest time to recite morning Shema using a day from alos at 16.1 degrees to tzais at 7.083 degrees.\n\n",
            "3 shaos zmaniyos after alos at 16.1 degrees, with the day measured from alos at 16.1 degrees to tzais at 7.083 degrees. By this calculation, chatzos is not at midday.\n\n",
            "At some northern and southern locations, this zman may not be available or cannot be calculated if the sun does not reach low enough below the horizon.",
        ),
    },

    /// Sof zman krias shema - the latest time to recite morning Shema according to the Ateret Torah calculation of
    /// Chacham Yosef Harari-Raful of Yeshivat Ateret Torah.
    ///
    /// Ateret Torah zmanim use a day that begins at alos 1/10 of the day before sunrise and ends the configured
    /// Ateret Torah offset after sunset. Sof zman krias shema is 3 of those shaos zmaniyos after that alos. By this
    /// calculation, chatzos is not at midday.
    SofZmanShmaAteretTorah {
        java: "getSofZmanShmaAteretTorah",
        name: "Sof Zman Shma (Ateret Torah)",
        category: SofZmanShma,
        formula: Shema(&ZmanisOffset(&ConfiguredSunrise, -1.2), &TzaisAteretTorah, false),
        about: concat!(
            "Sof zman krias shema - the latest time to recite morning Shema according to the Ateret Torah calculation of Chacham Yosef Harari-Raful of Yeshivat Ateret Torah.\n\n",
            "Ateret Torah zmanim use a day that begins at alos 1/10 of the day before sunrise and ends {ateret_torah_offset} after sunset. Sof zman krias shema is 3 of those shaos zmaniyos after that alos. By this calculation, chatzos is not at midday.\n\n",
            "In places such as the Arctic Circle, where there is at least one day a year when the sun does not rise and one when it does not set, this zman may not be available or cannot be calculated.",
        ),
    },

    /// Sof zman krias shema - the latest time to recite morning Shema according to the Baal Hatanya.
    ///
    /// 3 shaos zmaniyos after netz amiti (true sunrise), with the day measured from Baal Hatanya sunrise to sunset.
    SofZmanShmaBaalHatanya {
        java: "getSofZmanShmaBaalHatanya",
        name: "Sof Zman Shma (Baal Hatanya)",
        category: SofZmanShma,
        formula: Shema(&SunriseOffsetByDegrees(1.583), &SunsetOffsetByDegrees(1.583), true),
        about: concat!(
            "Sof zman krias shema - the latest time to recite morning Shema according to the Baal Hatanya.\n\n",
            "3 shaos zmaniyos after netz amiti (true sunrise), with the day measured from Baal Hatanya sunrise to sunset.\n\n",
            "In places such as the Arctic Circle, where there is at least one day a year when the sun does not rise and one when it does not set, this zman may not be available or cannot be calculated.",
        ),
    },

    /// Sof zman krias shema - the latest time to recite morning Shema according to [Rav Moshe
    /// Feinstein's](https://en.wikipedia.org/wiki/Moshe_Feinstein) view of the
    /// [GRA](https://en.wikipedia.org/wiki/Vilna_Gaon) day, using only the first half of the day.
    ///
    /// 3 shaos zmaniyos after sunrise, with shaos zmaniyos measured from sunrise to fixed local chatzos (half of that
    /// half-day).
    SofZmanShmaGraSunriseToFixedLocalChatzos {
        java: "getSofZmanShmaGRASunriseToFixedLocalChatzos",
        name: "Sof Zman Shma (GR'A, Sunrise to Fixed Local Chatzos)",
        category: SofZmanShma,
        formula: HalfDayBasedOffset(&ConfiguredSunrise, &LocalMeanTime(12.0), 3.0),
        about: concat!(
            "Sof zman krias shema - the latest time to recite morning Shema according to [Rav Moshe Feinstein's](https://en.wikipedia.org/wiki/Moshe_Feinstein) view of the [GRA](https://en.wikipedia.org/wiki/Vilna_Gaon) day, using only the first half of the day.\n\n",
            "3 shaos zmaniyos after sunrise, with shaos zmaniyos measured from sunrise to fixed local chatzos (half of that half-day). \n\n",
            "At some northern and southern locations, this zman may not be available or cannot be calculated if the sun does not reach low enough below the horizon.",
        ),
    },

    /// Sof zman krias shema - the latest time to recite morning Shema according to [Rav Moshe
    /// Feinstein's](https://en.wikipedia.org/wiki/Moshe_Feinstein) view of the [Magen Avraham
    /// (MGA)](https://en.wikipedia.org/wiki/Avraham_Gombiner) day, using only the first half of the day.
    ///
    /// 3 shaos zmaniyos after alos at 18 degrees, with shaos zmaniyos measured from that alos to fixed local chatzos
    /// (half of that half-day).
    SofZmanShmaMga18DegreesToFixedLocalChatzos {
        java: "getSofZmanShmaMGA18DegreesToFixedLocalChatzos",
        name: "Sof Zman Shma (MGA, 18 Degrees to Fixed Local Chatzos)",
        category: SofZmanShma,
        formula: HalfDayBasedOffset(&SunriseOffsetByDegrees(18.0), &LocalMeanTime(12.0), 3.0),
        about: concat!(
            "Sof zman krias shema - the latest time to recite morning Shema according to [Rav Moshe Feinstein's](https://en.wikipedia.org/wiki/Moshe_Feinstein) view of the [Magen Avraham (MGA)](https://en.wikipedia.org/wiki/Avraham_Gombiner) day, using only the first half of the day.\n\n",
            "3 shaos zmaniyos after alos at 18 degrees, with shaos zmaniyos measured from that alos to fixed local chatzos (half of that half-day).\n\n",
            "At some northern and southern locations, this zman may not be available or cannot be calculated if the sun does not reach low enough below the horizon.",
        ),
    },

    /// Sof zman krias shema - the latest time to recite morning Shema according to [Rav Moshe
    /// Feinstein's](https://en.wikipedia.org/wiki/Moshe_Feinstein) view of the [Magen Avraham
    /// (MGA)](https://en.wikipedia.org/wiki/Avraham_Gombiner) day, using only the first half of the day.
    ///
    /// 3 shaos zmaniyos after alos at 16.1 degrees, with shaos zmaniyos measured from that alos to fixed local
    /// chatzos (half of that half-day).
    SofZmanShmaMga16Point1DegreesToFixedLocalChatzos {
        java: "getSofZmanShmaMGA16Point1DegreesToFixedLocalChatzos",
        name: "Sof Zman Shma (MGA, 16.1 Degrees to Fixed Local Chatzos)",
        category: SofZmanShma,
        formula: HalfDayBasedOffset(&SunriseOffsetByDegrees(16.1), &LocalMeanTime(12.0), 3.0),
        about: concat!(
            "Sof zman krias shema - the latest time to recite morning Shema according to [Rav Moshe Feinstein's](https://en.wikipedia.org/wiki/Moshe_Feinstein) view of the [Magen Avraham (MGA)](https://en.wikipedia.org/wiki/Avraham_Gombiner) day, using only the first half of the day.\n\n",
            "3 shaos zmaniyos after alos at 16.1 degrees, with shaos zmaniyos measured from that alos to fixed local chatzos (half of that half-day).\n\n",
            "At some northern and southern locations, this zman may not be available or cannot be calculated if the sun does not reach low enough below the horizon.",
        ),
    },

    /// Sof zman krias shema - the latest time to recite morning Shema according to [Rav Moshe
    /// Feinstein's](https://en.wikipedia.org/wiki/Moshe_Feinstein) view of the [Magen Avraham
    /// (MGA)](https://en.wikipedia.org/wiki/Avraham_Gombiner) day, using only the first half of the day.
    ///
    /// 3 shaos zmaniyos after alos 90 minutes before sunrise, with shaos zmaniyos measured from that alos to fixed
    /// local chatzos (half of that half-day).
    SofZmanShmaMga90MinutesToFixedLocalChatzos {
        java: "getSofZmanShmaMGA90MinutesToFixedLocalChatzos",
        name: "Sof Zman Shma (MGA, 90 Minutes to Fixed Local Chatzos)",
        category: SofZmanShma,
        formula: HalfDayBasedOffset(&Offset(&ConfiguredSunrise, Duration::from_mins(-90)), &LocalMeanTime(12.0), 3.0),
        about: concat!(
            "Sof zman krias shema - the latest time to recite morning Shema according to [Rav Moshe Feinstein's](https://en.wikipedia.org/wiki/Moshe_Feinstein) view of the [Magen Avraham (MGA)](https://en.wikipedia.org/wiki/Avraham_Gombiner) day, using only the first half of the day.\n\n",
            "3 shaos zmaniyos after alos 90 minutes before sunrise, with shaos zmaniyos measured from that alos to fixed local chatzos (half of that half-day).\n\n",
            "At some northern and southern locations, this zman may not be available or cannot be calculated if the sun does not reach low enough below the horizon.",
        ),
    },

    /// Sof zman krias shema - the latest time to recite morning Shema according to [Rav Moshe
    /// Feinstein's](https://en.wikipedia.org/wiki/Moshe_Feinstein) view of the [Magen Avraham
    /// (MGA)](https://en.wikipedia.org/wiki/Avraham_Gombiner) day, using only the first half of the day.
    ///
    /// 3 shaos zmaniyos after alos 72 minutes before sunrise, with shaos zmaniyos measured from that alos to fixed
    /// local chatzos (half of that half-day).
    SofZmanShmaMga72MinutesToFixedLocalChatzos {
        java: "getSofZmanShmaMGA72MinutesToFixedLocalChatzos",
        name: "Sof Zman Shma (MGA, 72 Minutes to Fixed Local Chatzos)",
        category: SofZmanShma,
        formula: HalfDayBasedOffset(&Offset(&ConfiguredSunrise, Duration::from_mins(-72)), &LocalMeanTime(12.0), 3.0),
        about: concat!(
            "Sof zman krias shema - the latest time to recite morning Shema according to [Rav Moshe Feinstein's](https://en.wikipedia.org/wiki/Moshe_Feinstein) view of the [Magen Avraham (MGA)](https://en.wikipedia.org/wiki/Avraham_Gombiner) day, using only the first half of the day.\n\n",
            "3 shaos zmaniyos after alos 72 minutes before sunrise, with shaos zmaniyos measured from that alos to fixed local chatzos (half of that half-day).\n\n",
            "At some northern and southern locations, this zman may not be available or cannot be calculated if the sun does not reach low enough below the horizon.",
        ),
    },

    // SofZmanTfila

    /// Sof zman tfila - the latest time to recite morning prayers (Shacharis) according to the
    /// [GRA](https://en.wikipedia.org/wiki/Vilna_Gaon).
    ///
    /// 4 shaos zmaniyos after sunrise.
    SofZmanTfilaGra {
        java: "getSofZmanTfilaGRA",
        name: "Sof Zman Tfila (GR'A)",
        category: SofZmanTfila,
        formula: Tefila(&ConfiguredSunrise, &ConfiguredSunset, true),
        about: concat!(
            "Sof zman tfila - the latest time to recite morning prayers (Shacharis) according to the [GRA](https://en.wikipedia.org/wiki/Vilna_Gaon).\n\n",
            "4 shaos zmaniyos after sunrise. \n\n",
            "In places such as the Arctic Circle, where there is at least one day a year when the sun does not rise and one when it does not set, this zman may not be available or cannot be calculated.",
        ),
    },

    /// Sof zman tfila - the latest time to recite morning prayers (Shacharis) according to the [Magen Avraham
    /// (MGA)](https://en.wikipedia.org/wiki/Avraham_Gombiner), using the 72-minute day.
    ///
    /// 4 shaos zmaniyos after alos 72 minutes before sunrise, with the day measured from alos 72 minutes before
    /// sunrise to tzais 72 minutes after sunset.
    SofZmanTfilaMga72Minutes {
        java: "getSofZmanTfilaMGA72Minutes",
        name: "Sof Zman Tfila (MGA, 72 Minutes)",
        category: SofZmanTfila,
        formula: Tefila(&Offset(&ConfiguredSunrise, Duration::from_mins(-72)), &Offset(&ConfiguredSunset, Duration::from_mins(72)), true),
        about: concat!(
            "Sof zman tfila - the latest time to recite morning prayers (Shacharis) according to the [Magen Avraham (MGA)](https://en.wikipedia.org/wiki/Avraham_Gombiner), using the 72-minute day.\n\n",
            "4 shaos zmaniyos after alos 72 minutes before sunrise, with the day measured from alos 72 minutes before sunrise to tzais 72 minutes after sunset. \n\n",
            "In places such as the Arctic Circle, where there is at least one day a year when the sun does not rise and one when it does not set, this zman may not be available or cannot be calculated.",
        ),
    },

    /// Sof zman tfila - the latest time to recite morning prayers (Shacharis) according to the [Magen Avraham
    /// (MGA)](https://en.wikipedia.org/wiki/Avraham_Gombiner), using the 19.8-degree day.
    ///
    /// 4 shaos zmaniyos after alos at 19.8 degrees, with the day measured from alos at 19.8 degrees to tzais at 19.8
    /// degrees.
    SofZmanTfilaMga19Point8Degrees {
        java: "getSofZmanTfilaMGA19Point8Degrees",
        name: "Sof Zman Tfila (MGA, 19.8 Degrees)",
        category: SofZmanTfila,
        formula: Tefila(&SunriseOffsetByDegrees(19.8), &SunsetOffsetByDegrees(19.8), true),
        about: concat!(
            "Sof zman tfila - the latest time to recite morning prayers (Shacharis) according to the [Magen Avraham (MGA)](https://en.wikipedia.org/wiki/Avraham_Gombiner), using the 19.8-degree day.\n\n",
            "4 shaos zmaniyos after alos at 19.8 degrees, with the day measured from alos at 19.8 degrees to tzais at 19.8 degrees.\n\n",
            "At some northern and southern locations, this zman may not be available or cannot be calculated if the sun does not reach low enough below the horizon.",
        ),
    },

    /// Sof zman tfila - the latest time to recite morning prayers (Shacharis) according to the [Magen Avraham
    /// (MGA)](https://en.wikipedia.org/wiki/Avraham_Gombiner), using the 16.1-degree day.
    ///
    /// 4 shaos zmaniyos after alos at 16.1 degrees, with the day measured from alos at 16.1 degrees to tzais at 16.1
    /// degrees.
    SofZmanTfilaMga16Point1Degrees {
        java: "getSofZmanTfilaMGA16Point1Degrees",
        name: "Sof Zman Tfila (MGA, 16.1 Degrees)",
        category: SofZmanTfila,
        formula: Tefila(&SunriseOffsetByDegrees(16.1), &SunsetOffsetByDegrees(16.1), true),
        about: concat!(
            "Sof zman tfila - the latest time to recite morning prayers (Shacharis) according to the [Magen Avraham (MGA)](https://en.wikipedia.org/wiki/Avraham_Gombiner), using the 16.1-degree day.\n\n",
            "4 shaos zmaniyos after alos at 16.1 degrees, with the day measured from alos at 16.1 degrees to tzais at 16.1 degrees.\n\n",
            "At some northern and southern locations, this zman may not be available or cannot be calculated if the sun does not reach low enough below the horizon.",
        ),
    },

    /// Sof zman tfila - the latest time to recite morning prayers (Shacharis) according to the [Magen Avraham
    /// (MGA)](https://en.wikipedia.org/wiki/Avraham_Gombiner), using the 18-degree day.
    ///
    /// 4 shaos zmaniyos after alos at 18 degrees, with the day measured from alos at 18 degrees to tzais at 18
    /// degrees.
    SofZmanTfilaMga18Degrees {
        java: "getSofZmanTfilaMGA18Degrees",
        name: "Sof Zman Tfila (MGA, 18 Degrees)",
        category: SofZmanTfila,
        formula: Tefila(&SunriseOffsetByDegrees(18.0), &SunsetOffsetByDegrees(18.0), true),
        about: concat!(
            "Sof zman tfila - the latest time to recite morning prayers (Shacharis) according to the [Magen Avraham (MGA)](https://en.wikipedia.org/wiki/Avraham_Gombiner), using the 18-degree day.\n\n",
            "4 shaos zmaniyos after alos at 18 degrees, with the day measured from alos at 18 degrees to tzais at 18 degrees.\n\n",
            "At some northern and southern locations, this zman may not be available or cannot be calculated if the sun does not reach low enough below the horizon.",
        ),
    },

    /// Sof zman tfila - the latest time to recite morning prayers (Shacharis) according to the [Magen Avraham
    /// (MGA)](https://en.wikipedia.org/wiki/Avraham_Gombiner), using the 72 zmaniyos-minute day.
    ///
    /// 4 shaos zmaniyos after alos 72 zmaniyos minutes before sunrise, with the day measured from that alos to tzais
    /// 72 zmaniyos minutes after sunset.
    SofZmanTfilaMga72MinutesZmanis {
        java: "getSofZmanTfilaMGA72MinutesZmanis",
        name: "Sof Zman Tfila (MGA, 72 Minutes in Shaos Zmanios)",
        category: SofZmanTfila,
        formula: Tefila(&ZmanisOffset(&ConfiguredSunrise, -1.2), &ZmanisOffset(&ConfiguredSunset, 1.2), true),
        about: concat!(
            "Sof zman tfila - the latest time to recite morning prayers (Shacharis) according to the [Magen Avraham (MGA)](https://en.wikipedia.org/wiki/Avraham_Gombiner), using the 72 zmaniyos-minute day.\n\n",
            "4 shaos zmaniyos after alos 72 zmaniyos minutes before sunrise, with the day measured from that alos to tzais 72 zmaniyos minutes after sunset.\n\n",
            "In places such as the Arctic Circle, where there is at least one day a year when the sun does not rise and one when it does not set, this zman may not be available or cannot be calculated.",
        ),
    },

    /// Sof zman tfila - the latest time to recite morning prayers (Shacharis) according to the [Magen Avraham
    /// (MGA)](https://en.wikipedia.org/wiki/Avraham_Gombiner), using the 90-minute day.
    ///
    /// 4 shaos zmaniyos after alos 90 minutes before sunrise, with the day measured from alos 90 minutes before
    /// sunrise to tzais 90 minutes after sunset.
    SofZmanTfilaMga90Minutes {
        java: "getSofZmanTfilaMGA90Minutes",
        name: "Sof Zman Tfila (MGA, 90 Minutes)",
        category: SofZmanTfila,
        formula: Tefila(&Offset(&ConfiguredSunrise, Duration::from_mins(-90)), &Offset(&ConfiguredSunset, Duration::from_mins(90)), true),
        about: concat!(
            "Sof zman tfila - the latest time to recite morning prayers (Shacharis) according to the [Magen Avraham (MGA)](https://en.wikipedia.org/wiki/Avraham_Gombiner), using the 90-minute day.\n\n",
            "4 shaos zmaniyos after alos 90 minutes before sunrise, with the day measured from alos 90 minutes before sunrise to tzais 90 minutes after sunset.\n\n",
            "In places such as the Arctic Circle, where there is at least one day a year when the sun does not rise and one when it does not set, this zman may not be available or cannot be calculated.",
        ),
    },

    /// Sof zman tfila - the latest time to recite morning prayers (Shacharis) according to the [Magen Avraham
    /// (MGA)](https://en.wikipedia.org/wiki/Avraham_Gombiner), using the 90 zmaniyos-minute day.
    ///
    /// 4 shaos zmaniyos after alos 90 zmaniyos minutes before sunrise, with the day measured from that alos to tzais
    /// 90 zmaniyos minutes after sunset.
    SofZmanTfilaMga90MinutesZmanis {
        java: "getSofZmanTfilaMGA90MinutesZmanis",
        name: "Sof Zman Tfila (MGA, 90 Minutes in Shaos Zmanios)",
        category: SofZmanTfila,
        formula: Tefila(&ZmanisOffset(&ConfiguredSunrise, -1.5), &ZmanisOffset(&ConfiguredSunset, 1.5), true),
        about: concat!(
            "Sof zman tfila - the latest time to recite morning prayers (Shacharis) according to the [Magen Avraham (MGA)](https://en.wikipedia.org/wiki/Avraham_Gombiner), using the 90 zmaniyos-minute day.\n\n",
            "4 shaos zmaniyos after alos 90 zmaniyos minutes before sunrise, with the day measured from that alos to tzais 90 zmaniyos minutes after sunset.\n\n",
            "In places such as the Arctic Circle, where there is at least one day a year when the sun does not rise and one when it does not set, this zman may not be available or cannot be calculated.",
        ),
    },

    /// Sof zman tfila - the latest time to recite morning prayers (Shacharis) according to the [Magen Avraham
    /// (MGA)](https://en.wikipedia.org/wiki/Avraham_Gombiner), using the 96-minute day.
    ///
    /// 4 shaos zmaniyos after alos 96 minutes before sunrise, with the day measured from alos 96 minutes before
    /// sunrise to tzais 96 minutes after sunset.
    SofZmanTfilaMga96Minutes {
        java: "getSofZmanTfilaMGA96Minutes",
        name: "Sof Zman Tfila (MGA, 96 Minutes)",
        category: SofZmanTfila,
        formula: Tefila(&Offset(&ConfiguredSunrise, Duration::from_mins(-96)), &Offset(&ConfiguredSunset, Duration::from_mins(96)), true),
        about: concat!(
            "Sof zman tfila - the latest time to recite morning prayers (Shacharis) according to the [Magen Avraham (MGA)](https://en.wikipedia.org/wiki/Avraham_Gombiner), using the 96-minute day.\n\n",
            "4 shaos zmaniyos after alos 96 minutes before sunrise, with the day measured from alos 96 minutes before sunrise to tzais 96 minutes after sunset.\n\n",
            "In places such as the Arctic Circle, where there is at least one day a year when the sun does not rise and one when it does not set, this zman may not be available or cannot be calculated.",
        ),
    },

    /// Sof zman tfila - the latest time to recite morning prayers (Shacharis) according to the [Magen Avraham
    /// (MGA)](https://en.wikipedia.org/wiki/Avraham_Gombiner), using the 96 zmaniyos-minute day.
    ///
    /// 4 shaos zmaniyos after alos 96 zmaniyos minutes before sunrise, with the day measured from that alos to tzais
    /// 96 zmaniyos minutes after sunset.
    SofZmanTfilaMga96MinutesZmanis {
        java: "getSofZmanTfilaMGA96MinutesZmanis",
        name: "Sof Zman Tfila (MGA, 96 Minutes in Shaos Zmanios)",
        category: SofZmanTfila,
        formula: Tefila(&ZmanisOffset(&ConfiguredSunrise, -1.6), &ZmanisOffset(&ConfiguredSunset, 1.6), true),
        about: concat!(
            "Sof zman tfila - the latest time to recite morning prayers (Shacharis) according to the [Magen Avraham (MGA)](https://en.wikipedia.org/wiki/Avraham_Gombiner), using the 96 zmaniyos-minute day.\n\n",
            "4 shaos zmaniyos after alos 96 zmaniyos minutes before sunrise, with the day measured from that alos to tzais 96 zmaniyos minutes after sunset.\n\n",
            "In places such as the Arctic Circle, where there is at least one day a year when the sun does not rise and one when it does not set, this zman may not be available or cannot be calculated.",
        ),
    },

    /// Sof zman tfila - the latest time to recite morning prayers (Shacharis), calculated as 2 regular clock hours
    /// before chatzos hayom (not shaos zmaniyos). Paired with sof zman krias shema at 3 clock hours before chatzos;
    /// often grouped with the "Komarno" zmanim after [Rav Yitzchak Eizik of
    /// Komarno](https://en.wikipedia.org/wiki/Komarno_(Hasidic_dynasty)#Rabbi_Yitzchak_Eisik_Safrin), though this
    /// calculation is much older.
    ///
    /// 2 clock hours before chatzos hayom.
    SofZmanTfila2HoursBeforeChatzos {
        java: "getSofZmanTfila2HoursBeforeChatzos",
        name: "Sof Zman Tfila (2 Hours Before Chatzos)",
        category: SofZmanTfila,
        formula: Offset(&ChatzosHayom, Duration::from_mins(-120)),
        about: concat!(
            "Sof zman tfila - the latest time to recite morning prayers (Shacharis), calculated as 2 regular clock hours before chatzos hayom (not shaos zmaniyos). Paired with sof zman krias shema at 3 clock hours before chatzos; often grouped with the \"Komarno\" zmanim after [Rav Yitzchak Eizik of Komarno](https://en.wikipedia.org/wiki/Komarno_(Hasidic_dynasty)#Rabbi_Yitzchak_Eisik_Safrin), though this calculation is much older.\n\n",
            "2 clock hours before chatzos hayom.\n\n",
            "This view is cited by the Shach in Nekudas Hakesef (Yoreh Deah 184), [Rav Moshe Lifshitz](https://hebrewbooks.org/pdfpager.aspx?req=21638&st=&pgnum=30) in [Lechem Mishneh on Brachos 1:2](https://hebrewbooks.org/pdfpager.aspx?req=21638&st=&pgnum=50), the [Yaavetz](https://en.wikipedia.org/wiki/Jacob_Emden), and later by Komarno, Shevus Yaakov, Chasan Sofer, and others. See also [Yisrael Vehazmanim vol. 1, 7:3](https://hebrewbooks.org/pdfpager.aspx?req=9765&st=&pgnum=83).\n\n",
            "In places such as the Arctic Circle, where there is at least one day a year when the sun does not rise and one when it does not set, this zman may not be available or cannot be calculated.",
        ),
    },

    /// Sof zman tfila - the latest time to recite morning prayers (Shacharis) according to the [Magen Avraham
    /// (MGA)](https://en.wikipedia.org/wiki/Avraham_Gombiner), using the 120-minute day. This is an extremely early
    /// time, used as a chumra.
    ///
    /// 4 shaos zmaniyos after alos 120 minutes before sunrise, with the day measured from alos 120 minutes before
    /// sunrise to tzais 120 minutes after sunset.
    SofZmanTfilaMga120Minutes {
        java: "getSofZmanTfilaMGA120Minutes",
        name: "Sof Zman Tfila (MGA, 120 Minutes)",
        category: SofZmanTfila,
        formula: Tefila(&Offset(&ConfiguredSunrise, Duration::from_mins(-120)), &Offset(&ConfiguredSunset, Duration::from_mins(120)), true),
        about: concat!(
            "Sof zman tfila - the latest time to recite morning prayers (Shacharis) according to the [Magen Avraham (MGA)](https://en.wikipedia.org/wiki/Avraham_Gombiner), using the 120-minute day. This is an extremely early time, used as a chumra.\n\n",
            "4 shaos zmaniyos after alos 120 minutes before sunrise, with the day measured from alos 120 minutes before sunrise to tzais 120 minutes after sunset.\n\n",
            "In places such as the Arctic Circle, where there is at least one day a year when the sun does not rise and one when it does not set, this zman may not be available or cannot be calculated.",
        ),
    },

    /// Sof zman tfila - the latest time to recite morning prayers (Shacharis) according to the Ateret Torah
    /// calculation of Chacham Yosef Harari-Raful of Yeshivat Ateret Torah.
    ///
    /// Ateret Torah zmanim use a day that begins at alos 1/10 of the day before sunrise and ends the configured
    /// Ateret Torah offset after sunset. Sof zman tfila is 4 of those shaos zmaniyos after that alos. By this
    /// calculation, chatzos is not at midday.
    SofZmanTfilaAteretTorah {
        java: "getSofZmanTfilaAteretTorah",
        name: "Sof Zman Tfila (Ateret Torah)",
        category: SofZmanTfila,
        formula: Tefila(&ZmanisOffset(&ConfiguredSunrise, -1.2), &TzaisAteretTorah, false),
        about: concat!(
            "Sof zman tfila - the latest time to recite morning prayers (Shacharis) according to the Ateret Torah calculation of Chacham Yosef Harari-Raful of Yeshivat Ateret Torah.\n\n",
            "Ateret Torah zmanim use a day that begins at alos 1/10 of the day before sunrise and ends {ateret_torah_offset} after sunset. Sof zman tfila is 4 of those shaos zmaniyos after that alos. By this calculation, chatzos is not at midday.\n\n",
            "In places such as the Arctic Circle, where there is at least one day a year when the sun does not rise and one when it does not set, this zman may not be available or cannot be calculated.",
        ),
    },

    /// Sof zman tfila - the latest time to recite morning prayers (Shacharis) according to the Baal Hatanya.
    ///
    /// 4 shaos zmaniyos after netz amiti (true sunrise), with the day measured from Baal Hatanya sunrise to sunset.
    SofZmanTfilaBaalHatanya {
        java: "getSofZmanTfilaBaalHatanya",
        name: "Sof Zman Tfila (Baal Hatanya)",
        category: SofZmanTfila,
        formula: Tefila(&SunriseOffsetByDegrees(1.583), &SunsetOffsetByDegrees(1.583), true),
        about: concat!(
            "Sof zman tfila - the latest time to recite morning prayers (Shacharis) according to the Baal Hatanya.\n\n",
            "4 shaos zmaniyos after netz amiti (true sunrise), with the day measured from Baal Hatanya sunrise to sunset.\n\n",
            "In places such as the Arctic Circle, where there is at least one day a year when the sun does not rise and one when it does not set, this zman may not be available or cannot be calculated.",
        ),
    },

    /// Sof zman tfila - the latest time to recite morning prayers (Shacharis) according to [Rav Moshe
    /// Feinstein's](https://en.wikipedia.org/wiki/Moshe_Feinstein) view of the
    /// [GRA](https://en.wikipedia.org/wiki/Vilna_Gaon) day, using only the first half of the day.
    ///
    /// 4 shaos zmaniyos after sunrise, with shaos zmaniyos measured from sunrise to fixed local chatzos (two-thirds
    /// of that half-day).
    SofZmanTfilaGraSunriseToFixedLocalChatzos {
        java: "getSofZmanTfilaGRASunriseToFixedLocalChatzos",
        name: "Sof Zman Tfila (GR'A, Sunrise to Fixed Local Chatzos)",
        category: SofZmanTfila,
        formula: HalfDayBasedOffset(&ConfiguredSunrise, &LocalMeanTime(12.0), 4.0),
        about: concat!(
            "Sof zman tfila - the latest time to recite morning prayers (Shacharis) according to [Rav Moshe Feinstein's](https://en.wikipedia.org/wiki/Moshe_Feinstein) view of the [GRA](https://en.wikipedia.org/wiki/Vilna_Gaon) day, using only the first half of the day.\n\n",
            "4 shaos zmaniyos after sunrise, with shaos zmaniyos measured from sunrise to fixed local chatzos (two-thirds of that half-day). \n\n",
            "At some northern and southern locations, this zman may not be available or cannot be calculated if the sun does not reach low enough below the horizon.",
        ),
    },

    // SofZmanAchilasChametz

    /// Sof zman achilas chametz - the latest time to eat chametz on Erev Pesach according to the
    /// [GRA](https://en.wikipedia.org/wiki/Vilna_Gaon). Same as sof zman tfila GRA.
    ///
    /// 4 shaos zmaniyos after sunrise, with the day measured from sunrise to sunset.
    SofZmanAchilasChametzGra {
        java: "getSofZmanAchilasChametzGRA",
        name: "Sof Zman Achilas Chametz (GR'A)",
        category: SofZmanAchilasChametz,
        formula: SofZmanAchilasChametz(&ConfiguredSunrise, &ConfiguredSunset, true),
        about: concat!(
            "Sof zman achilas chametz - the latest time to eat chametz on Erev Pesach according to the [GRA](https://en.wikipedia.org/wiki/Vilna_Gaon). Same as sof zman tfila GRA.\n\n",
            "4 shaos zmaniyos after sunrise, with the day measured from sunrise to sunset.",
        ),
    },

    /// Sof zman achilas chametz - the latest time to eat chametz on Erev Pesach according to the [Magen Avraham
    /// (MGA)](https://en.wikipedia.org/wiki/Avraham_Gombiner), using the 72-minute day. Same as sof zman tfila MGA 72
    /// minutes.
    ///
    /// 4 shaos zmaniyos after alos 72 minutes before sunrise, with the day measured from alos 72 minutes before
    /// sunrise to tzais 72 minutes after sunset.
    SofZmanAchilasChametzMga72Minutes {
        java: "getSofZmanAchilasChametzMGA72Minutes",
        name: "Sof Zman Achilas Chametz (MGA, 72 Minutes)",
        category: SofZmanAchilasChametz,
        formula: SofZmanAchilasChametz(&Offset(&ConfiguredSunrise, Duration::from_mins(-72)), &Offset(&ConfiguredSunset, Duration::from_mins(72)), true),
        about: concat!(
            "Sof zman achilas chametz - the latest time to eat chametz on Erev Pesach according to the [Magen Avraham (MGA)](https://en.wikipedia.org/wiki/Avraham_Gombiner), using the 72-minute day. Same as sof zman tfila MGA 72 minutes.\n\n",
            "4 shaos zmaniyos after alos 72 minutes before sunrise, with the day measured from alos 72 minutes before sunrise to tzais 72 minutes after sunset.",
        ),
    },

    /// Sof zman achilas chametz - the latest time to eat chametz on Erev Pesach according to the [Magen Avraham
    /// (MGA)](https://en.wikipedia.org/wiki/Avraham_Gombiner), using the 72 zmaniyos-minute day. Same as sof zman
    /// tfila MGA 72 minutes zmanis.
    ///
    /// 4 shaos zmaniyos after alos 72 zmaniyos minutes before sunrise, with the day measured from alos 72 zmaniyos
    /// minutes before sunrise to tzais 72 zmaniyos minutes after sunset.
    SofZmanAchilasChametzMga72MinutesZmanis {
        java: "getSofZmanAchilasChametzMGA72MinutesZmanis",
        name: "Sof Zman Achilas Chametz (MGA, 72 Minutes in Shaos Zmanios)",
        category: SofZmanAchilasChametz,
        formula: SofZmanAchilasChametz(&ZmanisOffset(&ConfiguredSunrise, -1.2), &ZmanisOffset(&ConfiguredSunset, 1.2), true),
        about: concat!(
            "Sof zman achilas chametz - the latest time to eat chametz on Erev Pesach according to the [Magen Avraham (MGA)](https://en.wikipedia.org/wiki/Avraham_Gombiner), using the 72 zmaniyos-minute day. Same as sof zman tfila MGA 72 minutes zmanis.\n\n",
            "4 shaos zmaniyos after alos 72 zmaniyos minutes before sunrise, with the day measured from alos 72 zmaniyos minutes before sunrise to tzais 72 zmaniyos minutes after sunset.",
        ),
    },

    /// Sof zman achilas chametz - the latest time to eat chametz on Erev Pesach according to the [Magen Avraham
    /// (MGA)](https://en.wikipedia.org/wiki/Avraham_Gombiner), using the 16.1-degree day.
    ///
    /// 4 shaos zmaniyos after alos at 16.1 degrees, with the day measured from alos at 16.1 degrees to tzais at 16.1
    /// degrees.
    SofZmanAchilasChametzMga16Point1Degrees {
        java: "getSofZmanAchilasChametzMGA16Point1Degrees",
        name: "Sof Zman Achilas Chametz (MGA, 16.1 Degrees)",
        category: SofZmanAchilasChametz,
        formula: SofZmanAchilasChametz(&SunriseOffsetByDegrees(16.1), &SunsetOffsetByDegrees(16.1), true),
        about: concat!(
            "Sof zman achilas chametz - the latest time to eat chametz on Erev Pesach according to the [Magen Avraham (MGA)](https://en.wikipedia.org/wiki/Avraham_Gombiner), using the 16.1-degree day.\n\n",
            "4 shaos zmaniyos after alos at 16.1 degrees, with the day measured from alos at 16.1 degrees to tzais at 16.1 degrees.",
        ),
    },

    /// Sof zman achilas chametz - the latest time to eat chametz on Erev Pesach according to the Baal Hatanya. Same
    /// as sof zman tfila Baal Hatanya.
    ///
    /// 4 shaos zmaniyos after netz amiti (true sunrise), with the day measured from Baal Hatanya sunrise to sunset.
    SofZmanAchilasChametzBaalHatanya {
        java: "getSofZmanAchilasChametzBaalHatanya",
        name: "Sof Zman Achilas Chametz (Baal Hatanya)",
        category: SofZmanAchilasChametz,
        formula: SofZmanAchilasChametz(&SunriseOffsetByDegrees(1.583), &SunsetOffsetByDegrees(1.583), true),
        about: concat!(
            "Sof zman achilas chametz - the latest time to eat chametz on Erev Pesach according to the Baal Hatanya. Same as sof zman tfila Baal Hatanya.\n\n",
            "4 shaos zmaniyos after netz amiti (true sunrise), with the day measured from Baal Hatanya sunrise to sunset.",
        ),
    },

    // SofZmanBiurChametz

    /// Sof zman biur chametz - the latest time to burn chametz on Erev Pesach according to the
    /// [GRA](https://en.wikipedia.org/wiki/Vilna_Gaon).
    ///
    /// 5 shaos zmaniyos after sunrise, with the day measured from sunrise to sunset.
    SofZmanBiurChametzGra {
        java: "getSofZmanBiurChametzGRA",
        name: "Sof Zman Biur Chametz (GR'A)",
        category: SofZmanBiurChametz,
        formula: SofZmanBiurChametz(&ConfiguredSunrise, &ConfiguredSunset, true),
        about: concat!(
            "Sof zman biur chametz - the latest time to burn chametz on Erev Pesach according to the [GRA](https://en.wikipedia.org/wiki/Vilna_Gaon).\n\n",
            "5 shaos zmaniyos after sunrise, with the day measured from sunrise to sunset.",
        ),
    },

    /// Sof zman biur chametz - the latest time to burn chametz on Erev Pesach according to the [Magen Avraham
    /// (MGA)](https://en.wikipedia.org/wiki/Avraham_Gombiner), using the 72-minute day.
    ///
    /// 5 shaos zmaniyos after alos 72 minutes before sunrise, with the day measured from alos 72 minutes before
    /// sunrise to tzais 72 minutes after sunset.
    SofZmanBiurChametzMga72Minutes {
        java: "getSofZmanBiurChametzMGA72Minutes",
        name: "Sof Zman Biur Chametz (MGA, 72 Minutes)",
        category: SofZmanBiurChametz,
        formula: SofZmanBiurChametz(&Offset(&ConfiguredSunrise, Duration::from_mins(-72)), &Offset(&ConfiguredSunset, Duration::from_mins(72)), true),
        about: concat!(
            "Sof zman biur chametz - the latest time to burn chametz on Erev Pesach according to the [Magen Avraham (MGA)](https://en.wikipedia.org/wiki/Avraham_Gombiner), using the 72-minute day.\n\n",
            "5 shaos zmaniyos after alos 72 minutes before sunrise, with the day measured from alos 72 minutes before sunrise to tzais 72 minutes after sunset.",
        ),
    },

    /// Sof zman biur chametz - the latest time to burn chametz on Erev Pesach according to the [Magen Avraham
    /// (MGA)](https://en.wikipedia.org/wiki/Avraham_Gombiner), using the 72 zmaniyos-minute day.
    ///
    /// 5 shaos zmaniyos after alos 72 zmaniyos minutes before sunrise, with the day measured from alos 72 zmaniyos
    /// minutes before sunrise to tzais 72 zmaniyos minutes after sunset.
    SofZmanBiurChametzMga72MinutesZmanis {
        java: "getSofZmanBiurChametzMGA72MinutesZmanis",
        name: "Sof Zman Biur Chametz (MGA, 72 Minutes in Shaos Zmanios)",
        category: SofZmanBiurChametz,
        formula: SofZmanBiurChametz(&ZmanisOffset(&ConfiguredSunrise, -1.2), &ZmanisOffset(&ConfiguredSunset, 1.2), true),
        about: concat!(
            "Sof zman biur chametz - the latest time to burn chametz on Erev Pesach according to the [Magen Avraham (MGA)](https://en.wikipedia.org/wiki/Avraham_Gombiner), using the 72 zmaniyos-minute day.\n\n",
            "5 shaos zmaniyos after alos 72 zmaniyos minutes before sunrise, with the day measured from alos 72 zmaniyos minutes before sunrise to tzais 72 zmaniyos minutes after sunset.",
        ),
    },

    /// Sof zman biur chametz - the latest time to burn chametz on Erev Pesach according to the [Magen Avraham
    /// (MGA)](https://en.wikipedia.org/wiki/Avraham_Gombiner), using the 16.1-degree day.
    ///
    /// 5 shaos zmaniyos after alos at 16.1 degrees, with the day measured from alos at 16.1 degrees to tzais at 16.1
    /// degrees.
    SofZmanBiurChametzMga16Point1Degrees {
        java: "getSofZmanBiurChametzMGA16Point1Degrees",
        name: "Sof Zman Biur Chametz (MGA, 16.1 Degrees)",
        category: SofZmanBiurChametz,
        formula: SofZmanBiurChametz(&SunriseOffsetByDegrees(16.1), &SunsetOffsetByDegrees(16.1), true),
        about: concat!(
            "Sof zman biur chametz - the latest time to burn chametz on Erev Pesach according to the [Magen Avraham (MGA)](https://en.wikipedia.org/wiki/Avraham_Gombiner), using the 16.1-degree day.\n\n",
            "5 shaos zmaniyos after alos at 16.1 degrees, with the day measured from alos at 16.1 degrees to tzais at 16.1 degrees.",
        ),
    },

    /// Sof zman biur chametz - the latest time to burn chametz on Erev Pesach according to the Baal Hatanya.
    ///
    /// 5 shaos zmaniyos after netz amiti (true sunrise), with the day measured from Baal Hatanya sunrise to sunset.
    SofZmanBiurChametzBaalHatanya {
        java: "getSofZmanBiurChametzBaalHatanya",
        name: "Sof Zman Biur Chametz (Baal Hatanya)",
        category: SofZmanBiurChametz,
        formula: SofZmanBiurChametz(&SunriseOffsetByDegrees(1.583), &SunsetOffsetByDegrees(1.583), true),
        about: concat!(
            "Sof zman biur chametz - the latest time to burn chametz on Erev Pesach according to the Baal Hatanya.\n\n",
            "5 shaos zmaniyos after netz amiti (true sunrise), with the day measured from Baal Hatanya sunrise to sunset.",
        ),
    },

    // ChatzosHayom

    /// Solar noon - when the sun transits the celestial meridian.
    ///
    /// Also called sundial noon or astronomical chatzos hayom. See [The Definition of
    /// Chatzos](https://kosherjava.com/2020/07/02/definition-of-chatzos/) for details on the proper definition of
    /// solar noon.
    SunTransit {
        java: "getSunTransit",
        name: "Solar Transit",
        category: ChatzosHayom,
        formula: SolarTransit,
        about: concat!(
            "Solar noon - when the sun transits the celestial meridian.\n\n",
            "Also called sundial noon or astronomical chatzos hayom. See [The Definition of Chatzos](https://kosherjava.com/2020/07/02/definition-of-chatzos/) for details on the proper definition of solar noon.\n\n",
            "In places such as the Arctic Circle, where there is at least one day a year when the sun does not rise and one when it does not set, this zman may not be available or cannot be calculated.",
        ),
    },

    /// Chatzos hayom.
    ///
    /// For how chatzos can be defined and calculated, see [The Definition of
    /// Chatzos](https://kosherjava.com/2020/07/02/definition-of-chatzos/) on the KosherJava blog.
    ChatzosHayom {
        java: "getChatzosHayom",
        name: "Chatzos Hayom",
        category: ChatzosHayom,
        formula: ChatzosHayom,
        about: concat!(
            "Chatzos hayom.\n\n",
            "For how chatzos can be defined and calculated, see [The Definition of Chatzos](https://kosherjava.com/2020/07/02/definition-of-chatzos/) on the KosherJava blog.",
        ),
    },

    /// Chatzos hayom calculated as the halfway point between sunrise and sunset.
    ///
    /// This is the same as six shaos zmaniyos after sunrise when the day is measured from sunrise to sunset. Many
    /// hold that chatzos is the midpoint between sea level sunrise and sea level sunset, even though astronomical
    /// chatzos is usually a slightly different time.
    ChatzosHayomAsHalfDay {
        java: "getChatzosHayomAsHalfDay",
        name: "Chatzos Hayom (Midpoint of Sunrise and Sunset)",
        category: ChatzosHayom,
        formula: ChatzosHayomAsHalfDay,
        about: concat!(
            "Chatzos hayom calculated as the halfway point between sunrise and sunset.\n\n",
            "This is the same as six shaos zmaniyos after sunrise when the day is measured from sunrise to sunset. Many hold that chatzos is the midpoint between sea level sunrise and sea level sunset, even though astronomical chatzos is usually a slightly different time.\n\n",
            "A day measured from alos to tzais with the same offset on both sides can also have the same midpoint.\n\n",
            "For how chatzos can be defined and calculated, see [The Definition of Chatzos](https://kosherjava.com/2020/07/02/definition-of-chatzos/) on the KosherJava blog.\n\n",
            "In places such as the Arctic Circle, where there is at least one day a year when the sun does not rise and one when it does not set, this zman may not be available or cannot be calculated.",
        ),
    },

    /// Fixed local chatzos - clock noon adjusted for the location's longitude and time zone, not tied only to
    /// theoretical 15-degree time zones.
    ///
    /// The globe is divided into 24 hours over 360 degrees, or 15 degrees per hour (4 minutes per degree). At
    /// longitudes 0, 15, 30, and so on, chatzos is exactly 12:00 noon. The result is adjusted to the actual time zone
    /// and [daylight saving time](https://en.wikipedia.org/wiki/Daylight_saving_time).
    FixedLocalChatzosHayom {
        java: "getFixedLocalChatzosHayom",
        name: "Chatzos Hayom (Fixed Local Chatzos)",
        category: ChatzosHayom,
        formula: LocalMeanTime(12.0),
        about: concat!(
            "Fixed local chatzos - clock noon adjusted for the location's longitude and time zone, not tied only to theoretical 15-degree time zones.\n\n",
            "The globe is divided into 24 hours over 360 degrees, or 15 degrees per hour (4 minutes per degree). At longitudes 0, 15, 30, and so on, chatzos is exactly 12:00 noon. The result is adjusted to the actual time zone and [daylight saving time](https://en.wikipedia.org/wiki/Daylight_saving_time).\n\n",
            "This is the time of chatzos according to the [Aruch Hashulchan](https://en.wikipedia.org/wiki/Aruch_HaShulchan) ([Orach Chaim 233:14](https://hebrewbooks.org/pdfpager.aspx?req=7705&pgnum=426)) and [Rabbi Moshe Feinstein](https://en.wikipedia.org/wiki/Moshe_Feinstein) ([Igros Moshe, Orach Chaim 1:24](https://hebrewbooks.org/pdfpager.aspx?req=916&st=&pgnum=67), [2:20](https://hebrewbooks.org/pdfpager.aspx?req=14675&pgnum=191)).",
        ),
    },

    // MinchaGedola

    /// Mincha gedola according to the [Vilna Gaon](https://en.wikipedia.org/wiki/Vilna_Gaon).
    ///
    /// 6.5 shaos zmaniyos after sunrise. The day is measured from sunrise to sunset for the proportional hour.
    MinchaGedolaGra {
        java: "getMinchaGedolaGRA",
        name: "Mincha Gedola (GR'A)",
        category: MinchaGedola,
        formula: MinchaGedola(&ConfiguredSunrise, &ConfiguredSunset, true),
        about: concat!(
            "Mincha gedola according to the [Vilna Gaon](https://en.wikipedia.org/wiki/Vilna_Gaon).\n\n",
            "6.5 shaos zmaniyos after sunrise. The day is measured from sunrise to sunset for the proportional hour.\n\n",
            "This is the earliest time one can pray mincha.\n\n",
            "In places where sunrise or sunset cannot be calculated, this zman may not be available.",
        ),
    },

    /// Mincha gedola according to the Magen Avraham, using the 16.1-degree day.
    ///
    /// Half a shaah zmanis after chatzos hayom, using a day that begins and ends at 16.1 degrees.
    MinchaGedola16Point1Degrees {
        java: "getMinchaGedola16Point1Degrees",
        name: "Mincha Gedola (16.1 Degrees)",
        category: MinchaGedola,
        formula: MinchaGedola(&SunriseOffsetByDegrees(16.1), &SunsetOffsetByDegrees(16.1), true),
        about: concat!(
            "Mincha gedola according to the Magen Avraham, using the 16.1-degree day.\n\n",
            "Half a shaah zmanis after chatzos hayom, using a day that begins and ends at 16.1 degrees.\n\n",
            "At some northern and southern locations, this zman may not be available if the sun does not reach low enough below the horizon.",
        ),
    },

    /// Mincha gedola calculated as 30 minutes after chatzos hayom.
    ///
    /// Some use this in winter when half a shaah zmanis is less than 30 minutes, to delay the start of mincha.
    MinchaGedola30Minutes {
        java: "getMinchaGedola30Minutes",
        name: "Mincha Gedola (30 Minutes)",
        category: MinchaGedola,
        formula: Offset(&ChatzosHayom, Duration::from_mins(30)),
        about: concat!(
            "Mincha gedola calculated as 30 minutes after chatzos hayom.\n\n",
            "Some use this in winter when half a shaah zmanis is less than 30 minutes, to delay the start of mincha.\n\n",
            "Do not use this time to begin mincha before standard mincha gedola GRA.\n\n",
            "In places where chatzos cannot be calculated, this zman may not be available.",
        ),
    },

    /// Mincha gedola according to the Magen Avraham.
    ///
    /// Half a shaah zmanis after chatzos hayom, using a day from alos 72 minutes to tzais 72 minutes.
    MinchaGedola72Minutes {
        java: "getMinchaGedola72Minutes",
        name: "Mincha Gedola (72 Minutes)",
        category: MinchaGedola,
        formula: MinchaGedola(&Offset(&ConfiguredSunrise, Duration::from_mins(-72)), &Offset(&ConfiguredSunset, Duration::from_mins(72)), true),
        about: concat!(
            "Mincha gedola according to the Magen Avraham.\n\n",
            "Half a shaah zmanis after chatzos hayom, using a day from alos 72 minutes to tzais 72 minutes.\n\n",
            "In places where sunrise or sunset cannot be calculated, this zman may not be available.",
        ),
    },

    /// Mincha gedola according to Rabbi Yaakov Moshe Hillel, as published in the luach of the Bais Horaah of Yeshivat
    /// Chevrat Ahavat Shalom.
    ///
    /// Half a shaah zmanis after chatzos, using a day from alos 16.1 degrees to tzais 3.7 degrees.
    MinchaGedolaAhavatShalom {
        java: "getMinchaGedolaAhavatShalom",
        name: "Mincha Gedola (Ahavat Shalom)",
        category: MinchaGedola,
        formula: MinchaGedolaAhavatShalom,
        about: concat!(
            "Mincha gedola according to Rabbi Yaakov Moshe Hillel, as published in the luach of the Bais Horaah of Yeshivat Chevrat Ahavat Shalom.\n\n",
            "Half a shaah zmanis after chatzos, using a day from alos 16.1 degrees to tzais 3.7 degrees.\n\n",
            "The later of this time or 30 clock minutes after chatzos is used.\n\n",
            "At some northern and southern locations, this zman may not be available if the sun does not reach low enough below the horizon.",
        ),
    },

    /// Mincha gedola calculated as the later of mincha gedola GRA and 30 minutes after chatzos hayom.
    ///
    /// In the winter, when half a shaah zmanis is less than 30 minutes, the 30-minutes-after-chatzos time is used.
    /// Otherwise, mincha gedola GRA is used.
    MinchaGedolaGraGreaterThan30 {
        java: "getMinchaGedolaGRAGreaterThan30",
        name: "Mincha Gedola (GR'A, Greater Than 30)",
        category: MinchaGedola,
        formula: MinchaGedolaGraGreaterThan30,
        about: concat!(
            "Mincha gedola calculated as the later of mincha gedola GRA and 30 minutes after chatzos hayom.\n\n",
            "In the winter, when half a shaah zmanis is less than 30 minutes, the 30-minutes-after-chatzos time is used. Otherwise, mincha gedola GRA is used.\n\n",
            "In places where sunrise, sunset, or chatzos cannot be calculated, this zman may not be available.",
        ),
    },

    /// Mincha gedola according to the Ateret Torah calculation of Chacham Yosef Harari-Raful of Yeshivat Ateret
    /// Torah.
    ///
    /// 6.5 shaos zmaniyos after alos.
    MinchaGedolaAteretTorah {
        java: "getMinchaGedolaAteretTorah",
        name: "Mincha Gedola (Ateret Torah)",
        category: MinchaGedola,
        formula: MinchaGedola(&ZmanisOffset(&ConfiguredSunrise, -1.2), &TzaisAteretTorah, false),
        about: concat!(
            "Mincha gedola according to the Ateret Torah calculation of Chacham Yosef Harari-Raful of Yeshivat Ateret Torah.\n\n",
            "6.5 shaos zmaniyos after alos.\n\n",
            "The day begins at alos 1/10 of the day before sunrise and ends {ateret_torah_offset} after sunset.\n\n",
            "In places where sunrise or sunset cannot be calculated, this zman may not be available.",
        ),
    },

    /// Mincha gedola according to the Baal Hatanya.
    ///
    /// 6.5 shaos zmaniyos after netz amiti, using a day from Baal Hatanya sunrise to sunset.
    MinchaGedolaBaalHatanya {
        java: "getMinchaGedolaBaalHatanya",
        name: "Mincha Gedola (Baal Hatanya)",
        category: MinchaGedola,
        formula: MinchaGedola(&SunriseOffsetByDegrees(1.583), &SunsetOffsetByDegrees(1.583), true),
        about: concat!(
            "Mincha gedola according to the Baal Hatanya.\n\n",
            "6.5 shaos zmaniyos after netz amiti, using a day from Baal Hatanya sunrise to sunset.\n\n",
            "In places where sunrise or sunset cannot be calculated, this zman may not be available.",
        ),
    },

    /// Mincha gedola according to Rav Moshe Feinstein's opinion.
    ///
    /// 30 minutes after fixed local chatzos.
    MinchaGedolaGraFixedLocalChatzos30Minutes {
        java: "getMinchaGedolaGRAFixedLocalChatzos30Minutes",
        name: "Mincha Gedola (GR'A, Fixed Local Chatzos, 30 Minutes)",
        category: MinchaGedola,
        formula: Offset(&LocalMeanTime(12.0), Duration::from_mins(30)),
        about: concat!(
            "Mincha gedola according to Rav Moshe Feinstein's opinion.\n\n",
            "30 minutes after fixed local chatzos.\n\n",
            "In places where fixed local chatzos cannot be calculated, this zman may not be available.",
        ),
    },

    // PlagHamincha

    /// Plag hamincha according to the [Vilna Gaon](https://en.wikipedia.org/wiki/Vilna_Gaon).
    ///
    /// 10.75 shaos zmaniyos after sunrise, using a day from sunrise to sunset.
    PlagHaminchaGra {
        java: "getPlagHaminchaGRA",
        name: "Plag Hamincha (GR'A)",
        category: PlagHamincha,
        formula: PlagHamincha(&ConfiguredSunrise, &ConfiguredSunset, true),
        about: concat!(
            "Plag hamincha according to the [Vilna Gaon](https://en.wikipedia.org/wiki/Vilna_Gaon).\n\n",
            "10.75 shaos zmaniyos after sunrise, using a day from sunrise to sunset. \n\n",
            "This is the earliest time Shabbos can be started.\n\n",
            "In places where sunrise or sunset cannot be calculated, this zman may not be available.",
        ),
    },

    /// Plag hamincha according to [Rabbi Yaakov Moshe Hillel](https://en.wikipedia.org/wiki/Yaakov_Moshe_Hillel), as
    /// published in the luach of the Bais Horaah of Yeshivat Chevrat Ahavat Shalom.
    ///
    /// 1.25 shaos zmaniyos before tzais 3.8 degrees, using a day from alos 16.1 degrees to tzais 3.8 degrees.
    PlagAhavatShalom {
        java: "getPlagAhavatShalom",
        name: "Plag Hamincha (Ahavat Shalom)",
        category: PlagHamincha,
        formula: PlagAhavatShalom,
        about: concat!(
            "Plag hamincha according to [Rabbi Yaakov Moshe Hillel](https://en.wikipedia.org/wiki/Yaakov_Moshe_Hillel), as published in the luach of the Bais Horaah of Yeshivat Chevrat Ahavat Shalom.\n\n",
            "1.25 shaos zmaniyos before tzais 3.8 degrees, using a day from alos 16.1 degrees to tzais 3.8 degrees.\n\n",
            "At some northern and southern locations, this zman may not be available if the sun does not reach low enough below the horizon.",
        ),
    },

    /// Plag hamincha based on a day from alos 16.1 degrees to tzais Geonim 7.083 degrees.
    ///
    /// 10.75 shaos zmaniyos after alos 16.1 degrees.
    PlagAlos16Point1DegreesToTzaisGeonim7Point083Degrees {
        java: "getPlagAlos16Point1DegreesToTzaisGeonim7Point083Degrees",
        name: "Plag Hamincha (Alos 16.1 to Tzais Geonim 7.083 Degrees)",
        category: PlagHamincha,
        formula: PlagHamincha(&SunriseOffsetByDegrees(16.1), &SunsetOffsetByDegrees(7.083333333333333), false),
        about: concat!(
            "Plag hamincha based on a day from alos 16.1 degrees to tzais Geonim 7.083 degrees.\n\n",
            "10.75 shaos zmaniyos after alos 16.1 degrees.\n\n",
            "At some northern and southern locations, this zman may not be available if the sun does not reach low enough below the horizon.",
        ),
    },

    /// Plag hamincha based on a day from alos 16.1 degrees to sunset.
    ///
    /// 10.75 shaos zmaniyos after alos 16.1 degrees.
    #[deprecated(note = "KosherJava deprecates this zman")]
    PlagAlosToSunset {
        java: "getPlagAlosToSunset",
        name: "Plag Hamincha (Alos 16.1 to Sunset)",
        category: PlagHamincha,
        formula: PlagHamincha(&SunriseOffsetByDegrees(16.1), &ConfiguredSunset, false),
        about: concat!(
            "Plag hamincha based on a day from alos 16.1 degrees to sunset.\n\n",
            "10.75 shaos zmaniyos after alos 16.1 degrees.\n\n",
            "This zman should be used lechumra only. It can be a very late time, often after shkiah, and using it leniently can lead to chillul Shabbos.\n\n",
            "At some northern and southern locations, this zman may not be available if the sun does not reach low enough below the horizon.",
        ),
    },

    /// Plag hamincha according to the Magen Avraham, using the 60-minute day.
    ///
    /// 10.75 shaos zmaniyos after alos 60 minutes before sunrise, using a day that starts 60 minutes before sunrise
    /// and ends 60 minutes after sunset.
    PlagHamincha60Minutes {
        java: "getPlagHamincha60Minutes",
        name: "Plag Hamincha (60 Minutes)",
        category: PlagHamincha,
        formula: PlagHamincha(&Offset(&ConfiguredSunrise, Duration::from_mins(-60)), &Offset(&ConfiguredSunset, Duration::from_mins(60)), true),
        about: concat!(
            "Plag hamincha according to the Magen Avraham, using the 60-minute day.\n\n",
            "10.75 shaos zmaniyos after alos 60 minutes before sunrise, using a day that starts 60 minutes before sunrise and ends 60 minutes after sunset.\n\n",
            "In places where sunrise or sunset cannot be calculated, this zman may not be available.",
        ),
    },

    /// Plag hamincha according to the Magen Avraham, using the 72-minute day.
    ///
    /// 10.75 shaos zmaniyos after alos 72 minutes before sunrise, using a day that starts 72 minutes before sunrise
    /// and ends 72 minutes after sunset.
    #[deprecated(note = "KosherJava deprecates this zman")]
    PlagHamincha72Minutes {
        java: "getPlagHamincha72Minutes",
        name: "Plag Hamincha (72 Minutes)",
        category: PlagHamincha,
        formula: PlagHamincha(&Offset(&ConfiguredSunrise, Duration::from_mins(-72)), &Offset(&ConfiguredSunset, Duration::from_mins(72)), true),
        about: concat!(
            "Plag hamincha according to the Magen Avraham, using the 72-minute day.\n\n",
            "10.75 shaos zmaniyos after alos 72 minutes before sunrise, using a day that starts 72 minutes before sunrise and ends 72 minutes after sunset.\n\n",
            "This zman should be used lechumra only. It can be a very late time, often after shkiah, and using it leniently can lead to chillul Shabbos.\n\n",
            "In places where sunrise or sunset cannot be calculated, this zman may not be available.",
        ),
    },

    /// Plag hamincha based on alos 72 zmaniyos minutes before sunrise.
    ///
    /// 10.75 shaos zmaniyos after that alos.
    #[deprecated(note = "KosherJava deprecates this zman")]
    PlagHamincha72MinutesZmanis {
        java: "getPlagHamincha72MinutesZmanis",
        name: "Plag Hamincha (72 Minutes in Shaos Zmanios)",
        category: PlagHamincha,
        formula: PlagHamincha(&ZmanisOffset(&ConfiguredSunrise, -1.2), &ZmanisOffset(&ConfiguredSunset, 1.2), true),
        about: concat!(
            "Plag hamincha based on alos 72 zmaniyos minutes before sunrise.\n\n",
            "10.75 shaos zmaniyos after that alos.\n\n",
            "This zman should be used lechumra only. It can be a very late time, often after shkiah, and using it leniently can lead to chillul Shabbos.\n\n",
            "In places where sunrise or sunset cannot be calculated, this zman may not be available.",
        ),
    },

    /// Plag hamincha according to the Magen Avraham, using the 90-minute day.
    ///
    /// 10.75 shaos zmaniyos after alos 90 minutes before sunrise, using a day that starts 90 minutes before sunrise
    /// and ends 90 minutes after sunset.
    #[deprecated(note = "KosherJava deprecates this zman")]
    PlagHamincha90Minutes {
        java: "getPlagHamincha90Minutes",
        name: "Plag Hamincha (90 Minutes)",
        category: PlagHamincha,
        formula: PlagHamincha(&Offset(&ConfiguredSunrise, Duration::from_mins(-90)), &Offset(&ConfiguredSunset, Duration::from_mins(90)), true),
        about: concat!(
            "Plag hamincha according to the Magen Avraham, using the 90-minute day.\n\n",
            "10.75 shaos zmaniyos after alos 90 minutes before sunrise, using a day that starts 90 minutes before sunrise and ends 90 minutes after sunset.\n\n",
            "This zman should be used lechumra only. It can be a very late time, often after shkiah, and using it leniently can lead to chillul Shabbos.\n\n",
            "In places where sunrise or sunset cannot be calculated, this zman may not be available.",
        ),
    },

    /// Plag hamincha based on alos 90 zmaniyos minutes before sunrise.
    ///
    /// 10.75 shaos zmaniyos after that alos.
    #[deprecated(note = "KosherJava deprecates this zman")]
    PlagHamincha90MinutesZmanis {
        java: "getPlagHamincha90MinutesZmanis",
        name: "Plag Hamincha (90 Minutes in Shaos Zmanios)",
        category: PlagHamincha,
        formula: PlagHamincha(&ZmanisOffset(&ConfiguredSunrise, -1.5), &ZmanisOffset(&ConfiguredSunset, 1.5), true),
        about: concat!(
            "Plag hamincha based on alos 90 zmaniyos minutes before sunrise.\n\n",
            "10.75 shaos zmaniyos after that alos.\n\n",
            "This zman should be used lechumra only. It can be a very late time, often after shkiah, and using it leniently can lead to chillul Shabbos.\n\n",
            "In places where sunrise or sunset cannot be calculated, this zman may not be available.",
        ),
    },

    /// Plag hamincha according to the Magen Avraham, using the 96-minute day.
    ///
    /// 10.75 shaos zmaniyos after alos 96 minutes before sunrise, using a day that starts 96 minutes before sunrise
    /// and ends 96 minutes after sunset.
    #[deprecated(note = "KosherJava deprecates this zman")]
    PlagHamincha96Minutes {
        java: "getPlagHamincha96Minutes",
        name: "Plag Hamincha (96 Minutes)",
        category: PlagHamincha,
        formula: PlagHamincha(&Offset(&ConfiguredSunrise, Duration::from_mins(-96)), &Offset(&ConfiguredSunset, Duration::from_mins(96)), true),
        about: concat!(
            "Plag hamincha according to the Magen Avraham, using the 96-minute day.\n\n",
            "10.75 shaos zmaniyos after alos 96 minutes before sunrise, using a day that starts 96 minutes before sunrise and ends 96 minutes after sunset.\n\n",
            "This zman should be used lechumra only. It can be a very late time, often after shkiah, and using it leniently can lead to chillul Shabbos.\n\n",
            "In places where sunrise or sunset cannot be calculated, this zman may not be available.",
        ),
    },

    /// Plag hamincha based on alos 96 zmaniyos minutes before sunrise.
    ///
    /// 10.75 shaos zmaniyos after that alos.
    #[deprecated(note = "KosherJava deprecates this zman")]
    PlagHamincha96MinutesZmanis {
        java: "getPlagHamincha96MinutesZmanis",
        name: "Plag Hamincha (96 Minutes in Shaos Zmanios)",
        category: PlagHamincha,
        formula: PlagHamincha(&ZmanisOffset(&ConfiguredSunrise, -1.6), &ZmanisOffset(&ConfiguredSunset, 1.6), true),
        about: concat!(
            "Plag hamincha based on alos 96 zmaniyos minutes before sunrise.\n\n",
            "10.75 shaos zmaniyos after that alos.\n\n",
            "This zman should be used lechumra only. It can be a very late time, often after shkiah, and using it leniently can lead to chillul Shabbos.\n\n",
            "In places where sunrise or sunset cannot be calculated, this zman may not be available.",
        ),
    },

    /// Plag hamincha according to the Magen Avraham, using the 120-minute day.
    ///
    /// 10.75 shaos zmaniyos after alos 120 minutes before sunrise, using a day that starts 120 minutes before sunrise
    /// and ends 120 minutes after sunset.
    #[deprecated(note = "KosherJava deprecates this zman")]
    PlagHamincha120Minutes {
        java: "getPlagHamincha120Minutes",
        name: "Plag Hamincha (120 Minutes)",
        category: PlagHamincha,
        formula: PlagHamincha(&Offset(&ConfiguredSunrise, Duration::from_mins(-120)), &Offset(&ConfiguredSunset, Duration::from_mins(120)), true),
        about: concat!(
            "Plag hamincha according to the Magen Avraham, using the 120-minute day.\n\n",
            "10.75 shaos zmaniyos after alos 120 minutes before sunrise, using a day that starts 120 minutes before sunrise and ends 120 minutes after sunset.\n\n",
            "This zman should be used lechumra only. It can be a very late time, often after shkiah, and using it leniently can lead to chillul Shabbos.\n\n",
            "In places where sunrise or sunset cannot be calculated, this zman may not be available.",
        ),
    },

    /// Plag hamincha based on alos 120 zmaniyos minutes (one-sixth of the day) before sunrise.
    ///
    /// 10.75 shaos zmaniyos after that alos.
    #[deprecated(note = "KosherJava deprecates this zman")]
    PlagHamincha120MinutesZmanis {
        java: "getPlagHamincha120MinutesZmanis",
        name: "Plag Hamincha (120 Minutes in Shaos Zmanios)",
        category: PlagHamincha,
        formula: PlagHamincha(&ZmanisOffset(&ConfiguredSunrise, -2.0), &ZmanisOffset(&ConfiguredSunset, 2.0), true),
        about: concat!(
            "Plag hamincha based on alos 120 zmaniyos minutes (one-sixth of the day) before sunrise.\n\n",
            "10.75 shaos zmaniyos after that alos.\n\n",
            "This zman should be used lechumra only. It can be a very late time, often after shkiah, and using it leniently can lead to chillul Shabbos.\n\n",
            "In places where sunrise or sunset cannot be calculated, this zman may not be available.",
        ),
    },

    /// Plag hamincha based on the 16.1-degree day.
    ///
    /// 10.75 shaos zmaniyos after alos 16.1 degrees, using a day that begins and ends at 16.1 degrees.
    #[deprecated(note = "KosherJava deprecates this zman")]
    PlagHamincha16Point1Degrees {
        java: "getPlagHamincha16Point1Degrees",
        name: "Plag Hamincha (16.1 Degrees)",
        category: PlagHamincha,
        formula: PlagHamincha(&SunriseOffsetByDegrees(16.1), &SunsetOffsetByDegrees(16.1), true),
        about: concat!(
            "Plag hamincha based on the 16.1-degree day.\n\n",
            "10.75 shaos zmaniyos after alos 16.1 degrees, using a day that begins and ends at 16.1 degrees.\n\n",
            "This zman should be used lechumra only. It can be a very late time, often after shkiah, and using it leniently can lead to chillul Shabbos.\n\n",
            "At some northern and southern locations, this zman may not be available if the sun does not reach low enough below the horizon.",
        ),
    },

    /// Plag hamincha based on the 18-degree day.
    ///
    /// 10.75 shaos zmaniyos after alos 18 degrees, using a day that begins and ends at 18 degrees.
    #[deprecated(note = "KosherJava deprecates this zman")]
    PlagHamincha18Degrees {
        java: "getPlagHamincha18Degrees",
        name: "Plag Hamincha (18 Degrees)",
        category: PlagHamincha,
        formula: PlagHamincha(&SunriseOffsetByDegrees(18.0), &SunsetOffsetByDegrees(18.0), true),
        about: concat!(
            "Plag hamincha based on the 18-degree day.\n\n",
            "10.75 shaos zmaniyos after alos 18 degrees, using a day that begins and ends at 18 degrees.\n\n",
            "This zman should be used lechumra only. It can be a very late time, often after shkiah, and using it leniently can lead to chillul Shabbos.\n\n",
            "At some northern and southern locations, this zman may not be available if the sun does not reach low enough below the horizon.",
        ),
    },

    /// Plag hamincha based on the 19.8-degree day.
    ///
    /// 10.75 shaos zmaniyos after alos 19.8 degrees, using a day that begins and ends at 19.8 degrees.
    #[deprecated(note = "KosherJava deprecates this zman")]
    PlagHamincha19Point8Degrees {
        java: "getPlagHamincha19Point8Degrees",
        name: "Plag Hamincha (19.8 Degrees)",
        category: PlagHamincha,
        formula: PlagHamincha(&SunriseOffsetByDegrees(19.8), &SunsetOffsetByDegrees(19.8), true),
        about: concat!(
            "Plag hamincha based on the 19.8-degree day.\n\n",
            "10.75 shaos zmaniyos after alos 19.8 degrees, using a day that begins and ends at 19.8 degrees.\n\n",
            "This zman should be used lechumra only. It can be a very late time, often after shkiah, and using it leniently can lead to chillul Shabbos.\n\n",
            "At some northern and southern locations, this zman may not be available if the sun does not reach low enough below the horizon.",
        ),
    },

    /// Plag hamincha based on the 26-degree day.
    ///
    /// 10.75 shaos zmaniyos after alos 26 degrees, using a day that begins and ends at 26 degrees.
    #[deprecated(note = "KosherJava deprecates this zman")]
    PlagHamincha26Degrees {
        java: "getPlagHamincha26Degrees",
        name: "Plag Hamincha (26 Degrees)",
        category: PlagHamincha,
        formula: PlagHamincha(&SunriseOffsetByDegrees(26.0), &SunsetOffsetByDegrees(26.0), true),
        about: concat!(
            "Plag hamincha based on the 26-degree day.\n\n",
            "10.75 shaos zmaniyos after alos 26 degrees, using a day that begins and ends at 26 degrees.\n\n",
            "This zman should be used lechumra only. It can be a very late time, often after shkiah, and using it leniently can lead to chillul Shabbos.\n\n",
            "At some northern and southern locations, this zman may not be available if the sun does not reach low enough below the horizon.",
        ),
    },

    /// Plag hamincha according to the Ateret Torah calculation of Chacham Yosef Harari-Raful of Yeshivat Ateret
    /// Torah.
    ///
    /// 10.75 shaos zmaniyos after alos.
    PlagHaminchaAteretTorah {
        java: "getPlagHaminchaAteretTorah",
        name: "Plag Hamincha (Ateret Torah)",
        category: PlagHamincha,
        formula: PlagHamincha(&ZmanisOffset(&ConfiguredSunrise, -1.2), &TzaisAteretTorah, false),
        about: concat!(
            "Plag hamincha according to the Ateret Torah calculation of Chacham Yosef Harari-Raful of Yeshivat Ateret Torah.\n\n",
            "10.75 shaos zmaniyos after alos.\n\n",
            "The day begins at alos 1/10 of the day before sunrise and ends {ateret_torah_offset} after sunset.\n\n",
            "In places where sunrise or sunset cannot be calculated, this zman may not be available.",
        ),
    },

    /// Plag hamincha according to the Baal Hatanya.
    ///
    /// 10.75 shaos zmaniyos after netz amiti, using a day from Baal Hatanya sunrise to sunset.
    PlagHaminchaBaalHatanya {
        java: "getPlagHaminchaBaalHatanya",
        name: "Plag Hamincha (Baal Hatanya)",
        category: PlagHamincha,
        formula: PlagHamincha(&SunriseOffsetByDegrees(1.583), &SunsetOffsetByDegrees(1.583), true),
        about: concat!(
            "Plag hamincha according to the Baal Hatanya.\n\n",
            "10.75 shaos zmaniyos after netz amiti, using a day from Baal Hatanya sunrise to sunset.\n\n",
            "In places where sunrise or sunset cannot be calculated, this zman may not be available.\n\n",
            "See [About Our Zmanim Calculations @ Chabad.org](https://www.chabad.org/library/article_cdo/aid/3209349/jewish/About-Our-Zmanim-Calculations.htm).",
        ),
    },

    /// Plag hamincha according to [Rav Moshe Feinstein](https://en.wikipedia.org/wiki/Moshe_Feinstein)'s opinion,
    /// following the [Vilna Gaon](https://en.wikipedia.org/wiki/Vilna_Gaon) with the day ending at sunset.
    ///
    /// 4.75 shaos zmaniyos after fixed local chatzos.
    PlagHaminchaGraFixedLocalChatzosToSunset {
        java: "getPlagHaminchaGRAFixedLocalChatzosToSunset",
        name: "Plag Hamincha (GR'A, Fixed Local Chatzos to Sunset)",
        category: PlagHamincha,
        formula: HalfDayBasedOffset(&LocalMeanTime(12.0), &ConfiguredSunset, 4.75),
        about: concat!(
            "Plag hamincha according to [Rav Moshe Feinstein](https://en.wikipedia.org/wiki/Moshe_Feinstein)'s opinion, following the [Vilna Gaon](https://en.wikipedia.org/wiki/Vilna_Gaon) with the day ending at sunset.\n\n",
            "4.75 shaos zmaniyos after fixed local chatzos.\n\n",
            "In places where sunset cannot be calculated, this zman may not be available.",
        ),
    },

    // SamuchLeMinchaKetana

    /// The point near mincha ketana when eating or other activity should not begin before praying mincha, following
    /// the [Vilna Gaon](https://en.wikipedia.org/wiki/Vilna_Gaon).
    ///
    /// 9 shaos zmaniyos after sunrise, using a day from sunrise to sunset.
    SamuchLeMinchaKetanaGra {
        java: "getSamuchLeMinchaKetanaGRA",
        name: "Samuch Le Mincha Ketana (GR'A)",
        category: SamuchLeMinchaKetana,
        formula: SamuchLeMinchaKetana(&ConfiguredSunrise, &ConfiguredSunset, true),
        about: concat!(
            "The point near mincha ketana when eating or other activity should not begin before praying mincha, following the [Vilna Gaon](https://en.wikipedia.org/wiki/Vilna_Gaon).\n\n",
            "9 shaos zmaniyos after sunrise, using a day from sunrise to sunset. \n\n",
            "This is half a shaah zmanis before mincha ketana.\n\n",
            "In places where sunrise or sunset cannot be calculated, this zman may not be available.\n\n",
            "See [Mechaber and Mishna Berurah 232](https://hebrewbooks.org/pdfpager.aspx?req=60387&st=&pgnum=294) and [249:2](https://hebrewbooks.org/pdfpager.aspx?req=60388&pgnum=34).",
        ),
    },

    /// The point near mincha ketana when eating or other activity should not begin before praying mincha, using the
    /// 16.1-degree day.
    ///
    /// 9 shaos zmaniyos after alos at 16.1 degrees below the horizon, using a day that begins and ends at 16.1
    /// degrees.
    SamuchLeMinchaKetana16Point1Degrees {
        java: "getSamuchLeMinchaKetana16Point1Degrees",
        name: "Samuch Le Mincha Ketana (16.1 Degrees)",
        category: SamuchLeMinchaKetana,
        formula: SamuchLeMinchaKetana(&SunriseOffsetByDegrees(16.1), &SunsetOffsetByDegrees(16.1), true),
        about: concat!(
            "The point near mincha ketana when eating or other activity should not begin before praying mincha, using the 16.1-degree day.\n\n",
            "9 shaos zmaniyos after alos at 16.1 degrees below the horizon, using a day that begins and ends at 16.1 degrees.\n\n",
            "This is half a shaah zmanis before mincha ketana for this calculation.\n\n",
            "At some northern and southern locations, including places even south of the Arctic Circle and north of the Antarctic Circle, this zman may not be available or cannot be calculated if the sun does not reach low enough below the horizon.\n\n",
            "See [Mechaber and Mishna Berurah 232](https://hebrewbooks.org/pdfpager.aspx?req=60387&st=&pgnum=294) and [249:2](https://hebrewbooks.org/pdfpager.aspx?req=60388&pgnum=34).",
        ),
    },

    /// The point near mincha ketana when eating or other activity should not begin before praying mincha, using the
    /// 72-minute day.
    ///
    /// 9 shaos zmaniyos after alos 72 minutes before sunrise, using a day that starts 72 minutes before sunrise and
    /// ends 72 minutes after sunset.
    SamuchLeMinchaKetana72Minutes {
        java: "getSamuchLeMinchaKetana72Minutes",
        name: "Samuch Le Mincha Ketana (72 Minutes)",
        category: SamuchLeMinchaKetana,
        formula: SamuchLeMinchaKetana(&Offset(&ConfiguredSunrise, Duration::from_mins(-72)), &Offset(&ConfiguredSunset, Duration::from_mins(72)), true),
        about: concat!(
            "The point near mincha ketana when eating or other activity should not begin before praying mincha, using the 72-minute day.\n\n",
            "9 shaos zmaniyos after alos 72 minutes before sunrise, using a day that starts 72 minutes before sunrise and ends 72 minutes after sunset.\n\n",
            "This is half a shaah zmanis before mincha ketana for this calculation.\n\n",
            "In places such as the Arctic Circle, where there is at least one day a year when the sun does not rise and one when it does not set, this zman may not be available or cannot be calculated.\n\n",
            "See [Mechaber and Mishna Berurah 232](https://hebrewbooks.org/pdfpager.aspx?req=60387&st=&pgnum=294) and [249:2](https://hebrewbooks.org/pdfpager.aspx?req=60388&pgnum=34).",
        ),
    },

    // MinchaKetana

    /// Mincha ketana according to the [Vilna Gaon](https://en.wikipedia.org/wiki/Vilna_Gaon).
    ///
    /// 9.5 shaos zmaniyos after sunrise, using a day from sunrise to sunset.
    MinchaKetanaGra {
        java: "getMinchaKetanaGRA",
        name: "Mincha Ketana (GR'A)",
        category: MinchaKetana,
        formula: MinchaKetana(&ConfiguredSunrise, &ConfiguredSunset, true),
        about: concat!(
            "Mincha ketana according to the [Vilna Gaon](https://en.wikipedia.org/wiki/Vilna_Gaon).\n\n",
            "9.5 shaos zmaniyos after sunrise, using a day from sunrise to sunset. \n\n",
            "This is the preferred earliest time to pray mincha according to the Rambam and others.\n\n",
            "In places where sunrise or sunset cannot be calculated, this zman may not be available.",
        ),
    },

    /// Mincha ketana according to the Magen Avraham, using the 16.1-degree day.
    ///
    /// 9.5 shaos zmaniyos after alos 16.1 degrees, using a day that begins and ends at 16.1 degrees.
    MinchaKetana16Point1Degrees {
        java: "getMinchaKetana16Point1Degrees",
        name: "Mincha Ketana (16.1 Degrees)",
        category: MinchaKetana,
        formula: MinchaKetana(&SunriseOffsetByDegrees(16.1), &SunsetOffsetByDegrees(16.1), true),
        about: concat!(
            "Mincha ketana according to the Magen Avraham, using the 16.1-degree day.\n\n",
            "9.5 shaos zmaniyos after alos 16.1 degrees, using a day that begins and ends at 16.1 degrees.\n\n",
            "At some northern and southern locations, this zman may not be available if the sun does not reach low enough below the horizon.",
        ),
    },

    /// Mincha ketana according to the Magen Avraham, using the 72-minute day.
    ///
    /// 9.5 shaos zmaniyos after alos 72 minutes before sunrise, using a day that starts 72 minutes before sunrise and
    /// ends 72 minutes after sunset.
    MinchaKetana72Minutes {
        java: "getMinchaKetana72Minutes",
        name: "Mincha Ketana (72 Minutes)",
        category: MinchaKetana,
        formula: MinchaKetana(&Offset(&ConfiguredSunrise, Duration::from_mins(-72)), &Offset(&ConfiguredSunset, Duration::from_mins(72)), true),
        about: concat!(
            "Mincha ketana according to the Magen Avraham, using the 72-minute day.\n\n",
            "9.5 shaos zmaniyos after alos 72 minutes before sunrise, using a day that starts 72 minutes before sunrise and ends 72 minutes after sunset.\n\n",
            "In places where sunrise or sunset cannot be calculated, this zman may not be available.",
        ),
    },

    /// Mincha ketana according to Rabbi Yaakov Moshe Hillel, as published in the luach of the Bais Horaah of Yeshivat
    /// Chevrat Ahavat Shalom.
    ///
    /// 2.5 shaos zmaniyos before tzais 3.8 degrees, using a day from alos 16.1 degrees to tzais 3.8 degrees.
    MinchaKetanaAhavatShalom {
        java: "getMinchaKetanaAhavatShalom",
        name: "Mincha Ketana (Ahavat Shalom)",
        category: MinchaKetana,
        formula: MinchaKetanaAhavatShalom,
        about: concat!(
            "Mincha ketana according to Rabbi Yaakov Moshe Hillel, as published in the luach of the Bais Horaah of Yeshivat Chevrat Ahavat Shalom.\n\n",
            "2.5 shaos zmaniyos before tzais 3.8 degrees, using a day from alos 16.1 degrees to tzais 3.8 degrees.\n\n",
            "At some northern and southern locations, this zman may not be available if the sun does not reach low enough below the horizon.",
        ),
    },

    /// Mincha ketana according to the Ateret Torah calculation of Chacham Yosef Harari-Raful of Yeshivat Ateret
    /// Torah.
    ///
    /// 9.5 shaos zmaniyos after alos.
    MinchaKetanaAteretTorah {
        java: "getMinchaKetanaAteretTorah",
        name: "Mincha Ketana (Ateret Torah)",
        category: MinchaKetana,
        formula: MinchaKetana(&ZmanisOffset(&ConfiguredSunrise, -1.2), &TzaisAteretTorah, false),
        about: concat!(
            "Mincha ketana according to the Ateret Torah calculation of Chacham Yosef Harari-Raful of Yeshivat Ateret Torah.\n\n",
            "9.5 shaos zmaniyos after alos.\n\n",
            "The day begins at alos 1/10 of the day before sunrise and ends {ateret_torah_offset} after sunset.\n\n",
            "In places where sunrise or sunset cannot be calculated, this zman may not be available.",
        ),
    },

    /// Mincha ketana according to the Baal Hatanya.
    ///
    /// 9.5 shaos zmaniyos after netz amiti, using a day from Baal Hatanya sunrise to sunset.
    MinchaKetanaBaalHatanya {
        java: "getMinchaKetanaBaalHatanya",
        name: "Mincha Ketana (Baal Hatanya)",
        category: MinchaKetana,
        formula: MinchaKetana(&SunriseOffsetByDegrees(1.583), &SunsetOffsetByDegrees(1.583), true),
        about: concat!(
            "Mincha ketana according to the Baal Hatanya.\n\n",
            "9.5 shaos zmaniyos after netz amiti, using a day from Baal Hatanya sunrise to sunset.\n\n",
            "In places where sunrise or sunset cannot be calculated, this zman may not be available.",
        ),
    },

    /// Mincha ketana according to Rav Moshe Feinstein's opinion, following the view of the Vilna Gaon.
    ///
    /// 3.5 shaos zmaniyos after fixed local chatzos.
    MinchaKetanaGraFixedLocalChatzosToSunset {
        java: "getMinchaKetanaGRAFixedLocalChatzosToSunset",
        name: "Mincha Ketana (GR'A, Fixed Local Chatzos to Sunset)",
        category: MinchaKetana,
        formula: HalfDayBasedOffset(&LocalMeanTime(12.0), &ConfiguredSunset, 3.5),
        about: concat!(
            "Mincha ketana according to Rav Moshe Feinstein's opinion, following the view of the Vilna Gaon.\n\n",
            "3.5 shaos zmaniyos after fixed local chatzos.\n\n",
            "In places where fixed local chatzos or sunset cannot be calculated, this zman may not be available.",
        ),
    },

    // BainHashmashos

    /// The beginning of Rabbeinu Tam's bain hashmashos.
    ///
    /// When the sun is 13.24 degrees below the western geometric horizon after sunset. This is the degree-based
    /// equivalent of bain hashmashos 58.5 minutes after sunset. In Jerusalem [around the equinox or
    /// equilux](https://kosherjava.com/2022/01/12/equinox-vs-equilux-zmanim-calculations/), the sun is 13.24 degrees
    /// below geometric zenith about 58.5 minutes after sunset. The source cited in the original documentation says
    /// that the proper dip should be slightly less than 13 degrees; this project's recalculation gives about 13.2456
    /// degrees, so 13.24 degrees is used as a truncation that yields a time about 1.5 seconds earlier, lechumra.
    BainHashmashosRt13Point24Degrees {
        java: "getBainHashmashosRT13Point24Degrees",
        name: "Bain Hashmashos (Rabbeinu Tam, 13.24 Degrees)",
        category: BainHashmashos,
        formula: SunsetOffsetByDegrees(13.24),
        about: concat!(
            "The beginning of Rabbeinu Tam's bain hashmashos.\n\n",
            "When the sun is 13.24 degrees below the western geometric horizon after sunset. This is the degree-based equivalent of bain hashmashos 58.5 minutes after sunset: in Jerusalem [around the equinox or equilux](https://kosherjava.com/2022/01/12/equinox-vs-equilux-zmanim-calculations/), the sun is 13.24 degrees below geometric zenith about 58.5 minutes after sunset. The source cited in the original documentation says that the proper dip should be slightly less than 13 degrees; this project's recalculation gives about 13.2456 degrees, so 13.24 degrees is used as a truncation that yields a time about 1.5 seconds earlier, lechumra.\n\n",
            "At some northern and southern locations, including places even south of the Arctic Circle and north of the Antarctic Circle, this zman may not be available or cannot be calculated if the sun does not reach low enough below the horizon.",
        ),
    },

    /// The beginning of Rabbeinu Tam's bain hashmashos.
    ///
    /// 58.5 minutes after sunset. Bain hashmashos is 3/4 of a
    /// [mil](https://en.wikipedia.org/wiki/Biblical_and_Talmudic_units_of_measurement) before tzais, or 3 1/4 mil
    /// after sunset. With an 18-minute mil, 3.25 * 18 = 58.5 minutes.
    BainHashmashosRt58Point5Minutes {
        java: "getBainHashmashosRT58Point5Minutes",
        name: "Bain Hashmashos (Rabbeinu Tam, 58.5 Minutes)",
        category: BainHashmashos,
        formula: Offset(&ConfiguredSunset, Duration::from_millis(3510000)),
        about: concat!(
            "The beginning of Rabbeinu Tam's bain hashmashos.\n\n",
            "58.5 minutes after sunset. Bain hashmashos is 3/4 of a [mil](https://en.wikipedia.org/wiki/Biblical_and_Talmudic_units_of_measurement) before tzais, or 3 1/4 mil after sunset. With an 18-minute mil, 3.25 * 18 = 58.5 minutes.\n\n",
            "In places such as the Arctic Circle, where there is at least one day a year when the sun does not rise and one when it does not set, this zman may not be available or cannot be calculated.",
        ),
    },

    /// The beginning of Rabbeinu Tam's bain hashmashos.
    ///
    /// 13.5 minutes (3/4 of an 18-minute
    /// [mil](https://en.wikipedia.org/wiki/Biblical_and_Talmudic_units_of_measurement)) before shkiah calculated at
    /// 7.083 degrees below the horizon.
    BainHashmashosRt13Point5MinutesBefore7Point083Degrees {
        java: "getBainHashmashosRT13Point5MinutesBefore7Point083Degrees",
        name: "Bain Hashmashos (Rabbeinu Tam, 13.5 Minutes Before 7.083 Degrees)",
        category: BainHashmashos,
        formula: Offset(&SunsetOffsetByDegrees(7.083333333333333), Duration::from_millis(-810000)),
        about: concat!(
            "The beginning of Rabbeinu Tam's bain hashmashos.\n\n",
            "13.5 minutes (3/4 of an 18-minute [mil](https://en.wikipedia.org/wiki/Biblical_and_Talmudic_units_of_measurement)) before shkiah calculated at 7.083 degrees below the horizon.\n\n",
            "At some northern and southern locations, including places even south of the Arctic Circle and north of the Antarctic Circle, this zman may not be available or cannot be calculated if the sun does not reach low enough below the horizon.",
        ),
    },

    /// The beginning of Rabbeinu Tam's bain hashmashos, according to the Divrei Yosef (see Yisrael Vehazmanim).
    ///
    /// Calculated as 5/18 (about 27.77%) of the time from alos at 19.8 degrees before sunrise to sunrise; that
    /// interval is added after sunset to reach Rabbeinu Tam's bain hashmashos.
    BainHashmashosRt2Stars {
        java: "getBainHashmashosRT2Stars",
        name: "Bain Hashmashos (Rabbeinu Tam, 2 Stars)",
        category: BainHashmashos,
        formula: BainHashmashosRt2Stars,
        about: concat!(
            "The beginning of Rabbeinu Tam's bain hashmashos, according to the Divrei Yosef (see Yisrael Vehazmanim).\n\n",
            "Calculated as 5/18 (about 27.77%) of the time from alos at 19.8 degrees before sunrise to sunrise; that interval is added after sunset to reach Rabbeinu Tam's bain hashmashos.\n\n",
            "At some northern and southern locations, including places even south of the Arctic Circle and north of the Antarctic Circle, this zman may not be available or cannot be calculated if the sun does not reach low enough below the horizon.",
        ),
    },

    /// The beginning of bain hashmashos (twilight) according to the [Yereim (Rabbi Eliezer of
    /// Metz)](https://en.wikipedia.org/wiki/Eliezer_ben_Samuel).
    ///
    /// 18 minutes, or 3/4 of a 24-minute
    /// [mil](https://en.wikipedia.org/wiki/Biblical_and_Talmudic_units_of_measurement), before sunset. According to
    /// the Yereim, bain hashmashos starts 3/4 of a mil before sunset and tzais (nightfall) is at sunset.
    BainHashmashosYereim18Minutes {
        java: "getBainHashmashosYereim18Minutes",
        name: "Bain Hashmashos (Yereim, 18 Minutes)",
        category: BainHashmashos,
        formula: Offset(&ConfiguredSunset, Duration::from_mins(-18)),
        about: concat!(
            "The beginning of bain hashmashos (twilight) according to the [Yereim (Rabbi Eliezer of Metz)](https://en.wikipedia.org/wiki/Eliezer_ben_Samuel).\n\n",
            "18 minutes, or 3/4 of a 24-minute [mil](https://en.wikipedia.org/wiki/Biblical_and_Talmudic_units_of_measurement), before sunset. According to the Yereim, bain hashmashos starts 3/4 of a mil before sunset and tzais (nightfall) is at sunset.\n\n",
            "In places such as the Arctic Circle, where there is at least one day a year when the sun does not rise and one when it does not set, this zman may not be available or cannot be calculated.",
        ),
    },

    /// The beginning of bain hashmashos (twilight) according to the [Yereim (Rabbi Eliezer of
    /// Metz)](https://en.wikipedia.org/wiki/Eliezer_ben_Samuel).
    ///
    /// 16.875 minutes, or 3/4 of a 22.5-minute
    /// [mil](https://en.wikipedia.org/wiki/Biblical_and_Talmudic_units_of_measurement), before sunset. According to
    /// the Yereim, bain hashmashos starts 3/4 of a mil before sunset and tzais (nightfall) is at sunset.
    BainHashmashosYereim16Point875Minutes {
        java: "getBainHashmashosYereim16Point875Minutes",
        name: "Bain Hashmashos (Yereim, 16.875 Minutes)",
        category: BainHashmashos,
        formula: Offset(&ConfiguredSunset, Duration::from_millis(-1012500)),
        about: concat!(
            "The beginning of bain hashmashos (twilight) according to the [Yereim (Rabbi Eliezer of Metz)](https://en.wikipedia.org/wiki/Eliezer_ben_Samuel).\n\n",
            "16.875 minutes, or 3/4 of a 22.5-minute [mil](https://en.wikipedia.org/wiki/Biblical_and_Talmudic_units_of_measurement), before sunset. According to the Yereim, bain hashmashos starts 3/4 of a mil before sunset and tzais (nightfall) is at sunset.\n\n",
            "In places such as the Arctic Circle, where there is at least one day a year when the sun does not rise and one when it does not set, this zman may not be available or cannot be calculated.",
        ),
    },

    /// The beginning of bain hashmashos (twilight) according to the [Yereim (Rabbi Eliezer of
    /// Metz)](https://en.wikipedia.org/wiki/Eliezer_ben_Samuel).
    ///
    /// 13.5 minutes, or 3/4 of an 18-minute
    /// [mil](https://en.wikipedia.org/wiki/Biblical_and_Talmudic_units_of_measurement), before sunset. According to
    /// the Yereim, bain hashmashos starts 3/4 of a mil before sunset and tzais (nightfall) is at sunset.
    BainHashmashosYereim13Point5Minutes {
        java: "getBainHashmashosYereim13Point5Minutes",
        name: "Bain Hashmashos (Yereim, 13.5 Minutes)",
        category: BainHashmashos,
        formula: Offset(&ConfiguredSunset, Duration::from_millis(-810000)),
        about: concat!(
            "The beginning of bain hashmashos (twilight) according to the [Yereim (Rabbi Eliezer of Metz)](https://en.wikipedia.org/wiki/Eliezer_ben_Samuel).\n\n",
            "13.5 minutes, or 3/4 of an 18-minute [mil](https://en.wikipedia.org/wiki/Biblical_and_Talmudic_units_of_measurement), before sunset. According to the Yereim, bain hashmashos starts 3/4 of a mil before sunset and tzais (nightfall) is at sunset.\n\n",
            "In places such as the Arctic Circle, where there is at least one day a year when the sun does not rise and one when it does not set, this zman may not be available or cannot be calculated.",
        ),
    },

    /// The beginning of bain hashmashos (twilight) according to the [Yereim (Rabbi Eliezer of
    /// Metz)](https://en.wikipedia.org/wiki/Eliezer_ben_Samuel).
    ///
    /// When the sun is 2.1 degrees above the horizon - in Jerusalem [around the equinox or
    /// equilux](https://kosherjava.com/2022/01/12/equinox-vs-equilux-zmanim-calculations/), about 13.5 minutes or 3/4
    /// of an 18-minute [mil](https://en.wikipedia.org/wiki/Biblical_and_Talmudic_units_of_measurement) before sunset.
    /// According to the Yereim, bain hashmashos starts 3/4 of a mil before sunset and tzais (nightfall) is at sunset.
    BainHashmashosYereim2Point1Degrees {
        java: "getBainHashmashosYereim2Point1Degrees",
        name: "Bain Hashmashos (Yereim, 2.1 Degrees)",
        category: BainHashmashos,
        formula: SunsetOffsetByDegrees(-2.1),
        about: concat!(
            "The beginning of bain hashmashos (twilight) according to the [Yereim (Rabbi Eliezer of Metz)](https://en.wikipedia.org/wiki/Eliezer_ben_Samuel).\n\n",
            "When the sun is 2.1 degrees above the horizon - in Jerusalem [around the equinox or equilux](https://kosherjava.com/2022/01/12/equinox-vs-equilux-zmanim-calculations/), about 13.5 minutes or 3/4 of an 18-minute [mil](https://en.wikipedia.org/wiki/Biblical_and_Talmudic_units_of_measurement) before sunset. According to the Yereim, bain hashmashos starts 3/4 of a mil before sunset and tzais (nightfall) is at sunset.\n\n",
            "In places such as the Arctic Circle, where there is at least one day a year when the sun does not rise and one when it does not set, this zman may not be available or cannot be calculated.",
        ),
    },

    /// The beginning of bain hashmashos (twilight) according to the [Yereim (Rabbi Eliezer of
    /// Metz)](https://en.wikipedia.org/wiki/Eliezer_ben_Samuel).
    ///
    /// When the sun is 2.8 degrees above the horizon - in Jerusalem [around the equinox or
    /// equilux](https://kosherjava.com/2022/01/12/equinox-vs-equilux-zmanim-calculations/), about 16.875 minutes or
    /// 3/4 of a 22.5-minute [mil](https://en.wikipedia.org/wiki/Biblical_and_Talmudic_units_of_measurement) before
    /// sunset. According to the Yereim, bain hashmashos starts 3/4 of a mil before sunset and tzais (nightfall) is at
    /// sunset.
    BainHashmashosYereim2Point8Degrees {
        java: "getBainHashmashosYereim2Point8Degrees",
        name: "Bain Hashmashos (Yereim, 2.8 Degrees)",
        category: BainHashmashos,
        formula: SunsetOffsetByDegrees(-2.8),
        about: concat!(
            "The beginning of bain hashmashos (twilight) according to the [Yereim (Rabbi Eliezer of Metz)](https://en.wikipedia.org/wiki/Eliezer_ben_Samuel).\n\n",
            "When the sun is 2.8 degrees above the horizon - in Jerusalem [around the equinox or equilux](https://kosherjava.com/2022/01/12/equinox-vs-equilux-zmanim-calculations/), about 16.875 minutes or 3/4 of a 22.5-minute [mil](https://en.wikipedia.org/wiki/Biblical_and_Talmudic_units_of_measurement) before sunset. According to the Yereim, bain hashmashos starts 3/4 of a mil before sunset and tzais (nightfall) is at sunset.\n\n",
            "In places such as the Arctic Circle, where there is at least one day a year when the sun does not rise and one when it does not set, this zman may not be available or cannot be calculated.",
        ),
    },

    /// The beginning of bain hashmashos (twilight) according to the [Yereim (Rabbi Eliezer of
    /// Metz)](https://en.wikipedia.org/wiki/Eliezer_ben_Samuel).
    ///
    /// When the sun is 3.05 degrees above the horizon. In Jerusalem [around the equinox or
    /// equilux](https://kosherjava.com/2022/01/12/equinox-vs-equilux-zmanim-calculations/), that matches about 18
    /// minutes, or 3/4 of a 24-minute
    /// [mil](https://en.wikipedia.org/wiki/Biblical_and_Talmudic_units_of_measurement), before sunset.
    BainHashmashosYereim3Point05Degrees {
        java: "getBainHashmashosYereim3Point05Degrees",
        name: "Bain Hashmashos (Yereim, 3.05 Degrees)",
        category: BainHashmashos,
        formula: SunsetOffsetByDegrees(-3.05),
        about: concat!(
            "The beginning of bain hashmashos (twilight) according to the [Yereim (Rabbi Eliezer of Metz)](https://en.wikipedia.org/wiki/Eliezer_ben_Samuel).\n\n",
            "When the sun is 3.05 degrees above the horizon. In Jerusalem [around the equinox or equilux](https://kosherjava.com/2022/01/12/equinox-vs-equilux-zmanim-calculations/), that matches about 18 minutes, or 3/4 of a 24-minute [mil](https://en.wikipedia.org/wiki/Biblical_and_Talmudic_units_of_measurement), before sunset.\n\n",
            "The Yereim holds that bain hashmashos begins 3/4 of a mil before sunset and that tzais (nightfall) is at sunset. This degree-based version uses 0.5166 degrees of refraction instead of the traditional 0.566 degrees, which shifts the time earlier by about 14 seconds lechumra and is closer to refraction in Eretz Yisrael per [Rabbi Yedidya Manet](http://beinenu.com/rabbis/%D7%94%D7%A8%D7%91-%D7%99%D7%93%D7%99%D7%93%D7%99%D7%94-%D7%9E%D7%A0%D7%AA) ([Zmanei HaHalacha Lema'aseh](https://www.nli.org.il/en/books/NNL_ALEPH002542826/NLI), p. 11) and the [Luach Itim Lebinah](https://zmanim.online/). For background, see the [The Yereim's Bain Hashmashos](https://kosherjava.com/2020/12/07/the-yereims-bein-hashmashos/) article on the KosherJava blog.\n\n",
            "In places such as the Arctic Circle, where there is at least one day a year when the sun does not rise and one when it does not set, this zman may not be available or cannot be calculated.",
        ),
    },

    // CandleLighting

    /// The time to light candles before Shabbos or Yom Tov.
    ///
    /// The configured candle-lighting offset before sea level sunset.
    CandleLighting {
        java: "getCandleLighting",
        name: "Candle Lighting",
        category: CandleLighting,
        formula: CandleLighting,
        about: concat!(
            "The time to light candles before Shabbos or Yom Tov.\n\n",
            "{candle_lighting_offset} before sea level sunset.\n\n",
            "In places such as the Arctic Circle, where there is at least one day a year when the sun does not rise and one when it does not set, this zman may not be available or cannot be calculated.",
        ),
    },

    // Shkiya

    /// Sunset calculated as the time when the sun is directly due west (azimuth 270 degrees) in polar regions on days
    /// when there is no sunset.
    ///
    /// If sunset occurs that day, this zman is not available. In polar regions (the Arctic or Antarctic circles),
    /// there are days with no sunrise or sunset. Some opinions hold that during these periods, sunset (the day-night
    /// boundary) is when the sun is directly due west (azimuth 270 degrees), and sunrise is when the sun is directly
    /// due east (azimuth 90 degrees), given by [`Zman::PolarSunriseBenIshChai`]. This follows [Rabbi Yehosef
    /// Schwarz](https://en.wikipedia.org/wiki/Joseph_Schwarz_(geographer)) in [Devarim Yosef - Derech Mevo
    /// Hashemesh](https://hebrewbooks.org/pdfpager.aspx?req=31703&pgnum=134) and [Devarim Yosef - Teshuvot, She'elah
    /// 8](https://hebrewbooks.org/pdfpager.aspx?req=159&pgnum=83), brought lehalacha by the [Ben Ish
    /// Chai](https://en.wikipedia.org/wiki/Yosef_Hayyim) in [Rav Pe'alim, chelek 2, Sod Yesharim siman
    /// 4](https://hebrewbooks.org/pdfpager.aspx?req=1401&pgnum=461). This time is close to six hours after
    /// astronomical chatzos, but depending on the season and location in the Arctic or Antarctic, it can be up to 46
    /// minutes earlier or later.
    PolarSunsetBenIshChai {
        java: "getPolarSunsetBenIshChai",
        name: "Polar Sunset Ben Ish Chai",
        category: Shkiya,
        formula: PolarSunsetBenIshChai,
        about: concat!(
            "Sunset calculated as the time when the sun is directly due west (azimuth 270 degrees) in polar regions on days when there is no sunset.\n\n",
            "If sunset occurs that day, this zman is not available. In polar regions (the Arctic or Antarctic circles), there are days with no sunrise or sunset. Some opinions hold that during these periods, sunset (the day-night boundary) is when the sun is directly due west (azimuth 270 degrees), and sunrise is when the sun is directly due east (azimuth 90 degrees). This follows [Rabbi Yehosef Schwarz](https://en.wikipedia.org/wiki/Joseph_Schwarz_(geographer)) in [Devarim Yosef - Derech Mevo Hashemesh](https://hebrewbooks.org/pdfpager.aspx?req=31703&pgnum=134) and [Devarim Yosef - Teshuvot, She'elah 8](https://hebrewbooks.org/pdfpager.aspx?req=159&pgnum=83), brought lehalacha by the [Ben Ish Chai](https://en.wikipedia.org/wiki/Yosef_Hayyim) in [Rav Pe'alim, chelek 2, Sod Yesharim siman 4](https://hebrewbooks.org/pdfpager.aspx?req=1401&pgnum=461). This time is close to six hours after astronomical chatzos, but depending on the season and location in the Arctic or Antarctic, it can be up to 46 minutes earlier or later.\n\n",
            "If there is no sunset that day and the sun does not reach azimuth 270 degrees, this zman may not be available or cannot be calculated.",
        ),
    },

    /// Sunset, adjusted for the location's elevation.
    ///
    /// The time when the upper edge of the sun disappears below the horizon, accounting for atmospheric refraction
    /// and the sun's radius.
    Sunset {
        java: "getSunset",
        name: "Sunset",
        category: Shkiya,
        formula: ElevationAdjustedSunset,
        about: concat!(
            "Sunset, adjusted for the location's elevation.\n\n",
            "The time when the upper edge of the sun disappears below the horizon, accounting for atmospheric refraction and the sun's radius.\n\n",
            "In places such as the Arctic Circle, where there is at least one day a year when the sun does not rise and one when it does not set, this zman may not be available or cannot be calculated.",
        ),
    },

    /// Sunset at sea level, without adjusting for the location's elevation.
    ///
    /// This is the astronomical sunset used as the reference for dusk times measured as degrees below the horizon.
    /// Dawn and dusk depend on visible light, which is not affected by elevation the way sunrise and sunset at a
    /// raised location can be.
    SeaLevelSunset {
        java: "getSeaLevelSunset",
        name: "Sea Level Sunset",
        category: Shkiya,
        formula: SeaLevelSunset,
        about: concat!(
            "Sunset at sea level, without adjusting for the location's elevation.\n\n",
            "This is the astronomical sunset used as the reference for dusk times measured as degrees below the horizon. Dawn and dusk depend on visible light, which is not affected by elevation the way sunrise and sunset at a raised location can be.\n\n",
            "In places such as the Arctic Circle, where there is at least one day a year when the sun does not rise and one when it does not set, this zman may not be available or cannot be calculated.",
        ),
    },

    // Tzais

    /// Tzais (nightfall) according to the Geonim - when the sun is 8.5 degrees below the western horizon after
    /// sunset.
    ///
    /// Based on Rabbi Meir Posen's [Ohr Meir](https://www.worldcat.org/oclc/29283612) calculation for when 3 small
    /// stars are visible, which is later than the required 3 medium stars. About 36 minutes after sunset in Jerusalem
    /// [around the equinox/equilux](https://kosherjava.com/2022/01/12/equinox-vs-equilux-zmanim-calculations/).
    TzaisGeonim8Point5Degrees {
        java: "getTzaisGeonim8Point5Degrees",
        name: "Tzais (Geonim, 8.5 Degrees)",
        category: Tzais,
        formula: SunsetOffsetByDegrees(8.5),
        about: concat!(
            "Tzais (nightfall) according to the Geonim - when the sun is 8.5 degrees below the western horizon after sunset.\n\n",
            "Based on Rabbi Meir Posen's [Ohr Meir](https://www.worldcat.org/oclc/29283612) calculation for when 3 small stars are visible, which is later than the required 3 medium stars. About 36 minutes after sunset in Jerusalem [around the equinox/equilux](https://kosherjava.com/2022/01/12/equinox-vs-equilux-zmanim-calculations/).\n\n",
            "At some northern and southern locations, this zman may not be available or cannot be calculated if the sun does not reach low enough below the horizon.",
        ),
    },

    /// Tzais (nightfall) according to [Rav Moshe Feinstein](https://en.wikipedia.org/wiki/Moshe_Feinstein) for the
    /// New York area - 50 minutes after sunset.
    ///
    /// 50 minutes after sunset.
    Tzais50Minutes {
        java: "getTzais50Minutes",
        name: "Tzais (50 Minutes)",
        category: Tzais,
        formula: Offset(&ConfiguredSunset, Duration::from_mins(50)),
        about: concat!(
            "Tzais (nightfall) according to [Rav Moshe Feinstein](https://en.wikipedia.org/wiki/Moshe_Feinstein) for the New York area - 50 minutes after sunset.\n\n",
            "50 minutes after sunset. \n\n",
            "This zman should not be used for latitudes other than ones similar to the New York area.\n\n",
            "In places such as the Arctic Circle, where there is at least one day a year when the sun does not rise and one when it does not set, this zman may not be available or cannot be calculated.",
        ),
    },

    /// Tzais (nightfall) according to the [Chavas Yair](https://en.wikipedia.org/wiki/Yair_Bacharach) and [Divrei
    /// Malkiel](https://he.wikipedia.org/wiki/%D7%9E%D7%9C%D7%9B%D7%99%D7%90%D7%9C_%D7%A6%D7%91%D7%99_%D7%98%D7%A0%D7%A0%D7%91%D7%95%D7%99%D7%9D)
    /// - 60 minutes after sunset.
    ///
    /// Based on a 15-minute mil, for a total of 4 mil after sunset.
    Tzais60Minutes {
        java: "getTzais60Minutes",
        name: "Tzais (60 Minutes)",
        category: Tzais,
        formula: Offset(&ConfiguredSunset, Duration::from_mins(60)),
        about: concat!(
            "Tzais (nightfall) according to the [Chavas Yair](https://en.wikipedia.org/wiki/Yair_Bacharach) and [Divrei Malkiel](https://he.wikipedia.org/wiki/%D7%9E%D7%9C%D7%9B%D7%99%D7%90%D7%9C_%D7%A6%D7%91%D7%99_%D7%98%D7%A0%D7%A0%D7%91%D7%95%D7%99%D7%9D) - 60 minutes after sunset.\n\n",
            "Based on a 15-minute mil, for a total of 4 mil after sunset. \n\n",
            "In places such as the Arctic Circle, where there is at least one day a year when the sun does not rise and one when it does not set, this zman may not be available or cannot be calculated.",
        ),
    },

    /// Tzais (nightfall) according to Rabbeinu Tam - 72 minutes after sunset, the time to walk 4 mil at 18 minutes
    /// per mil.
    ///
    /// 72 standard clock minutes after sunset, any time of year and in any location.
    Tzais72Minutes {
        java: "getTzais72Minutes",
        name: "Tzais (72 Minutes)",
        category: Tzais,
        formula: Offset(&ConfiguredSunset, Duration::from_mins(72)),
        about: concat!(
            "Tzais (nightfall) according to Rabbeinu Tam - 72 minutes after sunset, the time to walk 4 mil at 18 minutes per mil.\n\n",
            "72 standard clock minutes after sunset, any time of year and in any location. \n\n",
            "In places such as the Arctic Circle, where there is at least one day a year when the sun does not rise and one when it does not set, this zman may not be available or cannot be calculated.",
        ),
    },

    /// Tzais (nightfall) according to the [Minchas Cohen](https://en.wikipedia.org/wiki/Abraham_Cohen_Pimentel) - 72
    /// zmaniyos minutes (1/10 of the day) after sunset.
    ///
    /// This is the Minchas Cohen's calculation of Rabbeinu Tam's tzais. Note that twilight does not vary in direct
    /// proportion to the length of the day, so this zman does not match astronomical reality.
    Tzais72Zmanis {
        java: "getTzais72Zmanis",
        name: "Tzais (72 Minutes in Shaos Zmanios)",
        category: Tzais,
        formula: ZmanisOffset(&ConfiguredSunset, 1.2),
        about: concat!(
            "Tzais (nightfall) according to the [Minchas Cohen](https://en.wikipedia.org/wiki/Abraham_Cohen_Pimentel) - 72 zmaniyos minutes (1/10 of the day) after sunset.\n\n",
            "This is the Minchas Cohen's calculation of Rabbeinu Tam's tzais. Note that twilight does not vary in direct proportion to the length of the day, so this zman does not match astronomical reality.\n\n",
            "In places such as the Arctic Circle, where there is at least one day a year when the sun does not rise and one when it does not set, this zman may not be available or cannot be calculated.",
        ),
    },

    /// Tzais (nightfall) according to the Magen Avraham - 90 minutes after sunset.
    ///
    /// Based on Ula's calculation of tzais as 5 mil after sunset, using the Rambam's 18-minute mil.
    Tzais90Minutes {
        java: "getTzais90Minutes",
        name: "Tzais (90 Minutes)",
        category: Tzais,
        formula: Offset(&ConfiguredSunset, Duration::from_mins(90)),
        about: concat!(
            "Tzais (nightfall) according to the Magen Avraham - 90 minutes after sunset.\n\n",
            "Based on Ula's calculation of tzais as 5 mil after sunset, using the Rambam's 18-minute mil. \n\n",
            "In places such as the Arctic Circle, where there is at least one day a year when the sun does not rise and one when it does not set, this zman may not be available or cannot be calculated.",
        ),
    },

    /// Tzais (nightfall) - 90 zmaniyos minutes (1/8 of the day) after sunset.
    ///
    /// Known in Yiddish as the achtel zman, used in various kehilos.
    Tzais90Zmanis {
        java: "getTzais90Zmanis",
        name: "Tzais (90 Minutes in Shaos Zmanios)",
        category: Tzais,
        formula: ZmanisOffset(&ConfiguredSunset, 1.5),
        about: concat!(
            "Tzais (nightfall) - 90 zmaniyos minutes (1/8 of the day) after sunset.\n\n",
            "Known in Yiddish as the achtel zman, used in various kehilos.\n\n",
            "In places such as the Arctic Circle, where there is at least one day a year when the sun does not rise and one when it does not set, this zman may not be available or cannot be calculated.",
        ),
    },

    /// Tzais (nightfall) - 96 minutes after sunset.
    ///
    /// Based on the time to walk 4 mil at 24 minutes per mil.
    Tzais96Minutes {
        java: "getTzais96Minutes",
        name: "Tzais (96 Minutes)",
        category: Tzais,
        formula: Offset(&ConfiguredSunset, Duration::from_mins(96)),
        about: concat!(
            "Tzais (nightfall) - 96 minutes after sunset.\n\n",
            "Based on the time to walk 4 mil at 24 minutes per mil. \n\n",
            "In places such as the Arctic Circle, where there is at least one day a year when the sun does not rise and one when it does not set, this zman may not be available or cannot be calculated.",
        ),
    },

    /// Tzais (nightfall) - 96 zmaniyos minutes (1/7.5 of the day) after sunset.
    ///
    /// In places such as the Arctic Circle, where there is at least one day a year when the sun does not rise and one
    /// when it does not set, this zman may not be available or cannot be calculated.
    Tzais96Zmanis {
        java: "getTzais96Zmanis",
        name: "Tzais (96 Minutes in Shaos Zmanios)",
        category: Tzais,
        formula: ZmanisOffset(&ConfiguredSunset, 1.6),
        about: concat!(
            "Tzais (nightfall) - 96 zmaniyos minutes (1/7.5 of the day) after sunset.\n\n",
            "In places such as the Arctic Circle, where there is at least one day a year when the sun does not rise and one when it does not set, this zman may not be available or cannot be calculated.",
        ),
    },

    /// Tzais (nightfall) according to [Rav Chaim Naeh](https://en.wikipedia.org/wiki/Avraham_Chaim_Naeh) - 120
    /// minutes after sunset.
    ///
    /// Based on Ula's calculation of tzais as 5 mil after sunset, using the Rambam's 24-minute mil.
    #[deprecated(note = "KosherJava deprecates this zman")]
    Tzais120Minutes {
        java: "getTzais120Minutes",
        name: "Tzais (120 Minutes)",
        category: Tzais,
        formula: Offset(&ConfiguredSunset, Duration::from_mins(120)),
        about: concat!(
            "Tzais (nightfall) according to [Rav Chaim Naeh](https://en.wikipedia.org/wiki/Avraham_Chaim_Naeh) - 120 minutes after sunset.\n\n",
            "Based on Ula's calculation of tzais as 5 mil after sunset, using the Rambam's 24-minute mil. \n\n",
            "This zman should be used lechumra only, such as delaying the start of nighttime mitzvos. Using it leniently can lead to chillul Shabbos and similar serious errors.\n\n",
            "In places such as the Arctic Circle, where there is at least one day a year when the sun does not rise and one when it does not set, this zman may not be available or cannot be calculated.",
        ),
    },

    /// Tzais (nightfall) - 120 zmaniyos minutes after sunset.
    ///
    /// This zman should be used lechumra only, such as delaying the start of nighttime mitzvos. The sun is well below
    /// the 18-degree point in most places. Using it leniently can lead to chillul Shabbos and similar serious errors.
    #[deprecated(note = "KosherJava deprecates this zman")]
    Tzais120Zmanis {
        java: "getTzais120Zmanis",
        name: "Tzais (120 Minutes in Shaos Zmanios)",
        category: Tzais,
        formula: ZmanisOffset(&ConfiguredSunset, 2.0),
        about: concat!(
            "Tzais (nightfall) - 120 zmaniyos minutes after sunset.\n\n",
            "This zman should be used lechumra only, such as delaying the start of nighttime mitzvos. The sun is well below the 18-degree point in most places. Using it leniently can lead to chillul Shabbos and similar serious errors.\n\n",
            "In places such as the Arctic Circle, where there is at least one day a year when the sun does not rise and one when it does not set, this zman may not be available or cannot be calculated.",
        ),
    },

    /// Tzais (nightfall) - when the sun is 16.1 degrees below the western horizon after sunset.
    ///
    /// Matches Rabbeinu Tam's 72-minute tzais [around the
    /// equinox/equilux](https://kosherjava.com/2022/01/12/equinox-vs-equilux-zmanim-calculations/) in Jerusalem.
    Tzais16Point1Degrees {
        java: "getTzais16Point1Degrees",
        name: "Tzais (16.1 Degrees)",
        category: Tzais,
        formula: SunsetOffsetByDegrees(16.1),
        about: concat!(
            "Tzais (nightfall) - when the sun is 16.1 degrees below the western horizon after sunset.\n\n",
            "Matches Rabbeinu Tam's 72-minute tzais [around the equinox/equilux](https://kosherjava.com/2022/01/12/equinox-vs-equilux-zmanim-calculations/) in Jerusalem.\n\n",
            "At some northern and southern locations, this zman may not be available or cannot be calculated if the sun does not reach low enough below the horizon.",
        ),
    },

    /// Tzais (nightfall) - when the sun is 18 degrees below the western horizon after sunset.
    ///
    /// Calculated the same way as alos at 18 degrees, but for the evening.
    Tzais18Degrees {
        java: "getTzais18Degrees",
        name: "Tzais (18 Degrees)",
        category: Tzais,
        formula: SunsetOffsetByDegrees(18.0),
        about: concat!(
            "Tzais (nightfall) - when the sun is 18 degrees below the western horizon after sunset.\n\n",
            "Calculated the same way as alos at 18 degrees, but for the evening.\n\n",
            "At some northern and southern locations, this zman may not be available or cannot be calculated if the sun does not reach low enough below the horizon.",
        ),
    },

    /// Tzais (nightfall) - when the sun is 19.8 degrees below the western horizon after sunset.
    ///
    /// Degree-based calculation corresponding to 90 minutes after sunset [around the
    /// equinox/equilux](https://kosherjava.com/2022/01/12/equinox-vs-equilux-zmanim-calculations/) in Jerusalem.
    Tzais19Point8Degrees {
        java: "getTzais19Point8Degrees",
        name: "Tzais (19.8 Degrees)",
        category: Tzais,
        formula: SunsetOffsetByDegrees(19.8),
        about: concat!(
            "Tzais (nightfall) - when the sun is 19.8 degrees below the western horizon after sunset.\n\n",
            "Degree-based calculation corresponding to 90 minutes after sunset [around the equinox/equilux](https://kosherjava.com/2022/01/12/equinox-vs-equilux-zmanim-calculations/) in Jerusalem.\n\n",
            "At some northern and southern locations, this zman may not be available or cannot be calculated if the sun does not reach low enough below the horizon.",
        ),
    },

    /// Tzais (nightfall) - when the sun is 26 degrees below the western horizon after sunset.
    ///
    /// This zman should be used lechumra only, such as delaying the start of nighttime mitzvos. Using it leniently
    /// can lead to chillul Shabbos and similar serious errors.
    #[deprecated(note = "KosherJava deprecates this zman")]
    Tzais26Degrees {
        java: "getTzais26Degrees",
        name: "Tzais (26 Degrees)",
        category: Tzais,
        formula: SunsetOffsetByDegrees(26.0),
        about: concat!(
            "Tzais (nightfall) - when the sun is 26 degrees below the western horizon after sunset.\n\n",
            "This zman should be used lechumra only, such as delaying the start of nighttime mitzvos. Using it leniently can lead to chillul Shabbos and similar serious errors.\n\n",
            "At some northern and southern locations, this zman may not be available or cannot be calculated if the sun does not reach low enough below the horizon.",
        ),
    },

    /// Tzais (nightfall) according to the Ateret Torah calculation of Chacham Yosef Harari-Raful of Yeshivat Ateret
    /// Torah.
    ///
    /// The configured Ateret Torah offset after sunset. Chacham Harari-Raful uses this timing for calculating other
    /// zmanim (such as sof zman krias shema and plag hamincha), but his calendars do not publish a separate tzais
    /// zman. A 25-minute offset was provided for Israel.
    TzaisAteretTorah {
        java: "getTzaisAteretTorah",
        name: "Tzais (Ateret Torah)",
        category: Tzais,
        formula: TzaisAteretTorah,
        about: concat!(
            "Tzais (nightfall) according to the Ateret Torah calculation of Chacham Yosef Harari-Raful of Yeshivat Ateret Torah.\n\n",
            "{ateret_torah_offset} after sunset. Chacham Harari-Raful uses this timing for calculating other zmanim (such as sof zman krias shema and plag hamincha), but his calendars do not publish a separate tzais zman. A 25-minute offset was provided for Israel.\n\n",
            "In places such as the Arctic Circle, where there is at least one day a year when the sun does not rise and one when it does not set, this zman may not be available or cannot be calculated.",
        ),
    },

    /// Tzais (nightfall) according to the [Baal Hatanya](https://en.wikipedia.org/wiki/Shneur_Zalman_of_Liadi) - when
    /// the sun is 6 degrees below the western horizon after sunset.
    ///
    /// Based on shkiah amitis plus 18 minutes (3/4 of a 24-minute mil) and 2 minutes for bain hashmashos of Rav Yosi,
    /// about 24 minutes after sunset in Jerusalem [around the
    /// equinox/equilux](https://kosherjava.com/2022/01/12/equinox-vs-equilux-zmanim-calculations/). See [About Our
    /// Zmanim Calculations at
    /// Chabad.org](https://www.chabad.org/library/article_cdo/aid/3209349/jewish/About-Our-Zmanim-Calculations.htm).
    TzaisBaalHatanya {
        java: "getTzaisBaalHatanya",
        name: "Tzais (Baal Hatanya)",
        category: Tzais,
        formula: SunsetOffsetByDegrees(6.0),
        about: concat!(
            "Tzais (nightfall) according to the [Baal Hatanya](https://en.wikipedia.org/wiki/Shneur_Zalman_of_Liadi) - when the sun is 6 degrees below the western horizon after sunset.\n\n",
            "Based on shkiah amitis plus 18 minutes (3/4 of a 24-minute mil) and 2 minutes for bain hashmashos of Rav Yosi, about 24 minutes after sunset in Jerusalem [around the equinox/equilux](https://kosherjava.com/2022/01/12/equinox-vs-equilux-zmanim-calculations/). See [About Our Zmanim Calculations at Chabad.org](https://www.chabad.org/library/article_cdo/aid/3209349/jewish/About-Our-Zmanim-Calculations.htm).\n\n",
            "At some northern and southern locations, this zman may not be available or cannot be calculated if the sun does not reach low enough below the horizon.",
        ),
    },

    /// Tzais (nightfall) according to the Geonim - when the sun is 3.7 degrees below the western horizon after
    /// sunset.
    ///
    /// Corresponds to 13.5 minutes after sunset (3/4 of a mil at 18 minutes per mil) in Jerusalem [around the
    /// equinox/equilux](https://kosherjava.com/2022/01/12/equinox-vs-equilux-zmanim-calculations/). Does not include
    /// the time to walk 49 amos for bain hashmashos of Rav Yosi.
    #[deprecated(note = "KosherJava deprecates this zman")]
    TzaisGeonim3Point7Degrees {
        java: "getTzaisGeonim3Point7Degrees",
        name: "Tzais (Geonim, 3.7 Degrees)",
        category: Tzais,
        formula: SunsetOffsetByDegrees(3.7),
        about: concat!(
            "Tzais (nightfall) according to the Geonim - when the sun is 3.7 degrees below the western horizon after sunset.\n\n",
            "Corresponds to 13.5 minutes after sunset (3/4 of a mil at 18 minutes per mil) in Jerusalem [around the equinox/equilux](https://kosherjava.com/2022/01/12/equinox-vs-equilux-zmanim-calculations/). Does not include the time to walk 49 amos for bain hashmashos of Rav Yosi.",
        ),
    },

    /// Tzais (nightfall) according to the Geonim - when the sun is 3.8 degrees below the western horizon after
    /// sunset.
    ///
    /// Corresponds to 14 minutes after sunset: 13.5 minutes for 3/4 of an 18-minute mil, plus 30 seconds for 49 amos
    /// (bain hashmashos of Rav Yosi), in Jerusalem [around the
    /// equinox/equilux](https://kosherjava.com/2022/01/12/equinox-vs-equilux-zmanim-calculations/).
    #[deprecated(note = "KosherJava deprecates this zman")]
    TzaisGeonim3Point8Degrees {
        java: "getTzaisGeonim3Point8Degrees",
        name: "Tzais (Geonim, 3.8 Degrees)",
        category: Tzais,
        formula: SunsetOffsetByDegrees(3.8),
        about: concat!(
            "Tzais (nightfall) according to the Geonim - when the sun is 3.8 degrees below the western horizon after sunset.\n\n",
            "Corresponds to 14 minutes after sunset: 13.5 minutes for 3/4 of an 18-minute mil, plus 30 seconds for 49 amos (bain hashmashos of Rav Yosi), in Jerusalem [around the equinox/equilux](https://kosherjava.com/2022/01/12/equinox-vs-equilux-zmanim-calculations/).",
        ),
    },

    /// Tzais (nightfall) according to the Geonim - when the sun is 5.95 degrees below the western horizon after
    /// sunset.
    ///
    /// About 24 minutes after sunset in Jerusalem [around the
    /// equinox/equilux](https://kosherjava.com/2022/01/12/equinox-vs-equilux-zmanim-calculations/), based on 18
    /// minutes (3/4 of a 24-minute mil) plus shkiah amitis and bain hashmashos of Rav Yosi. Chabad calendars usually
    /// use the related 6-degree [Baal Hatanya](https://en.wikipedia.org/wiki/Shneur_Zalman_of_Liadi) tzais built on
    /// this calculation.
    TzaisGeonim5Point95Degrees {
        java: "getTzaisGeonim5Point95Degrees",
        name: "Tzais (Geonim, 5.95 Degrees)",
        category: Tzais,
        formula: SunsetOffsetByDegrees(5.95),
        about: concat!(
            "Tzais (nightfall) according to the Geonim - when the sun is 5.95 degrees below the western horizon after sunset.\n\n",
            "About 24 minutes after sunset in Jerusalem [around the equinox/equilux](https://kosherjava.com/2022/01/12/equinox-vs-equilux-zmanim-calculations/), based on 18 minutes (3/4 of a 24-minute mil) plus shkiah amitis and bain hashmashos of Rav Yosi. Chabad calendars usually use the related 6-degree [Baal Hatanya](https://en.wikipedia.org/wiki/Shneur_Zalman_of_Liadi) tzais built on this calculation.\n\n",
            "At some northern and southern locations, this zman may not be available or cannot be calculated if the sun does not reach low enough below the horizon.",
        ),
    },

    /// Tzais (nightfall) according to the Geonim - when the sun is 4.66 degrees below the western horizon after
    /// sunset.
    ///
    /// Based on 3/4 of a 24-minute mil (18 minutes after sunset). This is a very early zman and should not be relied
    /// on without rabbinical guidance. Does not include the time to walk 49 amos for bain hashmashos of Rav Yosi.
    #[deprecated(note = "KosherJava deprecates this zman")]
    TzaisGeonim4Point66Degrees {
        java: "getTzaisGeonim4Point66Degrees",
        name: "Tzais (Geonim, 4.66 Degrees)",
        category: Tzais,
        formula: SunsetOffsetByDegrees(4.66),
        about: concat!(
            "Tzais (nightfall) according to the Geonim - when the sun is 4.66 degrees below the western horizon after sunset.\n\n",
            "Based on 3/4 of a 24-minute mil (18 minutes after sunset). This is a very early zman and should not be relied on without rabbinical guidance. Does not include the time to walk 49 amos for bain hashmashos of Rav Yosi.\n\n",
            "At some northern and southern locations, this zman may not be available or cannot be calculated if the sun does not reach low enough below the horizon.",
        ),
    },

    /// Tzais (nightfall) according to the Geonim - when the sun is 4.42 degrees below the western horizon after
    /// sunset.
    ///
    /// Based on 3/4 of a 22.5-minute mil (16 7/8 minutes after sunset). This is a very early zman and should not be
    /// relied on without rabbinical guidance. Does not include the time to walk 49 amos for bain hashmashos of Rav
    /// Yosi.
    #[deprecated(note = "KosherJava deprecates this zman")]
    TzaisGeonim4Point42Degrees {
        java: "getTzaisGeonim4Point42Degrees",
        name: "Tzais (Geonim, 4.42 Degrees)",
        category: Tzais,
        formula: SunsetOffsetByDegrees(4.42),
        about: concat!(
            "Tzais (nightfall) according to the Geonim - when the sun is 4.42 degrees below the western horizon after sunset.\n\n",
            "Based on 3/4 of a 22.5-minute mil (16 7/8 minutes after sunset). This is a very early zman and should not be relied on without rabbinical guidance. Does not include the time to walk 49 amos for bain hashmashos of Rav Yosi.\n\n",
            "At some northern and southern locations, this zman may not be available or cannot be calculated if the sun does not reach low enough below the horizon.",
        ),
    },

    /// Tzais (nightfall) according to the Geonim - when the sun is 4.8 degrees below the western horizon after
    /// sunset.
    ///
    /// 18.6 minutes after sunset: 3/4 of a 24-minute mil plus time for 49 amos (bain hashmashos of Rav Yosi). Based
    /// on [Rav Yechiel Michel
    /// Shlezinger's](https://he.wikipedia.org/wiki/%D7%99%D7%97%D7%99%D7%90%D7%9C_%D7%9E%D7%99%D7%9B%D7%9C_%D7%A9%D7%9C%D7%96%D7%99%D7%A0%D7%92%D7%A8)
    /// [Aizehu Bain Hashmashos](https://www.nli.org.il/he/books/NNL_ALEPH997010042055805171/NLI) and [Rabbi Yehuda
    /// (Leo) Levi's](https://en.wikipedia.org/wiki/Yehuda_(Leo)_Levi) calculations in [Zmanei Hayom
    /// BaHalacha](https://www.nli.org.il/en/items/NNL_ALEPH990022548970205171/NLI). This is an early zman and should
    /// not be relied on without rabbinical guidance.
    TzaisGeonim4Point8Degrees {
        java: "getTzaisGeonim4Point8Degrees",
        name: "Tzais (Geonim, 4.8 Degrees)",
        category: Tzais,
        formula: SunsetOffsetByDegrees(4.8),
        about: concat!(
            "Tzais (nightfall) according to the Geonim - when the sun is 4.8 degrees below the western horizon after sunset.\n\n",
            "18.6 minutes after sunset: 3/4 of a 24-minute mil plus time for 49 amos (bain hashmashos of Rav Yosi). Based on [Rav Yechiel Michel Shlezinger's](https://he.wikipedia.org/wiki/%D7%99%D7%97%D7%99%D7%90%D7%9C_%D7%9E%D7%99%D7%9B%D7%9C_%D7%A9%D7%9C%D7%96%D7%99%D7%A0%D7%92%D7%A8) [Aizehu Bain Hashmashos](https://www.nli.org.il/he/books/NNL_ALEPH997010042055805171/NLI) and [Rabbi Yehuda (Leo) Levi's](https://en.wikipedia.org/wiki/Yehuda_(Leo)_Levi) calculations in [Zmanei Hayom BaHalacha](https://www.nli.org.il/en/items/NNL_ALEPH990022548970205171/NLI). This is an early zman and should not be relied on without rabbinical guidance.\n\n",
            "At some northern and southern locations, this zman may not be available or cannot be calculated if the sun does not reach low enough below the horizon.",
        ),
    },

    /// Tzais (nightfall) according to the Geonim - when the sun is 6.45 degrees below the western horizon after
    /// sunset.
    ///
    /// Commonly used in Israel. Based on [Rabbi Yechiel Michel
    /// Tucazinsky's](https://en.wikipedia.org/wiki/Yechiel_Michel_Tucazinsky) calculation, about 31 minutes after sea
    /// level sunset in Jerusalem and about 26.5 minutes at the equinox. Also used in [Luach Itim
    /// Lebinah](https://www.worldcat.org/oclc/243303103). See [Birur Halacha Yoreh Deah
    /// 262](https://hebrewbooks.org/pdfpager.aspx?req=50536&st=&pgnum=51).
    TzaisGeonim6Point45Degrees {
        java: "getTzaisGeonim6Point45Degrees",
        name: "Tzais (Geonim, 6.45 Degrees)",
        category: Tzais,
        formula: SunsetOffsetByDegrees(6.45),
        about: concat!(
            "Tzais (nightfall) according to the Geonim - when the sun is 6.45 degrees below the western horizon after sunset.\n\n",
            "Commonly used in Israel. Based on [Rabbi Yechiel Michel Tucazinsky's](https://en.wikipedia.org/wiki/Yechiel_Michel_Tucazinsky) calculation, about 31 minutes after sea level sunset in Jerusalem and about 26.5 minutes at the equinox. Also used in [Luach Itim Lebinah](https://www.worldcat.org/oclc/243303103). See [Birur Halacha Yoreh Deah 262](https://hebrewbooks.org/pdfpager.aspx?req=50536&st=&pgnum=51).\n\n",
            "At some northern and southern locations, this zman may not be available or cannot be calculated if the sun does not reach low enough below the horizon.",
        ),
    },

    /// Tzais (nightfall) according to the Geonim - when the sun is 7.083 degrees (7 degrees 5 minutes) below the
    /// western horizon after sunset.
    ///
    /// Based on Dr. Baruch (Berthold) Cohn's observation of 3 medium-sized stars in his [1899
    /// luach](https://sammlungen.ub.uni-frankfurt.de/freimann/content/titleinfo/983088). Endorsed by [Rav Dovid Tzvi
    /// Hoffman](https://en.wikipedia.org/wiki/David_Zvi_Hoffmann) in [Melamed Leho'il Orach Chaim
    /// 30](https://hebrewbooks.org/pdfpager.aspx?req=1053&st=&pgnum=37). Close to the [Makor
    /// Chessed](https://hebrewbooks.org/22044) of the Sefer Chasidim and to about 30 minutes after sunset in
    /// Jerusalem [around the
    /// equinox/equilux](https://kosherjava.com/2022/01/12/equinox-vs-equilux-zmanim-calculations/), but not exactly.
    TzaisGeonim7Point083Degrees {
        java: "getTzaisGeonim7Point083Degrees",
        name: "Tzais (Geonim, 7.083 Degrees)",
        category: Tzais,
        formula: SunsetOffsetByDegrees(7.083333333333333),
        about: concat!(
            "Tzais (nightfall) according to the Geonim - when the sun is 7.083 degrees (7 degrees 5 minutes) below the western horizon after sunset.\n\n",
            "Based on Dr. Baruch (Berthold) Cohn's observation of 3 medium-sized stars in his [1899 luach](https://sammlungen.ub.uni-frankfurt.de/freimann/content/titleinfo/983088). Endorsed by [Rav Dovid Tzvi Hoffman](https://en.wikipedia.org/wiki/David_Zvi_Hoffmann) in [Melamed Leho'il Orach Chaim 30](https://hebrewbooks.org/pdfpager.aspx?req=1053&st=&pgnum=37). Close to the [Makor Chessed](https://hebrewbooks.org/22044) of the Sefer Chasidim and to about 30 minutes after sunset in Jerusalem [around the equinox/equilux](https://kosherjava.com/2022/01/12/equinox-vs-equilux-zmanim-calculations/), but not exactly.\n\n",
            "At some northern and southern locations, this zman may not be available or cannot be calculated if the sun does not reach low enough below the horizon.",
        ),
    },

    /// Tzais (nightfall) according to the Geonim - when the sun is 7.67 degrees below the western horizon after
    /// sunset.
    ///
    /// Corresponds to 45 minutes after sunset during the summer solstice in New York, when twilight is longest. Cited
    /// in [Igros Moshe Even Haezer 4, ch. 4](https://hebrewbooks.org/pdfpager.aspx?req=921&pgnum=149) regarding tzais
    /// for krias shema, and in Rabbi Heber's [Shaarei Zmanim](https://hebrewbooks.org/53000) ([chapter 10, page
    /// 87](https://hebrewbooks.org/pdfpager.aspx?req=53055&pgnum=101) and [chapter 12, page
    /// 108](https://hebrewbooks.org/pdfpager.aspx?req=53055&pgnum=122)). Also endorsed by [Rabbi Shmuel
    /// Kamenetsky](https://en.wikipedia.org/wiki/Shmuel_Kamenetsky).
    TzaisGeonim7Point67Degrees {
        java: "getTzaisGeonim7Point67Degrees",
        name: "Tzais (Geonim, 7.67 Degrees)",
        category: Tzais,
        formula: SunsetOffsetByDegrees(7.67),
        about: concat!(
            "Tzais (nightfall) according to the Geonim - when the sun is 7.67 degrees below the western horizon after sunset.\n\n",
            "Corresponds to 45 minutes after sunset during the summer solstice in New York, when twilight is longest. Cited in [Igros Moshe Even Haezer 4, ch. 4](https://hebrewbooks.org/pdfpager.aspx?req=921&pgnum=149) regarding tzais for krias shema, and in Rabbi Heber's [Shaarei Zmanim](https://hebrewbooks.org/53000) ([chapter 10, page 87](https://hebrewbooks.org/pdfpager.aspx?req=53055&pgnum=101) and [chapter 12, page 108](https://hebrewbooks.org/pdfpager.aspx?req=53055&pgnum=122)). Also endorsed by [Rabbi Shmuel Kamenetsky](https://en.wikipedia.org/wiki/Shmuel_Kamenetsky).\n\n",
            "At some northern and southern locations, this zman may not be available or cannot be calculated if the sun does not reach low enough below the horizon.",
        ),
    },

    /// Tzais (nightfall) according to the Geonim - when the sun is 9.3 degrees below the western horizon after
    /// sunset.
    ///
    /// The stringent tzais used in [Luach Itim Lebinah](https://www.worldcat.org/oclc/243303103).
    TzaisGeonim9Point3Degrees {
        java: "getTzaisGeonim9Point3Degrees",
        name: "Tzais (Geonim, 9.3 Degrees)",
        category: Tzais,
        formula: SunsetOffsetByDegrees(9.3),
        about: concat!(
            "Tzais (nightfall) according to the Geonim - when the sun is 9.3 degrees below the western horizon after sunset.\n\n",
            "The stringent tzais used in [Luach Itim Lebinah](https://www.worldcat.org/oclc/243303103).\n\n",
            "At some northern and southern locations, this zman may not be available or cannot be calculated if the sun does not reach low enough below the horizon.",
        ),
    },

    /// Tzais (nightfall) according to the Geonim - when the sun is 9.75 degrees below the western horizon after
    /// sunset.
    ///
    /// Corresponds to 60 minutes after sunset [around the
    /// equinox/equilux](https://kosherjava.com/2022/01/12/equinox-vs-equilux-zmanim-calculations/) in New York, when
    /// a solar hour is 60 minutes. The opinion of [Rabbi Eliyahu
    /// Henkin](https://en.wikipedia.org/wiki/Yosef_Eliyahu_Henkin) and [Rabbi Shmuel
    /// Kamenetsky](https://en.wikipedia.org/wiki/Shmuel_Kamenetsky).
    TzaisGeonim9Point75Degrees {
        java: "getTzaisGeonim9Point75Degrees",
        name: "Tzais (Geonim, 9.75 Degrees)",
        category: Tzais,
        formula: SunsetOffsetByDegrees(9.75),
        about: concat!(
            "Tzais (nightfall) according to the Geonim - when the sun is 9.75 degrees below the western horizon after sunset.\n\n",
            "Corresponds to 60 minutes after sunset [around the equinox/equilux](https://kosherjava.com/2022/01/12/equinox-vs-equilux-zmanim-calculations/) in New York, when a solar hour is 60 minutes. The opinion of [Rabbi Eliyahu Henkin](https://en.wikipedia.org/wiki/Yosef_Eliyahu_Henkin) and [Rabbi Shmuel Kamenetsky](https://en.wikipedia.org/wiki/Shmuel_Kamenetsky).\n\n",
            "At some northern and southern locations, this zman may not be available or cannot be calculated if the sun does not reach low enough below the horizon.",
        ),
    },

    // KidushLevana

    /// The latest time for Kiddush Levana according to the Shulchan Aruch (Orach Chaim 426) - 15 days after the
    /// molad.
    ///
    /// Some hold that the [Rema](https://en.wikipedia.org/wiki/Moses_Isserles), who cites the
    /// [Maharil's](https://en.wikipedia.org/wiki/Yaakov_ben_Moshe_Levi_Moelin) approach of calculating halfway
    /// between molad and molad, reflects the Mechaber's view as well. See Rabbi Dovid Heber's detailed write-up in
    /// siman daled (chapter 4) of [Shaarei Zmanim](https://hebrewbooks.org/53000).
    ///
    /// This time needs a time zone. Without one it returns
    /// [`ZmanimError::TimeZoneRequired`](super::ZmanimError).
    SofZmanKidushLevana15Days {
        java: "getSofZmanKidushLevana15Days",
        name: "Sof Zman Kidush Levana (15 Days)",
        category: KidushLevana,
        formula: SofZmanKidushLevana15Days,
        about: concat!(
            "The latest time for Kiddush Levana according to the Shulchan Aruch (Orach Chaim 426) - 15 days after the molad.\n\n",
            "Some hold that the [Rema](https://en.wikipedia.org/wiki/Moses_Isserles), who cites the [Maharil's](https://en.wikipedia.org/wiki/Yaakov_ben_Moshe_Levi_Moelin) approach of calculating halfway between molad and molad, reflects the Mechaber's view as well. See Rabbi Dovid Heber's detailed write-up in siman daled (chapter 4) of [Shaarei Zmanim](https://hebrewbooks.org/53000).\n\n",
            "Note that although this time may be during the daytime, Kiddush Levana cannot be said during the daytime.",
        ),
    },

    /// The latest time for Kiddush Levana according to the
    /// [Maharil](https://en.wikipedia.org/wiki/Yaakov_ben_Moshe_Levi_Moelin) - halfway between this molad and the
    /// next.
    ///
    /// Half of the 29 days, 12 hours, and 793 chalakim interval between molad and molad (14 days, 18 hours, 22
    /// minutes, and 1.666 seconds) after the month's molad.
    ///
    /// This time needs a time zone. Without one it returns
    /// [`ZmanimError::TimeZoneRequired`](super::ZmanimError).
    SofZmanKidushLevanaBetweenMoldos {
        java: "getSofZmanKidushLevanaBetweenMoldos",
        name: "Sof Zman Kidush Levana (Between Moldos)",
        category: KidushLevana,
        formula: SofZmanKidushLevanaBetweenMoldos,
        about: concat!(
            "The latest time for Kiddush Levana according to the [Maharil](https://en.wikipedia.org/wiki/Yaakov_ben_Moshe_Levi_Moelin) - halfway between this molad and the next.\n\n",
            "Half of the 29 days, 12 hours, and 793 chalakim interval between molad and molad (14 days, 18 hours, 22 minutes, and 1.666 seconds) after the month's molad.\n\n",
            "Note that although this time may be during the daytime, Kiddush Levana cannot be said during the daytime.",
        ),
    },

    /// The earliest time for Kiddush Levana according to [Rabbeinu
    /// Yonah](https://en.wikipedia.org/wiki/Yonah_Gerondi) - 3 days after the molad.
    ///
    /// Note that although this time may be during the daytime, Kiddush Levana cannot be said during the daytime.
    ///
    /// This time needs a time zone. Without one it returns
    /// [`ZmanimError::TimeZoneRequired`](super::ZmanimError).
    TchilasZmanKidushLevana3Days {
        java: "getTchilasZmanKidushLevana3Days",
        name: "Tchilas Zman Kidush Levana (3 Days)",
        category: KidushLevana,
        formula: TchilasZmanKidushLevana3Days,
        about: concat!(
            "The earliest time for Kiddush Levana according to [Rabbeinu Yonah](https://en.wikipedia.org/wiki/Yonah_Gerondi) - 3 days after the molad.\n\n",
            "Note that although this time may be during the daytime, Kiddush Levana cannot be said during the daytime.",
        ),
    },

    /// The earliest time for Kiddush Levana according to the opinion that it should not be said until 7 days after
    /// the molad.
    ///
    /// Note that although this time may be during the daytime, Kiddush Levana cannot be said during the daytime.
    ///
    /// This time needs a time zone. Without one it returns
    /// [`ZmanimError::TimeZoneRequired`](super::ZmanimError).
    TchilasZmanKidushLevana7Days {
        java: "getTchilasZmanKidushLevana7Days",
        name: "Tchilas Zman Kidush Levana (7 Days)",
        category: KidushLevana,
        formula: TchilasZmanKidushLevana7Days,
        about: concat!(
            "The earliest time for Kiddush Levana according to the opinion that it should not be said until 7 days after the molad.\n\n",
            "Note that although this time may be during the daytime, Kiddush Levana cannot be said during the daytime.",
        ),
    },

    // ChatzosHalayla

    /// Solar midnight - when the sun transits the lower celestial meridian (at its nadir).
    ///
    /// Calculated for the end of the current day. For example, solar midnight for February 8 is the moment between
    /// February 8 and February 9 when the sun is at its lowest point. See [The Definition of
    /// Chatzos](https://kosherjava.com/2020/07/02/definition-of-chatzos/) for details on the proper definition of
    /// solar noon and midnight.
    SolarMidnight {
        java: "getSolarMidnight",
        name: "Solar Midnight",
        category: ChatzosHalayla,
        formula: SolarMidnight,
        about: concat!(
            "Solar midnight - when the sun transits the lower celestial meridian (at its nadir).\n\n",
            "Calculated for the end of the current day. For example, solar midnight for February 8 is the moment between February 8 and February 9 when the sun is at its lowest point. See [The Definition of Chatzos](https://kosherjava.com/2020/07/02/definition-of-chatzos/) for details on the proper definition of solar noon and midnight.\n\n",
            "In places such as the Arctic Circle, where there is at least one day a year when the sun does not rise and one when it does not set, this zman may not be available or cannot be calculated.",
        ),
    },

    /// Chatzos halayla.
    ///
    /// For how chatzos can be defined and calculated, see [The Definition of
    /// Chatzos](https://kosherjava.com/2020/07/02/definition-of-chatzos/) on the KosherJava blog.
    ChatzosHalayla {
        java: "getChatzosHalayla",
        name: "Chatzos Halayla",
        category: ChatzosHalayla,
        formula: ChatzosHalayla,
        about: concat!(
            "Chatzos halayla.\n\n",
            "For how chatzos can be defined and calculated, see [The Definition of Chatzos](https://kosherjava.com/2020/07/02/definition-of-chatzos/) on the KosherJava blog.",
        ),
    },
}
