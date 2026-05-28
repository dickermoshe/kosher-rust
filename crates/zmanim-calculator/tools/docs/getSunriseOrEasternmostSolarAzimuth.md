# getSunriseOrEasternmostSolarAzimuth

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getSunriseOrEasternmostSolarAzimuth` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:3914)

```javadoc
{@summary A method that returns {@link #getSunriseBasedOnElevationSetting() sunrise} if it occurs, or the time that the sun
is at its easternmost position (azimuth of 90°), if sunrise will not occur that day. In Polar regions (the Arctic or Antarctic
circles), there are days of no sunrise or sunset, and there are opinions that during these periods, the day-night boundary is
when the sun is at its westernmost position}. Sunset in this opinion is when the sun is at {@link
#getSunsetOrWesternmostSolarAzimuth() azimuth 270°, its westernmost position}. This is the opinion of <a href=
"https://en.wikipedia.org/wiki/Joseph_Schwarz_(geographer)">Rabbi Yehosef Schwarz</a> in his <a href=
"https://hebrewbooks.org/pdfpager.aspx?req=31703&pgnum=134">דברי יוסף – דרך מבוא השמש</a> and <a href=
"https://hebrewbooks.org/pdfpager.aspx?req=159&pgnum=83">דברי יוסף – תשובות, שאלה ח׳</a>. This is brought down <em>lehalacha</em>
by The <a href="https://en.wikipedia.org/wiki/Yosef_Hayyim">Ben Ish Chai</a> in the <a href=
"https://hebrewbooks.org/pdfpager.aspx?req=1401&pgnum=461">רב פעלים – חלק ב׳, סוד ישרים ס׳ ד׳</a>. This time is close to six hours
before {@link #getSunTransit() astronomical <em>chatzos hayom</em>}, but depending on the time of year and location in the
 Arctic / Antarctic, it can be up to 46 minutes before or after this time.

@return sunrise if it occurs, or the time that the sun will reach its easternmost position (azimuth 90°), if sunrise will
        not occur that day. If there is no sunrise this day, and the azimuth 90° will not occur, a <code>null</code> will
        be returned.
@see #getSunsetOrWesternmostSolarAzimuth()
@see #getTimeAtAzimuth(double)
@see com.kosherjava.zmanim.util.AstronomicalCalculator#getTimeAtAzimuth(LocalDate, GeoLocation, double)
```

# Human docs

```markdown
Sunrise when it occurs, or the time when the sun reaches its easternmost position (azimuth 90 degrees) on days when sunrise does not occur.

In polar regions, where there are days with no sunrise or sunset, some opinions treat the day-night boundary as when the sun is at its easternmost or westernmost position. This follows [Rabbi Yehosef Schwarz](https://en.wikipedia.org/wiki/Joseph_Schwarz_(geographer)) in [Devarim Yosef - Derech Mevo Hashemesh](https://hebrewbooks.org/pdfpager.aspx?req=31703&pgnum=134) and [Devarim Yosef - Teshuvot, She'elah 8](https://hebrewbooks.org/pdfpager.aspx?req=159&pgnum=83), brought lehalacha by the [Ben Ish Chai](https://en.wikipedia.org/wiki/Yosef_Hayyim) in [Rav Pe'alim, chelek 2, Sod Yesharim siman 4](https://hebrewbooks.org/pdfpager.aspx?req=1401&pgnum=461). This time is close to six hours before astronomical chatzos, but depending on the season and location in the Arctic or Antarctic, it can be up to 46 minutes earlier or later.

If there is no sunrise that day and the sun does not reach azimuth 90 degrees, this zman may not be available or cannot be calculated.
```
