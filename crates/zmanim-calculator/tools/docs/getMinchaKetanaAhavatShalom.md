# getMinchaKetanaAhavatShalom

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getMinchaKetanaAhavatShalom` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:1715)

```javadoc
This method returns the time of <em>mincha ketana</em> based on the opinion of <a href=
"https://en.wikipedia.org/wiki/Yaakov_Moshe_Hillel">Rabbi Yaakov Moshe Hillel</a> as published in the <em>luach</em>
of the Bais Horaah of Yeshivat Chevrat Ahavat Shalom that <em>mincha ketana</em> is calculated as 2.5 <em>shaos
zmaniyos</em> before {@link #getTzaisGeonim3Point8Degrees() <em>tzais</em> 3.8°} with <em>shaos zmaniyos</em>
calculated based on a day starting at {@link #getAlos16Point1Degrees() <em>alos</em> 16.1°} and ending at
<em>tzais</em> 3.8°. <em>Mincha ketana</em> is the preferred earliest time to pray <em>mincha</em> according to
the opinion of the <a href="https://en.wikipedia.org/wiki/Maimonides">Rambam</a> and others. For more information
on this see the documentation on {@link #getMinchaKetanaGRA() <em>mincha ketana</em>}.

@return the <code>Instant</code> of the time of <em>mincha ketana</em>. If the calculation can't be computed such as
        northern and southern locations even south of the Arctic Circle and north of the Antarctic Circle where the
        sun may not reach low enough below the horizon for this calculation, a <code>null</code> will be returned.
        See detailed explanation on top of the {@link AstronomicalCalendar} documentation.

@see #getShaahZmanisAlos16Point1ToTzais3Point8()
@see #getMinchaGedolaAhavatShalom()
@see #getPlagAhavatShalom()
```

# Human docs

```markdown
Mincha ketana according to Rabbi Yaakov Moshe Hillel, as published in the luach of the Bais Horaah of Yeshivat Chevrat Ahavat Shalom.

2.5 shaos zmaniyos before tzais 3.8 degrees, using a day from alos 16.1 degrees to tzais 3.8 degrees.

At some northern and southern locations, this zman may not be available if the sun does not reach low enough below the horizon.
```
