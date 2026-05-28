# getMinchaGedola16Point1Degrees

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getMinchaGedola16Point1Degrees` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:1582)

```javadoc
This method returns the time of <em>mincha gedola</em> according to the Magen Avraham with the day starting and
ending 16.1° below the horizon. This is the earliest time to pray <em>mincha</em>. For more information on
this see the documentation on {@link #getMinchaGedolaGRA() <em>mincha gedola</em>}. This is calculated as 6.5
{@link #getTemporalHour() solar hours} after <em>alos</em>. The calculation used is 6.5 *
{@link #getShaahZmanis16Point1Degrees()} after {@link #getAlos16Point1Degrees() <em>alos</em>}. If {@link
#isUseAstronomicalChatzosForOtherZmanim()} is set to <code>true</code>, the calculation will be based on 0.5
{@link #getHalfDayBasedShaahZmanis(Instant, Instant) half-day based <em>sha'ah zmanis</em>} between {@link #getChatzosHayom()}
and {@link #getAlos16Point1Degrees()} after {@link #getChatzosHayom()}.
@return the <code>Instant</code> of the time of <em>mincha gedola</em>. If the calculation can't be computed such as
        northern and southern locations even south of the Arctic Circle and north of the Antarctic Circle where
        the sun  may not reach low enough below the horizon for this calculation, a <code>null</code> will be returned.
        See detailed explanation on top of the {@link AstronomicalCalendar} documentation.
@see #getShaahZmanis16Point1Degrees()
@see #getMinchaGedolaGRA()
@see #getMinchaKetanaGRA()
```

# Human docs

```markdown
Mincha gedola according to the Magen Avraham, using a day that begins and ends at 16.1 degrees below the horizon.

Half a shaah zmanis after chatzos hayom, where the proportional hour is based on the span from alos at 16.1 degrees to chatzos hayom.

At some northern and southern locations, including places even south of the Arctic Circle and north of the Antarctic Circle, this zman may not be available or cannot be calculated if the sun does not reach low enough below the horizon.
```
