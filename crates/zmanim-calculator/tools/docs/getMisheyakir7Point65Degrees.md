# getMisheyakir7Point65Degrees

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getMisheyakir7Point65Degrees` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:1014)

```javadoc
This method returns <em>misheyakir</em> based on the position of the sun when it is {@link ZENITH_7_POINT_65 7.65°} below
{@link GEOMETRIC_ZENITH geometric zenith} (90°). The degrees are based on a 35/36 minute <em>zman</em> <a href=
"https://kosherjava.com/2022/01/12/equinox-vs-equilux-zmanim-calculations/">around the equinox / equilux</a>, when the
<em>neshef</em> (twilight) is the shortest. This time is based on <a href=
"https://en.wikipedia.org/wiki/Moshe_Feinstein">Rabbi Moshe Feinstein</a> who writes in <a href=
"https://hebrewbooks.org/pdfpager.aspx?req=14677&pgnum=7">Ohr Hachaim Vol. 4, Ch. 6</a> that <em>misheyakir</em> in New York
is 35-40 minutes before sunrise, something that is a drop less than 8°. <a href="https://en.wikipedia.org/wiki/Yisroel_Taplin"
>Rabbi Yisroel Taplin</a> in <a href="https://www.worldcat.org/oclc/889556744">Zmanei Yisrael</a> (page 117) notes that
<a href="https://en.wikipedia.org/wiki/Yaakov_Kamenetsky">Rabbi Yaakov Kamenetsky</a> stated that it is not less than 36
minutes before sunrise (maybe it is 40 minutes). Sefer Yisrael Vehazmanim (p. 7) quotes the Tamar Yifrach in the name of the
<a href="https://en.wikipedia.org/wiki/Joel_Teitelbaum">Satmar Rov</a> that one should be stringent not consider
<em>misheyakir</em> before 36 minutes. This is also the accepted <a href="https://en.wikipedia.org/wiki/Minhag">minhag</a> in
<a href="https://en.wikipedia.org/wiki/Lakewood_Township,_New_Jersey">Lakewood</a> that is used in the <a href=
"https://en.wikipedia.org/wiki/Beth_Medrash_Govoha">Yeshiva</a>. This follows the opinion of <a href=
"https://en.wikipedia.org/wiki/Shmuel_Kamenetsky">Rabbi Shmuel Kamenetsky</a> who provided the time of 35/36 minutes, but did
not provide a degree-based time. Since this <em>zman</em> depends on the level of light, Rabbi Yaakov Shakow presented this
degree-based calculations to Rabbi Shmuel Kamenetsky who agreed to them.

@return the <code>Instant</code> of <em>misheyakir</em>. If the calculation can't be computed such as
        northern and southern locations even south of the Arctic Circle and north of the Antarctic Circle where
        the sun may not reach low enough below the horizon for this calculation, a <code>null</code> will be returned.
        See detailed explanation on top of the {@link AstronomicalCalendar} documentation.
@see ZENITH_7_POINT_65
@see #getMisheyakir9Point5Degrees()
```

# Human docs

```markdown
Misheyakir when the sun is 7.65 degrees below geometric zenith.

Based on a 35-36 minute zman [around the equinox or equilux](https://kosherjava.com/2022/01/12/equinox-vs-equilux-zmanim-calculations/), when twilight is shortest. [Rabbi Moshe Feinstein](https://en.wikipedia.org/wiki/Moshe_Feinstein) writes in [Ohr Hachaim vol. 4, ch. 6](https://hebrewbooks.org/pdfpager.aspx?req=14677&pgnum=7) that misheyakir in New York is 35-40 minutes before sunrise. This follows [Rabbi Shmuel Kamenetsky](https://en.wikipedia.org/wiki/Shmuel_Kamenetsky)'s 35-36 minute time (degree-based times were presented by Rabbi Yaakov Shakow and accepted by him). The accepted [minhag](https://en.wikipedia.org/wiki/Minhag) in [Lakewood](https://en.wikipedia.org/wiki/Lakewood_Township,_New_Jersey) [Yeshiva](https://en.wikipedia.org/wiki/Beth_Medrash_Govoha) is not to consider misheyakir before 36 minutes.

At some northern and southern locations, this zman may not be available or cannot be calculated if the sun does not reach low enough below the horizon.
```
