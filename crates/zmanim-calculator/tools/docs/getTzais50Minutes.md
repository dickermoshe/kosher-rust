# getTzais50Minutes

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getTzais50Minutes` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:3796)

```javadoc
Method to return <em>tzais</em> (dusk) calculated as 50 minutes after {@link #getSunset() sunset} or {@link
#getSeaLevelSunset() sea level sunset} (depending on the {@link #isUseElevation()} setting). This method returns
<em>tzais</em> (nightfall) based on the opinion of Rabbi Moshe Feinstein for the New York area. This time should
not be used for latitudes other than ones similar to the latitude of the NY area.

@return the <code>Instant</code> representing the time. If the calculation can't be computed such as in the Arctic
        Circle where there is at least one day a year where the sun does not rise, and one where it does not set,
        a <code>null</code> will be returned. See detailed explanation on top of the {@link AstronomicalCalendar}
        documentation.
```

# Human docs

```markdown
```
