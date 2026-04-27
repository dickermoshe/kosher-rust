/// Convert a Gregorian year to an approximate UNIX timestamp in seconds.
double yearToTimestamp(int year) {
  return (year - 1970) * 365.25 * 24 * 60 * 60;
}
