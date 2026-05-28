# getPlagHaminchaGRA

Source: `com.kosherjava.zmanim.ZmanimCalendar.getPlagHaminchaGRA` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ZmanimCalendar.java:1105)

```javadoc
This method returns <em>plag hamincha</em>, that is 10.75 * {@link getShaahZmanisGRA() <em>shaos zmaniyos</em>}
(solar hours) after {@link getSunrise() sunrise} or {@link getSeaLevelSunrise() sea level sunrise} (depending on
the {@link isUseElevation()} setting), according to the <a href="https://en.wikipedia.org/wiki/Vilna_Gaon"
>GRA</a>. <em>Plag hamincha</em> is the earliest time that <em>Shabbos</em> can be started.
The day is calculated from {@link getSeaLevelSunrise() sea level sunrise} to {@link getSeaLevelSunset() sea level
sunset} or {@link getSunrise() sunrise} to {@link getSunset() sunset} (depending on the {@link isUseElevation()}

@see getPlagHamincha(Instant, Instant, boolean)
@see getPlagHamincha(Instant, Instant)
@see ComprehensiveZmanimCalendar#getPlagHaminchaBaalHatanya()
@return the <code>Instant</code> of the time of <em>plag hamincha</em>. If the calculation can't be computed such as
        in the Arctic Circle where there is at least one day a year where the sun does not rise, and one where it
        does not set, a <code>null</code> will be returned. See detailed explanation on top of the
        {@link AstronomicalCalendar} documentation.
```

# Human docs

```markdown
Plag hamincha according to the [Vilna Gaon](https://en.wikipedia.org/wiki/Vilna_Gaon).

10.75 shaos zmaniyos after sunrise, using a day from sunrise to sunset. {uses_elevation}

This is the earliest time Shabbos can be started.

In places where sunrise or sunset cannot be calculated, this zman may not be available.
```
