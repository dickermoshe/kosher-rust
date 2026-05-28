# getBainHashmashosYereim13Point5Minutes

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getBainHashmashosYereim13Point5Minutes` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:2239)

```javadoc
This method returns the beginning of <em>bain hashmashos</em> (twilight) according to the <a href=
"https://en.wikipedia.org/wiki/Eliezer_ben_Samuel">Yereim (Rabbi Eliezer of Metz)</a> calculated as 13.5 minutes
or 3/4 of an 18-minute <a href="https://en.wikipedia.org/wiki/Biblical_and_Talmudic_units_of_measurement">mil</a>
before sunset. According to the Yereim, <em>bain hashmashos</em> starts 3/4 of a mil before sunset and
<em>tzais</em> or nightfall starts at sunset.

@return the <code>Instant</code> of 13.5 minutes before sunset. If the calculation can't be computed such as in the
        Arctic Circle where there is at least one day a year where the sun does not rise, and one where it does
        not set, a <code>null</code> will be returned. See detailed explanation on top of the
        {@link AstronomicalCalendar} documentation.
@see #getBainHashmashosYereim2Point1Degrees()
```

# Human docs

```markdown
The beginning of bain hashmashos (twilight) according to the [Yereim (Rabbi Eliezer of Metz)](https://en.wikipedia.org/wiki/Eliezer_ben_Samuel).

13.5 minutes, or 3/4 of an 18-minute [mil](https://en.wikipedia.org/wiki/Biblical_and_Talmudic_units_of_measurement), before sunset. According to the Yereim, bain hashmashos starts 3/4 of a mil before sunset and tzais (nightfall) is at sunset.

In places such as the Arctic Circle, where there is at least one day a year when the sun does not rise and one when it does not set, this zman may not be available or cannot be calculated.
```
