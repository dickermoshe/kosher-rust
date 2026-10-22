// Calls the generated .NET bindings. Run from this directory after `cargo build -p kosherz`
// (see .github/workflows/bindings.yaml):
//
//     KOSHERZ_LIBRARY=../../../target/debug/libkosherz.so dotnet run
using System;
using Kosherz;

var lakewood = Times.SeaLevelLocation(40.09, -74.22);
var date = new GregorianDate(2024, 4, 23);

using (var day = new Zmanim(lakewood, date, Times.DefaultConfig(), new FixedOffset()))
{
    var seconds = day.Get(Zman.Sunrise) / 1000;
    Check(seconds == 1713866795, $"sunrise is {seconds}, expected 1713866795 (06:06:35 EDT)");
}

using (var hebrew = HebrewDate.FromGregorian(date))
{
    var daf = CalendarLimudim.DafYomiBavli(hebrew);
    Check(daf?.Display == "Bava Metzia 55", $"daf yomi is {daf?.Display ?? "null"}, expected Bava Metzia 55");
}

try
{
    using var invalid = new Zmanim(Times.SeaLevelLocation(91, 0), date, Times.DefaultConfig(), new FixedOffset());
    Check(false, "latitude 91 did not throw");
}
catch (ZmanimException.InvalidLatitude)
{
}

Console.WriteLine("ok");

static void Check(bool condition, string message)
{
    if (condition) return;
    Console.Error.WriteLine($"FAIL: {message}");
    Environment.Exit(1);
}

/// <summary>Eastern Daylight Time, UTC-4, at every instant.</summary>
sealed class FixedOffset : ITimeZoneOffset
{
    public int OffsetSeconds(long unixSeconds) => -4 * 3600;
}
