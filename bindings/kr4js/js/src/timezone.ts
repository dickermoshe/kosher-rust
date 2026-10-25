// Time zones from the runtime's own data, through `Intl.DateTimeFormat`, which every browser,
// Node, Bun, Deno and Cloudflare Workers has. The package bundles no time zone database.
import type { GregorianDate, TimeZone, TimeZoneInput } from "./wasm/kosher_rust.js";

const formatters = new Map<string, Intl.DateTimeFormat>();

function formatter(name: string): Intl.DateTimeFormat {
  let format = formatters.get(name);
  if (!format) {
    format = new Intl.DateTimeFormat("en-US", {
      timeZone: name,
      hourCycle: "h23",
      era: "short",
      year: "numeric",
      month: "numeric",
      day: "numeric",
      hour: "numeric",
      minute: "numeric",
      second: "numeric",
    });
    formatters.set(name, format);
  }
  return format;
}

/** Unix milliseconds of a date and time read as UTC, for any year including those before 100. */
function utc(year: number, month: number, day: number, hour: number, minute: number, second: number): number {
  const date = new Date(0);
  date.setUTCFullYear(year, month - 1, day);
  date.setUTCHours(hour, minute, second, 0);
  return date.getTime();
}

/**
 * The time zone `name` from the runtime's own time zone data, such as `"America/New_York"`,
 * `"Asia/Jerusalem"` or `"UTC"`. Passing the name itself as a `timeZone` does the same.
 *
 * @throws {RangeError} When the runtime doesn't know the name.
 */
export function timeZone(name: string): TimeZone {
  const format = formatter(name);
  const resolved = format.resolvedOptions().timeZone;
  return {
    name: resolved,
    offsetAt(epochMilliseconds: number): number {
      const seconds = Math.floor(epochMilliseconds / 1000) * 1000;
      const parts: Record<string, string> = {};
      for (const part of format.formatToParts(seconds)) parts[part.type] = part.value;
      const year = parts.era === "BC" || parts.era === "B" ? 1 - Number(parts.year) : Number(parts.year);
      const local = utc(
        year,
        Number(parts.month),
        Number(parts.day),
        Number(parts.hour) % 24,
        Number(parts.minute),
        Number(parts.second),
      );
      return (local - seconds) / 60_000;
    },
  };
}

/** A time zone that is always `minutes` east of UTC, such as `-300` for US Eastern Standard Time. */
export function fixedOffset(minutes: number): TimeZone {
  if (!Number.isFinite(minutes)) throw new RangeError("the offset must be a finite number of minutes");
  const sign = minutes < 0 ? "-" : "+";
  const total = Math.abs(minutes);
  const name = `${sign}${String(Math.floor(total / 60)).padStart(2, "0")}:${String(total % 60).padStart(2, "0")}`;
  return { name, offsetAt: () => minutes };
}

/** `input` as a {@link TimeZone}. */
function toTimeZone(input: TimeZoneInput): TimeZone {
  if (typeof input === "string") return timeZone(input);
  if (typeof input === "function") return { offsetAt: input };
  if (input && typeof input.offsetAt === "function") return input;
  throw new TypeError("a time zone is an IANA name, an object with offsetAt, or a function");
}

/** A wall-clock date and time in some time zone. */
export interface LocalTime {
  readonly year: number;
  /** 1 for January to 12 for December. */
  readonly month: number;
  readonly day: number;
  readonly hour: number;
  readonly minute: number;
  readonly second: number;
  readonly millisecond: number;
  /** The UTC offset at that moment, in minutes east of UTC. */
  readonly offset: number;
}

/**
 * `instant` on the clock of `zone`.
 *
 * ```ts
 * const shma = zmanim.get("sofZmanShmaGra")!;
 * const { hour, minute } = localTime(shma, "America/New_York"); // 9, 31
 * ```
 */
export function localTime(instant: Date | number, zone: TimeZoneInput): LocalTime {
  const epochMilliseconds = typeof instant === "number" ? instant : instant.getTime();
  if (!Number.isFinite(epochMilliseconds)) throw new RangeError("the instant is not a valid date");
  const offset = toTimeZone(zone).offsetAt(epochMilliseconds);
  const shifted = new Date(epochMilliseconds + Math.round(offset * 60_000));
  return {
    year: shifted.getUTCFullYear(),
    month: shifted.getUTCMonth() + 1,
    day: shifted.getUTCDate(),
    hour: shifted.getUTCHours(),
    minute: shifted.getUTCMinutes(),
    second: shifted.getUTCSeconds(),
    millisecond: shifted.getUTCMilliseconds(),
    offset,
  };
}

/** Today's date in `zone`, or in the runtime's local time zone without one. */
export function today(zone?: TimeZoneInput): GregorianDate {
  if (zone === undefined) {
    const now = new Date();
    return { year: now.getFullYear(), month: now.getMonth() + 1, day: now.getDate() };
  }
  const { year, month, day } = localTime(Date.now(), zone);
  return { year, month, day };
}
