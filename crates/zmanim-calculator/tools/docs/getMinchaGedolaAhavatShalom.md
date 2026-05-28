# getMinchaGedolaAhavatShalom

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getMinchaGedolaAhavatShalom` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:1611)

```javadoc
This method returns the time of <em>mincha gedola</em> based on the opinion of <a href=
"https://en.wikipedia.org/wiki/Yaakov_Moshe_Hillel">Rabbi Yaakov Moshe Hillel</a> as published in the <em>luach</em>
of the Bais Horaah of Yeshivat Chevrat Ahavat Shalom that <em>mincha gedola</em> is calculated as half a <em>shaah
zmanis</em> after <em>chatzos</em> with <em>shaos zmaniyos</em> calculated based on a day starting 72 minutes before sunrise
{@link #getAlos16Point1Degrees() <em>alos</em> 16.1°} and ending 13.5 minutes after sunset {@link
#getTzaisGeonim3Point7Degrees() <em>tzais</em> 3.7°}. <em>Mincha gedola</em> is the earliest time to pray <em>mincha</em>.
The later of this time or 30 clock minutes after <em>chatzos</em> is returned. See {@link
#getMinchaGedolaGreaterThan30(Instant)} for a way to claculate the later of 30 minutes or this <em>mincha gedola</em>.
For more information about <em>mincha gedola</em> see the documentation on {@link #getMinchaGedolaGRA() <em>mincha gedola</em>}.
Since calculation of this <em>zman</em> involves <em>chatzos</em> that is offset from the center of the astronomical day,
{@link #isUseAstronomicalChatzosForOtherZmanim()} is N/A here.
@return the <code>Instant</code> of the <em>mincha gedola</em>. If the calculation can't be computed such as northern and
        southern locations even south of the Arctic Circle and north of the Antarctic Circle where the sun may not
        reach low enough below the horizon for this calculation, a <code>null</code> will be returned. See detailed
        explanation on top of the {@link AstronomicalCalendar} documentation.
@see #getAlos16Point1Degrees()
@see #getTzaisGeonim3Point7Degrees()
@see #getShaahZmanisAlos16Point1ToTzais3Point7()
@see #getMinchaGedolaGreaterThan30(Instant)
```

# Human docs

```markdown
Mincha gedola according to Rabbi Yaakov Moshe Hillel, as published in the luach of the Bais Horaah of Yeshivat Chevrat Ahavat Shalom.

Half a shaah zmanis after chatzos, using a day from alos 16.1 degrees to tzais 3.7 degrees.

The later of this time or 30 clock minutes after chatzos is used.

At some northern and southern locations, this zman may not be available if the sun does not reach low enough below the horizon.
```
