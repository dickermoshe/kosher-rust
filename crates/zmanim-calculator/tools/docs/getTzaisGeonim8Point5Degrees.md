# getTzaisGeonim8Point5Degrees

Source: `com.kosherjava.zmanim.ZmanimCalendar.getTzaisGeonim8Point5Degrees` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ZmanimCalendar.java:300)

```javadoc
A method that returns <em>tzais</em> (nightfall) when the sun is {@link ZENITH_8_POINT_5 8.5°} below the {@link
GEOMETRIC_ZENITH geometric horizon} (90°) after {@link getSunset() sunset}, a time that Rabbi Meir Posen in his the <em><a
href="https://www.worldcat.org/oclc/29283612">Ohr Meir</a></em> calculated that 3 small stars are visible, which is later than
the required 3 medium stars. This calculation is based on the sun's position below the horizon 36 minutes after {@link
getSeaLevelSunrise() sunset} in Jerusalem <a href="https://kosherjava.com/2022/01/12/equinox-vs-equilux-zmanim-calculations/"
>around the equinox / equilux</a>, which is 8.5° below {@link GEOMETRIC_ZENITH geometric zenith}.

@return The <code>Instant</code> of nightfall. If the calculation can't be computed such as northern and southern locations
        even south of the Arctic Circle and north of the Antarctic Circle where the sun may not reach low enough below the
        horizon for this calculation, a <code>null</code> will be returned. See detailed explanation on top of the {@link
        AstronomicalCalendar} documentation.
ComprehensiveZmanimCalendar#getTzaisGeonim8Point5Degrees() that returns an identical time to this generic <em>tzais</em>
```

# Human docs

```markdown
Tzais (nightfall) according to the Geonim - when the sun is 8.5 degrees below the western horizon after sunset.

Based on Rabbi Meir Posen's [Ohr Meir](https://www.worldcat.org/oclc/29283612) calculation for when 3 small stars are visible, which is later than the required 3 medium stars. About 36 minutes after sunset in Jerusalem [around the equinox/equilux](https://kosherjava.com/2022/01/12/equinox-vs-equilux-zmanim-calculations/).

At some northern and southern locations, this zman may not be available or cannot be calculated if the sun does not reach low enough below the horizon.
```
