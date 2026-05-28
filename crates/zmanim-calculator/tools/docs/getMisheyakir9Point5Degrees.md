# getMisheyakir9Point5Degrees

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getMisheyakir9Point5Degrees` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:1041)

```javadoc
This method returns <em>misheyakir</em> based on the position of the sun when it is {@link ZENITH_9_POINT_5
9.5°} below {@link GEOMETRIC_ZENITH geometric zenith} (90°). This calculation is based on <a href=
"https://en.wikipedia.org/wiki/Dovid_Kronglas">Rabbi Dovid Kronglass's</a> Calculation of 45 minutes in Baltimore
as mentioned in <a href="https://hebrewbooks.org/pdfpager.aspx?req=20287&pgnum=29">Divrei Chachamim No. 24</a>
brought down by the <a href="https://hebrewbooks.org/pdfpager.aspx?req=50535&pgnum=87">Birur Halacha, Tinyana, Ch.
18</a>. This calculates to 9.5°. Also see <a href="https://en.wikipedia.org/wiki/Jacob_Isaac_Neiman">Rabbi Yaakov
Yitzchok Neiman</a> in Kovetz Eitz Chaim Vol. 9, p. 202 that the Vya'an Yosef did not want to rely on times earlier
than 45 minutes in New York. This <em>zman</em> is also used in the calendars published by Rabbi Hershel Edelstein.
As mentioned in Yisroel Vehazmanim, Rabbi Edelstein who was given the 45 minute <em>zman</em> by Rabbi Bick. The
calendars published by the <em><a href="https://en.wikipedia.org/wiki/Mizrahi_Jews">Edot Hamizrach</a></em> communities
also use this <em>zman</em>. This also follows the opinion of <a href="https://en.wikipedia.org/wiki/Shmuel_Kamenetsky"
>Rabbi Shmuel Kamenetsky</a> who provided the time of 36 and 45 minutes, but did not provide a degree-based time. Since
this <em>zman</em> depends on the level of light, Rabbi Yaakov Shakow presented these degree-based times to Rabbi Shmuel
Kamenetsky who agreed to them.

@return the <code>Instant</code> of <em>misheyakir</em>. If the calculation can't be computed such as
        northern and southern locations even south of the Arctic Circle and north of the Antarctic Circle where
        the sun may not reach low enough below the horizon for this calculation, a <code>null</code> will be returned.
        See detailed explanation on top of the {@link AstronomicalCalendar} documentation.
@see ZENITH_9_POINT_5
@see #getMisheyakir7Point65Degrees()
```

# Human docs

```markdown
Misheyakir according to the 45-minute approach used by some communities.

The time when the sun is 9.5 degrees below the horizon before sunrise.

This is based on a 45-minute misheyakir calculation.

At some northern and southern locations, this zman may not be available if the sun does not reach low enough below the horizon.
```
