# getChatzosHayomAsHalfDay

Source: `com.kosherjava.zmanim.ZmanimCalendar.getChatzosHayomAsHalfDay` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ZmanimCalendar.java:490)

```javadoc
Returns <em>chatzos</em> calculated as halfway between sunrise and sunset. Many are of the opinion that
<em>chatzos</em> is calculated as the midpoint between {@link getSeaLevelSunrise() sea level sunrise} and
{@link getSeaLevelSunset() sea level sunset}, despite it not being the most accurate way to calculate it. A day
starting at <em>alos</em> and ending at <em>tzais</em> using the same time or degree offset will also return
the same time. In reality due to lengthening or shortening of day, this is not necessarily the exact midpoint of
the day, but it is very close. This method allows you to use the NOAACalculator and still calculate <em>chatzos
</em> as six <em>shaos zmaniyos</em> after sunrise. There are currently two {@link
com.kosherjava.zmanim.util.AstronomicalCalculator calculators} available in the API, the {@link
com.kosherjava.zmanim.util.NOAACalculator} and the {@link com.kosherjava.zmanim.util.SunTimesCalculator}.
The SunTimesCalculator calculates <em>chatzos</em> as halfway between sunrise and sunset (and of six <em>shaos
zmaniyos</em>), while the NOAACalculator calculates it as astronomical <em>chatzos</em> that is slightly more
accurate. This method allows you to use the NOAACalculator and still calculate <em>chatzos</em> as six <em>shaos
zmaniyos</em> after sunrise. See <a href="https://kosherjava.com/2020/07/02/definition-of-chatzos/">The Definition
of <em>Chatzos</em></a> for a detailed explanation of the ways to calculate <em>Chatzos</em>.

@return the <code>Instant</code> of the latest <em>chatzos</em>. If the calculation can't be computed such
        as in the Arctic Circle where there is at least one day a year where the sun does not rise, and one where
        it does not set, a <code>null</code> will be returned. See detailed explanation on top of the
        {@link AstronomicalCalendar} documentation.
@see com.kosherjava.zmanim.util.NOAACalculator#getUTCNoon(LocalDate, GeoLocation)
@see com.kosherjava.zmanim.util.SunTimesCalculator#getUTCNoon(LocalDate, GeoLocation)
@see com.kosherjava.zmanim.util.AstronomicalCalculator#getUTCNoon(LocalDate, GeoLocation)
@see getSunTransit(Instant, Instant)
@see getChatzosHayom()
@see getChatzosHalayla()
@see getChatzos(Instant, Instant)
@see getSunTransit()
@see isUseAstronomicalChatzos()
```

# Human docs

```markdown
Chatzos hayom calculated as the halfway point between sunrise and sunset.

This is the same as six shaos zmaniyos after sunrise when the day is measured from sunrise to sunset. Many hold that chatzos is the midpoint between sea level sunrise and sea level sunset, even though astronomical chatzos is usually a slightly different time.

A day measured from alos to tzais with the same offset on both sides can also have the same midpoint.

For how chatzos can be defined and calculated, see [The Definition of Chatzos](https://kosherjava.com/2020/07/02/definition-of-chatzos/) on the KosherJava blog.

In places such as the Arctic Circle, where there is at least one day a year when the sun does not rise and one when it does not set, this zman may not be available or cannot be calculated.
```
