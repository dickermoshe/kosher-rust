# Changelog

## Unreleased (0.3.0)

0.3 rebuilds the public API around what users do. The calculations are unchanged and still
checked against KosherJava and hebcal.

### Migrating from 0.2

#### Calendar

| 0.2 | 0.3 |
|---|---|
| `use kosher_rust::calendar::prelude::*` | `use kosher_rust::{HebrewDate, Region}` |
| `date.holidays(in_israel, use_modern)` on any date | `HebrewDate::from(date).holidays(Region::Israel)`; modern holidays are always included, filter with `Holiday::is_modern()` |
| `date.is_assur_bemelacha(in_israel)`, `has_candle_lighting(in_israel)` | same names on `HebrewDate`, taking a `Region` |
| `date.todays_parsha(..)`, `upcoming_parsha(..)` | `HebrewDate::parsha(..)`, `next_parsha(..)` |
| `date.hebrew_date()` (`HebrewCalendarDate`) | `HebrewDate::from(date)` |
| `Hebrew::is_hebrew_leap_year(y)`, `days_in_hebrew_year(y)`, `days_in_hebrew_month(y, m)` | `HebrewYear::new(y).is_leap()`, `.days()`, `.days_in(month)` |
| `Hebrew::cheshvan_kislev_kviah(y)` → `YearLengthType` | `HebrewYear::new(y).length()` → `YearLength` |
| `calendar::month::NISAN` and the other ICU `Month` constants | `HebrewMonth::Nisan` |
| `month.he(year)` (`HebrewMonthExt`) | `HebrewMonth::hebrew()` |
| `Holiday::he()`, `Parsha::he()` | `hebrew()` |
| `Holiday::is_modern_holiday()` | `Holiday::is_modern()` |
| `zmanim::molad::MoladCalendar::molad(tz)` | `HebrewDate::molad()` → `Molad`, the molad of the date's month |

`Display` on `Holiday` and `Parsha` now prints the English name ("Pesach", "Bereshis"); use
`hebrew()` for the Hebrew one.

`HebrewYear::days()`, `is_cheshvan_long()` and `is_kislev_short()` no longer return `Option`.
