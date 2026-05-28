# getTzais19Point8Degrees

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getTzais19Point8Degrees` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:2896)

```javadoc
For information on how this is calculated see the comments on {@link #getAlos19Point8Degrees()}.

@return the <code>Instant</code> representing the time. If the calculation can't be computed such as northern and
        southern locations even south of the Arctic Circle and north of the Antarctic Circle where the sun may
        not reach low enough below the horizon for this calculation, a <code>null</code> will be returned. See
        detailed explanation on top of the {@link AstronomicalCalendar} documentation.
@see #getTzais90Minutes()
@see #getAlos19Point8Degrees()
```

# Human docs

```markdown
Tzais (nightfall) - when the sun is 19.8 degrees below the western horizon after sunset.

Degree-based calculation corresponding to 90 minutes after sunset [around the equinox/equilux](https://kosherjava.com/2022/01/12/equinox-vs-equilux-zmanim-calculations/) in Jerusalem.

At some northern and southern locations, this zman may not be available or cannot be calculated if the sun does not reach low enough below the horizon.
```
