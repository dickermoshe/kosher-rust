# getSofZmanShmaMGA16Point1DegreesToFixedLocalChatzos

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getSofZmanShmaMGA16Point1DegreesToFixedLocalChatzos` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:3641)

```javadoc
This method returns <a href="https://en.wikipedia.org/wiki/Moshe_Feinstein">Rav Moshe Feinstein's</a> opinion of the
calculation of <em>sof zman krias shema</em> (latest time to recite <em>Shema</em> in the morning) according to the
opinion of the <a href="https://en.wikipedia.org/wiki/Avraham_Gombiner">Magen Avraham (MGA)</a> that the day is calculated
from dawn to nightfall, but calculated using the first half of the day only. The half a day starts at <em>alos</em> defined
as {@link #getAlos16Point1Degrees() 16.1°} and ends at {@link #getFixedLocalChatzosHayom() fixed local chatzos}. <em>Sof Zman
Shema</em> is 3 <em>shaos zmaniyos</em> (solar hours) after this <em>alos</em> or half of this half-day.

@return the <code>Instant</code> of the latest <em>zman krias shema</em>. If the calculation can't be computed such
        as northern and southern locations even south of the Arctic Circle and north of the Antarctic Circle
        where the sun may not reach low enough below the horizon for this calculation, a <code>null</code> will be returned.
        See detailed explanation on top of the {@link AstronomicalCalendar} documentation.
@see #getAlos16Point1Degrees()
@see #getFixedLocalChatzosHayom()
@see #getHalfDayBasedZman(Instant, Instant, double)
```

# Human docs

```markdown
```
