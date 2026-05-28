# getTzaisBaalHatanya

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getTzaisBaalHatanya` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:3601)

```javadoc
A method that returns <em>tzais</em> (nightfall) when the sun is 6° below the western geometric horizon (90°)
after {@link #getSunset() sunset}.  This <em>tzais</em> / nightfall based on the opinion of the  <a href=
"https://en.wikipedia.org/wiki/Shneur_Zalman_of_Liadi">Baal Hatanya</a>. This calculation is based on the position of the
sun about 24 minutes after {@link #getSeaLevelSunset() sunset} in Jerusalem <a href=
"https://kosherjava.com/2022/01/12/equinox-vs-equilux-zmanim-calculations/">around the equinox / equilux</a>, which
is 6° below {@link GEOMETRIC_ZENITH geometric zenith}. See <a href=
"https://www.chabad.org/library/article_cdo/aid/3209349/jewish/About-Our-Zmanim-Calculations.htm">About Our <em>Zmanim</em>
Calculations @ Chabad.org</a> that is based on {@link #getSunsetBaalHatanya() <em>shkiah amitis</em> as
1.583° below the horizon} calculated <a href="https://kosherjava.com/2022/01/12/equinox-vs-equilux-zmanim-calculations/"
>around the equinox / equilux</a> that computes 3.516 minutes after sunset. To this, 18 minutes of 3/4 of a 24-minute mil and
two minutes for <em>bain hashmashos</em> of Rav Yosi is added. This calculation computes the the sun being 5.83° below
the horizon (very close to the slightly later {@link #getTzaisGeonim5Point95Degrees()} that was calculated based on 4 fixed
minutes) and it is rounded up to 6°.

@return The <code>Instant</code> of nightfall. If the calculation can't be computed such as northern and southern
        locations even south of the Arctic Circle and north of the Antarctic Circle where the sun may not reach
        low enough below the horizon for this calculation, a <code>null</code> will be returned. See detailed
        explanation on top of the {@link AstronomicalCalendar} documentation.
@see #getTzaisGeonim5Point95Degrees()
@see #getSunsetBaalHatanya()
```

# Human docs

```markdown
```
