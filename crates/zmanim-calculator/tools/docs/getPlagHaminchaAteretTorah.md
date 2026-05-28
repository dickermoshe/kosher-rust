# getPlagHaminchaAteretTorah

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getPlagHaminchaAteretTorah` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:2703)

```javadoc
This method returns the time of <em>plag hamincha</em> based on the calculation of <em>Chacham</em> Yosef Harari-Raful
of Yeshivat Ateret Torah, that the day starts {@link #getAlos72Zmanis() 1/10th of the day} before sunrise and is
usually calculated as ending {@link #getTzaisAteretTorah() 40 minutes after sunset} (configurable to any offset
via {@link #setAteretTorahSunsetOffset(double)}). <em>shaos zmaniyos</em> are calculated based on this day and
added to {@link #getAlos72Zmanis() <em>alos</em>} to reach this time. This time is 10.75
{@link #getShaahZmanisAteretTorah() <em>shaos zmaniyos</em>} (temporal hours) after {@link #getAlos72Zmanis()
dawn}.

@return the <code>Instant</code> of the <em>plag</em>. If the calculation can't be computed such as in the Arctic Circle
        where there is at least one day a year where the sun does not rise, and one where it does not set, a null
        will be returned. See detailed explanation on top of the {@link AstronomicalCalendar} documentation.
@see #getAlos72Zmanis()
@see #getTzaisAteretTorah()
@see #getShaahZmanisAteretTorah()
@see #setAteretTorahSunsetOffset(double)
@see #getAteretTorahSunsetOffset()
```

# Human docs

```markdown
Plag hamincha according to the Ateret Torah calculation of Chacham Yosef Harari-Raful of Yeshivat Ateret Torah.

10.75 shaos zmaniyos after alos.

The day begins at alos 1/10 of the day before sunrise and ends {ateret_torah_offset} after sunset.

In places where sunrise or sunset cannot be calculated, this zman may not be available.
```
