# getMinchaGedolaGRA

Source: `com.kosherjava.zmanim.ZmanimCalendar.getMinchaGedolaGRA` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ZmanimCalendar.java:896)

```javadoc
This method returns the latest <em>mincha gedola</em>,the earliest time one can pray <em>mincha</em> that is 6.5 *
{@link getShaahZmanisGRA() <em>shaos zmaniyos</em>} (solar hours) after {@link getSunrise() sunrise} or
{@link getSeaLevelSunrise() sea level sunrise} (depending on the {@link isUseElevation()} setting), according
to the <a href="https://en.wikipedia.org/wiki/Vilna_Gaon">GRA</a>. <em>Mincha gedola</em> is the earliest
time one can pray <em>mincha</em>. The Ramba"m is of the opinion that it is better to delay <em>mincha</em> until
{@link getMinchaKetanaGRA() <em>mincha ketana GRA</em>} while the Ra"sh, Tur, GRA and others are of the
opinion that <em>mincha</em> can be prayed <em>lechatchila</em> starting at <em>mincha gedola</em>.
The day is calculated from {@link getSeaLevelSunrise() sea level sunrise} to {@link getSeaLevelSunset() sea level
sunset} or {@link getSunrise() sunrise} to {@link getSunset() sunset} (depending on the {@link isUseElevation()}
setting).
@todo Consider adjusting this to calculate the time as half an hour <em>zmaniyos</em> after {@link getChatzosHayom()}
        that uses {@link isUseAstronomicalChatzos()} to determine the type of <em>chatzos</em> to utilize (if the
        {@link com.kosherjava.zmanim.util.AstronomicalCalculator calculator} support astronomical <em>chatzos</em>),
        based on the {@link isUseAstronomicalChatzos()} setting.
@see getMinchaGedola(Instant, Instant)
@see getShaahZmanisGRA()
@see getMinchaKetanaGRA()
@see ComprehensiveZmanimCalendar#getMinchaGedolaBaalHatanya()
@return the <code>Instant</code> of the time of mincha gedola. If the calculation can't be computed such as in the
        Arctic Circle where there is at least one day a year where the sun does not rise, and one where it does
        not set, a <code>null</code> will be returned. See detailed explanation on top of the
        {@link AstronomicalCalendar} documentation.
```

# Human docs

```markdown
Mincha gedola according to the [Vilna Gaon](https://en.wikipedia.org/wiki/Vilna_Gaon).

6.5 shaos zmaniyos after sunrise. {uses_elevation} The day is measured from sunrise to sunset for the proportional hour.

In places such as the Arctic Circle, where there is at least one day a year when the sun does not rise and one when it does not set, this zman may not be available or cannot be calculated.
```
