# Examples

Each example uses the package from `../..`, so build it first with `npm run build` there.

| Example | Shows | Test |
|---|---|---|
| [vite-react](vite-react) | A zmanim and calendar page built with Vite and React, and a page that loads `kosher-rust/slim`. | `npm test` builds it for production and runs it in Chrome on a desktop and a phone screen with Playwright. |
| [node](node) | A command line app that prints the day's Hebrew date, learning and zmanim. | `npm test` runs under Node and `npm run test:bun` under Bun. Each starts the app with both. |
| [cloudflare-worker](cloudflare-worker) | A Worker that answers with the zmanim for a place and day, using `kosher-rust/slim`. | `npm test` bundles it with Wrangler and runs it in workerd. |

```bash
cd vite-react && npm install && npx playwright install chromium && npm test
cd node && npm install && npm test && npm run test:bun
cd cloudflare-worker && npm install && npm test
```
