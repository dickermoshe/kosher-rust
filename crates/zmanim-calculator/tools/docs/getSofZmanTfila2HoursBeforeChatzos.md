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
```
