# getFixedLocalChatzosHayom

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getFixedLocalChatzosHayom` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:2934)

```javadoc
A method that returns the local time for fixed <em>chatzos</em>. This time is noon and adjusted from
standard time to account for the local latitude. The 360° of the globe divided by 24 calculates to 15°
per hour with 4 minutes per degree, so at a longitude of 0 , 15, 30 etc... <em>Chatzos</em> is at exactly 12:00
noon. This is the time of <em>chatzos</em> according to the <a href=
"https://en.wikipedia.org/wiki/Aruch_HaShulchan">Aruch Hashulchan</a> in <a href=
"https://hebrewbooks.org/pdfpager.aspx?req=7705&pgnum=426">Orach Chaim 233:14</a> and <a href=
"https://en.wikipedia.org/wiki/Moshe_Feinstein">Rabbi Moshe Feinstein</a> in Igros Moshe <a href=
"https://hebrewbooks.org/pdfpager.aspx?req=916&st=&pgnum=67">Orach Chaim 1:24</a> and <a href=
"https://hebrewbooks.org/pdfpager.aspx?req=14675&pgnum=191">2:20</a>.
Lakewood, N.J., with a longitude of -74.222, is 0.778 away from the closest multiple of 15 at -75°. This
is multiplied by 4 to yield 3 minutes and 7 seconds for a <em>chatzos</em> of 11:56:53. This method is not tied
to the theoretical 15° time zones, but will adjust to the actual time zone and <a
href="https://en.wikipedia.org/wiki/Daylight_saving_time">Daylight saving time</a>.

@return the Instant representing the local <em>chatzos</em>
@see GeoLocation#getLocalMeanTimeOffset(Instant)
@see #getLocalMeanTime(LocalTime)
```

# Human docs

```markdown
Fixed local chatzos - clock noon adjusted for the location's longitude and time zone, not tied only to theoretical 15-degree time zones.

The globe is divided into 24 hours over 360 degrees, or 15 degrees per hour (4 minutes per degree). At longitudes 0, 15, 30, and so on, chatzos is exactly 12:00 noon. The result is adjusted to the actual time zone and [daylight saving time](https://en.wikipedia.org/wiki/Daylight_saving_time).

This is the time of chatzos according to the [Aruch Hashulchan](https://en.wikipedia.org/wiki/Aruch_HaShulchan) ([Orach Chaim 233:14](https://hebrewbooks.org/pdfpager.aspx?req=7705&pgnum=426)) and [Rabbi Moshe Feinstein](https://en.wikipedia.org/wiki/Moshe_Feinstein) ([Igros Moshe, Orach Chaim 1:24](https://hebrewbooks.org/pdfpager.aspx?req=916&st=&pgnum=67), [2:20](https://hebrewbooks.org/pdfpager.aspx?req=14675&pgnum=191)).
```
