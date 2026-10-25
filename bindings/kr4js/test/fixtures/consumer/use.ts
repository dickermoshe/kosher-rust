// Code a TypeScript project writes against the installed package. test/package.test.ts compiles it
// with each module resolution, without skipLibCheck.
import { HebrewDate, Zmanim, type ZmanTime } from "kosher-rust";
import { init } from "kosher-rust/slim";
import wasm from "kosher-rust/kosher_rust.wasm";

const hebrew: string = HebrewDate.from({ year: 2024, month: 4, day: 23 }).toString();
const zmanim = new Zmanim({ location: { latitude: 40.09, longitude: -74.22 }, date: { year: 2024, month: 4, day: 23 } });
const sunrise: Date | null = zmanim.get("sunrise");
const all: ZmanTime[] = zmanim.all();
const loaded: Promise<void> = init(wasm);

export { all, hebrew, loaded, sunrise };
