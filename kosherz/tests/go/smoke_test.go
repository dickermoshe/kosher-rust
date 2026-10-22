// Calls the generated Go bindings: sunrise in Lakewood through a time zone callback, the daf
// yomi, and the typed error for an invalid latitude. Run after `cargo build -p kosherz`, from
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

// edt is a fixed UTC-4 time zone.
type edt struct{}

func (edt) OffsetSeconds(int64) int32 { return -4 * 3600 }

func (edt) LocalToUnixSeconds(year, month, day, hour, minute, second int32) int64 {
	local := time.Date(int(year), time.Month(month), int(day), int(hour), int(minute), int(second), 0, time.UTC)
	return local.Unix() + 4*3600
}

var april23 = kosherz.GregorianDate{Year: 2024, Month: 4, Day: 23}

func TestSunrise(t *testing.T) {
	day, err := kosherz.NewZmanim(kosherz.SeaLevelLocation(40.09, -74.22), april23, kosherz.DefaultConfig(), edt{})
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
	day, err := kosherz.NewZmanim(kosherz.SeaLevelLocation(91, 0), april23, kosherz.DefaultConfig(), edt{})
	if err == nil {
		day.Close()
		t.Fatal("NewZmanim accepted latitude 91")
	}
	var invalid *kosherz.InvalidLatitudeError
	if !errors.As(err, &invalid) {
		t.Fatalf("error is %T (%v), want *kosherz.InvalidLatitudeError", err, err)
	}
}
