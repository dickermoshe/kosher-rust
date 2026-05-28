# getSofZmanTfilaGRA

Source: `com.kosherjava.zmanim.ZmanimCalendar.getSofZmanTfilaGRA` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ZmanimCalendar.java:784)

```javadoc
This method returns the latest <em>zman tfila</em> (time to recite shema in the morning) that is 4 *
{@link getShaahZmanisGRA() <em>shaos zmaniyos</em> }(solar hours) after {@link getSunrise() sunrise} or
{@link getSeaLevelSunrise() sea level sunrise} (depending on the {@link isUseElevation()} setting), according
to the <a href="https://en.wikipedia.org/wiki/Vilna_Gaon">GRA</a>.
The day is calculated from {@link getSeaLevelSunrise() sea level sunrise} to {@link getSeaLevelSunset() sea level
sunset} or from {@link getSunrise() sunrise} to {@link getSunset() sunset} (depending on the
{@link isUseElevation()} setting).

@see getSofZmanTfila(Instant, Instant)
@see getShaahZmanisGRA()
@see ComprehensiveZmanimCalendar#getSofZmanTfilaBaalHatanya()
@return the <code>Instant</code> of the latest <em>zman tfilah</em>. If the calculation can't be computed such as in
        the Arctic Circle where there is at least one day a year where the sun does not rise, and one where it
        does not set, a <code>null</code> will be returned. See detailed explanation on top of the
        {@link AstronomicalCalendar} documentation.
```

# Human docs

```markdown
```
