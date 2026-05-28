# getSofZmanTfilaBaalHatanya

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getSofZmanTfilaBaalHatanya` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:3475)

```javadoc
This method returns the latest <em>zman tfilah</em> (time to recite the morning prayers). This time is 4
hours into the day based on the opinion of the Baal Hatanya that the day is
calculated from sunrise to sunset. This returns the time 4 * {@link #getShaahZmanisBaalHatanya()} after
{@link #getSunriseBaalHatanya() <em>netz amiti</em> (sunrise)}.

@see #getSofZmanTfila(Instant, Instant)
@see #getShaahZmanisBaalHatanya()
@return the <code>Instant</code> of the latest <em>zman tfilah</em>. If the calculation can't be computed such as in
        the Arctic Circle where there is at least one day a year where the sun does not rise, and one where it does
        not set, a <code>null</code> will be returned. See detailed explanation on top of the
        {@link AstronomicalCalendar} documentation.
```

# Human docs

```markdown
Sof zman tfila - the latest time to recite morning prayers (Shacharis) according to the Baal Hatanya.

4 shaos zmaniyos after netz amiti (true sunrise), with the day measured from Baal Hatanya sunrise to sunset.

In places such as the Arctic Circle, where there is at least one day a year when the sun does not rise and one when it does not set, this zman may not be available or cannot be calculated.
```
