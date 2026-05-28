# getPlagHaminchaGRAFixedLocalChatzosToSunset

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getPlagHaminchaGRAFixedLocalChatzosToSunset` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:3781)

```javadoc
This method returns <a href="https://en.wikipedia.org/wiki/Moshe_Feinstein">Rav Moshe Feinstein's</a> opinion
of the calculation of <em>plag hamincha</em>. This method returns <em>plag hamincha</em> calculated according to the
<a href="https://en.wikipedia.org/wiki/Vilna_Gaon">GRA</a> that the day ends at sunset and is 4.75 <em>shaos
zmaniyos</em> (solar hours) after {@link #getFixedLocalChatzosHayom() fixed local chatzos}.

@return the <code>Instant</code> of the time of <em>mincha gedola</em>. If the calculation can't be computed such as
        in the Arctic Circle where there is at least one day a year where the sun does not rise, and one where it
        does not set, a <code>null</code> will be returned. See detailed explanation on top of the
        {@link AstronomicalCalendar} documentation.
@see #getPlagHaminchaGRA()
@see #getFixedLocalChatzosHayom()
@see #getMinchaKetanaGRAFixedLocalChatzosToSunset
@see #getMinchaGedolaGRAFixedLocalChatzos30Minutes
@see #getHalfDayBasedZman(Instant, Instant, double)
```

# Human docs

```markdown
Plag hamincha according to [Rav Moshe Feinstein](https://en.wikipedia.org/wiki/Moshe_Feinstein)'s opinion, following the [Vilna Gaon](https://en.wikipedia.org/wiki/Vilna_Gaon) with the day ending at sunset.

4.75 shaos zmaniyos after fixed local chatzos.

In places where sunset cannot be calculated, this zman may not be available.
```
