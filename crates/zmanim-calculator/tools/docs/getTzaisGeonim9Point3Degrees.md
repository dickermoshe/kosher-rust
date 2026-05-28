# getTzaisGeonim9Point3Degrees

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getTzaisGeonim9Point3Degrees` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:2495)

```javadoc
This method returns the <em>tzais</em> (nightfall) based on the calculations used in the <a href=
"https://www.worldcat.org/oclc/243303103">Luach Itim Lebinah</a> as the stringent time for <em>tzais</em>. It is calculated
as the sun's position at {@link ZENITH_9_POINT_3 9.3°} below the western horizon.

@return the <code>Instant</code> representing the time when the sun is 9.3° below sea level. If the calculation can't be
        computed such as northern and southern locations even south of the Arctic Circle and north of the Antarctic Circle
        where the sun may not reach low enough below the horizon for this calculation, a <code>null</code> will be returned.
        See detailed explanation on top of the {@link AstronomicalCalendar} documentation.
```

# Human docs

```markdown
Tzais (nightfall) according to the Geonim - when the sun is 9.3 degrees below the western horizon after sunset.

The stringent tzais used in [Luach Itim Lebinah](https://www.worldcat.org/oclc/243303103).

At some northern and southern locations, this zman may not be available or cannot be calculated if the sun does not reach low enough below the horizon.
```
