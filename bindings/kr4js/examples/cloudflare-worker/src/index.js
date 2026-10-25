// Answers with the Hebrew date, Daf Yomi and zmanim for a place and day.
//
//   GET /?lat=31.778&lon=35.235&tz=Asia/Jerusalem&date=2024-04-23&region=israel
//
// Without `tz`, the time zone Cloudflare reports for the visitor is used, and without `lat` and
// `lon`, their approximate location.
//
// Workers don't compile WebAssembly while running, so this uses kosher-rust/slim and passes `init`
// the WebAssembly module, which Wrangler compiles when it uploads the Worker.
import wasm from "kosher-rust/kosher_rust.wasm";
import { HebrewDate, Zmanim, dafYomiBavli, init, today } from "kosher-rust/slim";

/** Responds with `body` as JSON. */
function json(body, status = 200) {
  return new Response(JSON.stringify(body, null, 2), {
    status,
    headers: { "content-type": "application/json; charset=utf-8" },
  });
}

export default {
  async fetch(request) {
    await init(wasm);
    const url = new URL(request.url);
    const params = url.searchParams;
    const cf = request.cf ?? {};
    const latitude = Number(params.get("lat") ?? cf.latitude);
    const longitude = Number(params.get("lon") ?? cf.longitude);
    const timeZone = params.get("tz") ?? cf.timezone ?? "UTC";
    const region = params.get("region") === "israel" ? "israel" : "diaspora";

    try {
      const [year, month, day] = (params.get("date") ?? "").split("-").map(Number);
      const date = params.has("date") ? { year, month, day } : today(timeZone);
      const hebrew = HebrewDate.from(date);
      const zmanim = new Zmanim({ location: { latitude, longitude }, timeZone, date });
      return json({
        date: hebrew.toJSON(),
        hebrewDate: hebrew.toString(),
        specialDays: hebrew.specialDays(region).map((special) => special.name),
        parsha: hebrew.parsha(region)?.name ?? null,
        dafYomi: dafYomiBavli(hebrew)?.toString() ?? null,
        timeZone,
        zmanim: Object.fromEntries(zmanim.all().map(({ zman, time }) => [zman.id, time])),
      });
    } catch (error) {
      if (error instanceof RangeError || error instanceof TypeError || error.name === "ZmanimError") {
        return json({ error: error.message }, 400);
      }
      throw error;
    }
  },
};
