# getSofZmanBiurChametzMGA72MinutesZmanis

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getSofZmanBiurChametzMGA72MinutesZmanis` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:3372)

```javadoc
This method returns the latest time for burning <em>chametz</em> on <em>Erev Pesach</em> according to the opinion of the
<a href="https://en.wikipedia.org/wiki/Avraham_Gombiner">Magen Avraham (MGA)</a> based on <em>alos</em> being {@link
#getAlos72Zmanis() 72} minutes zmanis before {@link #getSunset() sunrise}. This time is 5 {@link
#getShaahZmanis72MinutesZmanis() <em>shaos zmaniyos</em>} (temporal hours) after {@link #getAlos72Zmanis() dawn} based on the
opinion of the MGA that the day is calculated from a {@link #getAlos72Zmanis() dawn} of 72 minutes zmanis before sunrise to
{@link #getTzais72Zmanis() nightfall} of 72 minutes zmanis after sunset. This returns the time of 5 * {@link
#getShaahZmanis72MinutesZmanis()} after {@link #getAlos72Zmanis() dawn}. If it is not  <em>erev Pesach</em>, a
<code>null</code> will be returned.
@return the <code>Instant</code> of the latest time for burning <em>chametz</em> on <em>Erev Pesach</em>. If it is not
        <em>erev Pesach</em> or the calculation can't be computed such as in the Arctic Circle where there is at
        least one day a year where the sun does not rise, and one where it does not set, a <code>null</code> will be
        returned. See detailed explanation on top of the {@link AstronomicalCalendar} documentation.
@see #getShaahZmanis72MinutesZmanis()
@see #getAlos72Zmanis()
@see #getSofZmanBiurChametz(Instant, Instant, boolean)
```

# Human docs

```markdown
Sof zman biur chametz - the latest time to burn chametz on Erev Pesach according to the [Magen Avraham (MGA)](https://en.wikipedia.org/wiki/Avraham_Gombiner), using the 72 zmaniyos-minute day.

5 shaos zmaniyos after alos 72 zmaniyos minutes before sunrise, with the day measured from alos 72 zmaniyos minutes before sunrise to tzais 72 zmaniyos minutes after sunset.
```
