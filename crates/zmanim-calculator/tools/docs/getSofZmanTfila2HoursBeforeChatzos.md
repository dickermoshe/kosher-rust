# getSofZmanTfila2HoursBeforeChatzos

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getSofZmanTfila2HoursBeforeChatzos` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:1502)

```javadoc
This method returns the latest <em>zman tfila</em> (time to recite the morning prayers) calculated as 2 hours
before {@link #getChatzosHayom()}. This is based on the opinions that calculate
<em>sof zman krias shema</em> as {@link #getSofZmanShma3HoursBeforeChatzos()}. This returns the time of 2 hours
before {@link #getChatzosHayom()}.

@return the <code>Instant</code> of the latest <em>zman krias shema</em>. If the calculation can't be computed such
        as in the Arctic Circle where there is at least one day a year where the sun does not rise, and one where
        it does not set, a <code>null</code> will be returned. See detailed explanation on top of the
        {@link AstronomicalCalendar} documentation.
@see #getChatzosHayom()
@see #getSofZmanShma3HoursBeforeChatzos()
```

# Human docs

```markdown
Sof zman tfila - the latest time to recite morning prayers (Shacharis), calculated as 2 regular clock hours before chatzos hayom (not shaos zmaniyos). Paired with sof zman krias shema at 3 clock hours before chatzos; often grouped with the "Komarno" zmanim after [Rav Yitzchak Eizik of Komarno](https://en.wikipedia.org/wiki/Komarno_(Hasidic_dynasty)#Rabbi_Yitzchak_Eisik_Safrin), though this calculation is much older.

2 clock hours before chatzos hayom.

This view is cited by the Shach in Nekudas Hakesef (Yoreh Deah 184), [Rav Moshe Lifshitz](https://hebrewbooks.org/pdfpager.aspx?req=21638&st=&pgnum=30) in [Lechem Mishneh on Brachos 1:2](https://hebrewbooks.org/pdfpager.aspx?req=21638&st=&pgnum=50), the [Yaavetz](https://en.wikipedia.org/wiki/Jacob_Emden), and later by Komarno, Shevus Yaakov, Chasan Sofer, and others. See also [Yisrael Vehazmanim vol. 1, 7:3](https://hebrewbooks.org/pdfpager.aspx?req=9765&st=&pgnum=83).

In places such as the Arctic Circle, where there is at least one day a year when the sun does not rise and one when it does not set, this zman may not be available or cannot be calculated.
```
