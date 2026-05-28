# getAlos72Minutes

Source: `com.kosherjava.zmanim.ZmanimCalendar.getAlos72Minutes` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ZmanimCalendar.java:339)

```javadoc
Method to return <em>alos</em> (dawn) calculated as 72 minutes before {@link getSunrise() sunrise} or
{@link getSeaLevelSunrise() sea level sunrise} (depending on the {@link isUseElevation()} setting). This time
is based on the time to walk the distance of 4 <a href=
"https://en.wikipedia.org/wiki/Biblical_and_Talmudic_units_of_measurement">mil</a> at 18 minutes a mil. The
72-minute time (but not the concept of fixed minutes) is based on the opinion that the time of the <em>Neshef</em>
(twilight between dawn and sunrise) does not vary by the time of year or location but depends on the time it takes
to walk the distance of 4 mil.

@return the <code>Instant</code> representing the time. If the calculation can't be computed such as in the Arctic
        Circle where there is at least one day a year where the sun does not rise, and one where it does not set,
        a <code>null</code> will be returned. See detailed explanation on top of the {@link AstronomicalCalendar}
        documentation.
```

# Human docs

```markdown
Alos (dawn), the beginning of morning twilight before sunrise.

72 minutes before sunrise. {uses_elevation} Based on the time to walk 4 [mil](https://en.wikipedia.org/wiki/Biblical_and_Talmudic_units_of_measurement) at 18 minutes per mil. The fixed 72-minute interval (not the general idea of measuring by walking time) reflects the view that the neshef period between dawn and sunrise does not vary by season or location, but depends on the time to walk 4 mil.

In places such as the Arctic Circle, where there is at least one day a year when the sun does not rise and one when it does not set, this zman may not be available or cannot be calculated.
```
