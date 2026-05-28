# getAlos72Zmanis

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getAlos72Zmanis` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:685)

```javadoc
Method to return <em>alos</em> (dawn) calculated using 72 minutes <em>zmaniyos</em> or 1/10th of the day before sunrise. This
is based on an 18-minute <a href="https://en.wikipedia.org/wiki/Biblical_and_Talmudic_units_of_measurement">mil</a> so the
time for 4 mil is 72 minutes which is 1/10th of a day (12 * 60 = 720) based on the day being from {@link #getSeaLevelSunrise()}
to {@link #getSeaLevelSunset() sea level sunset} or {@link #getSunset() sunrise} to {@link #getSunset()} (depending on the
{@link #isUseElevation()} setting). The actual calculation is {@link #getSunriseBasedOnElevationSetting()} - ({@link
#getShaahZmanisGRA()} * 1.2). This calculation is used in the calendars published by the <a href=
"https://en.wikipedia.org/wiki/Central_Rabbinical_Congress">Hisachdus Harabanim D'Artzos Habris Ve'Canada</a>.

@return the <code>Instant</code> representing the time. If the calculation can't be computed such as in the Arctic Circle where
        there is at least one day a year where the sun does not rise, and one where it does not set, a <code>null</code> will
        be returned. See detailed explanation on top of the {@link AstronomicalCalendar} documentation.
@see #getShaahZmanisGRA()
```

# Human docs

```markdown
Alos (dawn), the beginning of morning twilight before sunrise.

72 zmaniyos minutes (one-tenth of the day) before sunrise. {uses_elevation} Based on an 18-minute [mil](https://en.wikipedia.org/wiki/Biblical_and_Talmudic_units_of_measurement), so 4 mil equals 72 minutes, which is one-tenth of a 12-hour day (720 minutes). The day length used for the proportional hour is from sunrise to sunset. Used in calendars published by the [Hisachdus Harabanim D'Artzos Habris Ve'Canada](https://en.wikipedia.org/wiki/Central_Rabbinical_Congress).

In places such as the Arctic Circle, where there is at least one day a year when the sun does not rise and one when it does not set, this zman may not be available or cannot be calculated.
```
