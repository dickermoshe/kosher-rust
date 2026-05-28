# getSofZmanKidushLevana15Days

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getSofZmanKidushLevana15Days` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:3085)

```javadoc
Returns the latest time of <em>Kiddush Levana</em> calculated as 15 days after the molad. This is the opinion of
the Shulchan Aruch (Orach Chaim 426). It should be noted that some opinions hold that the
<a href="https://en.wikipedia.org/wiki/Moses_Isserles">Rema</a> who brings down the opinion of the <a
href="https://en.wikipedia.org/wiki/Yaakov_ben_Moshe_Levi_Moelin">Maharil's</a> of calculating
{@link #getSofZmanKidushLevanaBetweenMoldos(Instant, Instant) half way between <em>molad</em> and <em>molad</em>} is of
the opinion that the Mechaber agrees to his opinion. Also see the Aruch Hashulchan. For additional details on the subject,
See Rabbi Dovid Heber's very detailed write-up in Siman Daled (chapter 4) of <a href="https://hebrewbooks.org/53000">Shaarei
Zmanim</a>. The <em>sof zman Kiddush Levana</em> will be returned even if it occurs during the day. To limit the time to
between <em>tzais</em> and <em>alos</em>, see {@link #getSofZmanKidushLevana15Days(Instant, Instant)}.

@return the Instant representing the moment 15 days after the <em>molad</em>. If the time occurs between
        <em>alos</em> and <em>tzais</em>, <em>alos</em> will be returned. If the <em>zman</em> will not occur on this day, a
        <code>null</code> will be returned.
@see #getSofZmanKidushLevana15Days(Instant, Instant)
@see #getSofZmanKidushLevanaBetweenMoldos()
@see JewishCalendar#getSofZmanKidushLevana15Days()
```

# Human docs

```markdown
```
