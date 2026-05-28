# getAlos18Degrees

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getAlos18Degrees` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:850)

```javadoc
A method to return <em>alos</em> (dawn) calculated when the sun is {@link ASTRONOMICAL_ZENITH 18°} below the
eastern geometric horizon before sunrise.

@return the <code>Instant</code> representing <em>alos</em>. If the calculation can't be computed such as northern
        and southern locations even south of the Arctic Circle and north of the Antarctic Circle where the sun
        may not reach low enough below the horizon for this calculation, a <code>null</code> will be returned.
        See detailed explanation on top of the {@link AstronomicalCalendar} documentation.
@see ASTRONOMICAL_ZENITH
```

# Human docs

```markdown
Alos (dawn), the beginning of morning twilight before sunrise.

The time when the sun is 18 degrees below the eastern horizon before sunrise.

At some northern and southern locations, this zman may not be available if the sun does not reach low enough below the horizon.
```
