# getTzaisGeonim3Point7Degrees

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getTzaisGeonim3Point7Degrees` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:2284)

```javadoc
This method returns the <em>tzais</em> (nightfall) based on the opinion of the <em>Geonim</em> calculated at the sun's
position at {@link ZENITH_3_POINT_7 3.7°} below {@link GEOMETRIC_ZENITH geometric zenith} (90°), calculated
as the position of the sun 13.5 minutes after sunset, the time it takes to walk 3/4 of a <a href=
"https://en.wikipedia.org/wiki/Biblical_and_Talmudic_units_of_measurement">mil</a> at 18 minutes a mil, or 13.5 minutes
 after sunset. The sun is 3.7° below {@link GEOMETRIC_ZENITH geometric zenith} at this time in Jerusalem <a href=
"https://kosherjava.com/2022/01/12/equinox-vs-equilux-zmanim-calculations/">around the equinox / equilux</a>. This does
not cover the 26.46 it takes to walk 49 amos (the <em>heref ayin</em> of <em>bain hashmashos</em> of Rav Yosi) at the pace
of an 18-minute mil. It should be noted that Rabbi Yedidya Manet in his <a href=
"https://www.nli.org.il/en/books/NNL_ALEPH002542826/NLI">Zmanei HaHalacha Lema'aseh</a> (4th edition part 2, pages and 22
and 24) lists 3.65° that appears to be a drop too early.

@return the <code>Instant</code> representing the time when the sun is 3.7° below sea level.
@see ZENITH_3_POINT_7
```

# Human docs

```markdown
Tzais (nightfall) according to the Geonim - when the sun is 3.7 degrees below the western horizon after sunset.

Corresponds to 13.5 minutes after sunset (3/4 of a mil at 18 minutes per mil) in Jerusalem [around the equinox/equilux](https://kosherjava.com/2022/01/12/equinox-vs-equilux-zmanim-calculations/). Does not include the time to walk 49 amos for bain hashmashos of Rav Yosi.
```
