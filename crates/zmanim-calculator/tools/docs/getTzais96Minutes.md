# getTzais96Minutes

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getTzais96Minutes` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:2911)

```javadoc
A method to return <em>tzais</em> (dusk) calculated as 96 minutes after {@link #getSunset() sunset} or {@link
#getSeaLevelSunset() sea level sunset} (depending on the {@link #isUseElevation()} setting). For information on how
this is calculated see the comments on {@link #getAlos96Minutes()}.

@return the <code>Instant</code> representing the time. If the calculation can't be computed such as in the Arctic
        Circle where there is at least one day a year where the sun does not rise, and one where it does not set,
        a <code>null</code> will be returned. See detailed explanation on top of the {@link AstronomicalCalendar}
        documentation.
@see #getAlos96Minutes()
```

# Human docs

```markdown
```
