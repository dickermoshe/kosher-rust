// Calls the generated Swift bindings. Run from this directory after `cargo build -p kosherz`
// (see .github/workflows/bindings.yaml):
//
//     LD_LIBRARY_PATH=../../../../target/debug swift run -Xlinker -L$PWD/../../../../target/debug
import Foundation
import Kosherz

/// UTC plus a number of hours, at every instant.
final class FixedOffset: TimeZoneOffset {
    let hours: Int32

    init(hours: Int32) {
        self.hours = hours
    }

    func offsetSeconds(unixSeconds: Int64) throws -> Int32 {
        hours * 3600
    }

    func localToUnixSeconds(
        year: Int32, month: Int32, day: Int32, hour: Int32, minute: Int32, second: Int32
    ) throws -> Int64 {
        var calendar = Calendar(identifier: .gregorian)
        calendar.timeZone = TimeZone(secondsFromGMT: Int(hours) * 3600)!
        let components = DateComponents(
            year: Int(year), month: Int(month), day: Int(day),
            hour: Int(hour), minute: Int(minute), second: Int(second))
        return Int64(calendar.date(from: components)!.timeIntervalSince1970)
    }
}

/// A time zone whose callbacks fail.
final class BrokenOffset: TimeZoneOffset {
    struct NoData: Error {}

    func offsetSeconds(unixSeconds: Int64) throws -> Int32 {
        throw NoData()
    }

    func localToUnixSeconds(
        year: Int32, month: Int32, day: Int32, hour: Int32, minute: Int32, second: Int32
    ) throws -> Int64 {
        throw NoData()
    }
}

func check(_ condition: Bool, _ message: String) {
    if condition { return }
    FileHandle.standardError.write(Data("FAIL: \(message)\n".utf8))
    exit(1)
}

do {
    let lakewood = Times.seaLevelLocation(latitude: 40.09, longitude: -74.22)
    let date = GregorianDate(year: 2024, month: 4, day: 23)

    let day = try Zmanim(location: lakewood, date: date, config: Times.defaultConfig(), timeZone: FixedOffset(hours: -4))
    let seconds = try day.get(zman: .sunrise) / 1000
    check(seconds == 1713866795, "sunrise is \(seconds), expected 1713866795 (06:06:35 EDT)")

    let hebrew = try HebrewDate.fromGregorian(date: date)
    let daf = Kosherz.Calendar.Limudim.dafYomiBavli(date: hebrew)
    check(daf?.display == "Bava Metzia 55", "daf yomi is \(daf?.display ?? "nil"), expected Bava Metzia 55")

    do {
        _ = try Zmanim(location: Times.seaLevelLocation(latitude: 91, longitude: 0), date: date, config: Times.defaultConfig(), timeZone: FixedOffset(hours: -4))
        check(false, "latitude 91 did not throw")
    } catch ZmanimError.invalidLatitude(_) {
    }

    // Kiddush Levana is the one time whose day depends on the offset: with UTC-4 it falls on
    // 3 December 2024, and with UTC+4 it does not.
    let december = GregorianDate(year: 2024, month: 12, day: 3)
    let kiddushLevana = try Zmanim(
        location: lakewood, date: december, config: Times.defaultConfig(), timeZone: FixedOffset(hours: -4))
    let ms = try kiddushLevana.get(zman: .tchilasZmanKidushLevana3Days)
    check(ms == 1733279333504, "UTC-4: kiddush levana is \(ms), expected 1733279333504")
    do {
        _ = try Zmanim(
            location: lakewood, date: december, config: Times.defaultConfig(), timeZone: FixedOffset(hours: 4)
        ).get(zman: .tchilasZmanKidushLevana3Days)
        check(false, "UTC+4 gave a kiddush levana time")
    } catch ZmanimError.notApplicable(_) {
    }
    do {
        _ = try Zmanim(location: lakewood, date: december, config: Times.defaultConfig(), timeZone: nil)
            .get(zman: .tchilasZmanKidushLevana3Days)
        check(false, "no time zone gave a kiddush levana time")
    } catch ZmanimError.timeZoneRequired(_) {
    }
    do {
        _ = try Zmanim(location: lakewood, date: date, config: Times.defaultConfig(), timeZone: BrokenOffset())
            .get(zman: .sunrise)
        check(false, "a throwing callback gave a sunrise")
    } catch ZmanimError.timeZoneCallbackFailed(_) {
    }
} catch {
    check(false, "unexpected error: \(error)")
}

print("ok")
