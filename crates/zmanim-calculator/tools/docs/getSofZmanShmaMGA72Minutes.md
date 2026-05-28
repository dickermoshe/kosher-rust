# getSofZmanShmaMGA72Minutes

Source: `com.kosherjava.zmanim.ZmanimCalendar.getSofZmanShmaMGA72Minutes` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ZmanimCalendar.java:589)

```javadoc
This method returns the latest <em>zman krias shema</em> (time to recite Shema in the morning) according to the opinion of
the <a href="https://en.wikipedia.org/wiki/Avraham_Gombiner">Magen Avraham (MGA)</a> based on <em>alos</em> being {@link
getAlos72Minutes() 72} minutes before {@link getSunrise() sunrise}. This time is 3 {@link
getShaahZmanis72Minutes() <em>shaos zmaniyos</em>} (solar hours) after {@link getAlos72Minutes() dawn} based on the opinion
of the MGA that the day is calculated from a {@link getAlos72Minutes() dawn} of 72 minutes before sunrise to
{@link getTzais72Minutes() nightfall} of 72 minutes after sunset. This returns the time of 3 * {@link
getShaahZmanis72Minutes()} after {@link getAlos72Minutes() dawn}.

@return the <code>Instant</code> of the latest <em>zman krias shema</em>. If the calculation can't be computed such
        as in the Arctic Circle where there is at least one day a year where the sun does not rise, and one where
        it does not set, a <code>null</code> will be returned. See detailed explanation on top of the
        {@link AstronomicalCalendar} documentation.
@see isUseAstronomicalChatzosForOtherZmanim()
@see getShaahZmanis72Minutes()
@see getAlos72Minutes()
@see getSofZmanShmaMGA72Minutes()
@see getSofZmanShma(Instant, Instant, boolean)
```

# Human docs

```markdown
Sof zman krias shema - the latest time to recite morning Shema according to the [Magen Avraham (MGA)](https://en.wikipedia.org/wiki/Avraham_Gombiner), using the 72-minute day.

3 shaos zmaniyos after alos 72 minutes before sunrise, with the day measured from alos 72 minutes before sunrise to tzais 72 minutes after sunset.

In places such as the Arctic Circle, where there is at least one day a year when the sun does not rise and one when it does not set, this zman may not be available or cannot be calculated.
```
