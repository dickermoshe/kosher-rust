# getBainHashmashosRT2Stars

Source: `com.kosherjava.zmanim.ComprehensiveZmanimCalendar.getBainHashmashosRT2Stars` (C:\Users\Moshe\DickerSystems\rust-zmanim-project\crates\zmanim-calculator\java\src\main\java\com\kosherjava\zmanim\ComprehensiveZmanimCalendar.java:2122)

```javadoc
This method returns the beginning of <em>bain hashmashos</em> of Rabbeinu Tam calculated according to the
opinion of the <em>Divrei Yosef</em> (see Yisrael Vehazmanim) calculated 5/18th (27.77%) of the time between
<em>alos</em> (calculated as 19.8° before sunrise) and sunrise. This is added to sunset to arrive at the time
for <em>bain hashmashos</em> of Rabbeinu Tam.

@return the <code>Instant</code> of <em>bain hashmashos</em> of Rabbeinu Tam for this calculation. If the
        calculation can't be computed such as northern and southern locations even south of the Arctic Circle and
        north of the Antarctic Circle where the sun may not reach low enough below the horizon for this
        calculation, a <code>null</code> will be returned. See detailed explanation on top of the
        {@link AstronomicalCalendar} documentation.
```

# Human docs

```markdown
The beginning of Rabbeinu Tam's bain hashmashos, according to the Divrei Yosef (see Yisrael Vehazmanim).

Calculated as 5/18 (about 27.77%) of the time from alos at 19.8 degrees before sunrise to sunrise; that interval is added after sunset to reach Rabbeinu Tam's bain hashmashos.

At some northern and southern locations, including places even south of the Arctic Circle and north of the Antarctic Circle, this zman may not be available or cannot be calculated if the sun does not reach low enough below the horizon.
```
