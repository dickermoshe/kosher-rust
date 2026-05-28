# getSamuchLeMinchaKetana16Point1Degrees

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getSamuchLeMinchaKetana16Point1Degrees` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:3840)

```javadoc
A method for calculating <em>samuch lemincha ketana</em>, / near <em>mincha ketana</em> time that is half an hour before
{@link #getMinchaKetanaGRA()} or is 9 * <em>shaos zmaniyos</em> (solar hours) after the start of the day, calculated based
on a day from   and ending  a day starting at {@link #getMinchaGedola16Point1Degrees() <em>alos</em> 16.1°} and ending
at {@link #getTzais72Minutes() <em>tzais</em> 16.1°}. This is the time that eating or other activity can't begin prior to
praying <em>mincha</em>.  The calculation used is 9 * {@link #getShaahZmanis16Point1Degrees()} after {@link
#getAlos16Point1Degrees() <em>alos</em> 16.1°}. See the <a href=
"https://hebrewbooks.org/pdfpager.aspx?req=60387&st=&pgnum=294">Mechaber and Mishna Berurah 232</a> and <a href=
"https://hebrewbooks.org/pdfpager.aspx?req=60388&pgnum=34">249:2</a>.

@see #getSamuchLeMinchaKetana(Instant, Instant, boolean)
@see #getShaahZmanis16Point1Degrees()
@return the <code>Instant</code> of the time of <em>samuch lemincha ketana</em>. If the calculation can't be computed such
        as northern and southern locations even south of the Arctic Circle and north of the Antarctic Circle
        where the sun may not reach low enough below the horizon for this calculation, a <code>null</code> will be returned.
        See detailed explanation on top of the {@link AstronomicalCalendar} documentation.
```

# Human docs

```markdown
The point near mincha ketana when eating or other activity should not begin before praying mincha, using the 16.1-degree day.

9 shaos zmaniyos after alos at 16.1 degrees below the horizon, using a day that begins and ends at 16.1 degrees.

This is half a shaah zmanis before mincha ketana for this calculation.

At some northern and southern locations, including places even south of the Arctic Circle and north of the Antarctic Circle, this zman may not be available or cannot be calculated if the sun does not reach low enough below the horizon.

See [Mechaber and Mishna Berurah 232](https://hebrewbooks.org/pdfpager.aspx?req=60387&st=&pgnum=294) and [249:2](https://hebrewbooks.org/pdfpager.aspx?req=60388&pgnum=34).
```
