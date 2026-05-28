# getTzais120Zmanis

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getTzais120Zmanis` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:2822)

```javadoc
This method should be used <em>lechumra</em> only and returns <em>tzais</em> (dusk) calculated using 120 minutes
<em>zmaniyos</em> after {@link #getSeaLevelSunset() sea level sunset}. Since the <em>zman</em>
is extremely late and at a time when the sun is well below the 18° point (scientifically the darkest point) in
most places on the globe, it should only be used <em>lechumra</em>, such as delaying the start of nighttime
<em>mitzvos</em>.

@deprecated This method should be used <em>lechumra</em> only since it returns a very late time, and if used
        <em>lekula</em> can result in <em>chillul Shabbos</em> etc. There is no current plan to remove this
        method from the API, and this deprecation is intended to alert developers of the danger of using it.

@return the <code>Instant</code> representing the time. If the calculation can't be computed such as in the Arctic
        Circle where there is at least one day a year where the sun does not rise, and one where it does not set,
        a <code>null</code> will be returned. See detailed explanation on top of the {@link AstronomicalCalendar}
        documentation.
@see #getAlos120Zmanis()
@see #getTzais120Minutes()
@see #getTzais26Degrees()
```

# Human docs

```markdown
Tzais (nightfall) - 120 zmaniyos minutes after sea level sunset.

This zman should be used lechumra only, such as delaying the start of nighttime mitzvos. The sun is well below the 18-degree point in most places. Using it leniently can lead to chillul Shabbos and similar serious errors.

In places such as the Arctic Circle, where there is at least one day a year when the sun does not rise and one when it does not set, this zman may not be available or cannot be calculated.
```
