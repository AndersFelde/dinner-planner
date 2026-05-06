# Dinner Planner

A small web app for planning dinners and managing recipes.

Built with **Leptos** (Rust + WASM) and **Axum**.

## Prerequisites

- Rust toolchain (see `rust-toolchain.toml`; nightly recommended)
- `cargo-leptos`
- Node.js + npm (for Tailwind CSS)

Install `cargo-leptos`:

```bash
cargo install cargo-leptos --locked
```

Install JS deps:

```bash
npm install
```

## Development

Run the app with live-reload:

```bash
cargo leptos watch
```

Tailwind CSS (optional, if you are editing styles):

```bash
npm run watch
```

The server listens on the address configured in `Cargo.toml` under `[package.metadata.leptos].site-addr` (default: `0.0.0.0:3000`).

## Build (release)

```bash
cargo leptos build --release
```

Outputs:

- Server binary: `target/release/dinner-planner`
- Site bundle: `target/site`

## Run with Docker

Build and run:

```bash
docker compose up --build
```

By default `docker-compose.yml` publishes the app on:

- http://localhost:8080

It also mounts `./db.sqlite3` into the container at `/app/db.sqlite3` (ensure this file exists or adjust the volume mount).

## End-to-end tests

```bash
cargo leptos end-to-end
```

Tests live in `end2end/tests` and use Playwright. If prompted for browsers, install them with `npx playwright install`.

## Deploying without a Rust toolchain

After `cargo leptos build --release`, copy these to the server:

1. `target/release/dinner-planner`
2. `target/site` (entire directory)

Set environment variables as needed (typical values shown):

```sh
export LEPTOS_OUTPUT_NAME="dinner-planner"
export LEPTOS_SITE_ROOT="site"
export LEPTOS_SITE_PKG_DIR="pkg"
export LEPTOS_SITE_ADDR="127.0.0.1:3000"
export LEPTOS_RELOAD_PORT="3001"
```

Then run the binary.

## License

See [LICENSE](./LICENSE).