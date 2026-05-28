# getMinchaKetanaGRA

Source: `com.kosherjava.zmanim.ZmanimCalendar.getMinchaKetanaGRA` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ZmanimCalendar.java:1030)

```javadoc
This method returns <em>mincha ketana</em>,the preferred earliest time to pray <em>mincha</em> in the
opinion of the <em><a href="https://en.wikipedia.org/wiki/Maimonides">Rambam</a></em> and others, that is 9.5
* {@link getShaahZmanisGRA() <em>shaos zmaniyos</em>} (solar hours) after {@link getSunrise() sunrise} or
{@link getSeaLevelSunrise() sea level sunrise} (depending on the {@link isUseElevation()} setting), according
to the <a href="https://en.wikipedia.org/wiki/Vilna_Gaon">GRA</a>. For more information on this see the
documentation on {@link getMinchaGedolaGRA() <em>mincha gedola</em>}.
The day is calculated from {@link getSeaLevelSunrise() sea level sunrise} to {@link getSeaLevelSunset() sea level
sunset} or from {@link getSunrise() sunrise} to {@link getSunset() sunset} (depending on the {@link isUseElevation()}
setting.

@see getMinchaKetana(Instant, Instant)
@see getShaahZmanisGRA()
@see getMinchaGedolaGRA()
@see ComprehensiveZmanimCalendar#getMinchaKetanaBaalHatanya()
@return the <code>Instant</code> of the time of mincha ketana. If the calculation can't be computed such as in the
        Arctic Circle where there is at least one day a year where the sun does not rise, and one where it does
        not set, a <code>null</code> will be returned. See detailed explanation on top of the
        {@link AstronomicalCalendar} documentation.
```

# Human docs

```markdown
Mincha ketana according to the [Vilna Gaon](https://en.wikipedia.org/wiki/Vilna_Gaon).

9.5 shaos zmaniyos after sunrise, using a day from sunrise to sunset. {uses_elevation}

This is the preferred earliest time to pray mincha according to the Rambam and others.

In places where sunrise or sunset cannot be calculated, this zman may not be available.
```
