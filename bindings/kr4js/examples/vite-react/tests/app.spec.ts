import { expect, test } from "@playwright/test";

test.beforeEach(async ({ page }) => {
  page.on("pageerror", (error) => {
    throw error;
  });
});

test("shows the Hebrew date, the learning and the zmanim", async ({ page }) => {
  await page.goto("/?city=lakewood&date=2024-04-23");
  await expect(page.getByTestId("hebrew-date")).toHaveText("15 Nisan 5784");
  await expect(page.getByTestId("special-days")).toContainText("Pesach");
  await expect(page.getByTestId("learning")).toContainText("Bava Metzia 55");
  await expect(page.getByTestId("learning")).toContainText("Nazir 5:5-6");
  await expect(page.locator('[data-zman="sofZmanShmaGra"] .time')).toHaveText("9:31:00 AM");
});

test("loads without fetching a .wasm file", async ({ page }) => {
  const wasm: string[] = [];
  page.on("request", (request) => {
    if (request.url().endsWith(".wasm")) wasm.push(request.url());
  });
  await page.goto("/?city=jerusalem&date=2024-05-04");
  await expect(page.getByTestId("special-days")).toContainText("Parshas Achrei Mos");
  expect(wasm).toEqual([]);
});

test("follows the chosen place and date", async ({ page }) => {
  await page.goto("/?city=lakewood&date=2024-04-23");
  const shma = page.locator('[data-zman="sofZmanShmaGra"] .time');
  await expect(shma).toHaveText("9:31:00 AM");

  await page.getByTestId("city").selectOption("jerusalem");
  await expect(shma).not.toHaveText("9:31:00 AM");
  await expect(page).toHaveURL(/city=jerusalem/);

  await page.getByRole("button", { name: "Next day" }).click();
  await expect(page.getByTestId("hebrew-date")).toHaveText("16 Nisan 5784");
  await expect(page.getByTestId("special-days")).toContainText("Chol Hamoed Pesach");
});

test("shows why a time is missing", async ({ page }) => {
  await page.goto("/?city=tromso&date=2024-06-21");
  await expect(page.locator('[data-zman="sunrise"] .time')).toHaveText("The sun stays too high today");
});

test("lists every opinion", async ({ page }) => {
  await page.goto("/?city=lakewood&date=2024-04-23");
  await page.getByText("Every opinion").click();
  expect(await page.locator("[data-zman]").count()).toBeGreaterThan(140);
  await expect(page.getByRole("heading", { name: "Kiddush Levana" })).toBeVisible();
});
