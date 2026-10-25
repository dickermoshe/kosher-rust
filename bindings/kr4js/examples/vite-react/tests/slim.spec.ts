import { expect, test } from "@playwright/test";

test("@dickermoshe/kr4js/slim fetches the WebAssembly file Vite copied into the build", async ({ page }) => {
  page.on("pageerror", (error) => {
    throw error;
  });
  const wasm = page.waitForResponse((response) => response.url().endsWith(".wasm"));
  await page.goto("/slim.html?date=2024-04-23");
  const response = await wasm;
  expect(response.status()).toBe(200);
  expect(response.headers()["content-type"]).toContain("application/wasm");
  await expect(page.locator("#hebrew-date")).toHaveText("15 Nisan 5784");
  await expect(page.locator("#daf-yomi")).toHaveText("Bava Metzia 55");
});
