# getSofZmanShma3HoursBeforeChatzos

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getSofZmanShma3HoursBeforeChatzos` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:1235)

```javadoc
This method returns the latest <em>zman krias shema</em> (time to recite <em>Shema</em> in the morning) calculated as 3 hours
(regular clock hours and not <em>shaos zmaniyos</em>) before {@link #getChatzosHayom()}. Generally known as part of the
"Komarno" <em>zmanim</em> after <a href="https://en.wikipedia.org/wiki/Komarno_(Hasidic_dynasty)#Rabbi_Yitzchak_Eisik_Safrin"
>Rav Yitzchak Eizik of Komarno</a>, a proponent of this calculation, it actually predates him a lot. It is the opinion of the
<em>Shach</em> in the Nekudas Hakesef (Yoreh Deah 184), <a href=
"https://hebrewbooks.org/pdfpager.aspx?req=21638&st=&pgnum=30">Rav Moshe Lifshitz</a> in his commentary <a href=
"https://hebrewbooks.org/pdfpager.aspx?req=21638&st=&pgnum=50">Lechem Mishneh on Brachos 1:2</a>. It is next brought down
about 100 years later by the <a href="https://en.wikipedia.org/wiki/Jacob_Emden">Yaavetz</a> (in his <em>siddur</em>,
<a href="https://hebrewbooks.org/pdfpager.aspx?req=7920&st=&pgnum=6">Mor Uktziah Orach
Chaim 1</a>, <a href="https://hebrewbooks.org/pdfpager.aspx?req=22309&st=&pgnum=30">Lechem Shamayim, Brachos 1:2</a> and
<a href="https://hebrewbooks.org/pdfpager.aspx?req=1408&st=&pgnum=69">She'elos Yaavetz vol. 1 no. 40</a>), Rav Yitzchak Eizik
of Komarno in the Ma'aseh Oreg on Mishnayos Brachos 11:2, Shevus Yaakov, Chasan Sofer and others. See Yisrael Vehazmanim
<a href="https://hebrewbooks.org/pdfpager.aspx?req=9765&st=&pgnum=83">vol. 1 7:3, page 55 - 62</a>.

@return the <code>Instant</code> of the latest <em>zman krias shema</em>. If the calculation can't be computed such
        as in the Arctic Circle where there is at least one day a year where the sun does not rise, and one where
        it does not set, a <code>null</code> will be returned. See detailed explanation on top of the
        {@link AstronomicalCalendar} documentation.
@see #getChatzosHayom()
@see #getSofZmanTfila2HoursBeforeChatzos()
@see #isUseAstronomicalChatzos()
```

# Human docs

```markdown
```
