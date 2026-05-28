# getAlosBaalHatanya

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getAlosBaalHatanya` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:3440)

```javadoc
Returns the <a href="https://en.wikipedia.org/wiki/Shneur_Zalman_of_Liadi">Baal Hatanya</a>'s <em>alos</em>
(dawn) calculated as the time when the sun is 16.9° below the eastern {@link GEOMETRIC_ZENITH geometric horizon}
before {@link #getSunset() sunrise}. It is based on the calculation that the time between dawn and
<em>netz amiti</em> (sunrise) is 72 minutes, the time that is takes to walk 4 mil at 18 minutes
a mil (<a href="https://en.wikipedia.org/wiki/Maimonides">Rambam</a> and others). The sun's position at 72
minutes before {@link #getSunriseBaalHatanya <em>netz amiti</em> (sunrise)} in Jerusalem <a href=
"https://kosherjava.com/2022/01/12/equinox-vs-equilux-zmanim-calculations/">around the equinox / equilux</a> is
16.9° below {@link GEOMETRIC_ZENITH geometric zenith}.

@return The <code>Instant</code> of dawn. If the calculation can't be computed such as northern and southern
        locations even south of the Arctic Circle and north of the Antarctic Circle where the sun may not reach
        low enough below the horizon for this calculation, a <code>null</code> will be returned. See detailed
        explanation on top of the {@link AstronomicalCalendar} documentation.
```

# Human docs

```markdown
Alos (dawn) according to the [Baal Hatanya](https://en.wikipedia.org/wiki/Shneur_Zalman_of_Liadi) - the beginning of morning twilight before netz amiti (true sunrise).

The time when the sun is 16.9 degrees below the eastern geometric horizon before sunrise. Based on the view that the interval from dawn to netz amiti is 72 minutes - the time to walk 4 [mil](https://en.wikipedia.org/wiki/Biblical_and_Talmudic_units_of_measurement) at 18 minutes per mil, as cited by [Rambam](https://en.wikipedia.org/wiki/Maimonides) and others. In Jerusalem [around the equinox or equilux](https://kosherjava.com/2022/01/12/equinox-vs-equilux-zmanim-calculations/), the sun is 16.9 degrees below geometric zenith about 72 minutes before Baal Hatanya sunrise.

At some northern and southern locations, including places even south of the Arctic Circle and north of the Antarctic Circle, this zman may not be available or cannot be calculated if the sun does not reach low enough below the horizon.
```
