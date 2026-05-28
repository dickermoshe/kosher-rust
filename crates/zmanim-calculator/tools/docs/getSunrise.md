# getSunrise

Source: `com.kosherjava.zmanim.AstronomicalCalendar.getSunrise` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\AstronomicalCalendar.java:127)

```javadoc
The getSunrise method returns a <code>Instant</code> representing the {@link AstronomicalCalculator
#getElevationAdjustment(double) elevation adjusted} sunrise time. The zenith used for the calculation uses {@link
GEOMETRIC_ZENITH geometric zenith} of 90&deg; plus {@link AstronomicalCalculator#getElevationAdjustment(double)}. This is
adjusted by the {@link AstronomicalCalculator} to add approximately 50/60 of a degree to account for 34 archminutes of
refraction and 16 archminutes for the sun's radius for a total of {@link AstronomicalCalculator#adjustZenith 90.83333&deg;}.
See documentation for the specific implementation of the {@link AstronomicalCalculator} that you are using.

@return the <code>Instant</code> representing the exact sunrise time. If the calculation can't be computed such as in the
        Arctic Circle where there is at least one day a year where the sun does not rise, and one where it does not set, a
        <code>null</code> will be returned. See detailed explanation on top of the page.
@see AstronomicalCalculator#adjustZenith(double, double)
@see getSeaLevelSunrise()
@see getUTCSunrise(double)
```

# Human docs

```markdown
```
