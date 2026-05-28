# getSunTransit

Source: `com.kosherjava.zmanim.AstronomicalCalendar.getSunTransit` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\AstronomicalCalendar.java:462)

```javadoc
A method that returns sundial or solar noon. It occurs when the Sun is <a href=
"https://en.wikipedia.org/wiki/Transit_%28astronomy%29">transiting</a> the <a
href="https://en.wikipedia.org/wiki/Meridian_%28astronomy%29">celestial meridian</a>. The calculations used by this class
depend on the {@link AstronomicalCalculator} used. If this calendar instance is {@link setAstronomicalCalculator(
AstronomicalCalculator) set} to use the {@link com.kosherjava.zmanim.util.NOAACalculator} (the default) it will calculate
astronomical noon. If the calendar instance is  to use the {@link com.kosherjava.zmanim.util.SunTimesCalculator}, that does
not have code to calculate astronomical noon, the sun transit is calculated as halfway between sea level sunrise and sea level
sunset, which can be slightly off the real transit time due to changes in declination (the lengthening or shortening day). See
<a href="https://kosherjava.com/2020/07/02/definition-of-chatzos/">The Definition of Chatzos</a> for details on the proper
definition of solar noon / midday.

@return the <code>Instant</code> representing Sun's transit. If the calculation can't be computed such as when using the {@link
        com.kosherjava.zmanim.util.SunTimesCalculator USNO calculator} that does not support getting solar noon for the Arctic
        Circle (where there is at least one day a year where the sun does not rise, and one where it does not set), a
        <code>null</code> will be returned. See detailed explanation on top of the page.
@see getSunTransit(Instant, Instant)
@see getTemporalHour()
@see com.kosherjava.zmanim.util.NOAACalculator#getUTCNoon(Calendar, GeoLocation)
@see com.kosherjava.zmanim.util.SunTimesCalculator#getUTCNoon(Calendar, GeoLocation)
```

# Human docs

```markdown
Solar noon - when the sun transits the celestial meridian.

Also called sundial noon or astronomical chatzos hayom. See [The Definition of Chatzos](https://kosherjava.com/2020/07/02/definition-of-chatzos/) for details on the proper definition of solar noon.

In places such as the Arctic Circle, where there is at least one day a year when the sun does not rise and one when it does not set, this zman may not be available or cannot be calculated.
```
