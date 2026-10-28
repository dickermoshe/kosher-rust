/// A day on the Gregorian calendar, such as `(year: 2024, month: 4, day: 23)`.
///
/// The month runs from 1 for January to 12 for December. A date that does not exist, such as
/// 30 February, is an error.
typedef GregorianDate = ({int year, int month, int day});
