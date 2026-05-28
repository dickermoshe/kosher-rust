# getSunset

Source: `com.kosherjava.zmanim.AstronomicalCalendar.getSunset` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\AstronomicalCalendar.java:210)

```javadoc
The getSunset method returns an <code>Instant</code> representing the
{@link AstronomicalCalculator#getElevationAdjustment(double) elevation adjusted} sunset time. The zenith used for the
calculation uses {@link GEOMETRIC_ZENITH geometric zenith} of 90&deg; plus {@link AstronomicalCalculator
#getElevationAdjustment(double)}. This is adjusted by the {@link AstronomicalCalculator} to add approximately 50/60 of a
degree to account for 34 archminutes of refraction and 16 archminutes for the sun's radius for a total of {@link
AstronomicalCalculator#adjustZenith(double, double) 90.83333&deg;}. See documentation for the specific implementation of the
{@link AstronomicalCalculator} that you are using.
Note: In certain cases the calculates sunset will occur before sunrise. This will typically happen when a time zone other than
the local timezone is used (calculating Los Angeles sunset using a GMT time zone for example). In this case the sunset date
will be incremented to the following date.

@return the <code>Instant</code> representing the exact sunset time. If the calculation can't be computed such as in the Arctic
        Circle where there is at least one day a year where the sun does not rise, and one where it does not set, a
        <code>null</code> will be returned. See detailed explanation on top of the page.
@see AstronomicalCalculator#adjustZenith(double, double)
@see getSeaLevelSunset()
@see getUTCSunset(double)
```

# Human docs

```markdown
Sunset, adjusted for the location's elevation.

The time when the upper edge of the sun disappears below the horizon, accounting for atmospheric refraction and the sun's radius.

In places such as the Arctic Circle, where there is at least one day a year when the sun does not rise and one when it does not set, this zman may not be available or cannot be calculated.
```
