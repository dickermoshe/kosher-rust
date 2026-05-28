# getTzaisGeonim5Point95Degrees

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getTzaisGeonim5Point95Degrees` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:2329)

```javadoc
This method returns the <em>tzais</em> (nightfall) based on the opinion of the <em>Geonim</em> calculated at the
sun's position at {@link ZENITH_5_POINT_95 5.95°} below below {@link GEOMETRIC_ZENITH geometric zenith}
(90°), calculated as the position of the sun 24 minutes after sunset in Jerusalem <a href=
"https://kosherjava.com/2022/01/12/equinox-vs-equilux-zmanim-calculations/">around the equinox / equilux</a>. The
24 minutes is based on the Baal Hatanya's calculation of 18 minutes (3/4 of a 24 minute mil) + 4 minutes for
<em>shkiah amitis</em> + 2 minutes for bain hashmashos of Rav Yosi. See Hazmanim Bahalacha vol II, ch. 50, no. 5,
p. 512-513, ch. 47, and Yisrael Vehazmanim <a href="https://hebrewbooks.org/pdfpager.aspx?req=9764&st=&pgnum=266"
>Vol III, ch. 13, no. 53, p. 1026</a>. Among sources he mentions for this <em>zman</em> is <a href=
"https://en.wikipedia.org/wiki/Yehuda_(Leo)_Levi">Rabbi Yehuda (Leo) Levi's</a> calculations in Jewish Chrononomy
and other sources. Calculations show that the time is closer to 5.93° and was seemingly rounded to 5.95°.
Chabad calendars usually use the 6°-based {@link #getTzaisBaalHatanya()} that is built on this same calculation.
It should be noted that Rabbi Yedidya Manet in his <a href="https://www.nli.org.il/en/books/NNL_ALEPH002542826/NLI"
>Zmanei HaHalacha Lema'aseh</a> (4th edition part 2, pages and 22 and 24) lists 5.88° that appears to be a drop
too early.

@return the <code>Instant</code> representing the time when the sun is 5.95° below sea level. If the calculation
        can't be computed such as northern and southern locations even south of the Arctic Circle and north of
        the Antarctic Circle where the sun may not reach low enough below the horizon for this calculation, a
        <code>null</code> will be returned. See detailed explanation on top of the {@link AstronomicalCalendar}
        documentation.
@see #getTzaisBaalHatanya()
```

# Human docs

```markdown
```
