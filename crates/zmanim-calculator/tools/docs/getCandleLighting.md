# getCandleLighting

Source: `com.kosherjava.zmanim.ZmanimCalendar.getCandleLighting` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ZmanimCalendar.java:627)

```javadoc
A method to return candle lighting time, calculated as {@link getCandleLightingOffset()} minutes before
{@link getSeaLevelSunset() sea level sunset}. This will return the time for any day of the week, since it can be
used to calculate candle lighting time for <em>Yom Tov</em> (mid-week holidays) as well. Elevation adjustments
are intentionally not performed by this method, but you can calculate it by passing the elevation adjusted sunset
to {@link getTimeOffset(Instant, Duration)}.

@return candle lighting time. If the calculation can't be computed such as in the Arctic Circle where there is at
        least one day a year where the sun does not rise, and one where it does not set, a <code>null</code> will
        be returned. See detailed explanation on top of the {@link AstronomicalCalendar} documentation.
@see getSeaLevelSunset()
@see getCandleLightingOffset()
@see setCandleLightingOffset(double)
```

# Human docs

```markdown
The time to light candles before Shabbos or Yom Tov.

{candel_lighting_offset} before sea level sunset.

In places such as the Arctic Circle, where there is at least one day a year when the sun does not rise and one when it does not set, this zman may not be available or cannot be calculated.
```
