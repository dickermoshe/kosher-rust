# getSeaLevelSunset

Source: `com.kosherjava.zmanim.AstronomicalCalendar.getSeaLevelSunset` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\AstronomicalCalendar.java:231)

```javadoc
A method that returns the sunset without {@link AstronomicalCalculator#getElevationAdjustment(double) elevation adjustment}.
Non-sunrise and sunset calculations such as dawn and dusk, depend on the amount of visible light, something that is not
affected by elevation. This method returns sunset calculated at sea level. This forms the base for dusk calculations that are
calculated as a dip below the horizon after sunset.

@return the <code>Instant</code> representing the exact sea-level sunset time. If the calculation can't be computed
        such as in the Arctic Circle where there is at least one day a year where the sun does not rise, and one
        where it does not set, a <code>null</code> will be returned. See detailed explanation on top of the page.
@see getSunset()
@see getUTCSeaLevelSunset(double)
```

# Human docs

```markdown
```
