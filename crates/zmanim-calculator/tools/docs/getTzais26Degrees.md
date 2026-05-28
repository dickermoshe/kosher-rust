# getTzais26Degrees

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getTzais26Degrees` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:2869)

```javadoc
This method should be used <em>lechumra</em> only and returns <em>tzais</em> based on when the sun is 26°
below the horizon. For information on how this is calculated see the comments on {@link #getAlos26Degrees()}.
Since the <em>zman</em> is extremely late and at a point when it is long past the 18° point where the
darkest point is reached, it should only be used <em>lechumra</em> such as delaying the start of nighttime
<em>mitzvos</em>.

@deprecated This method should be used <em>lechumra</em> only since it returns a very late time, and if used
        <em>lekula</em> can result in <em>chillul Shabbos</em> etc. There is no current plan to remove this
        method from the API, and this deprecation is intended to alert developers of the danger of using it.

@return the <code>Instant</code> representing the time. If the calculation can't be computed such as northern and
        southern locations even south of the Arctic Circle and north of the Antarctic Circle where the sun may
        not reach low enough below the horizon for this calculation, a <code>null</code> will be returned. See
        detailed explanation on top of the {@link AstronomicalCalendar} documentation.
@see #getTzais120Minutes()
@see #getAlos26Degrees()
```

# Human docs

```markdown
```
