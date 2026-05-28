# getSofZmanTfilaMGA90MinutesZmanis

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getSofZmanTfilaMGA90MinutesZmanis` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:1423)

```javadoc
This method returns the latest <em>zman tfila</em> (time to the morning prayers) according to the opinion of the <a href=
"https://en.wikipedia.org/wiki/Avraham_Gombiner">Magen Avraham (MGA)</a> based on <em>alos</em> being {@link
#getAlos90Zmanis() 90} minutes <em>zmaniyos</em> before {@link #getSunset() sunrise}. This time is 4 {@link
#getShaahZmanis90MinutesZmanis() <em>shaos zmaniyos</em>} (solar hours) after {@link #getAlos90Zmanis() dawn} based on the
opinion of the MGA that the day is calculated from a {@link #getAlos90Zmanis() dawn} of 90 minutes <em>zmaniyos</em> before
sunrise to {@link #getTzais90Zmanis() nightfall} of 90 minutes <em>zmaniyos</em> after sunset. This returns the time of 4 *
{@link #getShaahZmanis90MinutesZmanis()} after {@link #getAlos90Zmanis() dawn}.

@return the <code>Instant</code> of the latest <em>zman krias shema</em>. If the calculation can't be computed such
        as in the Arctic Circle where there is at least one day a year where the sun does not rise, and one where
        it does not set, a <code>null</code> will be returned. See detailed explanation on top of the
        {@link AstronomicalCalendar} documentation.
@see #getShaahZmanis90MinutesZmanis()
@see #getAlos90Zmanis()
```

# Human docs

```markdown
```
