# getSolarMidnight

Source: `com.kosherjava.zmanim.AstronomicalCalendar.getSolarMidnight` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\AstronomicalCalendar.java:491)

```javadoc
A method that returns solar midnight as the <b>end of the day</b> (that may actually be after midnight of the  day it is
being calculated for). For example calculating solar midnight for February 8, will calculate it for midnight between February
8 and February 9. It occurs when the Sun is <a href="https://en.wikipedia.org/wiki/Transit_%28astronomy%29">transiting</a> the
lower <a href="https://en.wikipedia.org/wiki/Meridian_%28astronomy%29">celestial meridian</a>, or when the sun is at it's
<a href="https://en.wikipedia.org/wiki/Nadir">nadir</a>. The calculations used by this class depend on the {@link
AstronomicalCalculator} used. If this calendar instance is {@link setAstronomicalCalculator(AstronomicalCalculator) set} to use
the {@link com.kosherjava.zmanim.util.NOAACalculator} (the default) it will calculate astronomical midnight. If the calendar
instance is to use the {@link com.kosherjava.zmanim.util.SunTimesCalculator USNO Calculator}, that does not have code to
calculate astronomical noon, midnight is calculated as 12 hours after halfway between sea level sunrise and sea level sunset
of that day. This can be slightly off the real transit time due to changes in declination (the lengthening or shortening day).
See <a href="https://kosherjava.com/2020/07/02/definition-of-chatzos/">The Definition of Chatzos</a> for details on the proper
definition of solar noon / midday.

@return the <code>Instant</code> representing Sun's lower transit at the <b>end of the current day</b>. If the calculation
        can't be computed such as when using the {@link com.kosherjava.zmanim.util.SunTimesCalculator USNO calculator} that does
        not support getting solar noon or midnight for the Arctic Circle (where there is at least one day a year where the sun
        does not rise, and one where it does not set), a <code>null</code> will be returned. This is not relevant when using the
        {@link com.kosherjava.zmanim.util.NOAACalculator NOAA Calculator} that is never expected to return <code>null</code>.
        See the detailed explanation on top of the page.
@see getSunTransit()
@see com.kosherjava.zmanim.util.NOAACalculator#getUTCNoon(Calendar, GeoLocation)
@see com.kosherjava.zmanim.util.SunTimesCalculator#getUTCNoon(Calendar, GeoLocation)
```

# Human docs

```markdown
Solar midnight - when the sun transits the lower celestial meridian (at its nadir).

Calculated for the end of the current day. For example, solar midnight for February 8 is the moment between February 8 and February 9 when the sun is at its lowest point. See [The Definition of Chatzos](https://kosherjava.com/2020/07/02/definition-of-chatzos/) for details on the proper definition of solar noon and midnight.

In places such as the Arctic Circle, where there is at least one day a year when the sun does not rise and one when it does not set, this zman may not be available or cannot be calculated.
```
