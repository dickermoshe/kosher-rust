# getTzaisAteretTorah

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getTzaisAteretTorah` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:2555)

```javadoc
This method returns <em>tzais</em> usually calculated as 40 minutes (configurable to any offset via {@link
#setAteretTorahSunsetOffset(double)}) after sunset. Please note that <em>Chacham</em> Yosef Harari-Raful of Yeshivat Ateret
Torah who uses this time, does so only for calculating various other <em>zmanei hayom</em> such as <em>Sof Zman Krias
Shema</em> and <em>Plag Hamincha</em>. His calendars do not publish a <em>zman</em> for <em>Tzais</em>. It should also be
noted that <em>Chacham</em> Harari-Raful provided a 25 minute <em>zman</em> for Israel. This API uses 40 minutes year round in
any place on the globe by default. This offset can be changed by calling {@link #setAteretTorahSunsetOffset(double)}.

@return the <code>Instant</code> representing 40 minutes (configurable via {@link #setAteretTorahSunsetOffset}) after sea
        level sunset. If the calculation can't be computed such as in the Arctic Circle where there is at least one day a year
        where the sun does not rise, and one where it does not set, a <code>null</code> will be returned. See detailed
        explanation on top of the {@link AstronomicalCalendar} documentation.
@see #getAteretTorahSunsetOffset()
@see #setAteretTorahSunsetOffset(double)
```

# Human docs

```markdown
```
