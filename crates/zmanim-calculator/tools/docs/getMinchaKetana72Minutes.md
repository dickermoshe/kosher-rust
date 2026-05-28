# getMinchaKetana72Minutes

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getMinchaKetana72Minutes` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:1740)

```javadoc
This method returns the time of <em>mincha ketana</em> according to the Magen Avraham with the day
starting 72 minutes before sunrise and ending 72 minutes after sunset. This is the preferred earliest time to pray
<em>mincha</em> according to the opinion of the <a href="https://en.wikipedia.org/wiki/Maimonides">Rambam</a>
and others. For more information on this see the documentation on {@link #getMinchaGedolaGRA() <em>mincha gedola</em>}.
This is calculated as 9.5 {@link #getShaahZmanis72Minutes()} after <em>alos</em>. The calculation used is 9.5 *
{@link #getShaahZmanis72Minutes()} after {@link #getAlos72Minutes() <em>alos</em>}.

@see #getShaahZmanis16Point1Degrees()
@see #getMinchaGedolaGRA()
@see #getMinchaKetanaGRA()
@return the <code>Instant</code> of the time of <em>mincha ketana</em>. If the calculation can't be computed such as
        in the Arctic Circle where there is at least one day a year where the sun does not rise, and one where it
        does not set, a <code>null</code> will be returned. See detailed explanation on top of the
        {@link AstronomicalCalendar} documentation.
```

# Human docs

```markdown
Mincha ketana according to the Magen Avraham, using the 72-minute day.

9.5 shaos zmaniyos after alos 72 minutes before sunrise, using a day that starts 72 minutes before sunrise and ends 72 minutes after sunset.

In places where sunrise or sunset cannot be calculated, this zman may not be available.
```
