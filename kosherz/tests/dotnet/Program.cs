// Calls the generated .NET bindings. Run from this directory after `cargo build -p kosherz`
// (see .github/workflows/bindings.yaml):
//
//     KOSHERZ_LIBRARY=../../../target/debug/libkosherz.so dotnet run
using System;
using Kosherz;

var lakewood = Times.SeaLevelLocation(40.09, -74.22);
var date = new GregorianDate(2024, 4, 23);

using (var day = new Zmanim(lakewood, date, Times.DefaultConfig(), new FixedOffset(-4)))
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
    using var invalid = new Zmanim(Times.SeaLevelLocation(91, 0), date, Times.DefaultConfig(), new FixedOffset(-4));
    Check(false, "latitude 91 did not throw");
}
catch (ZmanimException.InvalidLatitude)
{
}

// Kiddush Levana is the one time whose day depends on the offset: with UTC-4 it falls on
// 3 December 2024, and with UTC+4 it does not.
var december = new GregorianDate(2024, 12, 3);
using (var day = new Zmanim(lakewood, december, Times.DefaultConfig(), new FixedOffset(-4)))
{
    var ms = day.Get(Zman.TchilasZmanKidushLevana3Days);
    Check(ms == 1733279333504, $"UTC-4: kiddush levana is {ms}, expected 1733279333504");
}
Throws<ZmanimException.NotApplicable>("UTC+4", () =>
{
    using var day = new Zmanim(lakewood, december, Times.DefaultConfig(), new FixedOffset(4));
    day.Get(Zman.TchilasZmanKidushLevana3Days);
});
Throws<ZmanimException.TimeZoneRequired>("no time zone", () =>
{
    using var day = new Zmanim(lakewood, december, Times.DefaultConfig(), null);
    day.Get(Zman.TchilasZmanKidushLevana3Days);
});
Throws<ZmanimException.TimeZoneCallbackFailed>("a throwing callback", () =>
{
    using var day = new Zmanim(lakewood, date, Times.DefaultConfig(), new BrokenOffset());
    day.Get(Zman.Sunrise);
});

Console.WriteLine("ok");

static void Throws<T>(string name, Action action) where T : Exception
{
    try
    {
        action();
    }
    catch (T)
    {
        return;
    }
    Check(false, $"{name} did not throw {typeof(T).Name}");
}

static void Check(bool condition, string message)
{
    if (condition) return;
    Console.Error.WriteLine($"FAIL: {message}");
    Environment.Exit(1);
}

/// <summary>UTC plus a number of hours, at every instant.</summary>
sealed class FixedOffset(int hours) : ITimeZoneOffset
{
    public int OffsetSeconds(long unixSeconds) => hours * 3600;

    public long LocalToUnixSeconds(int year, int month, int day, int hour, int minute, int second) =>
        new DateTimeOffset(year, month, day, hour, minute, second, TimeSpan.FromHours(hours)).ToUnixTimeSeconds();
}

/// <summary>A time zone whose callbacks fail.</summary>
sealed class BrokenOffset : ITimeZoneOffset
{
    public int OffsetSeconds(long unixSeconds) => throw new InvalidOperationException("no time zone data");

    public long LocalToUnixSeconds(int year, int month, int day, int hour, int minute, int second) =>
        throw new InvalidOperationException("no time zone data");
}
