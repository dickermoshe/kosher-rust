# getSofZmanTfilaMGA72MinutesZmanis

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getSofZmanTfilaMGA72MinutesZmanis` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:1383)

```javadoc
This method returns the latest <em>zman tfila</em> (time to the morning prayers) according to the opinion of the
<a href="https://en.wikipedia.org/wiki/Avraham_Gombiner">Magen Avraham (MGA)</a> based on <em>alos</em>
being {@link #getAlos72Zmanis() 72} minutes <em>zmaniyos</em> before {@link #getSunset() sunrise}. This time is 4
{@link #getShaahZmanis72MinutesZmanis() <em>shaos zmaniyos</em>} (solar hours) after {@link #getAlos72Zmanis() dawn}
based on the opinion of the MGA that the day is calculated from a {@link #getAlos72Zmanis() dawn} of 72
minutes <em>zmaniyos</em> before sunrise to {@link #getTzais72Zmanis() nightfall} of 72 minutes <em>zmaniyos</em>
after sunset. This returns the time of 4 * {@link #getShaahZmanis72MinutesZmanis()} after {@link #getAlos72Zmanis() dawn}.

@return the <code>Instant</code> of the latest <em>zman krias shema</em>. If the calculation can't be computed such
        as in the Arctic Circle where there is at least one day a year where the sun does not rise, and one where
        it does not set, a <code>null</code> will be returned. See detailed explanation on top of the
        {@link AstronomicalCalendar} documentation.
@see #getShaahZmanis72MinutesZmanis()
@see #getAlos72Zmanis()
```

# Human docs

```markdown
Sof zman tfila - the latest time to recite morning prayers (Shacharis) according to the [Magen Avraham (MGA)](https://en.wikipedia.org/wiki/Avraham_Gombiner), using the 72 zmaniyos-minute day.

4 shaos zmaniyos after alos 72 zmaniyos minutes before sunrise, with the day measured from that alos to tzais 72 zmaniyos minutes after sunset.

In places such as the Arctic Circle, where there is at least one day a year when the sun does not rise and one when it does not set, this zman may not be available or cannot be calculated.
```
