# frozen_string_literal: true

# Calls the generated Ruby bindings. Build the library with `cargo build -p kosherz` and install
# the ffi gem, then from kosherz:
#
#     KOSHERZ_LIBRARY=$PWD/../target/debug/libkosherz.so ruby -I generated/ruby/lib tests/ruby/smoke.rb
require 'kosherz'

# `date` at UTC plus `hours` all year: its two local midnights and the offsets at them. Ruby has no
# time zone database of its own, so the test uses fixed offsets.
def fixed_day(hours, date)
  start = (Time.utc(date.year, date.month, date.day).to_i - hours * 3600) * 1000
  Kosherz::LocalDay.new(start_ms: start, start_offset_seconds: hours * 3600,
                        end_ms: start + 86_400_000, end_offset_seconds: hours * 3600)
end

# Fails unless the block raises `error`.
def raises(error)
  yield
rescue error
  nil
else
  raise "no #{error}"
end

$failures = 0

# Runs the block, which raises a message on failure, and reports the result.
def check(name)
  yield
  puts "ok   #{name}"
rescue StandardError => e
  puts "FAIL #{name}: #{e.class}: #{e.message}"
  $failures += 1
end

date = Kosherz::GregorianDate.new(year: 2024, month: 4, day: 23)

check('sunrise in Lakewood on 2024-04-23 is 06:06:35 EDT') do
  day = Kosherz::Zmanim.new(Kosherz.sea_level_location(40.09, -74.22), date, Kosherz.default_config, fixed_day(-4, date))
  begin
    seconds = day.get(Kosherz::Zman::SUNRISE).div(1000)
    raise "got #{seconds}, expected 1713866795" unless seconds == 1_713_866_795
  ensure
    day.close
  end
end

check('daf yomi on 2024-04-23 is Bava Metzia 55') do
  hebrew = Kosherz::HebrewDate.from_gregorian(date)
  begin
    display = Kosherz.daf_yomi_bavli(hebrew)&.display
    raise "got #{display.inspect}" unless display == 'Bava Metzia 55'
  ensure
    hebrew.close
  end
end

check('latitude 91 raises ZmanimError::InvalidLatitude') do
  begin
    Kosherz::Zmanim.new(Kosherz.sea_level_location(91, 0), date, Kosherz.default_config, fixed_day(-4, date)).close
  rescue Kosherz::ZmanimError::InvalidLatitude
    next
  end
  raise 'no exception'
end

# Kiddush Levana is the one time whose day depends on the offset: with UTC-4 it falls on
# 3 December 2024, and with UTC+4 it does not.
check('kiddush levana in Lakewood on 2024-12-03 needs the right offset') do
  lakewood = Kosherz.sea_level_location(40.09, -74.22)
  december = Kosherz::GregorianDate.new(year: 2024, month: 12, day: 3)
  zman = Kosherz::Zman::TCHILAS_ZMAN_KIDUSH_LEVANA3_DAYS
  day = Kosherz::Zmanim.new(lakewood, december, Kosherz.default_config, fixed_day(-4, december))
  wrong_sign = Kosherz::Zmanim.new(lakewood, december, Kosherz.default_config, fixed_day(4, december))
  no_zone = Kosherz::Zmanim.new(lakewood, december, Kosherz.default_config, nil)
  begin
    ms = day.get(zman)
    raise "got #{ms}, expected 1733279333504" unless ms == 1_733_279_333_504
    raises(Kosherz::ZmanimError::NotApplicable) { wrong_sign.get(zman) }
    raises(Kosherz::ZmanimError::TimeZoneRequired) { no_zone.get(zman) }
  ensure
    [day, wrong_sign, no_zone].each(&:close)
  end
end

check('a local day that ends before it starts raises ZmanimError::InvalidLocalDay') do
  backwards = Kosherz::LocalDay.new(start_ms: 1, start_offset_seconds: 0, end_ms: 0, end_offset_seconds: 0)
  raises(Kosherz::ZmanimError::InvalidLocalDay) do
    Kosherz::Zmanim.new(Kosherz.sea_level_location(40.09, -74.22), date, Kosherz.default_config, backwards).close
  end
end

if $failures.positive?
  puts "#{$failures} check(s) failed"
  exit 1
end
