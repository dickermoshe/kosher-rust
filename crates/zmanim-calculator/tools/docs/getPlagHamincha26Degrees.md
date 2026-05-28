# getPlagHamincha26Degrees

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getPlagHamincha26Degrees` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:1953)

```javadoc
This method should be used <em>lechumra</em> only and returns the time of <em>plag hamincha</em> based on the
opinion that the day starts at {@link #getAlos26Degrees() <em>alos</em> 26°} and ends at {@link
#getTzais26Degrees() <em>tzais</em> 26°}. This is calculated as 10.75 hours <em>zmaniyos</em> after {@link
#getAlos26Degrees() dawn}. The formula used is 10.75 * {@link #getShaahZmanis26Degrees()} after {@link
#getAlos26Degrees()}. Since the <em>zman</em> based on an extremely early <em>alos</em> and a very late
<em>tzais</em>, it should only be used <em>lechumra</em>.

@deprecated This method should be used <em>lechumra</em> only since it returns a very late time (often after
        <em>shkiah</em>), and if used <em>lekula</em> can result in <em>chillul Shabbos</em> etc. There is no
        current plan to remove this method from the API, and this deprecation is intended to alert developers
        of the danger of using it.
@return the <code>Instant</code> of the time of <em>plag hamincha</em>. If the calculation can't be computed such as
        northern and southern locations even south of the Arctic Circle and north of the Antarctic Circle where
        the sun may not reach low enough below the horizon for this calculation, a <code>null</code> will be
        returned. See detailed explanation on top of the {@link AstronomicalCalendar} documentation.
@see #getShaahZmanis26Degrees()
@see #getPlagHamincha120Minutes()
```

# Human docs

```markdown
Plag hamincha based on the 26-degree day.

10.75 shaos zmaniyos after alos 26 degrees, using a day that begins and ends at 26 degrees.

This zman should be used lechumra only. It can return a very late time, often after shkiah, and using it leniently can lead to chillul Shabbos.

At some northern and southern locations, this zman may not be available if the sun does not reach low enough below the horizon.
```
