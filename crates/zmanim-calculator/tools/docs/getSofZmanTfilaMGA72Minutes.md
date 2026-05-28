# getSofZmanTfilaMGA72Minutes

Source: `com.kosherjava.zmanim.ZmanimCalendar.getSofZmanTfilaMGA72Minutes` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ZmanimCalendar.java:803)

```javadoc
This method returns the latest <em>zman tfila</em> (time to recite shema in the morning) that is 4 * {@link
getShaahZmanis72Minutes() <em>shaos zmaniyos</em>} (solar hours) after {@link getAlos72Minutes()}, according to the
<em><a href="https://en.wikipedia.org/wiki/Avraham_Gombiner">Magen Avraham (MGA)</a></em>. The day is calculated
from 72 minutes before {@link getSunriseBasedOnElevationSetting()} to 72 minutes after {@link
getSunsetBasedOnElevationSetting()}. The use of elevation depends on the {@link isUseElevation()} setting).

@return the <code>Instant</code> of the latest <em>zman tfila</em>. If the calculation can't be computed such as in
        the Arctic Circle where there is at least one day a year where the sun does not rise, and one where it
        does not set, a <code>null</code> will be returned. See detailed explanation on top of the
        {@link AstronomicalCalendar} documentation.
@see getSofZmanTfila(Instant, Instant, boolean)
@see getShaahZmanis72Minutes()
@see getAlos72Minutes()
```

# Human docs

```markdown
```
