# getTzais90Zmanis

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getTzais90Zmanis` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:2738)

```javadoc
Method to return <em>tzais</em> (dusk) calculated using 90 minutes zmaniyos or 1/8th of the day after {@link
#getSeaLevelSunset() sea level sunset}. This time is known in Yiddish as the <em>achtel</em> (an eighth)
<em>zman</em> used in various <em>kehilos</em>.

@return the <code>Instant</code> representing the time. If the calculation can't be computed such as in the Arctic
        Circle where there is at least one day a year where the sun does not rise, and one where it does not set,
        a <code>null</code> will be returned. See detailed explanation on top of the {@link AstronomicalCalendar}
        documentation.
@see #getAlos90Zmanis()
```

# Human docs

```markdown
Tzais (nightfall) - 90 zmaniyos minutes (1/8 of the day) after sea level sunset.

Known in Yiddish as the achtel zman, used in various kehilos.

In places such as the Arctic Circle, where there is at least one day a year when the sun does not rise and one when it does not set, this zman may not be available or cannot be calculated.
```
