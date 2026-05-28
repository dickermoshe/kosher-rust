# getAlos16Point1Degrees

Source: `com.kosherjava.zmanim.ZmanimCalendar.getAlos16Point1Degrees` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ZmanimCalendar.java:321)

```javadoc
Returns <em>alos</em> (dawn) based on the time when the sun is {@link ZENITH_16_POINT_1 16.1°} below the eastern {@link
GEOMETRIC_ZENITH geometric horizon} before {@link getSunrise() sunrise}. This is based on the calculation that the time
between dawn and sunrise (and sunset to nightfall) is 72 minutes, the time that is takes to walk 4 <a href=
"https://en.wikipedia.org/wiki/Biblical_and_Talmudic_units_of_measurement">mil</a> at 18 minutes a mil (<a href=
"https://en.wikipedia.org/wiki/Maimonides">Rambam</a> and others). The sun's position below the horizon 72 minutes before
{@link getSunrise() sunrise} in Jerusalem on the <a href=
"https://kosherjava.com/2022/01/12/equinox-vs-equilux-zmanim-calculations/">around the equinox / equilux</a> is 16.1° below
{@link GEOMETRIC_ZENITH}.

@see ComprehensiveZmanimCalendar#getTzais16Point1Degrees()

@return The <code>Instant</code> of dawn. If the calculation can't be computed such as northern and southern
        locations even south of the Arctic Circle and north of the Antarctic Circle where the sun may not reach
        low enough below the horizon for this calculation, a <code>null</code> will be returned. See detailed
        explanation on top of the {@link AstronomicalCalendar} documentation.
```

# Human docs

```markdown
Alos (dawn), the beginning of morning twilight before sunrise.

The time when the sun is 16.1 degrees below the eastern geometric horizon before sunrise. This reflects the traditional view that the interval from dawn to sunrise, and from sunset to nightfall, is 72 minutes - the time to walk 4 [mil](https://en.wikipedia.org/wiki/Biblical_and_Talmudic_units_of_measurement) at 18 minutes per mil, as cited by [Rambam](https://en.wikipedia.org/wiki/Maimonides) and others. In Jerusalem [around the equinox or equilux](https://kosherjava.com/2022/01/12/equinox-vs-equilux-zmanim-calculations/), the sun is 16.1 degrees below the geometric horizon about 72 minutes before sunrise.

At some northern and southern locations, including places even south of the Arctic Circle and north of the Antarctic Circle, this zman may not be available or cannot be calculated if the sun does not reach low enough below the horizon.
```
