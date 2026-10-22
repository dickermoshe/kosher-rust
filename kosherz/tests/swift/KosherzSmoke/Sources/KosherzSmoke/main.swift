// Calls the generated Swift bindings. Run from this directory after `cargo build -p kosherz`
// (see .github/workflows/bindings.yaml):
//
//     LD_LIBRARY_PATH=../../../../target/debug swift run -Xlinker -L$PWD/../../../../target/debug
import Foundation
import Kosherz

/// Eastern Daylight Time, UTC-4, at every instant.
final class FixedOffset: TimeZoneOffset {
    func offsetSeconds(unixSeconds: Int64) throws -> Int32 {
        -4 * 3600
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

    let day = try Zmanim(location: lakewood, date: date, config: Times.defaultConfig(), timeZone: FixedOffset())
    let seconds = try day.get(zman: .sunrise) / 1000
    check(seconds == 1713866795, "sunrise is \(seconds), expected 1713866795 (06:06:35 EDT)")

    let hebrew = try HebrewDate.fromGregorian(date: date)
    let daf = Kosherz.Calendar.Limudim.dafYomiBavli(date: hebrew)
    check(daf?.display == "Bava Metzia 55", "daf yomi is \(daf?.display ?? "nil"), expected Bava Metzia 55")

    do {
        _ = try Zmanim(location: Times.seaLevelLocation(latitude: 91, longitude: 0), date: date, config: Times.defaultConfig(), timeZone: FixedOffset())
        check(false, "latitude 91 did not throw")
    } catch ZmanimError.invalidLatitude(_) {
    }
} catch {
    check(false, "unexpected error: \(error)")
}

print("ok")
