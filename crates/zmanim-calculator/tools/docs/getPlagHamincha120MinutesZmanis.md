# getPlagHamincha120MinutesZmanis

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getPlagHamincha120MinutesZmanis` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:614)

```javadoc
This method should be used <em>lechumra</em> only and returns the time of <em>plag hamincha</em> based on sunrise being 120
minutes <em>zmaniyos</em> or 1/6th of the day before sunrise. This is calculated as 10.75 hours after {@link
#getAlos120Zmanis() dawn}. The formula used is 10.75 * {@link #getShaahZmanis120MinutesZmanis()} after {@link
#getAlos120Zmanis() dawn}. Since the <em>zman</em> based on an extremely early <em>alos</em> and a very late <em>tzais</em>,
it should only be used <em>lechumra</em>.

@deprecated This method should be used <em>lechumra</em> only since it returns a very late time (often after <em>shkiah</em>),
        and if used <em>lekula</em> can result in <em>chillul Shabbos</em> etc. There is no current plan to remove this method
        from the API, and this deprecation is intended to alert developers of the danger of using it.
@return the <code>Instant</code> of the time of <em>plag hamincha</em>. If the calculation can't be computed such as in the
        Arctic Circle where there is at least one day a year where the sun does not rise, and one where it does not set, a
        <code>null</code> will be returned. See detailed explanation on top of the {@link AstronomicalCalendar} documentation.
@see #getShaahZmanis120MinutesZmanis()
@see #getAlos120Minutes()
@see #getTzais120Minutes()
@see #getPlagHamincha26Degrees()
@see #getPlagHamincha120Minutes()
```

# Human docs

```markdown
Plag hamincha based on alos 120 zmaniyos minutes (one-sixth of the day) before sunrise.

10.75 shaos zmaniyos after that alos.

This zman should be used lechumra only. It can be a very late time, often after shkiah, and using it leniently can lead to chillul Shabbos.

In places where sunrise or sunset cannot be calculated, this zman may not be available.
```
