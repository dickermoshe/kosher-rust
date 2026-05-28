# getMinchaGedolaGRAFixedLocalChatzos30Minutes

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getMinchaGedolaGRAFixedLocalChatzos30Minutes` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:3741)

```javadoc
This method returns <a href="https://en.wikipedia.org/wiki/Moshe_Feinstein">Rav Moshe Feinstein's</a> opinion of
the calculation of <em>mincha gedola</em>, the earliest time one can pray <em>mincha</em> according to the<a href=
"https://en.wikipedia.org/wiki/Vilna_Gaon">GRA</a> calculated as 30 minutes after {@link #getFixedLocalChatzosHayom() fixed
local chatzos}.

@return the <code>Instant</code> of the time of <em>mincha gedola</em>. If the calculation can't be computed such as
        in the Arctic Circle where there is at least one day a year where the sun does not rise, and one where it
        does not set, a <code>null</code> will be returned. See detailed explanation on top of the
        {@link AstronomicalCalendar} documentation.
@see #getMinchaGedolaGRA()
@see #getFixedLocalChatzosHayom()
@see #getMinchaKetanaGRAFixedLocalChatzosToSunset
```

# Human docs

```markdown
Mincha gedola according to Rav Moshe Feinstein's opinion.

30 minutes after fixed local chatzos.

In places where fixed local chatzos cannot be calculated, this zman may not be available.
```
