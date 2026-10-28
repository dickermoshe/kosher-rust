import 'dart:async';

/// A time zone, described by its offset from UTC at each instant.
///
/// Implement [offset] with any time zone library, such as `package:timezone`, and pass it to
/// `ZmanimCalculator` or `MyZmanimCalculator`, as in
/// `ZmanimCalculator(..., offset: zone.offset)`.
///
/// ```dart
/// import 'package:timezone/timezone.dart' as tz;
///
/// class TzZone extends TimeZone {
///   TzZone(this.location);
///
///   final tz.Location location;
///
///   @override
///   Duration offset(int instant) =>
///       Duration(milliseconds: location.timeZone(instant * 1000).offset);
/// }
/// ```
abstract class TimeZone {
  /// Const constructor, for subclasses.
  const TimeZone();

  /// The time zone of this device, as `DateTime.timeZoneOffset` reports it.
  static const TimeZone local = _LocalTimeZone();

  /// Coordinated Universal Time.
  static const TimeZone utc = FixedTimeZone(Duration.zero);

  /// The offset from UTC in effect at `instant`, in seconds since the Unix epoch. It is positive
  /// east of Greenwich.
  FutureOr<Duration> offset(int instant);
}

/// A time zone whose offset never changes.
class FixedTimeZone extends TimeZone {
  /// The time zone `offset` ahead of UTC, or behind it when negative.
  const FixedTimeZone(this.utcOffset);

  /// The offset from UTC.
  final Duration utcOffset;

  @override
  Duration offset(int instant) => utcOffset;
}

class _LocalTimeZone extends TimeZone {
  const _LocalTimeZone();

  @override
  Duration offset(int instant) =>
      DateTime.fromMillisecondsSinceEpoch(instant * 1000).timeZoneOffset;
}
