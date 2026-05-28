# getMinchaKetanaGRAFixedLocalChatzosToSunset

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getMinchaKetanaGRAFixedLocalChatzosToSunset` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:3761)

```javadoc
This method returns <a href="https://en.wikipedia.org/wiki/Moshe_Feinstein">Rav Moshe Feinstein's</a> opinion
of the calculation of <em>mincha ketana</em> (the preferred time to recite the <em>mincha prayers</em> according to
the opinion of the <a href="https://en.wikipedia.org/wiki/Maimonides">Rambam</a> and others) calculated according
to the <a href="https://en.wikipedia.org/wiki/Vilna_Gaon">GRA</a> that is 3.5 <em>shaos zmaniyos</em> (solar
hours) after {@link #getFixedLocalChatzosHayom() fixed local chatzos}.

@return the <code>Instant</code> of the time of <em>mincha gedola</em>. If the calculation can't be computed such as
        in the Arctic Circle where there is at least one day a year where the sun does not rise, and one where it
        does not set, a <code>null</code> will be returned. See detailed explanation on top of the
        {@link AstronomicalCalendar} documentation.
@see #getMinchaGedolaGRA()
@see #getFixedLocalChatzosHayom()
@see #getMinchaGedolaGRAFixedLocalChatzos30Minutes
@see #getHalfDayBasedZman(Instant, Instant, double)
```

# Human docs

```markdown
Mincha ketana according to Rav Moshe Feinstein's opinion, following the view of the Vilna Gaon.

3.5 shaos zmaniyos after fixed local chatzos.

In places where fixed local chatzos or sunset cannot be calculated, this zman may not be available.
```
