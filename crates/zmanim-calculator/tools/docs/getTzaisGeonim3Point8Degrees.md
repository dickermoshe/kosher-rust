# getTzaisGeonim3Point8Degrees

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getTzaisGeonim3Point8Degrees` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:2302)

```javadoc
This method returns the <em>tzais</em> (nightfall) based on the opinion of the <em>Geonim</em> calculated at the sun's
position at {@link ZENITH_3_POINT_7 3.8°} below {@link GEOMETRIC_ZENITH geometric zenith} (90°), calculated
as the position of the sun 13.5 minutes after sunset, the time it takes to walk 3/4 of a <a href=
"https://en.wikipedia.org/wiki/Biblical_and_Talmudic_units_of_measurement">mil</a> at 18 minutes a mil, plus 30 seconds
for the time it takes to walk 49 amos (the <em>heref ayin</em> of <em>bain hashmashos</em> of Rav Yosi). With this being
on an 18-minutes mil, 49 amos would take 26.46, rounded to 30 seconds), for a total of 14 minutes after sunset. The sun is
{@link ZENITH_3_POINT_8 3.8°} below {@link GEOMETRIC_ZENITH geometric zenith} at this time in Jerusalem <a href=
"https://kosherjava.com/2022/01/12/equinox-vs-equilux-zmanim-calculations/">around the equinox / equilux</a>.

@return the <code>Instant</code> representing the time when the sun is 3.8° below sea level.
@see ZENITH_3_POINT_8
```

# Human docs

```markdown
Tzais (nightfall) according to the Geonim - when the sun is 3.8 degrees below the western horizon after sunset.

Corresponds to 14 minutes after sunset: 13.5 minutes for 3/4 of an 18-minute mil, plus 30 seconds for 49 amos (bain hashmashos of Rav Yosi), in Jerusalem [around the equinox/equilux](https://kosherjava.com/2022/01/12/equinox-vs-equilux-zmanim-calculations/).
```
