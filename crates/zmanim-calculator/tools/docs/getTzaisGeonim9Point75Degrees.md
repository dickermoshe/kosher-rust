# getTzaisGeonim9Point75Degrees

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getTzaisGeonim9Point75Degrees` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:2515)

```javadoc
This method returns the <em>tzais</em> (nightfall) based on the opinion of the <em>Geonim</em> calculated as 60 minutes after
sunset <a href="https://kosherjava.com/2022/01/12/equinox-vs-equilux-zmanim-calculations/">around the equinox / equilux</a>,
the day that a solar hour is 60 minutes in New York. The sun's position at this time computes to {@link ZENITH_9_POINT_75
9.75°} below the western horizon. This is the opinion of <a href="https://en.wikipedia.org/wiki/Yosef_Eliyahu_Henkin"
>Rabbi Eliyahu Henkin</a>. This also follows the opinion of <a href="https://en.wikipedia.org/wiki/Shmuel_Kamenetsky">Rabbi
Shmuel Kamenetsky</a>. Rabbi Yaakov Shakow presented these degree-based times to Rabbi Shmuel Kamenetsky who agreed to them.

@todo recalculate based on equinox / equilux.
@return the <code>Instant</code> representing the time when the sun is 9.75° below sea level. If the calculation can't be
        computed such as northern and southern locations even south of the Arctic Circle and north of the Antarctic Circle
        where the sun may not reach low enough below the horizon for this calculation, a <code>null</code> will be returned.
        See detailed explanation on top of the {@link AstronomicalCalendar} documentation.

@see #getTzais60Minutes()
```

# Human docs

```markdown
```
