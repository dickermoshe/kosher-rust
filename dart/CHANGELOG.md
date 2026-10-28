# Changelog

## 0.3.0-rc.2

The first release, binding kosher-rust 0.3.0-rc.2.

- **The calendar.** `HebrewDate` with its holidays, weekly *parsha*, special Shabbosos and molad,
  `HebrewYear` with its length and months, and the names of months, *parshiyos* and special days
  in English and Hebrew.
- **Zmanim.** `ZmanimCalculator` gives every time kosher-rust's `Zmanim` does, with a
  `ZmanimConfig` for the solar algorithm, elevation, *chatzos* and offsets.
  `MyZmanimCalculator` gives the times shown on the MyZmanim website.
- **Learning.** Daf Yomi, Daf HaShavua, Dirshu's Amud Yomi, Daf Yomi Yerushalmi, Mishna Yomis,
  monthly Tehillim and Pirkei Avos.
- **Your own time zone library.** Implement `TimeZone` with any time zone library and pass its
  `offset`. `TimeZone.local`, `TimeZone.utc` and `FixedTimeZone` are included.
- **No `DateTime`.** Dates are `GregorianDate` records and times are seconds since the Unix epoch.
- **Plain Dart and Flutter.** The build hook compiles the Rust crate, so there are no prebuilt
  binaries to download. A command-line program calls `disposeKosherRust` when it is done.
