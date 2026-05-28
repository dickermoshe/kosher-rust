# getTzaisGeonim4Point8Degrees

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getTzaisGeonim4Point8Degrees` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:2402)

```javadoc
This method returns the <em>tzais</em> (nightfall) based on the opinion of the <em>Geonim</em> calculated as the
sun's position below the horizon at a time of 18.6 minutes after sunset. This is calculated as 3/4 of a 24-minute
<a href="https://en.wikipedia.org/wiki/Biblical_and_Talmudic_units_of_measurement">mil</a>, plus 0.6 minutes for the
time to walk 49 amos for <em>bain hashmashos</em> of Rav Yosi (with this <em>zman</em> based on a 24-minute mil, 49
amos would take 35.28 seconds to walk), for a total of 18.6 minutes after sunset. This calculates to the sun's position
at {@link ZENITH_4_POINT_8 4.8°} below the western horizon. This is based on <a href=
"https://he.wikipedia.org/wiki/%D7%99%D7%97%D7%99%D7%90%D7%9C_%D7%9E%D7%99%D7%9B%D7%9C_%D7%A9%D7%9C%D7%96%D7%99%D7%A0%D7%92%D7%A8"
>Rav Yechiel Michel Shlezinger's</a> <em>sefer</em> <a href="https://www.nli.org.il/he/books/NNL_ALEPH997010042055805171/NLI"
>Aizehu Bain Hashmashos</a>, <a href="https://en.wikipedia.org/wiki/Yehuda_(Leo)_Levi">Rabbi Yehuda (Leo) Levi's</a>
calculations in <a href="https://www.nli.org.il/en/items/NNL_ALEPH990022548970205171/NLI">Zmanei Hayom BaHalacha</a> p. 37.
At this point, 3 medium sized stars are visible to a non-expert with good vision with effort. An expert knowing where to
look can see the 3 medium stars as early as 15 minutes after sunset. This is explained in detail in Hazmanim Bahalacha vol
II, ch. 41, no. 6 (p. 372-373, ch. 47, no. 11-12 (p. 491-493) where it is clear that medium sized stars would be visible as
early as 14 minutes after sunset (13.5 minutes for 3/4 of an 18 minute Mil, plus 0.5 minutes for <em>bain Hashmashos</em>
of Rav Yosi) to an expert. See more details on this earier <em>zman</em> at {@link #getTzaisGeonim3Point8Degrees()}.
This is an early <em>zman</em> for <em>tzais</em>and should not be relied on without Rabbinical guidance.

@return the <code>Instant</code> representing the time when the sun is 4.8° below sea level. If the calculation
        can't be computed such as northern and southern locations even south of the Arctic Circle and north of
        the Antarctic Circle where the sun may not reach low enough below the horizon for this calculation, a
        <code>null</code> will be returned. See detailed explanation on top of the {@link AstronomicalCalendar}
        documentation.
@see ZENITH_4_POINT_8
```

# Human docs

```markdown
```
