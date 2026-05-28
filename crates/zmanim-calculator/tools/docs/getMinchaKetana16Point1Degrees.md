# getMinchaKetana16Point1Degrees

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getMinchaKetana16Point1Degrees` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:1692)

```javadoc
This method returns the time of <em>mincha ketana</em> according to the Magen Avraham with the day starting and
ending 16.1° below the horizon. This is the preferred earliest time to pray <em>mincha</em> according to the
opinion of the <a href="https://en.wikipedia.org/wiki/Maimonides">Rambam</a> and others. For more information on
this see the documentation on {@link #getMinchaGedolaGRA() <em>mincha gedola</em>}. This is calculated as 9.5
{@link #getTemporalHour() solar hours} after <em>alos</em>. The calculation used is 9.5 *
{@link #getShaahZmanis16Point1Degrees()} after {@link #getAlos16Point1Degrees() <em>alos</em>}.

@see #getShaahZmanis16Point1Degrees()
@see #getMinchaGedolaGRA()
@see #getMinchaKetanaGRA()
@return the <code>Instant</code> of the time of <em>mincha ketana</em>. If the calculation can't be computed such
        as northern and southern locations even south of the Arctic Circle and north of the Antarctic Circle
        where the sun may not reach low enough below the horizon for this calculation, a <code>null</code> will be
        returned. See detailed explanation on top of the {@link AstronomicalCalendar} documentation.
```

# Human docs

```markdown
Mincha ketana according to the Magen Avraham.

9.5 shaos zmaniyos after alos at 16.1 degrees below the horizon, using a day that begins and ends at 16.1 degrees.

At some northern and southern locations, including places even south of the Arctic Circle and north of the Antarctic Circle, this zman may not be available or cannot be calculated if the sun does not reach low enough below the horizon.
```
