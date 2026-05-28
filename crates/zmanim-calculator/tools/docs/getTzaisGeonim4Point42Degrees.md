# getTzaisGeonim4Point42Degrees

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getTzaisGeonim4Point42Degrees` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:2373)

```javadoc
This method returns the <em>tzais</em> (nightfall) based on the opinion of the <em>Geonim</em> calculated as 3/4
of a <a href="https://en.wikipedia.org/wiki/Biblical_and_Talmudic_units_of_measurement">mil</a>, based on a
22.5-minute mil, or 16 7/8 minutes. It is the sun's position at {@link ZENITH_4_POINT_42 4.42°} below the western
horizon. This is a very early <em>zman</em> and should not be relied on without Rabbinical guidance. This does
not cover the 33.07 seconds it takes to walk 49 amos (the <em>heref ayin</em> of <em>bain hashmashos</em> of Rav Yosi)
at the pace of a 22.5 minute-mil. It should be noted that Rabbi Yedidya Manet in his <a href=
"https://www.nli.org.il/en/books/NNL_ALEPH002542826/NLI">Zmanei HaHalacha Lema'aseh</a> (4th edition part 2, pages and 22
and 24) lists 4.37° that appears to be a drop too early.

@return the <code>Instant</code> representing the time when the sun is 4.42° below sea level. If the calculation
        can't be computed such as northern and southern locations even south of the Arctic Circle and north of
        the Antarctic Circle where the sun may not reach low enough below the horizon for this calculation, a
        <code>null</code> will be returned. See detailed explanation on top of the {@link AstronomicalCalendar}
        documentation.
@see ZENITH_4_POINT_42
```

# Human docs

```markdown
```
