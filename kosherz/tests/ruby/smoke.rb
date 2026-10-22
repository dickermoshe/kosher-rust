# frozen_string_literal: true

# Calls the generated Ruby bindings. Build the library with `cargo build -p kosherz` and install
# the ffi gem, then from kosherz:
#
#     KOSHERZ_LIBRARY=$PWD/../target/debug/libkosherz.so ruby -I generated/ruby/lib tests/ruby/smoke.rb
require 'kosherz'

# Eastern daylight time, every day of the year.
class Edt
  include Kosherz::TimeZoneOffset

  def offset_seconds(_unix_seconds)
    -4 * 3600
  end

  def local_to_unix_seconds(year, month, day, hour, minute, second)
    Time.new(year, month, day, hour, minute, second, '-04:00').to_i
  end
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
  day = Kosherz::Zmanim.new(Kosherz.sea_level_location(40.09, -74.22), date, Kosherz.default_config, Edt.new)
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
    Kosherz::Zmanim.new(Kosherz.sea_level_location(91, 0), date, Kosherz.default_config, Edt.new).close
  rescue Kosherz::ZmanimError::InvalidLatitude
    next
  end
  raise 'no exception'
end

if $failures.positive?
  puts "#{$failures} check(s) failed"
  exit 1
end
