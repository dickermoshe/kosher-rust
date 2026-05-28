# getSamuchLeMinchaKetanaGRA

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getSamuchLeMinchaKetanaGRA` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:3818)

```javadoc
A method for calculating <em>samuch lemincha ketana</em>, / near <em>mincha ketana</em> time that is half an hour before
{@link #getMinchaKetanaGRA()} or is 9 * <em>shaos zmaniyos</em> (solar hours) after the start of
the day, calculated according to the <a href="https://en.wikipedia.org/wiki/Vilna_Gaon">GRA</a> using a day starting at
sunrise and ending at sunset. This is the time that eating or other activity can't begin prior to praying <em>mincha</em>.
The calculation used is 9 * {@link #getShaahZmanisGRA()} after {@link #getSunset() sunrise} or {@link
#getSunriseBasedOnElevationSetting() elevation adjusted sunrise} (depending on the {@link #isUseElevation()} setting). See the
<a href="https://hebrewbooks.org/pdfpager.aspx?req=60387&st=&pgnum=294">Mechaber and Mishna Berurah 232</a> and <a href=
"https://hebrewbooks.org/pdfpager.aspx?req=60388&pgnum=34">249:2</a>.

@see #getShaahZmanisGRA()
@see #getSamuchLeMinchaKetana(Instant, Instant, boolean)
@see #isUseAstronomicalChatzosForOtherZmanim()
@return the <code>Instant</code> of the time of <em>samuch lemincha ketana</em>. If the calculation can't be computed such
        as northern and southern locations even south of the Arctic Circle and north of the Antarctic Circle
        where the sun may not reach low enough below the horizon for this calculation, a <code>null</code> will be
        returned. See detailed explanation on top of the {@link AstronomicalCalendar} documentation.
```

# Human docs

```markdown
The point near mincha ketana when eating or other activity should not begin before praying mincha, following the [Vilna Gaon](https://en.wikipedia.org/wiki/Vilna_Gaon).

9 shaos zmaniyos after sunrise, using a day from sunrise to sunset. {uses_elevation}

This is half a shaah zmanis before mincha ketana.

In places where sunrise or sunset cannot be calculated, this zman may not be available.

See [Mechaber and Mishna Berurah 232](https://hebrewbooks.org/pdfpager.aspx?req=60387&st=&pgnum=294) and [249:2](https://hebrewbooks.org/pdfpager.aspx?req=60388&pgnum=34).
```
