# getBainHashmashosRT13Point24Degrees

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getBainHashmashosRT13Point24Degrees` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:2072)

```javadoc
Method to return the beginning of <em>bain hashmashos</em> of Rabbeinu Tam calculated when the sun is
{@link ZENITH_13_POINT_24 13.24°} below the western {@link GEOMETRIC_ZENITH geometric horizon} (90°)
after sunset. This calculation is based on the same calculation of {@link #getBainHashmashosRT58Point5Minutes()
<em>bain hashmashos</em> Rabbeinu Tam 58.5 minutes} but uses a degree-based calculation instead of 58.5 exact
minutes. This calculation is based on the position of the sun 58.5 minutes after sunset in Jerusalem <a href=
"https://kosherjava.com/2022/01/12/equinox-vs-equilux-zmanim-calculations/">around the equinox / equilux</a>,
which calculates to 13.24° below {@link GEOMETRIC_ZENITH geometric zenith}.
NOTE: As per Yisrael Vehazmanim Vol. III page 1028, No. 50, a dip of slightly less than 13° should be used.
Calculations show that the proper dip to be 13.2456° (truncated to 13.24 that provides about 1.5 second
earlier (<em>lechumra</em>) time) below the horizon at that time. This makes a difference of 1 minute and 10
seconds in Jerusalem during the Equinox, and 1 minute 29 seconds during the solstice as compared to the proper
13.24° versus 13°. For NY during the solstice, the difference is 1 minute 56 seconds.

@todo recalculate the above based on equilux/equinox calculations.
@return the <code>Instant</code> of the sun being 13.24° below {@link GEOMETRIC_ZENITH geometric zenith}
        (90°). If the calculation can't be computed such as northern and southern locations even south of the
        Arctic Circle and north of the Antarctic Circle where the sun may not reach low enough below the horizon
        for this calculation, a <code>null</code> will be returned. See detailed explanation on top of the
        {@link AstronomicalCalendar} documentation.
@see #getBainHashmashosRT58Point5Minutes()
```

# Human docs

```markdown
The beginning of Rabbeinu Tam's bain hashmashos.

When the sun is 13.24 degrees below the western geometric horizon after sunset. This is the degree-based equivalent of bain hashmashos 58.5 minutes after sunset: in Jerusalem [around the equinox or equilux](https://kosherjava.com/2022/01/12/equinox-vs-equilux-zmanim-calculations/), the sun is 13.24 degrees below geometric zenith about 58.5 minutes after sunset. The source notes that the proper dip is slightly less than 13 degrees (about 13.2456 degrees); 13.24 degrees is used as a truncation that yields a time about 1.5 seconds earlier, lechumra.

At some northern and southern locations, including places even south of the Arctic Circle and north of the Antarctic Circle, this zman may not be available or cannot be calculated if the sun does not reach low enough below the horizon.
```
