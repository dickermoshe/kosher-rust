# getSofZmanShmaMGA90Minutes

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getSofZmanShmaMGA90Minutes` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:1144)

```javadoc
This method returns the latest <em>zman krias shema</em> (time to recite <em>Shema</em> in the morning) according
to the opinion of the <a href="https://en.wikipedia.org/wiki/Avraham_Gombiner">Magen Avraham (MGA)</a> based on
<em>alos</em> being {@link #getAlos90Minutes() 90} minutes before {@link #getSunset() sunrise}. This time is 3
{@link #getShaahZmanis90Minutes() <em>shaos zmaniyos</em>} (solar hours) after {@link #getAlos90Minutes() dawn} based on
the opinion of the MGA that the day is calculated from a {@link #getAlos90Minutes() dawn} of 90 minutes before sunrise to
{@link #getTzais90Minutes() nightfall} of 90 minutes after sunset. This returns the time of 3 *
{@link #getShaahZmanis90Minutes()} after {@link #getAlos90Minutes() dawn}.

@return the <code>Instant</code> of the latest <em>zman krias shema</em>. If the calculation can't be computed such
        as in the Arctic Circle where there is at least one day a year where the sun does not rise, and one where
        it does not set, a <code>null</code> will be returned. See detailed explanation on top of the
        {@link AstronomicalCalendar} documentation.
@see #getShaahZmanis90Minutes()
@see #getAlos90Minutes()
@see #isUseAstronomicalChatzosForOtherZmanim()
```

# Human docs

```markdown
```
