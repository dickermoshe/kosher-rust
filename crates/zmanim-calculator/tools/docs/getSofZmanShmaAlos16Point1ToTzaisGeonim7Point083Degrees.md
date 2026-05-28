# getSofZmanShmaAlos16Point1ToTzaisGeonim7Point083Degrees

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getSofZmanShmaAlos16Point1ToTzaisGeonim7Point083Degrees` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:1303)

```javadoc
This method returns the latest <em>zman krias shema</em> (time to recite Shema in the morning) based on the
opinion that the day starts at {@link #getAlos16Point1Degrees() <em>alos</em> 16.1°} and ends at
{@link #getTzaisGeonim7Point083Degrees() <em>tzais</em> 7.083°}. 3 <em>shaos zmaniyos</em> are calculated
based on this day and added to {@link #getAlos16Point1Degrees() <em>alos</em>} to reach this time. This time is 3
<em>shaos zmaniyos</em> (temporal hours) after {@link #getAlos16Point1Degrees() <em>alos</em> 16.1°} based on
the opinion that the day is calculated from a {@link #getAlos16Point1Degrees() <em>alos</em> 16.1°} to
{@link #getTzaisGeonim7Point083Degrees() <em>tzais</em> 7.083°}.
<b>Note: </b> Based on this calculation <em>chatzos</em> will not be at midday and {@link
#isUseAstronomicalChatzosForOtherZmanim()} will be ignored.

@return the <code>Instant</code> of the latest <em>zman krias shema</em> based on this calculation. If the
        calculation can't be computed such as northern and southern locations even south of the Arctic Circle and
        north of the Antarctic Circle where the sun may not reach low enough below the horizon for this
        calculation, a <code>null</code> will be returned. See detailed explanation on top of the
        {@link AstronomicalCalendar} documentation.
@see #getAlos16Point1Degrees()
@see #getTzaisGeonim7Point083Degrees()
```

# Human docs

```markdown
```
