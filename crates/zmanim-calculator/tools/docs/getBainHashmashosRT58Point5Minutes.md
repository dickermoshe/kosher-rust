# getBainHashmashosRT58Point5Minutes

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getBainHashmashosRT58Point5Minutes` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:2088)

```javadoc
This method returns the beginning of <em>Bain hashmashos</em> of Rabbeinu Tam calculated as a 58.5-minute offset
after sunset. <em>bain hashmashos</em> is 3/4 of a <a href=
"https://en.wikipedia.org/wiki/Biblical_and_Talmudic_units_of_measurement">mil</a> before <em>tzais</em> or 3 1/4
mil after sunset. With a mil calculated as 18 minutes, 3.25 * 18 = 58.5 minutes.

@return the <code>Instant</code> of 58.5 minutes after sunset. If the calculation can't be computed such as in the
        Arctic Circle where there is at least one day a year where the sun does not rise, and one where it does
        not set, a <code>null</code> will be returned. See detailed explanation on top of the
        {@link AstronomicalCalendar} documentation.
```

# Human docs

```markdown
The beginning of Rabbeinu Tam's bain hashmashos.

58.5 minutes after sunset. Bain hashmashos is 3/4 of a [mil](https://en.wikipedia.org/wiki/Biblical_and_Talmudic_units_of_measurement) before tzais, or 3 1/4 mil after sunset. With an 18-minute mil, 3.25 * 18 = 58.5 minutes.

In places such as the Arctic Circle, where there is at least one day a year when the sun does not rise and one when it does not set, this zman may not be available or cannot be calculated.
```
