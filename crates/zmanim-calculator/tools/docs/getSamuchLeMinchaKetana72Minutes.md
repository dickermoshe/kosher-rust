# getSamuchLeMinchaKetana72Minutes

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getSamuchLeMinchaKetana72Minutes` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:3860)

```javadoc
A method for calculating <em>samuch lemincha ketana</em>, / near <em>mincha ketana</em> time that is half an hour before
{@link #getMinchaKetanaGRA()} or is 9 * <em>shaos zmaniyos</em> (solar hours) after the start of the day, calculated based
on a day from   and ending  a day starting at {@link #getAlos72Minutes() <em>alos</em> 72 minutes} and ending at {@link
#getTzais72Minutes() <em>tzais</em> 72 minutes}. This is the time that eating or other activity can't begin prior to praying
<em>mincha</em>. The calculation used is 9 * {@link #getShaahZmanis72Minutes()} after {@link #getAlos72Minutes() <em>alos</em>
72 minutes}. See the <a href="https://hebrewbooks.org/pdfpager.aspx?req=60387&st=&pgnum=294">Mechaber and Mishna Berurah
232</a> and <a href="https://hebrewbooks.org/pdfpager.aspx?req=60388&pgnum=34">249:2</a>.

@see #getSamuchLeMinchaKetana(Instant, Instant, boolean)
@see #getShaahZmanis72Minutes()
@return the <code>Instant</code> of the time of <em>samuch lemincha ketana</em>. If the calculation can't be computed such
        as northern and southern locations even south of the Arctic Circle and north of the Antarctic Circle
        where the sun may not reach low enough below the horizon for this calculation, a <code>null</code> will be returned.
        See detailed explanation on top of the {@link AstronomicalCalendar} documentation.
```

# Human docs

```markdown
```
