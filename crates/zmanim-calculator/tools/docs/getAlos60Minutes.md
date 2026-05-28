# getAlos60Minutes

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getAlos60Minutes` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:667)

```javadoc
Method to return <em>alos</em> (dawn) calculated as 60 minutes before {@link #getSunset() sunrise} or {@link
#getSeaLevelSunrise() sea level sunrise} (depending on the {@link #isUseElevation()} setting). This is the time to walk the
distance of 4 <a href="https://en.wikipedia.org/wiki/Biblical_and_Talmudic_units_of_measurement">mil</a> at 15 minutes a mil.
This seems to be the opinion of the <a href="https://en.wikipedia.org/wiki/Yair_Bacharach">Chavas Yair</a> in the Mekor Chaim,
Orach Chaim Ch. 90, though  the Mekor Chaim in Ch. 58 and in the <a href=
"https://hebrewbooks.org/pdfpager.aspx?req=45193&pgnum=214">Chut Hashani Ch. 97</a> states that a person walks 3 and a 1/3
mil in an hour, or an 18-minute mil. Also see the <a href=
"https://he.wikipedia.org/wiki/%D7%9E%D7%9C%D7%9B%D7%99%D7%90%D7%9C_%D7%A6%D7%91%D7%99_%D7%98%D7%A0%D7%A0%D7%91%D7%95%D7%99%D7%9D"
>Divrei Malkiel</a> <a href="https://hebrewbooks.org/pdfpager.aspx?req=803&pgnum=33">Vol. 4, Ch. 20, page 34</a>) who mentions
the 15 minute mil <em>lechumra</em> by baking matzos. Also see the <a href=
"https://en.wikipedia.org/wiki/Joseph_Colon_Trabotto">Maharik</a> <a href=
"https://hebrewbooks.org/pdfpager.aspx?req=1142&pgnum=216">Ch. 173</a> where the questioner quoting the <a href=
"https://en.wikipedia.org/wiki/Eliezer_ben_Nathan">Ra'avan</a> is of the opinion that the time to walk a mil is 15 minutes (5
mil in a little over an hour). There are many who believe that there is a <em>ta'us sofer</em> (scribe's error) in the
Ra'avan, and it should 4 mil in a little over an hour, or an 18-minute mil. Time based offset calculations are based on the
opinion of the <em><a href="https://en.wikipedia.org/wiki/Rishonim">Rishonim</a></em> who stated that the time of the
<em>neshef</em> (time between dawn and sunrise) does not vary by the time of year or location but purely depends on the time
it takes to walk the distance of 4* mil. {@link #getTzaisGeonim9Point75Degrees()} is a related <em>zman</em> that is a
degree-based calculation based on 60 minutes.

@return the <code>Instant</code> representing the time. If the calculation can't be computed such as in the Arctic Circle
        where there is at least one day a year where the sun does not rise, and one where it does not set, a <code>null</code>
        will be returned. See detailed explanation on top of the {@link AstronomicalCalendar} documentation.
@see #getTzais60Minutes()
@see #getPlagHamincha60Minutes()
@see #getShaahZmanis60Minutes()
```

# Human docs

```markdown
Alos (dawn), the beginning of morning twilight before sunrise.

60 minutes before sunrise. {uses_elevation}

Based on the time to walk 4 mil at 15 minutes per mil.

In places where sunrise cannot be calculated, this zman may not be available.
```
