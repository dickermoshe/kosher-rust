# getTzais72Zmanis

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getTzais72Zmanis` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:2723)

```javadoc
Method to return <em>tzais</em> (dusk) calculated as 72 minutes zmaniyos, or 1/10th of the day after {@link
#getSeaLevelSunset() sea level sunset}. This is the way that the <a href=
"https://en.wikipedia.org/wiki/Abraham_Cohen_Pimentel">Minchas Cohen</a> in Ma'amar 2:4 calculates Rebbeinu Tam's
time of <em>tzeis</em>. It should be noted that this calculation results in the shortest time from sunset to
<em>tzais</em> being during the winter solstice, the longest at the summer solstice and 72 clock minutes at the
equinox. This does not match reality, since there is no direct relationship between the length of the day and
twilight. The shortest twilight is during the equinox, the longest is during the summer solstice, and in the
winter with the shortest daylight, the twilight period is longer than during the equinoxes.

@return the <code>Instant</code> representing the time. If the calculation can't be computed such as in the Arctic
        Circle where there is at least one day a year where the sun does not rise, and one where it does not set,
        a <code>null</code> will be returned. See detailed explanation on top of the {@link AstronomicalCalendar}
        documentation.
@see #getAlos72Zmanis()
```

# Human docs

```markdown
```
