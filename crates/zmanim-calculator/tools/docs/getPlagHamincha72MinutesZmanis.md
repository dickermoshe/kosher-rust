# getPlagHamincha72MinutesZmanis

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getPlagHamincha72MinutesZmanis` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:1883)

```javadoc
This method should be used <em>lechumra</em> only and returns the time of <em>plag hamincha</em>. This is calculated as
10.75 hours after {@link #getAlos72Zmanis()}. The formula used is 10.75 * {@link #getShaahZmanis72MinutesZmanis()} after
{@link #getAlos72Zmanis() dawn}. Since <em>plag</em> by this calculation can occur after sunset, it should only be used
<em>lechumra</em>.

@deprecated This method should be used <em>lechumra</em> only since it returns a very late time (often after
        <em>shkiah</em>), and if used <em>lekula</em> can result in <em>chillul Shabbos</em> etc. There is no
        current plan to remove this method from the API, and this deprecation is intended to alert developers
        of the danger of using it.
@return the <code>Instant</code> of the time of <em>plag hamincha</em>. If the calculation can't be computed such as
        in the Arctic Circle where there is at least one day a year where the sun does not rise, and one where it
        does not set, a <code>null</code> will be returned. See detailed explanation on top of the
        {@link AstronomicalCalendar} documentation.
```

# Human docs

```markdown
Plag hamincha based on alos 72 zmaniyos minutes before sunrise.

10.75 shaos zmaniyos after that dawn.

This zman can return a very late time, often after shkiah, avoid using it leniently as it can lead to chillul Shabbos. One should not use this time lechumra.

In places such as the Arctic Circle, where there is at least one day a year when the sun does not rise and one when it does not set, this zman may not be available or cannot be calculated.
```
