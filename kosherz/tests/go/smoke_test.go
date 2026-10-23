// Calls the generated Go bindings: sunrise in Lakewood on a local day from Go's time zone data,
// Kiddush Levana, which needs the right offset, an impossible local day, the daf yomi, and the
// typed error for an invalid latitude. Run after `cargo build -p kosherz`, from this directory:
//
//	export CGO_LDFLAGS="-L$REPO/target/debug -Wl,-rpath,$REPO/target/debug"
//	go test -v ./...
package smoke

import (
	"errors"
	"testing"
	"time"

	kosherz "github.com/dickermoshe/kosher-rust/kosherz/generated/go"
)

// localDay is date in zone: its two local midnights and the offsets at them. time.Date does not
// say which instant it picks when the clocks skip midnight, so a zone that does needs more care.
func localDay(zone *time.Location, date kosherz.GregorianDate) *kosherz.LocalDay {
	start := time.Date(int(date.Year), time.Month(date.Month), int(date.Day), 0, 0, 0, 0, zone)
	end := time.Date(int(date.Year), time.Month(date.Month), int(date.Day)+1, 0, 0, 0, 0, zone)
	_, startOffset := start.Zone()
	_, endOffset := end.Zone()
	return &kosherz.LocalDay{
		StartMs:            start.UnixMilli(),
		StartOffsetSeconds: int32(startOffset),
		EndMs:              end.UnixMilli(),
		EndOffsetSeconds:   int32(endOffset),
	}
}

// fixedDay is date at UTC plus hours all year.
func fixedDay(hours int, date kosherz.GregorianDate) *kosherz.LocalDay {
	return localDay(time.FixedZone("fixed", hours*3600), date)
}

var april23 = kosherz.GregorianDate{Year: 2024, Month: 4, Day: 23}
var december3 = kosherz.GregorianDate{Year: 2024, Month: 12, Day: 3}

func TestSunrise(t *testing.T) {
	newYork, err := time.LoadLocation("America/New_York")
	if err != nil {
		t.Fatalf("LoadLocation: %v", err)
	}
	day, err := kosherz.NewZmanim(kosherz.SeaLevelLocation(40.09, -74.22), april23, kosherz.DefaultConfig(), localDay(newYork, april23))
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

// kiddushLevana is Tchilas Zman Kiddush Levana (3 days) in Lakewood on 2024-12-03.
func kiddushLevana(t *testing.T, localDay *kosherz.LocalDay) (int64, error) {
	t.Helper()
	day, err := kosherz.NewZmanim(kosherz.SeaLevelLocation(40.09, -74.22), december3, kosherz.DefaultConfig(), localDay)
	if err != nil {
		t.Fatalf("NewZmanim: %v", err)
	}
	defer day.Close()
	return day.Get(kosherz.ZmanTchilasZmanKidushLevana3Days)
}

// Kiddush Levana is the one time whose day depends on the offset: with UTC-4 it falls on
// 3 December 2024, and with UTC+4 it does not.
func TestKiddushLevanaNeedsTheRightOffset(t *testing.T) {
	if ms, err := kiddushLevana(t, fixedDay(-4, december3)); err != nil || ms != 1733279333504 {
		t.Fatalf("UTC-4: %d, %v; want 1733279333504", ms, err)
	}
	var notApplicable *kosherz.NotApplicableError
	if _, err := kiddushLevana(t, fixedDay(4, december3)); !errors.As(err, &notApplicable) {
		t.Fatalf("UTC+4: error is %T (%v), want *kosherz.NotApplicableError", err, err)
	}
	var required *kosherz.TimeZoneRequiredError
	if _, err := kiddushLevana(t, nil); !errors.As(err, &required) {
		t.Fatalf("no local day: error is %T (%v), want *kosherz.TimeZoneRequiredError", err, err)
	}
}

func TestImpossibleLocalDay(t *testing.T) {
	backwards := &kosherz.LocalDay{StartMs: 1, EndMs: 0}
	day, err := kosherz.NewZmanim(kosherz.SeaLevelLocation(40.09, -74.22), april23, kosherz.DefaultConfig(), backwards)
	if err == nil {
		day.Close()
		t.Fatal("NewZmanim accepted a local day that ends before it starts")
	}
	var invalid *kosherz.InvalidLocalDayError
	if !errors.As(err, &invalid) {
		t.Fatalf("error is %T (%v), want *kosherz.InvalidLocalDayError", err, err)
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
	day, err := kosherz.NewZmanim(kosherz.SeaLevelLocation(91, 0), april23, kosherz.DefaultConfig(), fixedDay(-4, april23))
	if err == nil {
		day.Close()
		t.Fatal("NewZmanim accepted latitude 91")
	}
	var invalid *kosherz.InvalidLatitudeError
	if !errors.As(err, &invalid) {
		t.Fatalf("error is %T (%v), want *kosherz.InvalidLatitudeError", err, err)
	}
}
