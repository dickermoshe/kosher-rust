# getTchilasZmanKidushLevana7Days

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getTchilasZmanKidushLevana7Days` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:3224)

```javadoc
Returns the earliest time of <em>Kiddush Levana</em> according to the opinions that it should not be said until 7
days after the <em>molad</em>. The time will be returned even if it occurs during the day when <em>Kiddush Levana</em>
can't be recited. Use {@link #getTchilasZmanKidushLevana7Days(Instant, Instant)} if you want to limit the time to night hours.

@return the Instant representing the moment 7 days after the molad regardless of it is day or night. If the <em>zman</em>
        will not occur on this day, a <code>null</code> will be returned.
@see #getTchilasZmanKidushLevana7Days(Instant, Instant)
@see JewishCalendar#getTchilasZmanKidushLevana7Days()
@see #getTchilasZmanKidushLevana3Days()
```

# Human docs

```markdown
The earliest time for Kiddush Levana according to the opinion that it should not be said until 7 days after the molad.

Note that although this time may be during the daytime, Kiddush Levana cannot be said during the daytime.
```
