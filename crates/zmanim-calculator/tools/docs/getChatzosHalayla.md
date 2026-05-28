# getChatzosHalayla

Source: `com.kosherjava.zmanim.ZmanimCalendar.getChatzosHalayla` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ZmanimCalendar.java:409)

```javadoc
A method that returns <em>chatzos halayla</em> at the <b>end of the day</b> (the last <em>zman</em> of the day
returned by the calendar, that may actually be after midnight of the day it is being calculated for). For example, if
calculating it for the date of <em>Erev Pesach</em>, the calculation will be for <em>Lail Pesach</em> to allow you to use the
<em>zman</em> as <em>sof zman achilas chametz</em>. {@link getSolarMidnight() Astronomical <em>chatzos halayla</em>} will be
returned if the {@link com.kosherjava.zmanim.util.AstronomicalCalculator calculator} class used supports it and {@link
isUseAstronomicalChatzos() isUseAstronomicalChatzos()} is set to <em>true</em>. Otherwise the {@link #getChatzos(Instant,
Instant) halfway point} between sunset and the following day's sunrise, if it does not support it, or it is not configured to
use it. There are currently two {@link com.kosherjava.zmanim.util.AstronomicalCalculator calculators} available in the API,
the default {@link com.kosherjava.zmanim.util.NOAACalculator NOAA calculator} and the {@link
com.kosherjava.zmanim.util.SunTimesCalculator USNO calculator}. The USNO calculator calculates <em>chatzos halayla</em> as
halfway between sunset and the following day's sunrise (identical to six <em>shaos zmaniyos</em> after sunset), while the
NOAACalculator calculates it more accurately as {@link getSolarMidnight() astronomical <em>chatzos halayla</em>}. See <a href=
"https://kosherjava.com/2020/07/02/definition-of-chatzos/">The Definition of <em>Chatzos</em></a> for a detailed explanation
of the ways to calculate <em>Chatzos</em>. Since half-night <em>chatzos</em> can be <code>null</code> in the Arctic on a day
when either sunset or the following day's sunrise did not happen and astronomical <em>chatzos halayla</em> can be calculated
even in the Arctic, if half-day <em>chatzos</em> calculates as <code>null</code> and astronomical <em>chatzos</em> is
supported by the calculator, astronomical <em>chatzos</em> will be returned to avoid returning a <code>null</code>.

@see getSolarMidnight()
@see getChatzos(Instant, Instant)
@see isUseAstronomicalChatzos()
@see setUseAstronomicalChatzos(boolean)
@return the <code>Instant</code> of <em>chatzos halayla</em> at the <b>end of the current day</b>. If the calculation can't be
        computed such as in the Arctic Circle where there is at least one day where the sun does not rise, and one where it
        does not set, and the calculator does not support astronomical calculations (that will never report a
        <code>null</code>) a <code>null</code> will be returned. See detailed explanation on top of the
        {@link AstronomicalCalendar} documentation.
```

# Human docs

```markdown
Chatzos halayla (Solar Midnight), when the sun is at its lowest point in the sky at the end of the day.

For how chatzos can be defined and calculated, see [The Definition of Chatzos](https://kosherjava.com/2020/07/02/definition-of-chatzos/) on the KosherJava blog.
```
