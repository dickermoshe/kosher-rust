# getPlagHamincha60Minutes

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getPlagHamincha60Minutes` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:1757)

```javadoc
This method returns the time of <em>plag hamincha</em> according to the Magen Avraham with the day starting 60
minutes before sunrise and ending 60 minutes after sunset. This is calculated as 10.75 hours after {@link
#getAlos60Minutes() dawn}. The formula used is 10.75 {@link #getShaahZmanis60Minutes()} after {@link #getAlos60Minutes()}.

@return the <code>Instant</code> of the time of <em>plag hamincha</em>. If the calculation can't be computed such as
        in the Arctic Circle where there is at least one day a year where the sun does not rise, and one where it
        does not set, a <code>null</code> will be returned. See detailed explanation on top of the
        {@link AstronomicalCalendar} documentation.
@see #getShaahZmanis60Minutes()
@see #getAlos60Minutes()
@see #getTzais60Minutes()
```

# Human docs

```markdown
Plag hamincha according to the Magen Avraham, using a day that starts 60 minutes before sunrise and ends 60 minutes after sunset.

10.75 shaos zmaniyos after alos 60 minutes before sunrise.

In places such as the Arctic Circle, where there is at least one day a year when the sun does not rise and one when it does not set, this zman may not be available or cannot be calculated.
```
