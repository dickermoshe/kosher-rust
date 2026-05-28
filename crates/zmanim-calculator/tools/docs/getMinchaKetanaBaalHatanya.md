# getMinchaKetanaBaalHatanya

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getMinchaKetanaBaalHatanya` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:3558)

```javadoc
This method returns the time of <em>mincha ketana</em>. This is the preferred earliest time to pray
<em>mincha</em> in the opinion of the <a href="https://en.wikipedia.org/wiki/Maimonides">Rambam</a> and others.
For more information on this see the documentation on {@link #getMinchaGedolaBaalHatanya() <em>mincha gedola</em>}.
This is calculated as 9.5 {@link #getShaahZmanisBaalHatanya()  sea level solar hours} after {@link #getSunriseBaalHatanya()
<em>netz amiti</em> (sunrise)}. This calculation is calculated based on the opinion of the Baal Hatanya that the
day is calculated from sunrise to sunset. This returns the time 9.5 * {@link #getShaahZmanisBaalHatanya()} after {@link
#getSunriseBaalHatanya() <em>netz amiti</em> (sunrise)}.

@see #getMinchaKetana(Instant, Instant)
@see #getShaahZmanisBaalHatanya()
@see #getMinchaGedolaBaalHatanya()
@return the <code>Instant</code> of the time of <em>mincha ketana</em>. If the calculation can't be computed such as
        in the Arctic Circle where there is at least one day a year where the sun does not rise, and one where it
        does not set, a <code>null</code> will be returned. See detailed explanation on top of the
        {@link AstronomicalCalendar} documentation.
```

# Human docs

```markdown
Mincha ketana according to the Baal Hatanya.

9.5 shaos zmaniyos after netz amiti (true sunrise), with the day measured from Baal Hatanya sunrise to sunset.

In places such as the Arctic Circle, where there is at least one day a year when the sun does not rise and one when it does not set, this zman may not be available or cannot be calculated.
```
