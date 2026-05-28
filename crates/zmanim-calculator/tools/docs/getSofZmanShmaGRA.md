# getSofZmanShmaGRA

Source: `com.kosherjava.zmanim.ZmanimCalendar.getSofZmanShmaGRA` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ZmanimCalendar.java:566)

```javadoc
This method returns the latest <em>zman krias shema</em> (time to recite shema in the morning) that is 3 *
{@link getShaahZmanisGRA() <em>shaos zmaniyos</em>} (solar hours) after {@link getSunrise() sunrise} or
{@link getSeaLevelSunrise() sea level sunrise} (depending on the {@link isUseElevation()} setting), according
to the <a href="https://en.wikipedia.org/wiki/Vilna_Gaon">GRA</a>.
 The day is calculated from {@link getSeaLevelSunrise() sea level sunrise} to {@link getSeaLevelSunset() sea level
 sunset} or from {@link getSunrise() sunrise} to {@link getSunset() sunset} (depending on the
 {@link isUseElevation()} setting).

@see getSofZmanShma(Instant, Instant)
@see getShaahZmanisGRA()
@see isUseElevation()
@see ComprehensiveZmanimCalendar#getSofZmanShmaBaalHatanya()
@return the <code>Instant</code> of the latest <em>zman shema</em> according to the GRA. If the calculation can't be
        computed such as in the Arctic Circle where there is at least one day a year where the sun does not rise,
        and one where it does not set, a <code>null</code> will be returned. See the detailed explanation on top
        of the {@link AstronomicalCalendar} documentation.
```

# Human docs

```markdown
Sof zman krias shema - the latest time to recite morning Shema according to the [GRA](https://en.wikipedia.org/wiki/Vilna_Gaon).

3 shaos zmaniyos after sunrise. {uses_elevation}

In places such as the Arctic Circle, where there is at least one day a year when the sun does not rise and one when it does not set, this zman may not be available or cannot be calculated.
```
