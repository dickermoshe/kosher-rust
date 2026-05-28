# getSofZmanShmaMGA18Degrees

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getSofZmanShmaMGA18Degrees` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:1100)

```javadoc
This method returns the latest <em>zman krias shema</em> (time to recite Shema in the morning) according to the
opinion of the <a href="https://en.wikipedia.org/wiki/Avraham_Gombiner">Magen Avraham (MGA)</a> based
on <em>alos</em> being {@link #getAlos18Degrees() 18°} before {@link #getSunset() sunrise}. This time is 3
{@link #getShaahZmanis18Degrees() <em>shaos zmaniyos</em>} (solar hours) after {@link #getAlos18Degrees() dawn}
based on the opinion of the MGA that the day is calculated from dawn to nightfall with both being 18°
below sunrise or sunset. This returns the time of 3 * {@link #getShaahZmanis18Degrees()} after
{@link #getAlos18Degrees() dawn}.

@return the <code>Instant</code> of the latest <em>zman krias shema</em>. If the calculation can't be computed such
        as northern and southern locations even south of the Arctic Circle and north of the Antarctic Circle
        where the sun may not reach low enough below the horizon for this calculation, a <code>null</code> will be
        returned. See detailed explanation on top of the {@link AstronomicalCalendar} documentation.
@see #getShaahZmanis18Degrees()
@see #getAlos18Degrees()
```

# Human docs

```markdown
Sof zman krias shema - the latest time to recite morning Shema according to the [Magen Avraham (MGA)](https://en.wikipedia.org/wiki/Avraham_Gombiner), using the 18-degree day.

3 shaos zmaniyos after alos at 18 degrees, with the day measured from alos at 18 degrees to tzais at 18 degrees.

At some northern and southern locations, this zman may not be available or cannot be calculated if the sun does not reach low enough below the horizon.
```
