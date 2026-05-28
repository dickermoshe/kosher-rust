# getSofZmanTfilaGRASunriseToFixedLocalChatzos

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getSofZmanTfilaGRASunriseToFixedLocalChatzos` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:3723)

```javadoc
This method returns <a href="https://en.wikipedia.org/wiki/Moshe_Feinstein">Rav Moshe Feinstein's</a> opinion of the
calculation of <em>sof zman tfila</em> (<em>zman tfilah</em> (the latest time to recite the morning prayers))
according to the opinion of the <a href="https://en.wikipedia.org/wiki/Vilna_Gaon">GRA</a> that the day is
calculated from sunrise to sunset, but calculated using the first half of the day only. The half a day starts at
{@link #getSunset() sunrise} and ends at {@link #getFixedLocalChatzosHayom() fixed local chatzos}. <em>Sof zman
tefila</em> is 4 <em>shaos zmaniyos</em> (solar hours) after sunrise or 2/3 of this half-day.

@return the <code>Instant</code> of the latest <em>zman krias shema</em>. If the calculation can't be computed such
        as northern and southern locations even south of the Arctic Circle and north of the Antarctic Circle
        where the sun may not reach low enough below the horizon for this calculation, a <code>null</code> will be
        returned. See detailed explanation on top of the {@link AstronomicalCalendar} documentation.
@see #getSunset()
@see #getFixedLocalChatzosHayom()
@see #getHalfDayBasedZman(Instant, Instant, double)
```

# Human docs

```markdown
Sof zman tfila - the latest time to recite morning prayers (Shacharis) according to [Rav Moshe Feinstein's](https://en.wikipedia.org/wiki/Moshe_Feinstein) view of the [GRA](https://en.wikipedia.org/wiki/Vilna_Gaon) day, using only the first half of the day.

4 shaos zmaniyos after sunrise, with shaos zmaniyos measured from sunrise to fixed local chatzos (two-thirds of that half-day). {uses_elevation}

At some northern and southern locations, this zman may not be available or cannot be calculated if the sun does not reach low enough below the horizon.
```
