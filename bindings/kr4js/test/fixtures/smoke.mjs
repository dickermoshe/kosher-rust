// Imports the installed package the way an app does and checks one answer from the calendar, the
// learning schedules and the zmanim. CI runs it in Node, Bun and Deno from a project that installed
// the packed package.
import { HebrewDate, Zmanim, dafYomiBavli } from "@dickermoshe/kr4js";

const date = { year: 2024, month: 4, day: 23 };
const zmanim = new Zmanim({ location: { latitude: 40.09, longitude: -74.22 }, timeZone: "America/New_York", date });
const actual = [HebrewDate.from(date).toString(), String(dafYomiBavli(date)), zmanim.get("sofZmanShmaGra")?.toISOString().slice(0, 16)];
const expected = ["15 Nisan 5784", "Bava Metzia 55", "2024-04-23T13:31"];
if (JSON.stringify(actual) !== JSON.stringify(expected)) {
  throw new Error(`expected ${JSON.stringify(expected)}, got ${JSON.stringify(actual)}`);
}
console.log("kr4js works:", actual.join(", "));
