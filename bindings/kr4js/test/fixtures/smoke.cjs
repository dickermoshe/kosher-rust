// Loads the installed package with `require`, as a CommonJS app does.
const { HebrewDate } = require("@dickermoshe/kr4js");

const hebrew = HebrewDate.from({ year: 2024, month: 4, day: 23 }).toString();
if (hebrew !== "15 Nisan 5784") throw new Error(`expected 15 Nisan 5784, got ${hebrew}`);
console.log("kr4js works with require:", hebrew);
