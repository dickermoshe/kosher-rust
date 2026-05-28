# getTzaisGeonim7Point083Degrees

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getTzaisGeonim7Point083Degrees` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:2453)

```javadoc
This method returns the <em>tzais</em> (nightfall) based on the opinion of the <em>Geonim</em> calculated when the sun's
position {@link ZENITH_7_POINT_083 7.083° (or 7° 5′}) below the western horizon. This is often referred to as 7° 5′ or 7° and
5′ minutes. This calculation is based on the observation of 3 medium-sized stars by Dr. Baruch (Berthold) Cohn in his
<em>luach</em> <a href="https://sammlungen.ub.uni-frankfurt.de/freimann/content/titleinfo/983088">Tabellen enthaltend die
Zeitangaben für den Beginn der Nacht und des Tages für die Breitengrade + 66 bis -38</a> published in Strasbourg, France in
1899. This calendar was very popular in Europe, and many other calendars based their <em>tzais</em> time on it. <a href=
"https://en.wikipedia.org/wiki/David_Zvi_Hoffmann">Rav Dovid Tzvi Hoffman</a> in his <a href="https://hebrewbooks.org/1053"
>שו״ת מלמד להועיל</a> in an exchange of letters with Baruch Cohn in <a href=
"https://hebrewbooks.org/pdfpager.aspx?req=1053&st=&pgnum=37">Orach Chaim 30</a> agreed to this <em>zman</em> (page 36), as
did the שו״ת בני ציון and the <a href="https://hebrewbooks.org/67373">תנובת שדה</a>. It is very close to the time of the <a href=
"https://hebrewbooks.org/22044">מקור חסד</a> of the Sefer chasidim. It is close to the position of the sun 30 minutes after
sunset in Jerusalem <a href="https://kosherjava.com/2022/01/12/equinox-vs-equilux-zmanim-calculations/">around the equinox /
equilux</a>, but not Exactly. The actual position of the sun 30 minutes after sunset in Jerusalem at the equilux is 7.205°
and 7.199° at the equinox. See Hazmanim Bahalacha vol 2, pages 520-521 for more details.

@return the <code>Instant</code> representing the time when the sun is 7.083° below sea level. If the calculation can't be
        computed such as northern and southern locations even south of the Arctic Circle and north of the Antarctic Circle
        where the sun may not reach low enough below the horizon for this calculation, a <code>null</code> will be returned.
        See detailed explanation on top of the {@link AstronomicalCalendar} documentation.
@see ZENITH_7_POINT_083
```

# Human docs

```markdown
Tzais (nightfall) according to the Geonim - when the sun is 7.083 degrees (7 degrees 5 minutes) below the western horizon after sunset.

Based on Dr. Baruch (Berthold) Cohn's observation of 3 medium-sized stars in his [1899 luach](https://sammlungen.ub.uni-frankfurt.de/freimann/content/titleinfo/983088). Endorsed by [Rav Dovid Tzvi Hoffman](https://en.wikipedia.org/wiki/David_Zvi_Hoffmann) in [Melamed Leho'il Orach Chaim 30](https://hebrewbooks.org/pdfpager.aspx?req=1053&st=&pgnum=37). Close to the [Makor Chessed](https://hebrewbooks.org/22044) of the Sefer Chasidim and to about 30 minutes after sunset in Jerusalem [around the equinox/equilux](https://kosherjava.com/2022/01/12/equinox-vs-equilux-zmanim-calculations/), but not exactly.

At some northern and southern locations, this zman may not be available or cannot be calculated if the sun does not reach low enough below the horizon.
```
