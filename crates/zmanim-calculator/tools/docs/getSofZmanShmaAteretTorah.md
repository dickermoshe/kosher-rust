# getSofZmanShmaAteretTorah

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getSofZmanShmaAteretTorah` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:2604)

```javadoc
This method returns the latest <em>zman krias shema</em> (time to recite Shema in the morning) based on the calculation of
<em>Chacham</em> Yosef Harari-Raful of Yeshivat Ateret Torah, that the day starts {@link #getAlos72Zmanis() 1/10th of the day}
before sunrise and is usually calculated as ending {@link #getTzaisAteretTorah() 40 minutes after sunset} (configurable to any
offset via {@link #setAteretTorahSunsetOffset(double)}). <em>Shaos zmaniyos</em> are calculated based on this day and added to
{@link #getAlos72Zmanis() <em>alos</em>} to reach this time. This time is 3 {@link #getShaahZmanisAteretTorah() <em>shaos
zmaniyos</em>} (temporal hours) after {@link #getAlos72Zmanis() <em>alos</em> 72 <em>zmaniyos</em>}. <b>Note: </b> Based on
this calculation <em>chatzos</em> will not be at midday.

@return the <code>Instant</code> of the latest <em>zman krias shema</em> based on this calculation. If the calculation can't
        be computed such as in the Arctic Circle where there is at least one day a year where the sun does not rise, and one
        where it does not set, a <code>null</code> will be returned. See detailed explanation on top of the
        {@link AstronomicalCalendar} documentation.
@see #getAlos72Zmanis()
@see #getTzaisAteretTorah()
@see #getAteretTorahSunsetOffset()
@see #setAteretTorahSunsetOffset(double)
@see #getShaahZmanisAteretTorah()
```

# Human docs

```markdown
```
