# getMinchaGedola30Minutes

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getMinchaGedola30Minutes` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:1532)

```javadoc
This method returns <em>mincha gedola</em> calculated as 30 minutes after {@link #getChatzosHayom() <em>chatzos</em>}
and not 1/2 of a {@link #getShaahZmanisGRA() <em>shaah zmanis</em>} after {@link #getChatzosHayom() <em>chatzos</em>} as
calculated by {@link #getMinchaGedola}. Some use this time to delay the start of <em>mincha</em> in the winter when
1/2 of a {@link #getShaahZmanisGRA() <em>shaah zmanis</em>} is less than 30 minutes. See
{@link #getMinchaGedolaGreaterThan30(Instant)} for a convenience method that returns the later of the 2 calculations. One
should not use this time to start <em>mincha</em> before the standard {@link #getMinchaGedolaGRA() <em>mincha gedola</em>}.
See Shulchan Aruch <a href="https://hebrewbooks.org/pdfpager.aspx?req=49624&st=&pgnum=291">Orach Chayim 234:1</a> and
the Shaar Hatziyon <em>seif katan ches</em>. Since this calculation is a fixed 30 minutes of regular clock time after
<em>chatzos</em>, even if {@link #isUseAstronomicalChatzosForOtherZmanim()} is <code>false</code>, this <em>mincha
gedola</em> time will be affected by {@link #isUseAstronomicalChatzos()} and not by
{@link #isUseAstronomicalChatzosForOtherZmanim()}.

@return the <code>Instant</code> of 30 minutes after <em>chatzos</em>. If the calculation can't be computed such as
        in the Arctic Circle where there is at least one day a year where the sun does not rise, and one where it
        does not set, a <code>null</code> will be returned. See detailed explanation on top of the
        {@link AstronomicalCalendar} documentation.
@todo Consider adjusting this to calculate the time as half an hour after {@link #getChatzosHayom()} that uses
        {@link #isUseAstronomicalChatzos()} to determine the type of <em>chatzos</em> to utilize. that support it,
        based on {@link #isUseAstronomicalChatzos()}.
@see #getMinchaGedolaGRA()
@see #getMinchaGedolaGreaterThan30(Instant)
@see #getChatzosHayom()
@see #isUseAstronomicalChatzos()
@see #isUseAstronomicalChatzosForOtherZmanim()
```

# Human docs

```markdown
Mincha gedola calculated as 30 minutes after astronomical chatzos hayom.

Some use this in winter when half a shaah zmanis is less than 30 minutes, to delay the start of mincha. Do not use this time to begin mincha before standard mincha gedola GRA. See Shulchan Aruch [Orach Chayim 234:1](https://hebrewbooks.org/pdfpager.aspx?req=49624&st=&pgnum=291) and Shaar Hatziyon seif katan ches.

In places such as the Arctic Circle, where there is at least one day a year when the sun does not rise and one when it does not set, this zman may not be available or cannot be calculated.
```
