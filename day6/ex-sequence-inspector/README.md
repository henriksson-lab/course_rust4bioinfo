# Sequence inspector — workspace template

This is a three-crate Rust workspace:

- **`shared/`** — types serialised over the wire (`SampleSummary`, `SampleRecord`). Imported by both other crates. The compiler guarantees the JSON shape cannot drift between server and client.
- **`backend/`** — an [axum](https://docs.rs/axum/) server on `http://127.0.0.1:3000`. Exposes `GET /api/samples` (list) and `GET /api/samples/{id}` (one record).
- **`frontend/`** — a [Yew](https://yew.rs) app served by [trunk](https://trunkrs.dev) on `http://127.0.0.1:8080`. Compiles to WebAssembly and runs entirely in the browser.

Trunk forwards anything the frontend requests at `/api/...` to the backend, so the frontend can use same-origin URLs and we never deal with CORS. The proxy config lives in `frontend/Trunk.toml`.

## Running it

You need **two terminals** open at the same time.

In the first, start the backend:

```bash
cargo run -p backend
```

You should see:

```
backend listening on http://127.0.0.1:3000
```

In the second, start the frontend:

```bash
cd frontend
trunk serve
```

The first run will download `wasm-bindgen-cli` and compile a few hundred crates — give it a minute. Once trunk says *"server listening at http://127.0.0.1:8080"*, open that URL in a browser. You should see the Yew page.

Edit `frontend/src/main.rs` and save: trunk auto-rebuilds and the page reloads.

## If you do not have trunk yet

```bash
cargo install trunk
rustup target add wasm32-unknown-unknown
```

These take a few minutes the first time; both are one-off.

## Where to go next

- [Day 6 — exercise 1](../01-hello-yew.qmd) walks through the toolchain and your first edit.
- [Day 6 — exercise 2](../02-reactive-stats.qmd) adds a `<textarea>` and a live GC-content readout.
- The remaining exercises (03, 04) layer fetch from the backend and the capstone visualisation on top.

The fully finished version of this workspace lives at `day6/ex-sequence-inspector-solution/`. Try not to peek at it before you have your own version of the capstone running.
