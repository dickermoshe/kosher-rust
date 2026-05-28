# getChatzosHayom

Source: `com.kosherjava.zmanim.ZmanimCalendar.getChatzosHayom` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ZmanimCalendar.java:367)

```javadoc
This method returns {@link getSunTransit() Astronomical <em>chatzos hayom</em>} if the
{@link com.kosherjava.zmanim.util.AstronomicalCalculator calculator} class used supports it and
{@link isUseAstronomicalChatzos() isUseAstronomicalChatzos()} is set to <em>true</em> or the {@link getChatzosHayomAsHalfDay()
halfway point between sunrise and sunset} if it does not support it, or it is not configured to use it. There are currently
two {@link com.kosherjava.zmanim.util.AstronomicalCalculator calculators} available in the API, the default {@link
com.kosherjava.zmanim.util.NOAACalculator NOAA calculator} and the {@link com.kosherjava.zmanim.util.SunTimesCalculator USNO
calculator}. The USNO calculator calculates <em>chatzos</em> as halfway between sunrise and sunset (identical to six <em>shaos
zmaniyos</em> after sunrise), while the NOAACalculator calculates it more accurately as {@link getSunTransit() astronomical
<em>chatzos</em>}. See <a href="https://kosherjava.com/2020/07/02/definition-of-chatzos/">The Definition of <em>Chatzos</em></a>
for a detailed explanation of the ways to calculate <em>Chatzos</em>. Since half-day <em>chatzos</em> can be <code>null</code> in
the Arctic on a day when either sunrise or sunset did not happen and astronomical <em>chatzos</em> can be calculated even in the
Arctic, if half-day <em>chatzos</em> calculates as <code>null</code> and astronomical <em>chatzos</em> is supported by the
calculator, astronomical <em>chatzos</em> will be returned to avoid returning a <code>null</code>.

@see getSunTransit()
@see getChatzosHayomAsHalfDay()
@see isUseAstronomicalChatzos()
@see setUseAstronomicalChatzos(boolean)
@return the <code>Instant</code> of <em>chatzos</em>. If the calculation can't be computed such as in the Arctic Circle
        where there is at least one day where the sun does not rise, and one where it does not set, and the calculator does not
        support astronomical calculations (that will never report a <code>null</code>) a <code>null</code> will be returned.
        See detailed explanation on top of the {@link AstronomicalCalendar} documentation.
```

# Human docs

```markdown
Chatzos hayom (Solar Noon), when the sun transits the celestial meridian.

For how chatzos can be defined and calculated, see [The Definition of Chatzos](https://kosherjava.com/2020/07/02/definition-of-chatzos/) on the KosherJava blog.
```
