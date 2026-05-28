# getSofZmanKidushLevanaBetweenMoldos

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getSofZmanKidushLevanaBetweenMoldos` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:3025)

```javadoc
Returns the latest time of Kiddush Levana according to the <a
href="https://en.wikipedia.org/wiki/Yaakov_ben_Moshe_Levi_Moelin">Maharil's</a> opinion that it is calculated as
halfway between <em>molad</em> and <em>molad</em>. This adds half the 29 days, 12 hours and 793 chalakim time between
<em>molad</em> and <em>molad</em> (14 days, 18 hours, 22 minutes and 666 milliseconds) to the month's <em>molad</em>.
The <em>sof zman Kiddush Levana</em> will be returned even if it occurs during the day. To limit the time to between
<em>tzais</em> and <em>alos</em>, see {@link #getSofZmanKidushLevanaBetweenMoldos(Instant, Instant)}.

@return the Instant representing the moment halfway between molad and molad. If the time occurs between
        <em>alos</em> and <em>tzais</em>, <em>alos</em> will be returned. If the <em>zman</em> will not occur on this
        day, a <code>null</code> will be returned.
@see #getSofZmanKidushLevanaBetweenMoldos(Instant, Instant)
@see #getSofZmanKidushLevana15Days()
@see JewishCalendar#getSofZmanKidushLevanaBetweenMoldos()
```

# Human docs

```markdown
```
