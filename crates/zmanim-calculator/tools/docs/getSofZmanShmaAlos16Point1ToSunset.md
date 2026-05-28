# getSofZmanShmaAlos16Point1ToSunset

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getSofZmanShmaAlos16Point1ToSunset` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:1280)

```javadoc
This method returns the latest <em>zman krias shema</em> (time to recite <em>Shema</em> in the morning) based on the
opinion that the day starts at {@link #getAlos16Point1Degrees() <em>alos</em> 16.1°} and ends at {@link
#getSeaLevelSunset() sea level sunset}. This is the opinion of the <a href=
"https://hebrewbooks.org/40357">חידושי וכללות הרז״ה</a> and the <a href="https://hebrewbooks.org/14799">מנורה הטהורה</a> as
mentioned by Yisrael Vehazmanim <a href="https://hebrewbooks.org/pdfpager.aspx?req=9765&pgnum=81">vol 1, sec. 7,
ch. 3 no. 16</a>. Three <em>shaos zmaniyos</em> are calculated based on this day and added to {@link
#getAlos16Point1Degrees() <em>alos</em>} to reach this time. This time is 3 <em>shaos zmaniyos</em> (solar hours)
after {@link #getAlos16Point1Degrees() dawn} based on the opinion that the day is calculated from a {@link
#getAlos16Point1Degrees() <em>alos</em> 16.1°} to {@link #getSeaLevelSunset() sea level sunset}.
<b>Note: </b> Based on this calculation <em>chatzos</em> will not be at midday and {@link
#isUseAstronomicalChatzosForOtherZmanim()} will be ignored.

@return the <code>Instant</code> of the latest <em>zman krias shema</em> based on this day. If the calculation can't
        be computed such as northern and southern locations even south of the Arctic Circle and north of the
        Antarctic Circle where the sun may not reach low enough below the horizon for this calculation, a null
        will be returned. See detailed explanation on top of the {@link AstronomicalCalendar} documentation.
@see #getAlos16Point1Degrees()
@see #getSeaLevelSunset()
```

# Human docs

```markdown
```
