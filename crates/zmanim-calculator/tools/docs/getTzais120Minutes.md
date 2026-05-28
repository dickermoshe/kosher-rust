# getTzais120Minutes

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getTzais120Minutes` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:2798)

```javadoc
This method should be used <em>lechumra</em> only and returns <em>tzais</em> (nightfall) based on the calculations
of <a href="https://en.wikipedia.org/wiki/Avraham_Chaim_Naeh">Rav Chaim Naeh</a> that the time to walk the distance
of a <a href="https://en.wikipedia.org/wiki/Biblical_and_Talmudic_units_of_measurement">mil</a> according to the <a
href="https://en.wikipedia.org/wiki/Maimonides">Rambam</a>'s opinion is 2/5 of an hour (24 minutes) for a total of 120
minutes based on the opinion of <em>Ula</em> who calculated <em>tzais</em> as 5 mil after {@link #getSunset()
sunset} or {@link #getSeaLevelSunset() sea level sunset} (depending on the {@link #isUseElevation()} setting).
A similar calculation {@link #getTzais26Degrees()} uses degree-based calculations based on this 120 minute calculation.
Since the <em>zman</em> is extremely late and at a point that is long past the 18° point where the darkest point is
reached, it should only be used <em>lechumra</em>, such as delaying the start of nighttime <em>mitzvos</em>.

@deprecated This method should be used <em>lechumra</em> only since it returns a very late time, and if used
        <em>lekula</em> can result in <em>chillul Shabbos</em> etc. There is no current plan to remove this
        method from the API, and this deprecation is intended to alert developers of the danger of using it.
@return the <code>Instant</code> representing the time. If the calculation can't be computed such as in the Arctic
        Circle where there is at least one day a year where the sun does not rise, and one where it does not set,
        a <code>null</code> will be returned. See detailed explanation on top of the {@link AstronomicalCalendar}.
        documentation.
@see #getTzais26Degrees()
@see #getAlos120Minutes()
```

# Human docs

```markdown
```
