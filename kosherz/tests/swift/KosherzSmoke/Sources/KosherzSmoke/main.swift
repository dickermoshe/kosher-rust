// Calls the generated Swift bindings. Run from this directory after `cargo build -p kosherz`
// (see .github/workflows/bindings.yaml):
//
//     LD_LIBRARY_PATH=../../../../target/debug swift run -Xlinker -L$PWD/../../../../target/debug
import Foundation
import Kosherz

/// `date` in `zone`: its two local midnights and the offsets at them.
func localDay(_ zone: TimeZone, _ date: GregorianDate) -> LocalDay {
    var calendar = Foundation.Calendar(identifier: .gregorian)
    calendar.timeZone = zone
    let components = DateComponents(year: Int(date.year), month: Int(date.month), day: Int(date.day))
    let start = calendar.startOfDay(for: calendar.date(from: components)!)
    let end = calendar.date(byAdding: .day, value: 1, to: start)!
    let nextDay = calendar.startOfDay(for: end)
    return LocalDay(
        startMs: Int64(start.timeIntervalSince1970 * 1000),
        startOffsetSeconds: Int32(zone.secondsFromGMT(for: start)),
        endMs: Int64(nextDay.timeIntervalSince1970 * 1000),
        endOffsetSeconds: Int32(zone.secondsFromGMT(for: nextDay)))
}

/// `date` at UTC plus `hours` all year.
func fixedDay(_ hours: Int, _ date: GregorianDate) -> LocalDay {
    localDay(TimeZone(secondsFromGMT: hours * 3600)!, date)
}

func check(_ condition: Bool, _ message: String) {
    if condition { return }
    FileHandle.standardError.write(Data("FAIL: \(message)\n".utf8))
    exit(1)
}

do {
    let lakewood = Times.seaLevelLocation(latitude: 40.09, longitude: -74.22)
    let date = GregorianDate(year: 2024, month: 4, day: 23)

    let day = try Zmanim(
        location: lakewood, date: date, config: Times.defaultConfig(),
        localDay: localDay(TimeZone(identifier: "America/New_York")!, date))
    let seconds = try day.get(zman: .sunrise) / 1000
    check(seconds == 1713866795, "sunrise is \(seconds), expected 1713866795 (06:06:35 EDT)")

    let hebrew = try HebrewDate.fromGregorian(date: date)
    let daf = Kosherz.Calendar.Limudim.dafYomiBavli(date: hebrew)
    check(daf?.display == "Bava Metzia 55", "daf yomi is \(daf?.display ?? "nil"), expected Bava Metzia 55")

    do {
        _ = try Zmanim(
            location: Times.seaLevelLocation(latitude: 91, longitude: 0), date: date, config: Times.defaultConfig(),
            localDay: fixedDay(-4, date))
        check(false, "latitude 91 did not throw")
    } catch ZmanimError.invalidLatitude(_) {
    }

    // Kiddush Levana is the one time whose day depends on the offset: with UTC-4 it falls on
    // 3 December 2024, and with UTC+4 it does not.
    let december = GregorianDate(year: 2024, month: 12, day: 3)
    let kiddushLevana = try Zmanim(
        location: lakewood, date: december, config: Times.defaultConfig(), localDay: fixedDay(-4, december))
    let ms = try kiddushLevana.get(zman: .tchilasZmanKidushLevana3Days)
    check(ms == 1733279333504, "UTC-4: kiddush levana is \(ms), expected 1733279333504")
    do {
        _ = try Zmanim(
            location: lakewood, date: december, config: Times.defaultConfig(), localDay: fixedDay(4, december)
        ).get(zman: .tchilasZmanKidushLevana3Days)
        check(false, "UTC+4 gave a kiddush levana time")
    } catch ZmanimError.notApplicable(_) {
    }
    do {
        _ = try Zmanim(location: lakewood, date: december, config: Times.defaultConfig(), localDay: nil)
            .get(zman: .tchilasZmanKidushLevana3Days)
        check(false, "no local day gave a kiddush levana time")
    } catch ZmanimError.timeZoneRequired(_) {
    }
    do {
        _ = try Zmanim(
            location: lakewood, date: date, config: Times.defaultConfig(),
            localDay: LocalDay(startMs: 1, startOffsetSeconds: 0, endMs: 0, endOffsetSeconds: 0))
        check(false, "a local day that ends before it starts did not throw")
    } catch ZmanimError.invalidLocalDay(_) {
    }
} catch {
    check(false, "unexpected error: \(error)")
}

print("ok")
