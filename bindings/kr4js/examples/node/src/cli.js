#!/usr/bin/env node
// @ts-check
// Prints the day's Hebrew date, learning and zmanim.
//
//   node src/cli.js --city jerusalem --date 2024-04-23
//   bun src/cli.js --json
import { parseArgs } from "node:util";

import { CITIES, format, report } from "./report.js";

const { values } = parseArgs({
  options: {
    city: { type: "string", default: "lakewood" },
    date: { type: "string" },
    json: { type: "boolean", default: false },
    help: { type: "boolean", short: "h", default: false },
  },
});

if (values.help) {
  console.log(`Usage: zmanim [--city ${Object.keys(CITIES).join("|")}] [--date YYYY-MM-DD] [--json]`);
  process.exit(0);
}

const city = CITIES[values.city];
if (!city) {
  console.error(`Unknown city "${values.city}". Try one of ${Object.keys(CITIES).join(", ")}.`);
  process.exit(1);
}

const [year, month, day] = (values.date ?? "").split("-").map(Number);
const data = report(city, values.date ? { year, month, day } : undefined);
console.log(values.json ? JSON.stringify(data, null, 2) : format(data));
