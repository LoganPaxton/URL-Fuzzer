# Fuzzer (Rust + Node.js)

A **blazing-fast wordlist-based URL fuzzer** built with Rust, [`reqwest`](https://docs.rs/reqwest/), and [`napi-rs`](https://napi.rs).
Exports a single async function `fuzz(baseUrl, wordlistPath)` that you can call from Node.js / TypeScript.

## Features

* Native-speed HTTP fuzzing from Node.js
* Uses Tokio for concurrency (default 2000 concurrent requests)
* Written in Rust, exported as a `.node` addon via napi-rs
* Simple async API

## Requirements

* Rust (stable)
* Node.js 18+
* `pnpm`/`npm`/`yarn`
* TypeScript (Recommended)

## Installation

Clone and build:

```bash
git clone https://github.com/LoganPaxton/URL-Fuzzer
cd fuzzer
cargo build --release
```

This will create `index.node` in your `target/release` directory (or copy it next to your JS/TS entrypoint).

## Usage

### TypeScript (ESM)

```ts
import { createRequire } from 'module';
const require = createRequire(import.meta.url);
const { fuzz } = require('./index.node');

(async () => {
  const results = await fuzz("https://example.com", "./wordlist.txt");
  console.log(results);
})();
```

Run with:

```bash
npx tsx index.ts
```

### JavaScript (CommonJS)

```js
const { fuzz } = require('./index.node');

(async () => {
  const results = await fuzz("https://example.com", "./wordlist.txt");
  console.log(results);
})();
```

### Output

`fuzz()` returns an array of strings in the format:

```
[
  "200 OK -> https://example.com/admin",
  "404 Not Found -> https://example.com/test"
]
```

## API

#### `fuzz(baseUrl: string, wordlistPath: string): Promise<string[]>`

* `baseUrl` – The base URL to fuzz.
* `wordlistPath` – Path to a text file containing one path per line.

## Wordlist Format

Plain text, one path per line:

```
admin
login
dashboard
```

## Performance Tips

* Adjust `max_concurrency` in `src/lib.rs` to tune concurrent requests.
* Run against local servers for very fast fuzzing.

## License

MIT
