# getSofZmanTfilaMGA120Minutes

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getSofZmanTfilaMGA120Minutes` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:1485)

```javadoc
This method returns the latest <em>zman tfila</em> (time to recite the morning prayers) according to the opinion
of the <a href="https://en.wikipedia.org/wiki/Avraham_Gombiner">Magen Avraham (MGA)</a> based on
<em>alos</em> being {@link #getAlos120Minutes() 120} minutes before {@link #getSunset() sunrise} . This time is 4
{@link #getShaahZmanis120Minutes() <em>shaos zmaniyos</em>} (solar hours) after {@link #getAlos120Minutes() dawn}
based on the opinion of the MGA that the day is calculated from a {@link #getAlos120Minutes() dawn} of 120
minutes before sunrise to {@link #getTzais120Minutes() nightfall} of 120 minutes after sunset. This returns the time of
4 * {@link #getShaahZmanis120Minutes()} after {@link #getAlos120Minutes() dawn}. This is an extremely early <em>zman</em>
that is very much a <em>chumra</em>.

@return the <code>Instant</code> of the latest <em>zman krias shema</em>. If the calculation can't be computed such
        as in the Arctic Circle where there is at least one day a year where the sun does not rise, and one where
        it does not set, a <code>null</code> will be returned. See detailed explanation on top of the
        {@link AstronomicalCalendar} documentation.
@see #getShaahZmanis120Minutes()
@see #getAlos120Minutes()
```

# Human docs

```markdown
```
