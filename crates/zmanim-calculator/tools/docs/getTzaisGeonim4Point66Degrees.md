# getTzaisGeonim4Point66Degrees

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getTzaisGeonim4Point66Degrees` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:2351)

```javadoc
This method returns the <em>tzais</em> (nightfall) based on the opinion of the <em>Geonim</em> calculated as 3/4
of a <a href="https://en.wikipedia.org/wiki/Biblical_and_Talmudic_units_of_measurement">mil</a> based on a
24-minute mil, or 18 minutes. It is the sun's position at {@link ZENITH_4_POINT_66 4.66°} below the
western horizon. This is a very early <em>zman</em> and should not be relied on without Rabbinical guidance.
This does not cover the 35.28 seconds it takes to walk 49 amos (the <em>heref ayin</em> of <em>bain hashmashos</em>
of Rav Yosi) at the pace of a 24-minute mil. See {@link #getTzaisGeonim4Point8Degrees()} for a time that covers the
<em>heref ayin</em>.

@return the <code>Instant</code> representing the time when the sun is 4.66° below sea level. If the calculation
        can't be computed such as northern and southern locations even south of the Arctic Circle and north of
        the Antarctic Circle where the sun may not reach low enough below the horizon for this calculation, a
        <code>null</code> will be returned. See detailed explanation on top of the {@link AstronomicalCalendar}
        documentation.
@see ZENITH_4_POINT_66
@see #getTzaisGeonim4Point8Degrees()
```

# Human docs

```markdown
Tzais (nightfall) according to the Geonim - when the sun is 4.66 degrees below the western horizon after sunset.

Based on 3/4 of a 24-minute mil (18 minutes after sunset). This is a very early zman and should not be relied on without rabbinical guidance. Does not include the time to walk 49 amos for bain hashmashos of Rav Yosi.

At some northern and southern locations, this zman may not be available or cannot be calculated if the sun does not reach low enough below the horizon.
```
