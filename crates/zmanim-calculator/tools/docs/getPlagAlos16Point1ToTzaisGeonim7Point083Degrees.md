# getPlagAlos16Point1ToTzaisGeonim7Point083Degrees

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getPlagAlos16Point1ToTzaisGeonim7Point083Degrees` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:2021)

```javadoc
This method returns the time of <em>plag hamincha</em> based on the opinion that the day starts at {@link
#getAlos16Point1Degrees()} and ends at {@link #getTzaisGeonim7Point083Degrees()}. 10.75 <em>shaos zmaniyos</em> are calculated
based on this day and added to {@link #getAlos16Point1Degrees()} to reach this time. This time is 10.75 <em>shaos zmaniyos</em>
(temporal hours) after {@link #getAlos16Point1Degrees()} based on the opinion that the day is calculated from a {@link
#getAlos16Point1Degrees() 16.1°} before sunrise to {@link #getTzaisGeonim7Point083Degrees() 7.083°} after sunset. This returns
the time of 10.75 * the calculated <em>shaah zmanis</em> after {@link #getAlos16Point1Degrees()}.

@return the <code>Instant</code> of the <em>plag</em>. If the calculation can't be computed such as northern and southern
        locations even south of the Arctic Circle and north of the Antarctic Circle where the sun may not reach low enough
        below the horizon for this calculation, a <code>null</code> will be returned. See detailed explanation on top of the
        {@link AstronomicalCalendar} documentation.
@see #getAlos16Point1Degrees()
@see #getTzaisGeonim7Point083Degrees()
```

# Human docs

```markdown
Plag hamincha based on a day from alos 16.1 degrees to tzais Geonim 7.083 degrees.

10.75 shaos zmaniyos after alos 16.1 degrees.

At some northern and southern locations, this zman may not be available if the sun does not reach low enough below the horizon.
```
