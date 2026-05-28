# getMinchaGedolaGRAGreaterThan30

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getMinchaGedolaGRAGreaterThan30` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:1672)

```javadoc
This is a convenience method that returns the later of {@link #getMinchaGedolaGRA()} and
{@link #getMinchaGedola30Minutes()}. In the winter when 1/2 of a {@link #getShaahZmanisGRA() <em>shaah zmanis</em>} is
less than 30 minutes {@link #getMinchaGedola30Minutes()} will be returned, otherwise {@link #getMinchaGedolaGRA()}
will be returned. Since this calculation can be an offset of <em>chatzos</em> (if 30 clock minutes > 1/2 of a <em>shaah
zmanis</em>), even if {@link #isUseAstronomicalChatzosForOtherZmanim()} is <code>false</code>, this <em>mincha</em> time
may be affected by {@link #isUseAstronomicalChatzos()}.

@return the <code>Date</code> of the later of {@link #getMinchaGedolaGRA()} and {@link #getMinchaGedola30Minutes()}.
        If the calculation can't be computed such as in the Arctic Circle where there is at least one day a year
        where the sun does not rise, and one where it does not set, a <code>null</code> will be returned. See detailed
        explanation on top of the {@link AstronomicalCalendar} documentation.
@todo Consider adjusting this to calculate the time as 30 minutes after {@link #getChatzosHayom()} that uses {@link
        isUseAstronomicalChatzos()} to determine the type of <em>chatzos</em> to utilize (if the {@link
        com.kosherjava.zmanim.util.AstronomicalCalculator calculator} support astronomical <em>chatzos</em>),
        based on the {@link #isUseAstronomicalChatzos()} setting.
@see #getMinchaGedolaGRA()
@see #getMinchaGedola30Minutes()
@see #getMinchaGedolaGreaterThan30(Instant)
@see #isUseAstronomicalChatzos()
```

# Human docs

```markdown
Mincha gedola calculated as the later of mincha gedola GRA and 30 minutes after astronomical chatzos hayom.

In the winter, when half a shaah zmanis is less than 30 minutes, the 30-minutes-after-chatzos time is used. Otherwise, mincha gedola GRA is used.

In places where sunrise, sunset, or chatzos cannot be calculated, this zman may not be available.
```
