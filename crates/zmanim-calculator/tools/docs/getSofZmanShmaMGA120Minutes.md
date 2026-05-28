# getSofZmanShmaMGA120Minutes

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getSofZmanShmaMGA120Minutes` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:1256)

```javadoc
This method returns the latest <em>zman krias shema</em> (time to recite Shema in the morning) according to the opinion of the
<a href="https://en.wikipedia.org/wiki/Avraham_Gombiner">Magen Avraham (MGA)</a> based on <em>alos</em> being {@link
#getAlos120Minutes() 120} minutes or 1/6th of the day before {@link #getSunset() sunrise}. This time is 3 {@link
#getShaahZmanis120Minutes() <em>shaos zmaniyos</em>} (solar hours) after {@link #getAlos120Minutes() dawn} based on the
opinion of the MGA that the day is calculated from a {@link #getAlos120Minutes() dawn} of 120 minutes before sunrise to {@link
#getTzais120Minutes() nightfall} of 120 minutes after sunset. This returns the time of 3 {@link #getShaahZmanis120Minutes()}
after {@link #getAlos120Minutes() dawn}. This is an extremely early <em>zman</em> that is very much a <em>chumra</em>.

@return the <code>Instant</code> of the latest <em>zman krias shema</em>. If the calculation can't be computed such
        as in the Arctic Circle where there is at least one day a year where the sun does not rise, and one where
        it does not set, a <code>null</code> will be returned. See detailed explanation on top of the
        {@link AstronomicalCalendar} documentation.
@see #getShaahZmanis120Minutes()
@see #getAlos120Minutes()
@see #isUseAstronomicalChatzosForOtherZmanim()
```

# Human docs

```markdown
Sof zman krias shema - the latest time to recite morning Shema according to the [Magen Avraham (MGA)](https://en.wikipedia.org/wiki/Avraham_Gombiner), using the 120-minute day. This is an extremely early time, used as a chumra.

3 shaos zmaniyos after alos 120 minutes before sunrise, with the day measured from alos 120 minutes before sunrise to tzais 120 minutes after sunset.

In places such as the Arctic Circle, where there is at least one day a year when the sun does not rise and one when it does not set, this zman may not be available or cannot be calculated.
```
