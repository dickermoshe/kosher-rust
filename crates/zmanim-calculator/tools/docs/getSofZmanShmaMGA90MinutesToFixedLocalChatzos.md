# getSofZmanShmaMGA90MinutesToFixedLocalChatzos

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getSofZmanShmaMGA90MinutesToFixedLocalChatzos` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:3662)

```javadoc
This method returns <a href="https://en.wikipedia.org/wiki/Moshe_Feinstein">Rav Moshe Feinstein's</a> opinion of the
calculation of <em>sof zman krias shema</em> (latest time to recite <em>Shema</em> in the morning) according to the
opinion of the <a href="https://en.wikipedia.org/wiki/Avraham_Gombiner">Magen Avraham (MGA)</a> that the
day is calculated from dawn to nightfall, but calculated using the first half of the day only. The half a day starts
at <em>alos</em> defined as {@link #getAlos90Minutes() 90 minutes before sunrise} and ends at {@link #getFixedLocalChatzosHayom()
fixed local chatzos}. <em>Sof Zman Shema</em> is 3 <em>shaos zmaniyos</em> (solar hours) after this <em>alos</em> or
half of this half-day.

@return the <code>Instant</code> of the latest <em>zman krias shema</em>. If the calculation can't be computed such
        as northern and southern locations even south of the Arctic Circle and north of the Antarctic Circle
        where the sun may not reach low enough below the horizon for this calculation, a <code>null</code> will be
        returned. See detailed explanation on top of the {@link AstronomicalCalendar} documentation.
@see #getAlos90Minutes()
@see #getFixedLocalChatzosHayom()
@see #getHalfDayBasedZman(Instant, Instant, double)
```

# Human docs

```markdown
Sof zman krias shema - the latest time to recite morning Shema according to [Rav Moshe Feinstein's](https://en.wikipedia.org/wiki/Moshe_Feinstein) view of the [Magen Avraham (MGA)](https://en.wikipedia.org/wiki/Avraham_Gombiner) day, using only the first half of the day.

3 shaos zmaniyos after alos 90 minutes before sunrise, with shaos zmaniyos measured from that alos to fixed local chatzos (half of that half-day).

At some northern and southern locations, this zman may not be available or cannot be calculated if the sun does not reach low enough below the horizon.
```
