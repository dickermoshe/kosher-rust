# getMinchaKetanaAteretTorah

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getMinchaKetanaAteretTorah` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:2681)

```javadoc
This method returns the time of <em>mincha ketana</em> based on the calculation of
<em>Chacham</em> Yosef Harari-Raful of Yeshivat Ateret Torah, that the day starts
{@link #getAlos72Zmanis() 1/10th of the day} before sunrise and is usually calculated as ending
{@link #getTzaisAteretTorah() 40 minutes after sunset} (configurable to any offset via
{@link #setAteretTorahSunsetOffset(double)}). This is the preferred earliest time to pray <em>mincha</em>
according to the opinion of the <a href="https://en.wikipedia.org/wiki/Maimonides">Rambam</a> and others.
For more information on this see the documentation on {@link #getMinchaGedolaGRA() <em>mincha gedola</em>}. This is
calculated as 9.5 {@link #getShaahZmanisAteretTorah() solar hours} after {@link #getAlos72Zmanis() <em>alos</em>}.
The calculation used is 9.5 * {@link #getShaahZmanisAteretTorah()} after {@link #getAlos72Zmanis() <em>alos</em>}.

@see #getAlos72Zmanis()
@see #getTzaisAteretTorah()
@see #getShaahZmanisAteretTorah()
@see #getAteretTorahSunsetOffset()
@see #setAteretTorahSunsetOffset(double)
@see #getMinchaGedolaGRA()
@see #getMinchaKetanaGRA()
@return the <code>Instant</code> of the time of <em>mincha ketana</em>. If the calculation can't be computed such as
        in the Arctic Circle where there is at least one day a year where the sun does not rise, and one where it
        does not set, a <code>null</code> will be returned. See detailed explanation on top of the
        {@link AstronomicalCalendar} documentation.
```

# Human docs

```markdown
Mincha ketana according to the Ateret Torah calculation of Chacham Yosef Harari-Raful of Yeshivat Ateret Torah.

9.5 shaos zmaniyos after alos.

The day begins at alos 1/10 of the day before sunrise and ends {ateret_torah_offset} after sunset.

In places where sunrise or sunset cannot be calculated, this zman may not be available.
```
