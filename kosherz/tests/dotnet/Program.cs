// Calls the generated .NET bindings. Run from this directory after `cargo build -p kosherz`
// (see .github/workflows/bindings.yaml):
//
//     KOSHERZ_LIBRARY=../../../target/debug/libkosherz.so dotnet run
using System;
using Kosherz;

var lakewood = Times.SeaLevelLocation(40.09, -74.22);
var date = new GregorianDate(2024, 4, 23);
var newYork = TimeZoneInfo.FindSystemTimeZoneById("America/New_York");

using (var day = new Zmanim(lakewood, date, Times.DefaultConfig(), LocalDays.In(newYork, date)))
{
    var seconds = day.Get(Zman.Sunrise) / 1000;
    Check(seconds == 1713866795, $"sunrise is {seconds}, expected 1713866795 (06:06:35 EDT)");
}

using (var hebrew = HebrewDate.FromGregorian(date))
{
    var daf = CalendarLimudim.DafYomiBavli(hebrew);
    Check(daf?.Display == "Bava Metzia 55", $"daf yomi is {daf?.Display ?? "null"}, expected Bava Metzia 55");
}

Throws<ZmanimException.InvalidLatitude>("latitude 91", () =>
{
    using var invalid = new Zmanim(Times.SeaLevelLocation(91, 0), date, Times.DefaultConfig(), LocalDays.Fixed(-4, date));
});

// Kiddush Levana is the one time whose day depends on the offset: with UTC-4 it falls on
// 3 December 2024, and with UTC+4 it does not.
var december = new GregorianDate(2024, 12, 3);
using (var day = new Zmanim(lakewood, december, Times.DefaultConfig(), LocalDays.Fixed(-4, december)))
{
    var ms = day.Get(Zman.TchilasZmanKidushLevana3Days);
    Check(ms == 1733279333504, $"UTC-4: kiddush levana is {ms}, expected 1733279333504");
}
Throws<ZmanimException.NotApplicable>("UTC+4", () =>
{
    using var day = new Zmanim(lakewood, december, Times.DefaultConfig(), LocalDays.Fixed(4, december));
    day.Get(Zman.TchilasZmanKidushLevana3Days);
});
Throws<ZmanimException.TimeZoneRequired>("no local day", () =>
{
    using var day = new Zmanim(lakewood, december, Times.DefaultConfig(), null);
    day.Get(Zman.TchilasZmanKidushLevana3Days);
});
Throws<ZmanimException.InvalidLocalDay>("a local day that ends before it starts", () =>
{
    using var day = new Zmanim(lakewood, date, Times.DefaultConfig(), new LocalDay(1, 0, 0, 0));
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

/// <summary>Local days: a date's two local midnights and the offsets at them.</summary>
static class LocalDays
{
    /// <summary><paramref name="date"/> in <paramref name="zone"/>. A midnight the clocks skip
    /// becomes the first instant after it.</summary>
    public static LocalDay In(TimeZoneInfo zone, GregorianDate date)
    {
        var start = Midnight(zone, new DateTime(date.Year, date.Month, date.Day));
        var end = Midnight(zone, new DateTime(date.Year, date.Month, date.Day).AddDays(1));
        return new LocalDay(
            start.ToUnixTimeMilliseconds(), (int)start.Offset.TotalSeconds,
            end.ToUnixTimeMilliseconds(), (int)end.Offset.TotalSeconds);
    }

    /// <summary><paramref name="date"/> at UTC plus <paramref name="hours"/> all year.</summary>
    public static LocalDay Fixed(int hours, GregorianDate date) =>
        In(TimeZoneInfo.CreateCustomTimeZone("fixed", TimeSpan.FromHours(hours), "fixed", "fixed"), date);

    static DateTimeOffset Midnight(TimeZoneInfo zone, DateTime local)
    {
        while (zone.IsInvalidTime(local)) local = local.AddMinutes(1);
        return new DateTimeOffset(local, zone.GetUtcOffset(local));
    }
}
