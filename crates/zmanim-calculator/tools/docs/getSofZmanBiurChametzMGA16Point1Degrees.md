# getSofZmanBiurChametzMGA16Point1Degrees

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getSofZmanBiurChametzMGA16Point1Degrees` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:3393)

```javadoc
This method returns the latest time for burning <em>chametz</em> on <em>Erev Pesach</em> according to the opinion
of the <a href="https://en.wikipedia.org/wiki/Avraham_Gombiner">Magen Avraham (MGA)</a> based on <em>alos</em>
being {@link #getAlos16Point1Degrees() 16.1°} before {@link #getSunset() sunrise}. This time is 5
{@link #getShaahZmanis16Point1Degrees() <em>shaos zmaniyos</em>} (solar hours) after {@link #getAlos16Point1Degrees()
dawn} based on the opinion of the MGA that the day is calculated from dawn to nightfall with both being 16.1°
below sunrise or sunset. This returns the time of 5 {@link #getShaahZmanis16Point1Degrees()} after
{@link #getAlos16Point1Degrees() dawn}. If it is not  <em>erev Pesach</em>, a <code>null</code> will be returned.
@return the <code>Instant</code> of the latest time for burning <em>chametz</em> on <em>Erev Pesach</em>. If it is not
        <em>erev Pesach</em> or the calculation can't be computed such as northern and southern locations even south
        of the Arctic Circle and north of the Antarctic Circle where the sun may not reach low enough below the
        horizon for this calculation, a <code>null</code> will be returned. See detailed explanation on top of the
        {@link AstronomicalCalendar} documentation.
@see #getShaahZmanis16Point1Degrees()
@see #getAlos16Point1Degrees()
@see #getSofZmanBiurChametz(Instant, Instant, boolean)
```

# Human docs

```markdown
Sof zman biur chametz - the latest time to burn chametz on Erev Pesach according to the [Magen Avraham (MGA)](https://en.wikipedia.org/wiki/Avraham_Gombiner), using the 16.1-degree day.

5 shaos zmaniyos after alos at 16.1 degrees, with the day measured from alos at 16.1 degrees to tzais at 16.1 degrees.
```
