// Prints whether @dickermoshe/kr4js/slim throws when used before `init`. Runs in a fresh process, where
// nothing else has loaded the WebAssembly yet.
import { HebrewDate } from "../../dist/slim.js";

try {
  HebrewDate.from({ year: 2024, month: 4, day: 23 });
  console.log("no error");
} catch {
  console.log("threw");
}
