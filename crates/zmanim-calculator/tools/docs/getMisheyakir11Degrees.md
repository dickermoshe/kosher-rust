# getMisheyakir11Degrees

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getMisheyakir11Degrees` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:966)

```javadoc
This method returns <em>misheyakir</em> based on the position of the sun when it is {@link ZENITH_11_DEGREES
11°} below {@link GEOMETRIC_ZENITH geometric zenith} (90°). This calculation is used for calculating
<em>misheyakir</em> according to some opinions. This calculation is based on the position of the sun 48 minutes
before {@link #getSunset() sunrise} in Jerusalem <a href=
"https://kosherjava.com/2022/01/12/equinox-vs-equilux-zmanim-calculations/">around the equinox / equilux</a>,
which calculates to 11° below {@link GEOMETRIC_ZENITH geometric zenith}.

@return If the calculation can't be computed such as northern and southern locations even south of the Arctic
        Circle and north of the Antarctic Circle where the sun may not reach low enough below the horizon for
        this calculation, a <code>null</code> will be returned. See detailed explanation on top of the
        {@link AstronomicalCalendar} documentation.
@see ZENITH_11_DEGREES
```

# Human docs

```markdown
Misheyakir according to some opinions.

The time when the sun is 11 degrees below the horizon before sunrise.

This is about 48 minutes before sunrise in Jerusalem around the equinox.

At some northern and southern locations, this zman may not be available if the sun does not reach low enough below the horizon.
```
