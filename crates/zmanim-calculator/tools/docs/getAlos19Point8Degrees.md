# getAlos19Point8Degrees

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getAlos19Point8Degrees` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:887)

```javadoc
Method to return <em>alos</em> (dawn) calculated when the sun is {@link ZENITH_19_POINT_8 19.8°} below the
eastern geometric horizon before sunrise. This calculation is based on the same calculation of
{@link #getAlos90Minutes() 90 minutes} before sunrise, but uses a degree-based calculation instead of 90 exact minutes.
This calculation is based on the position of the sun 90 minutes before sunrise in Jerusalem <a href=
"https://kosherjava.com/2022/01/12/equinox-vs-equilux-zmanim-calculations/">around the equinox / equilux</a>, which
calculates to 19.8° below {@link GEOMETRIC_ZENITH geometric zenith}.

@return the <code>Instant</code> representing <em>alos</em>. If the calculation can't be computed such as northern
        and southern locations even south of the Arctic Circle and north of the Antarctic Circle where the sun
        may not reach low enough below the horizon for this calculation, a <code>null</code> will be returned. See
        detailed explanation on top of the {@link AstronomicalCalendar} documentation.
@see #getAlos90Minutes()
@see #getTzais19Point8Degrees()
```

# Human docs

```markdown
Alos (dawn), the beginning of morning twilight before sunrise.

The time when the sun is 19.8 degrees below the eastern geometric horizon before sunrise. This is the degree-based equivalent of alos 90 minutes before sunrise: in Jerusalem [around the equinox or equilux](https://kosherjava.com/2022/01/12/equinox-vs-equilux-zmanim-calculations/), the sun is 19.8 degrees below geometric zenith about 90 minutes before sunrise.

At some northern and southern locations, including places even south of the Arctic Circle and north of the Antarctic Circle, this zman may not be available or cannot be calculated if the sun does not reach low enough below the horizon.
```
