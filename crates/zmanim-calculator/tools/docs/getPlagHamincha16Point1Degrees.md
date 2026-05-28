# getPlagHamincha16Point1Degrees

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getPlagHamincha16Point1Degrees` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:1906)

```javadoc
This method should be used <em>lechumra</em> only and returns the time of <em>plag hamincha</em> based on the
opinion that the day starts at {@link #getAlos16Point1Degrees() <em>alos</em> 16.1°} and ends at {@link
#getTzais16Point1Degrees() <em>tzais</em> 16.1°}. This is calculated as 10.75 hours <em>zmaniyos</em>
after {@link #getAlos16Point1Degrees() dawn}. The formula used is 10.75 * {@link #getShaahZmanis16Point1Degrees()}
after {@link #getAlos16Point1Degrees()}. Since <em>plag</em> by this calculation can occur after sunset, it
should only be used <em>lechumra</em>.

@deprecated This method should be used <em>lechumra</em> only since it returns a very late time (often after
        <em>shkiah</em>), and if used <em>lekula</em> can result in <em>chillul Shabbos</em> etc. There is no
        current plan to remove this method from the API, and this deprecation is intended to alert developers
        of the danger of using it.
@return the <code>Instant</code> of the time of <em>plag hamincha</em>. If the calculation can't be computed such as
        northern and southern locations even south of the Arctic Circle and north of the Antarctic Circle where
        the sun may not reach low enough below the horizon for this calculation, a <code>null</code> will be
        returned. See detailed explanation on top of the {@link AstronomicalCalendar} documentation.
@see #getShaahZmanis16Point1Degrees()
```

# Human docs

```markdown
Plag hamincha based on the 16.1-degree day.

10.75 shaos zmaniyos after alos 16.1 degrees, using a day that begins and ends at 16.1 degrees.

This zman should be used lechumra only. It can be a very late time, often after shkiah, and using it leniently can lead to chillul Shabbos.

At some northern and southern locations, this zman may not be available if the sun does not reach low enough below the horizon.
```
