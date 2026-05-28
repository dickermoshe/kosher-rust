# getBainHashmashosYereim3Point05Degrees

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getBainHashmashosYereim3Point05Degrees` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:2179)

```javadoc
This method returns the beginning of <em>bain hashmashos</em> (twilight) according to the <a href=
"https://en.wikipedia.org/wiki/Eliezer_ben_Samuel">Yereim (Rabbi Eliezer of Metz)</a> calculated as the sun's
position 3.05° above the horizon <a href=
"https://kosherjava.com/2022/01/12/equinox-vs-equilux-zmanim-calculations/">around the equinox / equilux</a>,
its position 18 minutes or 3/4 of an 24-minute <a href=
"https://en.wikipedia.org/wiki/Biblical_and_Talmudic_units_of_measurement">mil</a> before sunset. According to
the Yereim, <em>bain hashmashos</em> starts 3/4 of a mil before sunset and <em>tzais</em> or nightfall starts at
sunset. Note that <em>lechumra</em> (of about 14 seconds) a refraction value of 0.5166° as opposed to the
traditional 0.566° is used. This is more inline with the actual refraction in <em>Eretz Yisrael</em> and is
brought down by <a href=
"http://beinenu.com/rabbis/%D7%94%D7%A8%D7%91-%D7%99%D7%93%D7%99%D7%93%D7%99%D7%94-%D7%9E%D7%A0%D7%AA">Rabbi
Yedidya Manet</a> in his <a href="https://www.nli.org.il/en/books/NNL_ALEPH002542826/NLI">Zmanei HaHalacha
Lema'aseh</a> (p. 11). That is the first source that I am aware of that calculates degree-based Yereim
<em>zmanim</em>. The 0.5166° refraction is also used by the <a href="https://zmanim.online/">Luach Itim
Lebinah</a>. Calculating the Yereim's <em>bain hashmashos</em> using 18-minute based degrees is also suggested
in the upcoming 8th edition of the zmanim Kehilchasam. For more details, see the article <a href=
"https://kosherjava.com/2020/12/07/the-yereims-bein-hashmashos/">The Yereim's <em>Bain Hashmashos</em></a>.

@todo recalculate based on equinox/equilux
@return the <code>Instant</code> of the sun's position 3.05° minutes before sunset. If the calculation can't
        be computed such as in the Arctic Circle where there is at least one day a year where the sun does not
        rise, and one where it does not set, a <code>null</code> will be returned. See detailed explanation on
        top of the {@link AstronomicalCalendar} documentation.
@see ZENITH_MINUS_3_POINT_05
@see #getBainHashmashosYereim18Minutes()
@see #getBainHashmashosYereim2Point8Degrees()
@see #getBainHashmashosYereim2Point1Degrees()
```

# Human docs

```markdown
The beginning of bain hashmashos (twilight) according to the [Yereim (Rabbi Eliezer of Metz)](https://en.wikipedia.org/wiki/Eliezer_ben_Samuel).

When the sun is 3.05 degrees above the horizon. In Jerusalem [around the equinox or equilux](https://kosherjava.com/2022/01/12/equinox-vs-equilux-zmanim-calculations/), that matches about 18 minutes, or 3/4 of a 24-minute [mil](https://en.wikipedia.org/wiki/Biblical_and_Talmudic_units_of_measurement), before sunset.

The Yereim holds that bain hashmashos begins 3/4 of a mil before sunset and that tzais (nightfall) is at sunset. This degree-based version uses 0.5166 degrees of refraction instead of the traditional 0.566 degrees, which shifts the time earlier by about 14 seconds lechumra and is closer to refraction in Eretz Yisrael per [Rabbi Yedidya Manet](http://beinenu.com/rabbis/%D7%94%D7%A8%D7%91-%D7%99%D7%93%D7%99%D7%93%D7%99%D7%94-%D7%9E%D7%A0%D7%AA) ([Zmanei HaHalacha Lema'aseh](https://www.nli.org.il/en/books/NNL_ALEPH002542826/NLI), p. 11) and the [Luach Itim Lebinah](https://zmanim.online/). For background, see the [The Yereim's Bain Hashmashos](https://kosherjava.com/2020/12/07/the-yereims-bein-hashmashos/) article on the KosherJava blog.

In places such as the Arctic Circle, where there is at least one day a year when the sun does not rise and one when it does not set, this zman may not be available or cannot be calculated.
```
