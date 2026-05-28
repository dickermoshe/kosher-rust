# getSofZmanAchilasChametzBaalHatanya

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getSofZmanAchilasChametzBaalHatanya` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:3494)

```javadoc
This method returns the latest time one is allowed eating <em>chametz</em> on <em>Erev Pesach</em> according to the
opinion of the Baal Hatanya. This time is identical to the {@link #getSofZmanTfilaBaalHatanya() <em>Sof zman
tfilah</em> Baal Hatanya}. This time is 4 hours into the day based on the opinion of the Baal Hatanya that the day
is calculated from sunrise to sunset. This returns the time 4 {@link #getShaahZmanisBaalHatanya()} after
{@link #getSunriseBaalHatanya() <em>netz amiti</em> (sunrise)}. If it is not  <em>erev Pesach</em>, a <code>null</code> will
be returned.
@return the <code>Instant</code> one is allowed eating <em>chametz</em> on <em>Erev Pesach</em>. If it is not <em>erev
        Pesach</em> or the  calculation can't be computed such as in the Arctic Circle where there is at least one
        day a year where the sun does not rise, and one where it does not set, a <code>null</code> will be returned.
        See detailed explanation on top of the {@link AstronomicalCalendar} documentation.
@see #getShaahZmanisBaalHatanya()
@see #getSofZmanTfilaBaalHatanya()
@see #getSofZmanAchilasChametz(Instant, Instant, boolean)
```

# Human docs

```markdown
Sof zman achilas chametz - the latest time to eat chametz on Erev Pesach according to the Baal Hatanya. Same as sof zman tfila Baal Hatanya.

4 shaos zmaniyos after netz amiti (true sunrise), with the day measured from Baal Hatanya sunrise to sunset.
```
