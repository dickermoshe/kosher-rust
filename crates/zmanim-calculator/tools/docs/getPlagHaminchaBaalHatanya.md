# getPlagHaminchaBaalHatanya

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getPlagHaminchaBaalHatanya` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:3575)

```javadoc
This method returns the time of <em>plag hamincha</em>. This is calculated as 10.75 hours after sunrise. This calculation
is based on the opinion of the Baal Hatanya that the day is calculated from sunrise to sunset. This returns the time
10.75 * {@link #getShaahZmanisBaalHatanya()} after {@link #getSunriseBaalHatanya() <em>netz amiti</em> (sunrise)}. See
<a href="https://www.chabad.org/library/article_cdo/aid/3209349/jewish/About-Our-Zmanim-Calculations.htm">About Our
<em>Zmanim</em> Calculations @ Chabad.org</a> for more details on this calculation.

@see #getPlagHamincha(Instant, Instant)
@return the <code>Instant</code> of the time of <em>plag hamincha</em>. If the calculation can't be computed such as
        in the Arctic Circle where there is at least one day a year where the sun does not rise, and one where it
        does not set, a <code>null</code> will be returned. See detailed explanation on top of the
        {@link AstronomicalCalendar} documentation.
```

# Human docs

```markdown
Plag hamincha according to the Baal Hatanya.

10.75 shaos zmaniyos after netz amiti, using a day from Baal Hatanya sunrise to sunset.

In places where sunrise or sunset cannot be calculated, this zman may not be available.

See [About Our Zmanim Calculations @ Chabad.org](https://www.chabad.org/library/article_cdo/aid/3209349/jewish/About-Our-Zmanim-Calculations.htm).
```
