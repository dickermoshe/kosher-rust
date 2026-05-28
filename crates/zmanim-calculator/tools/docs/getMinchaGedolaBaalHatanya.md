# getMinchaGedolaBaalHatanya

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getMinchaGedolaBaalHatanya` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:3537)

```javadoc
This method returns the time of <em>mincha gedola</em>. <em>Mincha gedola</em> is the earliest time one can pray
<em>mincha</em>. The <a href="https://en.wikipedia.org/wiki/Maimonides">Rambam</a> is of the opinion that it is
better to delay <em>mincha</em> until {@link #getMinchaKetanaBaalHatanya() <em>mincha ketana</em>} while the
<a href="https://en.wikipedia.org/wiki/Asher_ben_Jehiel">Ra"sh</a>,
<a href="https://en.wikipedia.org/wiki/Jacob_ben_Asher">Tur</a>, <a href=
"https://en.wikipedia.org/wiki/Vilna_Gaon">GRA</a> and others are of the opinion that <em>mincha</em> can be prayed
<em>lechatchila</em> starting at <em>mincha gedola</em>. This is calculated as 6.5 {@link #getShaahZmanisBaalHatanya()
sea level solar hours} after {@link #getSunriseBaalHatanya() <em>netz amiti</em> (sunrise)}. This calculation is based
on the opinion of the Baal Hatanya that the day is calculated from sunrise to sunset. This returns the time 6.5
* {@link #getShaahZmanisBaalHatanya()} after {@link #getSunriseBaalHatanya() <em>netz amiti</em> ("real" sunrise)}.
@todo Consider adjusting this to calculate the time as 30 clock or <em>zmaniyos </em> minutes after either {@link
        getSunTransit() astronomical <em>chatzos</em>} or {@link #getChatzosHayomAsHalfDay() <em>chatzos</em> as half a
        day} for {@link AstronomicalCalculator calculators} that support it, based on {@link #isUseAstronomicalChatzos()}.
@see #getMinchaGedola(Instant, Instant)
@see #getShaahZmanisBaalHatanya()
@see #getMinchaKetanaBaalHatanya()
@return the <code>Instant</code> of the time of <em>mincha gedola</em> according to the Baal Hatanya. If the calculation
        can't be computed such as in the Arctic Circle where there is at least one day a year where the sun does not rise,
        and one where it does not set, a <code>null</code> will be returned. See detailed explanation on top of the
        {@link AstronomicalCalendar} documentation.
```

# Human docs

```markdown
Mincha gedola according to the Baal Hatanya.

6.5 shaos zmaniyos after netz amiti (true sunrise), with the day measured from Baal Hatanya sunrise to sunset.

In places such as the Arctic Circle, where there is at least one day a year when the sun does not rise and one when it does not set, this zman may not be available or cannot be calculated.
```
