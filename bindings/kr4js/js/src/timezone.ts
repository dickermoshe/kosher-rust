// Time zones from the runtime's own data, through `Intl.DateTimeFormat`, which every browser,
// Node, Bun, Deno and Cloudflare Workers has. The package bundles no time zone database.
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
 * The UTC offset of the time zone `name` at any instant, in minutes east of UTC, from the runtime's
 * own time zone data.
 *
 * @throws {RangeError} When the runtime doesn't know the name.
 */
export function offsetAt(name: string): (epochMilliseconds: number) => number {
  const format = formatter(name);
  return (epochMilliseconds) => {
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
  };
}
