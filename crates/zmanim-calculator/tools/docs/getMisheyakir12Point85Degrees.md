# getMisheyakir12Point85Degrees

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getMisheyakir12Point85Degrees` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:930)

```javadoc
This method returns <em>misheyakir</em> based on the position of the sun {@link ZENITH_12_POINT_85 12.85°}
below {@link GEOMETRIC_ZENITH geometric zenith} (90°). This is based on the position of the sun slightly
later than 57 minutes before {@link #getSunset() sunrise} in Jerusalem <a href=
"https://kosherjava.com/2022/01/12/equinox-vs-equilux-zmanim-calculations/">around the equinox / equilux</a>. This
<em>zman</em> is mentioned for use <b><em>bish'as hadchak</em></b> in the Birur Halacha <a href=
"https://hebrewbooks.org/pdfpager.aspx?req=50535&st=&pgnum=88">Tinyana</a> and <a href=
"https://hebrewbooks.org/pdfpager.aspx?req=50537&st=&pgnum=31">Tlisa'ah</a> in  Orach Chaim siman 18 as 12.85°.
Actual calculations show it to be slightly more than 12.9°, but the Birur Halacha indicates that 12.85° is a
slight <em>chumra</em> (on a <em>bedieved</em> time) VS the 12.9° that 57 minutes calculates as (a difference of
about 14 seconds at the equinox/equilux in Jerusalem). The <em>zman</em> of 12.9° is also mentioned in the Piskei
Tshuvos siman 18, page 190 (where a typo indicates that this is the degree equivalent to 60 minutes before sunrise,
when in fact at that point the sun is about 13.5° below the horizon). The 57 minute based time is mentioned by the
Minchas Yitzchak <a href="https://hebrewbooks.org/pdfpager.aspx?req=1601&st=&pgnum=21">vol. 9, siman 9</a> as 15 minutes
before <em>alos hashachar</em> (though he is not clear what location he refers to, and does not mention a degree-based
conversion). The Kaf Hachaim <a href="https://hebrewbooks.org/pdfpager.aspx?req=8140&st=&pgnum=81">vol.1 siman 18, no.
18</a> states that in Yerushalayim 60 fixed minutes are used year round. Calculations show that 60 fixed minutes in
Yerushalayim ranges from 13.5° at the spring equinox to 11.5° at the summer solstice. 57-minute
<em>misheyakir</em> range from 12.9° at the winter equinox to 11° at the summer solstice.
Analysis of the difference between 12.85° and 12.9°, shows that the maximum difference occurs at the summer
solstice. In Lakewood, NJ at a latitude of 40.096°, the maximum difference throughout the year is 23 seconds.
In the winter where there is the greatest need for very early <em>misheyakir</em> times, the difference is in the 16
second range. Going north to Montreal at latitude 45.5°, the maximum is 29 seconds and is about 18 seconds in the
winter. Moving farther north to the latitude of Vilnius at a latitude of 54.68°, things change. Firstly, around the
summer solstice it will not reach that far below the horizon. On the dates that both can be calculated, the maximum
difference can be pretty high on one or two days of the year (around Jul 8),  with about a week having over a two minute
difference between the two. Even at the latitude of Vilna, from Dec - March, the difference is about 22 seconds.

@deprecated This method returns a very early <em>misheyakir</em> time that should only be used <b><em>bish'as
        hadchak</em></b>. <em>Lechatchila</em>, a later <em>zman</em> should be used. There is no current plan to remove
        this method from the API, and this deprecation is intended to notify developers to add an alert to users of
        the risk of using it.
@return The <code>Instant</code> of <em>misheyakir</em>. If the calculation can't be computed such as northern and
        southern locations even south of the Arctic Circle and north of the Antarctic Circle where the sun may
        not reach low enough below the horizon for this calculation, a <code>null</code> will be returned. See
        detailed explanation on top of the {@link AstronomicalCalendar} documentation.
@see ZENITH_12_POINT_85
```

# Human docs

```markdown
Misheyakir using a very early calculation.

The time when the sun is 12.85 degrees below the horizon before sunrise.

This is slightly later than 57 minutes before sunrise in Jerusalem around the equinox.

This zman should be used only bish'as hadchak. A later zman should be used lechatchila.

At some northern and southern locations, this zman may not be available if the sun does not reach low enough below the horizon.
```
