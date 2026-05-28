# getZmanMolad

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getZmanMolad` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:3160)

```javadoc
Returns the point in time of <em>Molad</em> as a <code>Instant</code> Object. For the traditional day of week, hour,
minute and chalakim, {@link JewishCalendar#getMoladAsInstant()} and the not yet completed
{@link com.kosherjava.zmanim.hebrewcalendar.HebrewDateFormatter} that will have formatting for this.

@return the Instant representing the moment of the molad. If the <em>molad</em> does not occur on this day, a
        <code>null</code> will be returned.
@see #getTchilasZmanKidushLevana3Days()
@see #getTchilasZmanKidushLevana7Days(Instant, Instant)
@see JewishCalendar#getMoladAsInstant()
```

# Human docs

```markdown
The molad for the Hebrew month.

The traditional molad time, calculated from the Hebrew calendar's day, hour, minute, and chalakim for the month.

The time is expressed as a precise moment based on Yerushalayim standard time, adjusted for the traditional location used for the molad.
```
