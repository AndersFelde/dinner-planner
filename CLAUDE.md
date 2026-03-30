# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Development Commands

```bash
# Start development server with hot reload
cargo leptos watch

# Build for production
cargo leptos build --release

# Build Tailwind CSS (run alongside cargo leptos watch)
npm run watch

# Run E2E tests (requires dev server running)
cargo leptos end-to-end
```

**Prerequisites:**

- Rust nightly (pinned via `rust-toolchain.toml` to `nightly-2025-09-04`)
- `cargo install cargo-leptos`
- `rustup target add wasm32-unknown-unknown`
- Node.js/npm for Tailwind CSS

**Environment:** Requires a `.env` file with `DATABASE_URL` (defaults to `db.sqlite3`).

## Architecture

Full-stack Rust web app using **Leptos** (isomorphic/reactive WASM framework) + **Axum** (HTTP server) + **SQLite** via **Diesel ORM**. Styling is Tailwind CSS 4.

The app is a collaborative dinner planning and expense tracker for 3 people (Anders, AC, Andreas).

### Leptos Isomorphic Pattern

Code compiles to two targets:

- `ssr` feature flag → server binary (Axum)
- `hydrate` feature flag → WASM bundle (client hydration)

Server functions (`#[server]`) run only on the backend but are callable from frontend components. Feature-gated imports are common: `#[cfg(feature = "ssr")]`.

### Code Organization

```
src/
  main.rs          # Axum server setup, DB migrations, static file serving
  lib.rs           # WASM hydration entry point
  app.rs           # Root Leptos component, routing, global state
  db.rs            # SQLite connection pool (WAL mode, foreign keys enforced)
  schema.rs        # Diesel schema (auto-generated, do not edit manually)
  models/          # Diesel model structs (Queryable/Insertable)
  api/             # Server functions grouped by domain
  routes/          # Page-level Leptos components (week, meal_list, shopping_list, receipt)
  components/      # Reusable UI components
    forms/         # Create/edit forms
    buttons/       # Button components
    modal.rs       # Modal component
    notifications.rs # Toast notifications
migrations/        # Diesel SQL migrations (run automatically on server start)
end2end/           # Playwright E2E tests (TypeScript)
```

### State Management

Global app state is held in `reactive_stores` signals defined in `app.rs` and passed via Leptos context. Components access shared state via `use_context::<T>()`.

### Database Migrations

Migrations in `migrations/` run automatically at server startup via `db.rs`. To add a new migration:

```bash
diesel migration generate <name>
# edit the up.sql/down.sql files
# schema.rs is regenerated on next build
```

### Receipt OCR

The Dockerfile includes Python + PaddleOCR for receipt scanning. The OCR feature is only available in the containerized deployment.
