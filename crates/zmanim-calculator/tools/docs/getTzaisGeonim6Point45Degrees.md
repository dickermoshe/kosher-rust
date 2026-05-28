# getTzaisGeonim6Point45Degrees

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getTzaisGeonim6Point45Degrees` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:2427)

```javadoc
This method returns the <em>tzais</em> (nightfall) based on the opinion of the <em>Geonim</em> as calculated by <a href=
"https://en.wikipedia.org/wiki/Yechiel_Michel_Tucazinsky">Rabbi Yechiel Michel Tucazinsky</a> as the position of
the sun no later than 31 minutes after sea-level sunset in Jerusalem (the Birur halacha shows that Rav Tucazinsky's
calculations for sunset, listed as 28 minutes in this case, were about 3 minutes later than reality), and at the height of
the summer solstice,this <em>zman</em>, calculatons show that 30.75 minutes after <em>shkiah</em> computes to 6.45° below
{@link GEOMETRIC_ZENITH geometric zenith}. This calculation is found in the <a href=
"https://hebrewbooks.org/pdfpager.aspx?req=50536&st=&pgnum=51">Birur Halacha Yoreh Deah 262</a> and it is the commonly used
<em>zman</em> in Israel. It is also used in the <a href="https://www.worldcat.org/oclc/243303103">Luach Itim Lebinah</a>. it
should be noted that this differs from the 6.1° / 6.2° calculation for Rabbi Tucazinsky's time as calculated by the
Hazmanim Bahalacha Vol II chapter 50:7 (page 515). Calculations show that 6.45° at the equinox is 26.5 minutes after
<em>shkiah</em> <a href="https://kosherjava.com/2022/01/12/equinox-vs-equilux-zmanim-calculations/">around the equinox /
equilux</a>.

@return the <code>Instant</code> representing the time when the sun is 6.45° below sea level. If the
        calculation can't be computed such as northern and southern locations even south of the Arctic Circle and
        north of the Antarctic Circle where the sun may not reach low enough below the horizon for this
        calculation, a <code>null</code> will be returned. See detailed explanation on top of the
        {@link AstronomicalCalendar} documentation.
@see ZENITH_6_POINT_45
```

# Human docs

```markdown
Tzais (nightfall) according to the Geonim - when the sun is 6.45 degrees below the western horizon after sunset.

Commonly used in Israel. Based on [Rabbi Yechiel Michel Tucazinsky's](https://en.wikipedia.org/wiki/Yechiel_Michel_Tucazinsky) calculation, about 31 minutes after sea level sunset in Jerusalem and about 26.5 minutes at the equinox. Also used in [Luach Itim Lebinah](https://www.worldcat.org/oclc/243303103). See [Birur Halacha Yoreh Deah 262](https://hebrewbooks.org/pdfpager.aspx?req=50536&st=&pgnum=51).

At some northern and southern locations, this zman may not be available or cannot be calculated if the sun does not reach low enough below the horizon.
```
