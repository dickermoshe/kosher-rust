# getSeaLevelSunrise

Source: `com.kosherjava.zmanim.AstronomicalCalendar.getSeaLevelSunrise` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\AstronomicalCalendar.java:149)

```javadoc
A method that returns the sunrise without {@link AstronomicalCalculator#getElevationAdjustment(double) elevation
adjustment}. Non-sunrise and sunset calculations such as dawn and dusk, depend on the amount of visible light,
something that is not affected by elevation. This method returns sunrise calculated at sea level. This forms the
base for dawn calculations that are calculated as a dip below the horizon before sunrise.

@return the <code>Instant</code> representing the exact sea-level sunrise time. If the calculation can't be computed
        such as in the Arctic Circle where there is at least one day a year where the sun does not rise, and one
        where it does not set, a <code>null</code> will be returned. See detailed explanation on top of the page.
@see getSunrise()
@see getUTCSeaLevelSunrise(double)
@see getSeaLevelSunset()
```

# Human docs

```markdown
```
