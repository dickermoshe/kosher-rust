# getTzais60Minutes

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getTzais60Minutes` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:2536)

```javadoc
This method returns the <em>tzais</em> (nightfall) based on the opinion of the <a href=
"https://en.wikipedia.org/wiki/Yair_Bacharach">Chavas Yair</a> and <a href=
"https://he.wikipedia.org/wiki/%D7%9E%D7%9C%D7%9B%D7%99%D7%90%D7%9C_%D7%A6%D7%91%D7%99_%D7%98%D7%A0%D7%A0%D7%91%D7%95%D7%99%D7%9D"
>Divrei Malkiel</a> that the time to walk the distance of a <a href=
"https://en.wikipedia.org/wiki/Biblical_and_Talmudic_units_of_measurement">mil</a> is 15 minutes, for a total of 60 minutes
for 4 mil after {@link #getSunset() sunset} or {@link #getSeaLevelSunset() sea level sunset} (depending on the
{@link #isUseElevation()} setting). See detailed documentation explaining the 60 minute concept at {@link #getAlos60Minutes()}.

@return the <code>Instant</code> representing 60 minutes after sea level sunset. If the calculation can't be computed such as
        in the Arctic Circle where there is at least one day a year where the sun does not rise, and one where it does not set,
        a <code>null</code> will be returned. See detailed explanation on top of the {@link AstronomicalCalendar} documentation.

@see #getAlos60Minutes()
@see #getPlagHamincha60Minutes()
@see #getShaahZmanis60Minutes()
```

# Human docs

```markdown
```
