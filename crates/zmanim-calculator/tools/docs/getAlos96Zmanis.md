# getAlos96Zmanis

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getAlos96Zmanis` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:737)

```javadoc
This method returns <em>alos</em> (dawn) calculated using 96 minutes <em>zmaniyos</em> or 1/7.5th of the day before
{@link #getSunset() sunrise} or {@link #getSeaLevelSunrise() sea level sunrise} (depending on the {@link
#isUseElevation()} setting). This is based on a 24-minute <a href=
"https://en.wikipedia.org/wiki/Biblical_and_Talmudic_units_of_measurement">mil</a> so the time for 4 mil is 96 minutes
which is 1/7.5th of a day (12 * 60 / 7.5 = 96). The day is calculated from {@link #getSeaLevelSunrise() sea level sunrise}
to {@link #getSeaLevelSunset() sea level sunset} or {@link #getSunset() sunrise} to {@link
 getSunset() sunset} (depending on the {@link #isUseElevation()}. The actual calculation used is {@link
 getSunriseBasedOnElevationSetting()} - ({@link #getShaahZmanisGRA()} * 1.6).

@return the <code>Instant</code> representing the time. If the calculation can't be computed such as in the Arctic
        Circle where there is at least one day a year where the sun does not rise, and one where it does not set,
        a <code>null</code> will be returned. See detailed explanation on top of the {@link AstronomicalCalendar}
        documentation.
@see #getShaahZmanisGRA()
```

# Human docs

```markdown
Alos (dawn), the beginning of morning twilight before sunrise.

96 zmaniyos minutes before sunrise, or 1/7.5 of the day. {uses_elevation}

Based on 4 mil at 24 minutes per mil, measured in shaos zmaniyos.

In places where sunrise or sunset cannot be calculated, this zman may not be available.
```
