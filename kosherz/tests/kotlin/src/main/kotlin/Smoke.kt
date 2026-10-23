// Calls the generated Kotlin bindings on a desktop JVM. Run from this directory after
// `cargo build -p kosherz` (see .github/workflows/bindings.yaml):
//
//     cmake -S ../../generated/kotlin/src/main/cpp -B build/jni -DKOSHERZ_LIBRARY_DIR=$PWD/../../../target/debug
//     cmake --build build/jni
//     gradle run -PlibraryPath=$PWD/../../../target/debug:$PWD/build/jni
import java.time.LocalDate
import java.time.ZoneId
import java.time.ZoneOffset
import io.github.dickermoshe.kosherz.Calendar
import io.github.dickermoshe.kosherz.GregorianDate
import io.github.dickermoshe.kosherz.HebrewDate
import io.github.dickermoshe.kosherz.LocalDay
import io.github.dickermoshe.kosherz.Times
import io.github.dickermoshe.kosherz.Zman
import io.github.dickermoshe.kosherz.Zmanim
import io.github.dickermoshe.kosherz.ZmanimException
import kotlin.system.exitProcess

/** [date] in [zone]: its two local midnights and the offsets at them. */
fun localDay(zone: ZoneId, date: GregorianDate): LocalDay {
    val day = LocalDate.of(date.year, date.month, date.day)
    val start = day.atStartOfDay(zone)
    val end = day.plusDays(1).atStartOfDay(zone)
    return LocalDay(
        start.toInstant().toEpochMilli(),
        start.offset.totalSeconds,
        end.toInstant().toEpochMilli(),
        end.offset.totalSeconds,
    )
}

/** [date] at UTC plus [hours] all year. */
fun fixedDay(hours: Int, date: GregorianDate): LocalDay = localDay(ZoneOffset.ofHours(hours), date)

fun verify(condition: Boolean, message: String) {
    if (condition) return
    System.err.println("FAIL: $message")
    exitProcess(1)
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

fun main() {
    val lakewood = Times.seaLevelLocation(40.09, -74.22)
    val date = GregorianDate(2024, 4, 23)

    Zmanim(lakewood, date, Times.defaultConfig(), localDay(ZoneId.of("America/New_York"), date)).use { day ->
        val seconds = Math.floorDiv(day.get(Zman.Sunrise), 1000L)
        verify(seconds == 1713866795L, "sunrise is $seconds, expected 1713866795 (06:06:35 EDT)")
    }

    HebrewDate.fromGregorian(date).use { hebrew ->
        val daf = Calendar.Limudim.dafYomiBavli(hebrew)
        verify(daf?.display == "Bava Metzia 55", "daf yomi is ${daf?.display}, expected Bava Metzia 55")
    }

    verifyThrows<ZmanimException.InvalidLatitude>("latitude 91") {
        Zmanim(Times.seaLevelLocation(91.0, 0.0), date, Times.defaultConfig(), fixedDay(-4, date)).close()
    }

    // Kiddush Levana is the one time whose day depends on the offset: with UTC-4 it falls on
    // 3 December 2024, and with UTC+4 it does not.
    val december = GregorianDate(2024, 12, 3)
    Zmanim(lakewood, december, Times.defaultConfig(), fixedDay(-4, december)).use { day ->
        val ms = day.get(Zman.TchilasZmanKidushLevana3Days)
        verify(ms == 1733279333504L, "UTC-4: kiddush levana is $ms, expected 1733279333504")
    }
    verifyThrows<ZmanimException.NotApplicable>("UTC+4") {
        Zmanim(lakewood, december, Times.defaultConfig(), fixedDay(4, december)).use { it.get(Zman.TchilasZmanKidushLevana3Days) }
    }
    verifyThrows<ZmanimException.TimeZoneRequired>("no local day") {
        Zmanim(lakewood, december, Times.defaultConfig(), null).use { it.get(Zman.TchilasZmanKidushLevana3Days) }
    }
    verifyThrows<ZmanimException.InvalidLocalDay>("a local day that ends before it starts") {
        Zmanim(lakewood, date, Times.defaultConfig(), LocalDay(1, 0, 0, 0)).close()
    }

    println("ok")
}
