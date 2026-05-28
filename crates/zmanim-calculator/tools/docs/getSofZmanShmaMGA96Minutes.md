# getSofZmanShmaMGA96Minutes

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getSofZmanShmaMGA96Minutes` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:1187)

```javadoc
This method returns the latest <em>zman krias shema</em> (time to recite Shema in the morning) according to the opinion of
the <a href="https://en.wikipedia.org/wiki/Avraham_Gombiner">Magen Avraham (MGA)</a> based on <em>alos</em> being {@link
#getAlos96Minutes() 96} minutes before {@link #getSunset() sunrise}. This time is 3 {@link
#getShaahZmanis96Minutes() <em>shaos zmaniyos</em>} (solar hours) after {@link #getAlos96Minutes() dawn} based on the opinion
of the MGA that the day is calculated from a {@link #getAlos96Minutes() dawn} of 96 minutes before sunrise to {@link
#getTzais96Minutes() nightfall} of 96 minutes after sunset. This returns the time of 3 * {@link #getShaahZmanis96Minutes()}
after {@link #getAlos96Minutes() dawn}.

@return the <code>Instant</code> of the latest <em>zman krias shema</em>. If the calculation can't be computed such
        as in the Arctic Circle where there is at least one day a year where the sun does not rise, and one where
        it does not set, a <code>null</code> will be returned. See detailed explanation on top of the
        {@link AstronomicalCalendar} documentation.
@see #getShaahZmanis96Minutes()
@see #getAlos96Minutes()
@see #isUseAstronomicalChatzosForOtherZmanim()
```

# Human docs

```markdown
Sof zman krias shema - the latest time to recite morning Shema according to the [Magen Avraham (MGA)](https://en.wikipedia.org/wiki/Avraham_Gombiner), using the 96-minute day.

3 shaos zmaniyos after alos 96 minutes before sunrise, with the day measured from alos 96 minutes before sunrise to tzais 96 minutes after sunset.

In places such as the Arctic Circle, where there is at least one day a year when the sun does not rise and one when it does not set, this zman may not be available or cannot be calculated.
```
