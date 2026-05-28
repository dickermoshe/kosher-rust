# getTzais16Point1Degrees

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getTzais16Point1Degrees` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:2846)

```javadoc
This calculates the time of <em>tzais</em> at the point when the sun is 16.1° below the horizon. This is
the sun's dip below the horizon 72 minutes after sunset according Rabbeinu Tam's calculation of <em>tzais</em>
<a href=
"https://kosherjava.com/2022/01/12/equinox-vs-equilux-zmanim-calculations/">around the equinox / equilux</a> in
Jerusalem. The question of equinox VS equilux is complex, with Rabbi Meir Posen in the <a href=
"https://www.worldcat.org/oclc/956316270">Ohr Meir</a> of the opinion that the equilux should be used. See
Yisrael Vehazmanim vol I, 34:1:4. Rabbi Yedidya Manet in his <a href=
"https://www.nli.org.il/en/books/NNL_ALEPH002542826/NLI">Zmanei HaHalacha Lema'aseh</a> (4th edition part 2, pages
and 22 and 24) and Rabbi Yonah Mertzbuch (in a letter published by Rabbi Manet) are of the opinion that the
astronomical equinox should be used. The difference adds up to about 9 seconds, too trivial to make much of a
difference. For information on how this is calculated see the comments on {@link #getAlos16Point1Degrees()}.

@return the <code>Instant</code> representing the time. If the calculation can't be computed such as northern and
        southern locations even south of the Arctic Circle and north of the Antarctic Circle where the sun may
        not reach low enough below the horizon for this calculation, a <code>null</code> will be returned. See
        detailed explanation on top of the {@link AstronomicalCalendar} documentation.
@see #getTzais72Minutes()
@see #getAlos16Point1Degrees() for more information on this calculation.
```

# Human docs

```markdown
Tzais (nightfall) - when the sun is 16.1 degrees below the western horizon after sunset.

Matches Rabbeinu Tam's 72-minute tzais [around the equinox/equilux](https://kosherjava.com/2022/01/12/equinox-vs-equilux-zmanim-calculations/) in Jerusalem.

At some northern and southern locations, this zman may not be available or cannot be calculated if the sun does not reach low enough below the horizon.
```
