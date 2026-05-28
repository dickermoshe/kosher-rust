# getAlos26Degrees

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getAlos26Degrees` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:836)

```javadoc
This method should be used <em>lechumra</em> only and returns <em>alos</em> (dawn) calculated when the sun is {@link
ZENITH_26_DEGREES 26°} below the eastern geometric horizon before sunrise. This calculation is based on the same
calculation of {@link #getAlos120Minutes() 120 minutes} but uses a degree-based calculation instead of 120 exact minutes.
This calculation is based on the position of the sun 120 minutes before sunrise in Jerusalem <a href=
"https://kosherjava.com/2022/01/12/equinox-vs-equilux-zmanim-calculations/">around the equinox / equilux</a>, which
calculates to 26° below {@link GEOMETRIC_ZENITH geometric zenith}. Since this time is extremely early, it should
only be used <em>lechumra</em> only, such as not eating after this time on a fast day, and not as the start time for
<em>mitzvos</em> that can only be performed during the day.

@deprecated This method should be used <em>lechumra</em> only (such as stopping to eat at this time on a fast day),
        since it returns a very early time, and if used <em>lekula</em> can result in doing <em>mitzvos hayom</em>
        too early according to most opinions. There is no current plan to remove this  method from the API, and this
        deprecation is intended to alert developers of the danger of using it.
@return the <code>Instant</code> representing <em>alos</em>. If the calculation can't be computed such as northern
        and southern locations even south of the Arctic Circle and north of the Antarctic Circle where the sun
        may not reach low enough below the horizon for this calculation, a <code>null</code> will be returned. See
        detailed explanation on top of the {@link AstronomicalCalendar} documentation.
@see #getAlos120Minutes()
@see #getTzais120Minutes()
@see #getTzais26Degrees()
```

# Human docs

```markdown
Alos (dawn), an extremely early time that should be used lechumra only - for example, stopping to eat on a fast day - and not as the start of daytime mitzvos.

The time when the sun is 26 degrees below the eastern geometric horizon before sunrise. This is the degree-based equivalent of alos 120 minutes before sunrise: in Jerusalem [around the equinox or equilux](https://kosherjava.com/2022/01/12/equinox-vs-equilux-zmanim-calculations/), the sun is 26 degrees below geometric zenith about 120 minutes before sunrise. Using this zman leniently can lead to performing daytime mitzvos too early according to most opinions.

At some northern and southern locations, including places even south of the Arctic Circle and north of the Antarctic Circle, this zman may not be available or cannot be calculated if the sun does not reach low enough below the horizon.
```
