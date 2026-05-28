# getPlagHamincha120Minutes

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getPlagHamincha120Minutes` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:635)

```javadoc
{@summary This method <em>plag hamincha</em> according to the Magen Avraham with the day starting at {@link
#getAlos120Minutes()}. It should be used <em>lechumra</em> only}. This is calculated as* 10.75 {@link
#getShaahZmanis120Minutes()} after {@link #getAlos120Minutes()}. Since the <em>zman</em> based on an extremely early
<em>alos</em> and a very late <em>tzais</em>, it returns a very late time (often after <em>shkiah</em> and can result in
<em>chillul Shabbos</em> etc.) and should only be used <em>lechumra</em>.

@deprecated This method should be used <em>lechumra</em> only since it returns a very late time (often after <em>shkiah</em>),
        and if used <em>lekula</em> can result in <em>chillul Shabbos</em> etc. There is no current plan to remove this method
        from the API, and this deprecation is intended to alert developers of the danger of using it.
@return the <code>Instant</code> of the time of <em>plag hamincha</em>. If the calculation can't be computed such as in the
        Arctic Circle where there is at least one day a year where the sun does not rise, and one where it does not set, a
        <code>null</code> will be returned. See detailed explanation on top of the {@link AstronomicalCalendar} documentation.
@see #getShaahZmanis120Minutes()
@see #getPlagHamincha26Degrees()
```

# Human docs

```markdown
Plag hamincha according to the Magen Avraham, using the 120-minute day.

10.75 shaos zmaniyos after alos 120 minutes before sunrise, using a day that starts 120 minutes before sunrise and ends 120 minutes after sunset.

This zman should be used lechumra only. It can be a very late time, often after shkiah, and using it leniently can lead to chillul Shabbos.

In places where sunrise or sunset cannot be calculated, this zman may not be available.
```
