// Bundles the Worker with Wrangler and runs it in workerd, the Cloudflare Workers runtime.
import assert from "node:assert/strict";
import { after, before, describe, test } from "node:test";
import { fileURLToPath } from "node:url";

import { unstable_startWorker } from "wrangler";

describe("the Worker", () => {
  let worker;

  before(async () => {
    worker = await unstable_startWorker({
      config: fileURLToPath(new URL("../wrangler.jsonc", import.meta.url)),
      dev: { server: { port: 0 }, inspector: false },
    });
  });

  after(async () => {
    await worker?.dispose();
  });

  const get = async (query) => {
    const response = await worker.fetch(`http://example.com/?${query}`);
    return { status: response.status, body: await response.json() };
  };

  test("answers with the day's calendar and zmanim", async () => {
    const { status, body } = await get("lat=40.09&lon=-74.22&tz=America/New_York&date=2024-04-23");
    assert.equal(status, 200);
    assert.equal(body.hebrewDate, "15 Nisan 5784");
    assert.deepEqual(body.specialDays, ["Pesach"]);
    assert.equal(body.dafYomi, "Bava Metzia 55");
    // 09:31 in New York.
    assert.match(body.zmanim.sofZmanShmaGra, /^2024-04-23T13:31:00/);
    assert.equal(body.zmanim.sofZmanAchilasChametzGra, null);
  });

  test("uses time zones from the runtime's own data", async () => {
    const { body } = await get("lat=31.778&lon=35.235&tz=Asia/Jerusalem&date=2024-04-23&region=israel");
    assert.ok(body.zmanim.sofZmanKidushLevana15Days, "Kiddush Levana needs the time zone");
    assert.match(body.zmanim.sunrise, /^2024-04-23T03:/);
  });

  test("rejects bad input with a 400", async () => {
    assert.equal((await get("lat=91&lon=0&tz=UTC&date=2024-04-23")).status, 400);
    assert.equal((await get("lat=0&lon=0&tz=Nowhere/Special&date=2024-04-23")).status, 400);
    assert.equal((await get("lat=0&lon=0&tz=UTC&date=2024-02-30")).status, 400);
  });
});
