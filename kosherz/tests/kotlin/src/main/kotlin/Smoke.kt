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

/** UTC plus [hours], at every instant. */
class FixedOffset(private val hours: Int) : TimeZoneOffset {
    override fun offsetSeconds(unixSeconds: Long): Int = hours * 3600

    override fun localToUnixSeconds(year: Int, month: Int, day: Int, hour: Int, minute: Int, second: Int): Long =
        java.time.LocalDateTime.of(year, month, day, hour, minute, second)
            .toEpochSecond(java.time.ZoneOffset.ofHours(hours))
}

/** Eastern Daylight Time, UTC-4, at every instant. */
val edt = FixedOffset(-4)

/** A time zone whose callbacks fail. */
object BrokenOffset : TimeZoneOffset {
    override fun offsetSeconds(unixSeconds: Long): Int = throw IllegalStateException("no time zone data")

    override fun localToUnixSeconds(year: Int, month: Int, day: Int, hour: Int, minute: Int, second: Int): Long =
        throw IllegalStateException("no time zone data")
}

/** Fails unless [block] throws a [T]. */
inline fun <reified T : Throwable> verifyThrows(name: String, block: () -> Unit) {
    try {
        block()
    } catch (expected: Throwable) {
        if (expected is T) return
        throw expected
    }
    verify(false, "$name did not throw ${T::class.simpleName}")
}

fun verify(condition: Boolean, message: String) {
    if (condition) return
    System.err.println("FAIL: $message")
    exitProcess(1)
}

fun main() {
    val lakewood = Times.seaLevelLocation(40.09, -74.22)
    val date = GregorianDate(2024, 4, 23)

    Zmanim(lakewood, date, Times.defaultConfig(), edt).use { day ->
        val seconds = Math.floorDiv(day.get(Zman.Sunrise), 1000L)
        verify(seconds == 1713866795L, "sunrise is $seconds, expected 1713866795 (06:06:35 EDT)")
    }

    HebrewDate.fromGregorian(date).use { hebrew ->
        val daf = Calendar.Limudim.dafYomiBavli(hebrew)
        verify(daf?.display == "Bava Metzia 55", "daf yomi is ${daf?.display}, expected Bava Metzia 55")
    }

    try {
        Zmanim(Times.seaLevelLocation(91.0, 0.0), date, Times.defaultConfig(), edt).close()
        verify(false, "latitude 91 did not throw")
    } catch (expected: ZmanimException.InvalidLatitude) {
    }

    // Kiddush Levana is the one time whose day depends on the offset: with UTC-4 it falls on
    // 3 December 2024, and with UTC+4 it does not.
    val december = GregorianDate(2024, 12, 3)
    Zmanim(lakewood, december, Times.defaultConfig(), edt).use { day ->
        val ms = day.get(Zman.TchilasZmanKidushLevana3Days)
        verify(ms == 1733279333504L, "UTC-4: kiddush levana is $ms, expected 1733279333504")
    }
    verifyThrows<ZmanimException.NotApplicable>("UTC+4") {
        Zmanim(lakewood, december, Times.defaultConfig(), FixedOffset(4)).use { it.get(Zman.TchilasZmanKidushLevana3Days) }
    }
    verifyThrows<ZmanimException.TimeZoneRequired>("no time zone") {
        Zmanim(lakewood, december, Times.defaultConfig(), null).use { it.get(Zman.TchilasZmanKidushLevana3Days) }
    }
    verifyThrows<ZmanimException.TimeZoneCallbackFailed>("a throwing callback") {
        Zmanim(lakewood, date, Times.defaultConfig(), BrokenOffset).use { it.get(Zman.Sunrise) }
    }

    println("ok")
}
