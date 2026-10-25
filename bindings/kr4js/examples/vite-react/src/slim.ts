// The slim entry: the WebAssembly is a separate file that Vite copies into the build, fetched once
// by `init`.
import { HebrewDate, dafYomiBavli, init } from "kosher-rust/slim";

async function main() {
  await init();
  const [year, month, day] = (new URLSearchParams(window.location.search).get("date") ?? "2024-04-23").split("-").map(Number);
  const date = { year: year!, month: month!, day: day! };
  document.getElementById("hebrew-date")!.textContent = HebrewDate.from(date).toString();
  document.getElementById("daf-yomi")!.textContent = String(dafYomiBavli(date));
}

void main();
