// Loads the installed package with `require`, as a CommonJS app does.
const { HebrewDate } = require("kosher-rust");

const hebrew = HebrewDate.from({ year: 2024, month: 4, day: 23 }).toString();
if (hebrew !== "15 Nisan 5784") throw new Error(`expected 15 Nisan 5784, got ${hebrew}`);
console.log("kosher-rust works with require:", hebrew);
