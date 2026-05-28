# getAlos120Zmanis

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getAlos120Zmanis` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:809)

```javadoc
This method should be used <em>lechumra</em> only and  method returns <em>alos</em> (dawn) calculated using
120 minutes <em>zmaniyos</em> or 1/6th of the day before {@link #getSunset() sunrise} or {@link
#getSeaLevelSunrise() sea level sunrise} (depending on the {@link #isUseElevation()} setting). This is based
on a 24-minute <a href="https://en.wikipedia.org/wiki/Biblical_and_Talmudic_units_of_measurement">mil</a> so
the time for 5 mil is 120 minutes which is 1/6th of a day (12 * 60 / 6 = 120). The day is calculated from
{@link #getSeaLevelSunrise() sea level sunrise} to {@link #getSeaLevelSunset() sea level sunset} or {@link
#getSunset() sunrise} to {@link #getSunset() sunset} (depending on the {@link
#isUseElevation()}. The actual calculation used is {@link #getSunset()} - ({@link #getShaahZmanisGRA()}
* 2). Since this time is extremely early, it should only be used <em>lechumra</em>, such as not eating after this time
on a fast day, and not as the start time for <em>mitzvos</em> that can only be performed during the day.

@deprecated This method should be used <em>lechumra</em> only (such as stopping to eat at this time on a fast day),
        since it returns a very early time, and if used <em>lekula</em> can result in doing <em>mitzvos hayom</em>
        too early according to most opinions. There is no current plan to remove this method from the API, and this
        deprecation is intended to alert developers of the danger of using it.
@return the <code>Instant</code> representing the time. If the calculation can't be computed such as in the Arctic
        Circle where there is at least one day a year where the sun does not rise, and one where it does not set,
        a <code>null</code> will be returned. See detailed explanation on top of the {@link AstronomicalCalendar}
        documentation.
@see #getAlos120Minutes()
@see #getAlos26Degrees()
```

# Human docs

```markdown
Alos (dawn), an extremely early time that should be used lechumra only - for example, stopping to eat on a fast day - and not as the start of daytime mitzvos.

120 zmaniyos minutes (one-sixth of the day) before sunrise. {uses_elevation} Based on a 24-minute [mil](https://en.wikipedia.org/wiki/Biblical_and_Talmudic_units_of_measurement), so 5 mil equals 120 minutes, which is one-sixth of a 12-hour day. Using this zman leniently can lead to performing daytime mitzvos too early according to most opinions.

In places such as the Arctic Circle, where there is at least one day a year when the sun does not rise and one when it does not set, this zman may not be available or cannot be calculated.
```
