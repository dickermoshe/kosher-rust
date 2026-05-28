# getPlagAhavatShalom

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getPlagAhavatShalom` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:2041)

```javadoc
This method returns the time of <em>plag hamincha</em> (the earliest time that Shabbos can be started) based on the
opinion of <a href="https://en.wikipedia.org/wiki/Yaakov_Moshe_Hillel">Rabbi Yaakov Moshe Hillel</a> as published in
the <em>luach</em> of the Bais Horaah of Yeshivat Chevrat Ahavat Shalom that that <em>plag hamincha</em> is calculated
as 1.25 <em>shaos zmaniyos</em> before {@link #getTzaisGeonim3Point8Degrees() <em>tzais</em> 3.8°} with <em>shaos
zmaniyos</em> calculated based on a day starting at {@link #getAlos16Point1Degrees() <em>alos</em> 16.1°} and
ending at <em>tzais</em> 3.8°.

@return the <code>Instant</code> of the <em>plag</em>. If the calculation can't be computed such as northern and
        southern locations even south of the Arctic Circle and north of the Antarctic Circle where the sun may not
        reach low enough below the horizon for this calculation, a <code>null</code> will be returned. See detailed
        explanation on top of the {@link AstronomicalCalendar} documentation.
@see #getShaahZmanisAlos16Point1ToTzais3Point8()
@see #getMinchaGedolaAhavatShalom()
@see #getMinchaKetanaAhavatShalom()
```

# Human docs

```markdown
Plag hamincha according to [Rabbi Yaakov Moshe Hillel](https://en.wikipedia.org/wiki/Yaakov_Moshe_Hillel), as published in the luach of the Bais Horaah of Yeshivat Chevrat Ahavat Shalom.

1.25 shaos zmaniyos before tzais at 3.8 degrees, with shaos zmaniyos based on a day from alos at 16.1 degrees to tzais at 3.8 degrees.

At some northern and southern locations, this zman may not be available or cannot be calculated if the sun does not reach low enough below the horizon.
```
