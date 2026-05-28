# getSofZmanBiurChametzBaalHatanya

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getSofZmanBiurChametzBaalHatanya` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:3511)

```javadoc
This method returns the latest time for burning <em>chametz</em> on <em>Erev Pesach</em> according to the opinion of
the Baal Hatanya. This time is 5 hours into the day based on the opinion of the Baal Hatanya that the day is calculated
from sunrise to sunset. This returns the time 5 * {@link #getShaahZmanisBaalHatanya()} after
{@link #getSunriseBaalHatanya() <em>netz amiti</em> (sunrise)}. If it is not  <em>erev Pesach</em>, a <code>null</code> will
be returned.
@return the <code>Instant</code> of the latest time for burning <em>chametz</em> on <em>Erev Pesach</em>.  If it is not
        <em>erev Pesach</em> or the  calculation can't be computed such as in the Arctic Circle where there is at
        least one day a year where the sun does not rise, and one where it does not set, a <code>null</code> will be
        returned. See detailed explanation on top of the {@link AstronomicalCalendar} documentation.
@see #getShaahZmanisBaalHatanya()
@see #getSofZmanBiurChametz(Instant, Instant, boolean)
```

# Human docs

```markdown
```
