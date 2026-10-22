// Calls the generated Kotlin bindings on a desktop JVM. Run from this directory after
// `cargo build -p kosherz` (see .github/workflows/bindings.yaml):
//
//     cmake -S ../../generated/kotlin/src/main/cpp -B build/jni -DKOSHERZ_LIBRARY_DIR=$PWD/../../../target/debug
//     cmake --build build/jni
//     gradle run -PlibraryPath=$PWD/../../../target/debug:$PWD/build/jni
import kosherz.Calendar
import kosherz.GregorianDate
import kosherz.HebrewDate
import kosherz.TimeZoneOffset
import kosherz.Times
import kosherz.Zman
import kosherz.Zmanim
import kosherz.ZmanimException
import kotlin.system.exitProcess

/** Eastern Daylight Time, UTC-4, at every instant. */
object FixedOffset : TimeZoneOffset {
    override fun offsetSeconds(unixSeconds: Long): Int = -4 * 3600
}

fun verify(condition: Boolean, message: String) {
    if (condition) return
    System.err.println("FAIL: $message")
    exitProcess(1)
}

fun main() {
    val lakewood = Times.seaLevelLocation(40.09, -74.22)
    val date = GregorianDate(2024, 4, 23)

    Zmanim(lakewood, date, Times.defaultConfig(), FixedOffset).use { day ->
        val seconds = Math.floorDiv(day.get(Zman.Sunrise), 1000L)
        verify(seconds == 1713866795L, "sunrise is $seconds, expected 1713866795 (06:06:35 EDT)")
    }

    HebrewDate.fromGregorian(date).use { hebrew ->
        val daf = Calendar.Limudim.dafYomiBavli(hebrew)
        verify(daf?.display == "Bava Metzia 55", "daf yomi is ${daf?.display}, expected Bava Metzia 55")
    }

    try {
        Zmanim(Times.seaLevelLocation(91.0, 0.0), date, Times.defaultConfig(), FixedOffset).close()
        verify(false, "latitude 91 did not throw")
    } catch (expected: ZmanimException.InvalidLatitude) {
    }

    println("ok")
}
