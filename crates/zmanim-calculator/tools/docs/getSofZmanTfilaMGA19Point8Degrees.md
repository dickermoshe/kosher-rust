# getSofZmanTfilaMGA19Point8Degrees

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getSofZmanTfilaMGA19Point8Degrees` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:1324)

```javadoc
This method returns the latest <em>zman tfila</em> (time to recite the morning prayers) according to the opinion
of the <a href="https://en.wikipedia.org/wiki/Avraham_Gombiner">Magen Avraham (MGA)</a> based on
<em>alos</em> being {@link #getAlos19Point8Degrees() 19.8°} before {@link #getSunset() sunrise}. This time
is 4 {@link #getShaahZmanis19Point8Degrees() <em>shaos zmaniyos</em>} (solar hours) after {@link
#getAlos19Point8Degrees() dawn} based on the opinion of the MGA that the day is calculated from dawn to
nightfall with both being 19.8° below sunrise or sunset. This returns the time of 4 * {@link
#getShaahZmanis19Point8Degrees()} after {@link #getAlos19Point8Degrees() dawn}.

@return the <code>Instant</code> of the latest <em>zman krias shema</em>. If the calculation can't be computed such
        as northern and southern locations even south of the Arctic Circle and north of the Antarctic Circle
        where the sun may not reach low enough below the horizon for this calculation, a <code>null</code> will be
        returned. See detailed explanation on top of the {@link AstronomicalCalendar} documentation.
@see #getShaahZmanis19Point8Degrees()
@see #getAlos19Point8Degrees()
@see #isUseAstronomicalChatzosForOtherZmanim()
```

# Human docs

```markdown
Sof zman tfila - the latest time to recite morning prayers (Shacharis) according to the [Magen Avraham (MGA)](https://en.wikipedia.org/wiki/Avraham_Gombiner), using the 19.8-degree day.

4 shaos zmaniyos after alos at 19.8 degrees, with the day measured from alos at 19.8 degrees to tzais at 19.8 degrees.

At some northern and southern locations, this zman may not be available or cannot be calculated if the sun does not reach low enough below the horizon.
```
