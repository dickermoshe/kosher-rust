# getAlos120Minutes

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getAlos120Minutes` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:781)

```javadoc
This method should be used <em>lechumra</em> only and returns <em>alos</em> (dawn) calculated using 120 minutes before {@link
#getSunset() sunrise} or {@link #getSeaLevelSunrise() sea level sunrise} (depending on the {@link #isUseElevation()} setting)
based on the time to walk the distance of 5 <a href="https://en.wikipedia.org/wiki/Biblical_and_Talmudic_units_of_measurement"
>mil</a> (<em>Ula</em>) at 24 minutes a mil. Time based offset calculations for <em>alos</em> are based on the* opinion of the
<em><a href="https://en.wikipedia.org/wiki/Rishonim">Rishonim</a></em> who stated that the time of the <em>neshef</em> (time
between dawn and sunrise) does not vary by the time of year or location but purely depends on the time it takes to walk the
distance of 5 mil (<em>Ula</em>). Since this time is extremely early, it should only be used <em>lechumra</em>, such as not
eating after this time on a fast day, and not as the start time for <em>mitzvos</em> that can only be performed during the day.

@deprecated This method should be used <em>lechumra</em> only (such as stopping to eat at this time on a fast day),
        since it returns a very early time, and if used <em>lekula</em> can result in doing <em>mitzvos hayom</em>
        too early according to most opinions. There is no current plan to remove this method from the API, and this
        deprecation is intended to alert developers of the danger of using it.
@return the <code>Instant</code> representing the time. If the calculation can't be computed such as in the Arctic
        Circle where there is at least one day a year where the sun does not rise, and one where it does not set,
        a <code>null</code> will be returned. See detailed explanation on top of the {@link AstronomicalCalendar}
        documentation.
@see #getTzais120Minutes()
@see #getAlos26Degrees()
```

# Human docs

```markdown
Alos (dawn), using an extremely early 120-minute calculation.

120 minutes before sunrise. {uses_elevation}

Based on the time to walk 5 mil at 24 minutes per mil.

This zman should be used lechumra only, such as stopping to eat on a fast day, and not as the start of daytime mitzvos.

In places where sunrise cannot be calculated, this zman may not be available.
```
