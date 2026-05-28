# getSofZmanAchilasChametzMGA72MinutesZmanis

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getSofZmanAchilasChametzMGA72MinutesZmanis` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:3292)

```javadoc
This method returns the latest time one is allowed eating <em>chametz</em> on <em>Erev Pesach</em> according to the opinion
of the <a href="https://en.wikipedia.org/wiki/Avraham_Gombiner">Magen Avraham (MGA)</a> based on <em>alos</em> being {@link
#getAlos72Zmanis() 72 zmaniyos} minutes before {@link #getSunset() sunrise}. This time is identical to the
{@link #getSofZmanTfilaMGA72MinutesZmanis() <em>Sof zman tfilah</em> MGA 72 minutes zmanis}. This time is 4 {@link
#getShaahZmanis72MinutesZmanis() <em>shaos zmaniyos</em>} (temporal hours) after {@link #getAlos72Minutes() dawn} based on the
opinion of the MGA that the day is calculated from a {@link #getAlos72Zmanis() dawn} of 72 minutes zmanis before sunrise to
{@link #getTzais72Zmanis() nightfall} of 72 minutes zmanis after sunset. This returns the time of 4 * {@link
#getShaahZmanis72MinutesZmanis()} after {@link #getAlos72Zmanis() dawn}. If it is not <em>erev Pesach</em>, a <code>null</code>
will be
returned.

@return the <code>Instant</code> of the latest time of eating <em>chametz</em>. If it is not <em>erev Pesach</em> or the
        calculation can't be computed such as in the Arctic Circle where there is at least one day a year where the sun does
        not rise, and one where it does not set, a <code>null</code> will be returned. See detailed explanation on top of
        the {@link AstronomicalCalendar} documentation.
@see #getShaahZmanis72MinutesZmanis()
@see #getAlos72Zmanis()
@see #getSofZmanTfilaMGA72MinutesZmanis()
@see #getSofZmanAchilasChametz(Instant, Instant, boolean)
```

# Human docs

```markdown
```
