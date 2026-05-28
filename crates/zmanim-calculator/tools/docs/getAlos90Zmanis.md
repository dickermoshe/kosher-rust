# getAlos90Zmanis

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getAlos90Zmanis` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:717)

```javadoc
Method to return <em>alos</em> (dawn) calculated using 90 minutes <em>zmaniyos</em> or 1/8th of the day before {@link
#getSunrise()} or {@link #getSeaLevelSunrise()} (depending on the {@link #isUseElevation()} setting). This is based on a
22.5-minute <a href="https://en.wikipedia.org/wiki/Biblical_and_Talmudic_units_of_measurement">mil</a> so the time for 4 mil
is 90 minutes which is 1/8th of a day (12 * 60) / 8 = 90. The actual calculation used is {@link
#getSunriseBasedOnElevationSetting()} - ({@link #getShaahZmanisGRA()} * 1.5).

@return the <code>Instant</code> representing the time. If the calculation can't be computed such as in the Arctic Circle where
        there is at least one day a year where the sun does not rise, and one where it does not set, a <code>null</code> will
        be returned. See detailed explanation on top of the {@link AstronomicalCalendar} documentation.
@see #getShaahZmanisGRA()
```

# Human docs

```markdown
Alos (dawn), the beginning of morning twilight before sunrise.

90 zmaniyos minutes before sunrise, or 1/8 of the day. {uses_elevation}

Based on 4 mil at 22.5 minutes per mil, measured in shaos zmaniyos.

In places where sunrise or sunset cannot be calculated, this zman may not be available.
```
