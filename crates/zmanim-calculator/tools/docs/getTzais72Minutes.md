# getTzais72Minutes

Source: `com.kosherjava.zmanim.ZmanimCalendar.getTzais72Minutes` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ZmanimCalendar.java:609)

```javadoc
This method returns the <em>tzais</em> (nightfall) based on the opinion of <em>Rabbeinu Tam</em> that
<em>tzais hakochavim</em> is calculated as 72 minutes after sunset, the time it takes to walk 4 <a href=
"https://en.wikipedia.org/wiki/Biblical_and_Talmudic_units_of_measurement">mil</a> at 18 minutes a mil.
According to the <a href="https://en.wikipedia.org/wiki/Samuel_Loew">Machtzis Hashekel</a> in Orach Chaim
235:3, the <a href="https://en.wikipedia.org/wiki/Joseph_ben_Meir_Teomim">Pri Megadim</a> in Orach
Chaim 261:2 (see the Biur Halacha) and others (see Hazmanim Bahalacha 17:3 and 17:5) the 72 minutes are standard
clock minutes any time of the year in any location. Depending on the {@link isUseElevation()} setting, a 72-minute
offset from  either {@link getSunset() sunset} or {@link getSeaLevelSunset() sea level sunset} is used.

@return the <code>Instant</code> representing 72 minutes after sunset. If the calculation can't be
        computed such as in the Arctic Circle where there is at least one day a year where the sun does not rise,
        and one where it does not set, a <code>null</code> will be returned See detailed explanation on top of the
        {@link AstronomicalCalendar} documentation.
@see ComprehensiveZmanimCalendar#getTzais16Point1Degrees()
```

# Human docs

```markdown
```
