# getTchilasZmanKidushLevana3Days

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getTchilasZmanKidushLevana3Days` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:3101)

```javadoc
Returns the earliest time of <em>Kiddush Levana</em> according to <a href=
"https://en.wikipedia.org/wiki/Yonah_Gerondi">Rabbeinu Yonah</a>'s opinion that it can be said 3 days after the
<em>molad</em>. The time will be returned even if it occurs during the day when <em>Kiddush Levana</em> can't be said.
Use {@link #getTchilasZmanKidushLevana3Days(Instant, Instant)} if you want to limit the time to night hours.

@return the Instant representing the moment 3 days after the molad. If the <em>zman</em> will not occur on this day, a
        <code>null</code> will be returned.
@see #getTchilasZmanKidushLevana3Days(Instant, Instant)
@see #getTchilasZmanKidushLevana7Days()
@see JewishCalendar#getTchilasZmanKidushLevana3Days()
```

# Human docs

```markdown
```
