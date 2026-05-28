# getMinchaGedola72Minutes

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getMinchaGedola72Minutes` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:1557)

```javadoc
This method returns the time of <em>mincha gedola</em> according to the Magen Avraham with the day starting 72
minutes before sunrise and ending 72 minutes after sunset. This is the earliest time to pray <em>mincha</em>. For
more information on this see the documentation on {@link #getMinchaGedolaGRA() <em>mincha gedola</em>}. This is
calculated as 6.5 {@link #getTemporalHour() solar hours} after <em>alos</em>. The calculation used is 6.5 *
{@link #getShaahZmanis72Minutes()} after {@link #getAlos72Minutes() <em>alos</em>}. If {@link
#isUseAstronomicalChatzosForOtherZmanim()} is set to <code>true</code>, the calculation will be based on 0.5
{@link #getHalfDayBasedShaahZmanis(Instant, Instant) half-day based <em>sha'ah zmanis</em>} between
{@link #getChatzosHayom()} and {@link #getTzais72Minutes()} after {@link #getChatzosHayom()}.

@see #getAlos72Minutes()
@see #getMinchaGedolaGRA()
@see #getMinchaKetanaGRA()
@see #getMinchaGedolaGRA()
@see #getChatzosHayom()
@see #isUseAstronomicalChatzosForOtherZmanim()
@return the <code>Instant</code> of the time of <em>mincha gedola</em>. If the calculation can't be computed such as
        in the Arctic Circle where there is at least one day a year where the sun does not rise, and one where it
        does not set, a <code>null</code> will be returned. See detailed explanation on top of the
        {@link AstronomicalCalendar} documentation.
```

# Human docs

```markdown
Mincha gedola according to the Magen Avraham.

Half a shaah zmanis after chatzos hayom, using a day from alos 72 minutes to tzais 72 minutes.

In places where sunrise or sunset cannot be calculated, this zman may not be available.
```
