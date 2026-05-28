# getBainHashmashosRT13Point5MinutesBefore7Point083Degrees

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getBainHashmashosRT13Point5MinutesBefore7Point083Degrees` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:2105)

```javadoc
This method returns the beginning of <em>bain hashmashos</em> based on the calculation of 13.5 minutes (3/4 of an
18-minute <a href="https://en.wikipedia.org/wiki/Biblical_and_Talmudic_units_of_measurement">mil</a>) before
<em>shkiah</em> calculated as {@link #getTzaisGeonim7Point083Degrees() 7.083°}.

@return the <code>Instant</code> of the <em>bain hashmashos</em> of Rabbeinu Tam in this calculation. If the
        calculation can't be computed such as northern and southern locations even south of the Arctic Circle and
        north of the Antarctic Circle where the sun may not reach low enough below the horizon for this
        calculation, a <code>null</code> will be returned. See detailed explanation on top of the
        {@link AstronomicalCalendar} documentation.
@see #getTzaisGeonim7Point083Degrees()
```

# Human docs

```markdown
The beginning of Rabbeinu Tam's bain hashmashos.

13.5 minutes (3/4 of an 18-minute [mil](https://en.wikipedia.org/wiki/Biblical_and_Talmudic_units_of_measurement)) before shkiah calculated at 7.083 degrees below the horizon.

At some northern and southern locations, including places even south of the Arctic Circle and north of the Antarctic Circle, this zman may not be available or cannot be calculated if the sun does not reach low enough below the horizon.
```
