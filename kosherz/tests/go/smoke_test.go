// Calls the generated Go bindings: sunrise in Lakewood through a time zone callback, Kiddush
// Levana, which needs the right offset, a failing callback, the daf yomi, and the typed error for
// an invalid latitude. Run after `cargo build -p kosherz`, from
// this directory:
//
//	export CGO_LDFLAGS="-L$REPO/target/debug -Wl,-rpath,$REPO/target/debug"
//	go test -v ./...
package smoke

import (
	"errors"
	"testing"
	"time"

	"kosherz"
)

// fixed is UTC plus a number of hours, every day of the year.
type fixed int32

func (f fixed) OffsetSeconds(int64) int32 { return int32(f) * 3600 }

func (f fixed) LocalToUnixSeconds(year, month, day, hour, minute, second int32) int64 {
	local := time.Date(int(year), time.Month(month), int(day), int(hour), int(minute), int(second), 0, time.UTC)
	return local.Unix() - int64(f)*3600
}

// edt is a fixed UTC-4 time zone.
func edt() fixed { return fixed(-4) }

// broken is a time zone whose callbacks fail.
type broken struct{}

func (broken) OffsetSeconds(int64) int32 { panic("no time zone data") }

func (broken) LocalToUnixSeconds(int32, int32, int32, int32, int32, int32) int64 {
	panic("no time zone data")
}

var april23 = kosherz.GregorianDate{Year: 2024, Month: 4, Day: 23}
var december3 = kosherz.GregorianDate{Year: 2024, Month: 12, Day: 3}

// kiddushLevana is Tchilas Zman Kiddush Levana (3 days) in Lakewood on 2024-12-03.
func kiddushLevana(t *testing.T, zone kosherz.TimeZoneOffset) (int64, error) {
	t.Helper()
	day, err := kosherz.NewZmanim(kosherz.SeaLevelLocation(40.09, -74.22), december3, kosherz.DefaultConfig(), zone)
	if err != nil {
		t.Fatalf("NewZmanim: %v", err)
	}
	defer day.Close()
	return day.Get(kosherz.ZmanTchilasZmanKidushLevana3Days)
}

// Kiddush Levana is the one time whose day depends on the offset: with UTC-4 it falls on
// 3 December 2024, and with UTC+4 it does not.
func TestKiddushLevanaNeedsTheRightOffset(t *testing.T) {
	if ms, err := kiddushLevana(t, edt()); err != nil || ms != 1733279333504 {
		t.Fatalf("UTC-4: %d, %v; want 1733279333504", ms, err)
	}
	var notApplicable *kosherz.NotApplicableError
	if _, err := kiddushLevana(t, fixed(4)); !errors.As(err, &notApplicable) {
		t.Fatalf("UTC+4: error is %T (%v), want *kosherz.NotApplicableError", err, err)
	}
	var required *kosherz.TimeZoneRequiredError
	if _, err := kiddushLevana(t, nil); !errors.As(err, &required) {
		t.Fatalf("no time zone: error is %T (%v), want *kosherz.TimeZoneRequiredError", err, err)
	}
}

func TestFailingCallback(t *testing.T) {
	day, err := kosherz.NewZmanim(kosherz.SeaLevelLocation(40.09, -74.22), april23, kosherz.DefaultConfig(), broken{})
	if err != nil {
		t.Fatalf("NewZmanim: %v", err)
	}
	defer day.Close()
	var failed *kosherz.TimeZoneCallbackFailedError
	if _, err := day.Get(kosherz.ZmanSunrise); !errors.As(err, &failed) {
		t.Fatalf("error is %T (%v), want *kosherz.TimeZoneCallbackFailedError", err, err)
	}
}

func TestSunrise(t *testing.T) {
	day, err := kosherz.NewZmanim(kosherz.SeaLevelLocation(40.09, -74.22), april23, kosherz.DefaultConfig(), edt())
	if err != nil {
		t.Fatalf("NewZmanim: %v", err)
	}
	defer day.Close()
	ms, err := day.Get(kosherz.ZmanSunrise)
	if err != nil {
		t.Fatalf("Get(Sunrise): %v", err)
	}
	seconds := ms / 1000
	if ms < 0 && ms%1000 != 0 {
		seconds--
	}
	if seconds != 1713866795 {
		t.Fatalf("sunrise is %d, want 1713866795", seconds)
	}
}

func TestDafYomi(t *testing.T) {
	date, err := kosherz.FromGregorianHebrewDate(april23)
	if err != nil {
		t.Fatalf("FromGregorianHebrewDate: %v", err)
	}
	defer date.Close()
	daf := kosherz.DafYomiBavli(date)
	if daf == nil {
		t.Fatal("no daf yomi")
	}
	if daf.Display != "Bava Metzia 55" {
		t.Fatalf("daf yomi is %q, want \"Bava Metzia 55\"", daf.Display)
	}
}

func TestInvalidLatitude(t *testing.T) {
	day, err := kosherz.NewZmanim(kosherz.SeaLevelLocation(91, 0), april23, kosherz.DefaultConfig(), edt())
	if err == nil {
		day.Close()
		t.Fatal("NewZmanim accepted latitude 91")
	}
	var invalid *kosherz.InvalidLatitudeError
	if !errors.As(err, &invalid) {
		t.Fatalf("error is %T (%v), want *kosherz.InvalidLatitudeError", err, err)
	}
}
