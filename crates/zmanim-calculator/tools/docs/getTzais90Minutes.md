# getTzais90Minutes

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getTzais90Minutes` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:2772)

```javadoc
Method to return <em>tzais</em> (dusk) calculated as 90 minutes after {@link #getSunset() sunset} or {@link
#getSeaLevelSunset() sea level sunset} (depending on the {@link #isUseElevation()} setting). This method returns
<em>tzais</em> (nightfall) based on the opinion of the Magen Avraham that the time to walk the distance of a <a href=
"https://en.wikipedia.org/wiki/Biblical_and_Talmudic_units_of_measurement">mil</a> according to the <a href=
"https://en.wikipedia.org/wiki/Maimonides">Rambam</a>'s opinion is 18 minutes, for a total of 90 minutes based on the
opinion of Ula who calculated <em>tzais</em> as 5 mil after elevation adjusted <em>shkiah</em> (sunset). A similar
calculation {@link #getTzais19Point8Degrees()} uses solar position* calculations based on this time.

@return the <code>Instant</code> representing the time. If the calculation can't be computed such as in the Arctic
        Circle where there is at least one day a year where the sun does not rise, and one where it does not set,
        a <code>null</code> will be returned. See detailed explanation on top of the {@link AstronomicalCalendar}
        documentation.
@see #getTzais19Point8Degrees()
@see #getAlos90Minutes()
```

# Human docs

```markdown
```
