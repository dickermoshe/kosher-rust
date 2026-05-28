# getSofZmanBiurChametzGRA

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getSofZmanBiurChametzGRA` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:3331)

```javadoc
This method returns the latest time for burning <em>chametz</em> on <em>Erev Pesach</em> according to the opinion
of the <a href="https://en.wikipedia.org/wiki/Vilna_Gaon">GRA</a>. This time is 5 hours into the day based on the
opinion of the <a href="https://en.wikipedia.org/wiki/Vilna_Gaon">GRA</a> that the day is calculated from
sunrise to sunset. This returns the time 5 * {@link #getShaahZmanisGRA()} after {@link #getSeaLevelSunrise() sea
level sunrise}. If it is not  <em>erev Pesach</em>, a <code>null</code> will be returned.
@return the <code>Instant</code> of the latest time for burning <em>chametz</em> on <em>Erev Pesach</em>. If it is not
        <em>erev Pesach</em> or the calculation can't be computed such as in the Arctic Circle where there is at least
        one day a year where the sun does not rise, and one where it does not set, a <code>null</code> will be
        returned. See detailed explanation on top of the {@link AstronomicalCalendar} documentation.
@see #getShaahZmanisGRA()
@see #getSofZmanBiurChametz(Instant, Instant, boolean)
```

# Human docs

```markdown
Sof zman biur chametz - the latest time to burn chametz on Erev Pesach according to the [GRA](https://en.wikipedia.org/wiki/Vilna_Gaon).

5 shaos zmaniyos after sea level sunrise, with the day measured from sunrise to sunset.
```
