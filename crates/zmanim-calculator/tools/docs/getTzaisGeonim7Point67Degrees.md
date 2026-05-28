# getTzaisGeonim7Point67Degrees

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getTzaisGeonim7Point67Degrees` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:2481)

```javadoc
This method returns <em>tzais</em> (nightfall) based on the opinion of the <em>Geonim</em> calculated as 45 minutes
after sunset during the summer solstice in New York, when the <em>neshef</em> (twilight) is the longest. The sun's
position at this time computes to {@link ZENITH_7_POINT_67 7.75°} below the western horizon. See <a href=
"https://hebrewbooks.org/pdfpager.aspx?req=921&pgnum=149">Igros Moshe Even Haezer 4, Ch. 4</a> (regarding
<em>tzais</em> for <em>krias Shema</em>). It is also mentioned in Rabbi Heber's <a href=
"https://hebrewbooks.org/53000">Shaarei Zmanim</a> on in
<a href="https://hebrewbooks.org/pdfpager.aspx?req=53055&pgnum=101">chapter 10 (page 87)</a> and
<a href="https://hebrewbooks.org/pdfpager.aspx?req=53055&pgnum=122">chapter 12 (page 108)</a>. Also see the
time of 45 minutes in <a href="https://en.wikipedia.org/wiki/Simcha_Bunim_Cohen">Rabbi Simcha Bunim Cohen's</a> <a
href="https://www.worldcat.org/oclc/179728985">The radiance of Shabbos</a> as the earliest <em>zman</em> for New York.
This <em>zman</em> is also listed in the <a href="https://hebrewbooks.org/pdfpager.aspx?req=1927&pgnum=90">Divrei
Shalom Vol. III, chapter 75</a>, and <a href="https://hebrewbooks.org/pdfpager.aspx?req=892&pgnum=431">Bais Av"i
Vol. III, chapter 117</a>. This <em>zman</em> is also listed in the Divrei Shalom etc. chapter 177. Since this
<em>zman</em> depends on the level of light, Rabbi Yaakov Shakow presented this degree-based calculation to Rabbi
<a href="https://en.wikipedia.org/wiki/Shmuel_Kamenetsky">Rabbi Shmuel Kamenetsky</a> who agreed to it.
@todo add hyperlinks to source of Divrei Shalom once it is located.
@return the <code>Instant</code> representing the time when the sun is 7.67° below sea level. If the
        calculation can't be computed such as northern and southern locations even south of the Arctic Circle and
        north of the Antarctic Circle where the sun may not reach low enough below the horizon for this
        calculation, a <code>null</code> will be returned. See detailed explanation on top of the
        {@link AstronomicalCalendar} documentation.
@see ZENITH_7_POINT_67
```

# Human docs

```markdown
Tzais (nightfall) according to the Geonim - when the sun is 7.67 degrees below the western horizon after sunset.

Corresponds to 45 minutes after sunset during the summer solstice in New York, when twilight is longest. Cited in [Igros Moshe Even Haezer 4, ch. 4](https://hebrewbooks.org/pdfpager.aspx?req=921&pgnum=149) regarding tzais for krias shema, and in Rabbi Heber's [Shaarei Zmanim](https://hebrewbooks.org/53000) ([chapter 10, page 87](https://hebrewbooks.org/pdfpager.aspx?req=53055&pgnum=101) and [chapter 12, page 108](https://hebrewbooks.org/pdfpager.aspx?req=53055&pgnum=122)). Also endorsed by [Rabbi Shmuel Kamenetsky](https://en.wikipedia.org/wiki/Shmuel_Kamenetsky).

At some northern and southern locations, this zman may not be available or cannot be calculated if the sun does not reach low enough below the horizon.
```
