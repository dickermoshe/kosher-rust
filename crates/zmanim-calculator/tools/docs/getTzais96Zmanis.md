# getTzais96Zmanis

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getTzais96Zmanis` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:2752)

```javadoc
Method to return <em>tzais</em> (dusk) calculated using 96 minutes <em>zmaniyos</em> or 1/7.5 of the day after
{@link #getSeaLevelSunset() sea level sunset}.

@return the <code>Instant</code> representing the time. If the calculation can't be computed such as in the Arctic
        Circle where there is at least one day a year where the sun does not rise, and one where it does not set,
        a <code>null</code> will be returned. See detailed explanation on top of the {@link AstronomicalCalendar}
        documentation.
@see #getAlos96Zmanis()
```

# Human docs

```markdown
Tzais (nightfall) - 96 zmaniyos minutes (1/7.5 of the day) after sea level sunset.

In places such as the Arctic Circle, where there is at least one day a year when the sun does not rise and one when it does not set, this zman may not be available or cannot be calculated.
```
