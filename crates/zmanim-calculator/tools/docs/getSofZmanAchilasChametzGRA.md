# getSofZmanAchilasChametzGRA

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getSofZmanAchilasChametzGRA` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:3244)

```javadoc
This method returns the latest time one is allowed eating <em>chametz</em> on <em>Erev Pesach</em> according to
the opinion of the <a href="https://en.wikipedia.org/wiki/Vilna_Gaon">GRA</a>. This time is identical to the {@link
#getSofZmanTfilaGRA() <em>Sof zman tfilah</em> GRA} and is provided as a convenience method for those who are
unaware how this <em>zman</em> is calculated. This time is 4 hours into the day based on the opinion of the
<a href="https://en.wikipedia.org/wiki/Vilna_Gaon">GRA</a> that the day is calculated from sunrise to sunset.
This returns the time 4 * {@link #getShaahZmanisGRA()} after {@link #getSeaLevelSunrise() sea level sunrise}. If it
is not <em>erev Pesach</em>, a <code>null</code> will be returned.
@return the <code>Instant</code> one is allowed eating <em>chametz</em> on <em>Erev Pesach</em>. If it is not <em>erev
        Pesach</em> or the calculation can't be computed such as in the Arctic Circle where there is at least one
        day a year where the sun does not rise, and one where it does not set, a <code>null</code> will be returned.
        See detailed explanation on top of the {@link AstronomicalCalendar} documentation.
@see #getShaahZmanisGRA()
@see #getSofZmanTfilaGRA()
@see #getSofZmanAchilasChametz(Instant, Instant, boolean)
```

# Human docs

```markdown
Sof zman achilas chametz - the latest time to eat chametz on Erev Pesach according to the [GRA](https://en.wikipedia.org/wiki/Vilna_Gaon). Same as sof zman tfila GRA.

4 shaos zmaniyos after sea level sunrise, with the day measured from sunrise to sunset.
```
