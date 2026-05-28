# getSofZmanShmaBaalHatanya

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getSofZmanShmaBaalHatanya` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:3458)

```javadoc
This method returns the latest <em>zman krias shema</em> (time to recite Shema in the morning). This time is 3
{@link #getShaahZmanisBaalHatanya() <em>shaos zmaniyos</em>} (solar hours) after {@link #getSunriseBaalHatanya()
<em>netz amiti</em> (sunrise)} based on the opinion of the Baal Hatanya that the day is calculated from
sunrise to sunset. This returns the time 3 * {@link #getShaahZmanisBaalHatanya()} after {@link #getSunriseBaalHatanya()
<em>netz amiti</em> (sunrise)}.

@see #getSofZmanShma(Instant, Instant)
@see #getShaahZmanisBaalHatanya()
@return the <code>Instant</code> of the latest <em>zman shema</em> according to the Baal Hatanya. If the calculation
        can't be computed such as in the Arctic Circle where there is at least one day a year where the sun does
        not rise, and one where it does not set, a <code>null</code> will be returned. See detailed explanation on
        top of the {@link AstronomicalCalendar} documentation.
```

# Human docs

```markdown
```
