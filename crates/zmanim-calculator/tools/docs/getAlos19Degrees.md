# getAlos19Degrees

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getAlos19Degrees` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:868)

```javadoc
A method to return <em>alos</em> (dawn) calculated when the sun is {@link ZENITH_19_DEGREES 19°} below the
eastern geometric horizon before sunrise. This is the <a href="https://en.wikipedia.org/wiki/Maimonides"
>Rambam</a>'s <em>alos</em> according to Rabbi Moshe Kosower's <a href=
"https://www.worldcat.org/oclc/145454098">Maaglei Tzedek</a>, page 88, <a href=
"https://hebrewbooks.org/pdfpager.aspx?req=33464&pgnum=13">Ayeles Hashachar Vol. I, page 12</a>, <a href=
"https://hebrewbooks.org/pdfpager.aspx?req=55960&pgnum=258">Yom Valayla Shel Torah, Ch. 34, p. 222</a> and
Rabbi Yaakov Shakow's <a href="https://www.worldcat.org/oclc/1043573513">Luach Ikvei Hayom</a>.

@return the <code>Instant</code> representing <em>alos</em>. If the calculation can't be computed such as northern
        and southern locations even south of the Arctic Circle and north of the Antarctic Circle where the sun
        may not reach low enough below the horizon for this calculation, a <code>null</code> will be returned.
        See detailed explanation on top of the {@link AstronomicalCalendar} documentation.
```

# Human docs

```markdown
Alos (dawn), associated with the Rambam's alos.

The time when the sun is 19 degrees below the eastern horizon before sunrise.

At some northern and southern locations, this zman may not be available if the sun does not reach low enough below the horizon.
```
