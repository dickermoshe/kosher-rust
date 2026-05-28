# getEndCivilTwilight

Source: `com.kosherjava.zmanim.AstronomicalCalendar.getEndCivilTwilight` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\AstronomicalCalendar.java:247)

```javadoc
A method that returns the end of <a href="https://en.wikipedia.org/wiki/Twilight#Civil_twilight">civil twilight</a>
using a zenith of {@link CIVIL_ZENITH 96&deg;}.

@return The <code>Instant</code> of the end of civil twilight using a zenith of {@link CIVIL_ZENITH 96&deg;}. If the
        calculation can't be computed, <code>null</code> will be returned. See detailed explanation on top of the page.
```

# Human docs

```markdown
The end of [civil twilight](https://en.wikipedia.org/wiki/Twilight#Civil_twilight) in the evening.

Calculated using a zenith of 96 degrees.

This zman may not be available or cannot be calculated when the computation cannot be performed.
```
