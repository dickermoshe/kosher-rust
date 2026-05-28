# getPlagAlosToSunset

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getPlagAlosToSunset` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:2002)

```javadoc
This method should be used <em>lechumra</em> only and returns the time of <em>plag hamincha</em> based on the opinion that
the day starts at {@link #getAlos16Point1Degrees() <em>alos</em> 16.1°} and ends at {@link #getSunset()
sunset}. 10.75 <em>shaos zmaniyos</em> are calculated based on this day and added to {@link #getAlos16Point1Degrees()
<em>alos</em>} to reach this time. This time is 10.75 <em>shaos zmaniyos</em> (temporal hours) after {@link
#getAlos16Point1Degrees() dawn} based on the opinion that the day is calculated from a {@link #getAlos16Point1Degrees()
dawn} of 16.1° before sunrise to {@link #getSeaLevelSunset() sea level sunset}. This returns the time of 10.75 * the
calculated <em>shaah zmanis</em> after {@link #getAlos16Point1Degrees() dawn}. Since <em>plag</em> by this calculation can
occur after sunset, it should only be used <em>lechumra</em>.

@deprecated This method should be used <em>lechumra</em> only since it returns a very late time (often after
        <em>shkiah</em>), and if used <em>lekula</em> can result in <em>chillul Shabbos</em> etc. There is no
        current plan to remove this method from the API, and this deprecation is intended to alert developers
        of the danger of using it.
@return the <code>Instant</code> of the <em>plag</em>. If the calculation can't be computed such as northern and southern
        locations even south of the Arctic Circle and north of the Antarctic Circle where the sun may not reach
        low enough below the horizon for this calculation, a <code>null</code> will be returned. See detailed
        explanation on top of the {@link AstronomicalCalendar} documentation.
@see #getAlos16Point1Degrees()
@see #getSeaLevelSunset()
```

# Human docs

```markdown
Plag hamincha based on a day from alos at 16.1 degrees before sunrise to sea level sunset.

10.75 shaos zmaniyos after alos at 16.1 degrees..

This zman can return a very late time, often after shkiah, avoid using it leniently as it can lead to chillul Shabbos. One should not use this time lechumra.

At some northern and southern locations, this zman may not be available or cannot be calculated if the sun does not reach low enough below the horizon.
```
