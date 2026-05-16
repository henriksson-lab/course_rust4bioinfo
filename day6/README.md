# Day 6 — Exercises

Build a **pure-Rust web application**: a Yew frontend (compiled to WebAssembly), an axum backend, and a shared crate of types that compiles to both targets. The four exercises walk you through one workspace, starting from a "Hello, world!" page and ending with a real interactive sequence inspector.

| # | Topic | New concept |
|---|---|---|
| 1 | [Hello Yew](01-hello-yew.qmd) | trunk, the two-terminal dev loop, `html!`, hot reload |
| 2 | [Reactive sequence statistics](02-reactive-stats.qmd) | `use_state`, controlled inputs, computing on every render |
| 3 | [Fetch from the backend](03-fetch-from-backend.qmd) | `use_effect_with`, `spawn_local`, `gloo_net::Request`, async/await |
| 4 | [Capstone — GC sliding window](04-sequence-inspector.qmd) | inline SVG via `html!`, mapping `Vec<f64>` to a `<polyline>` |

## Working pattern

The whole day shares one workspace:

- `ex-sequence-inspector/` — the **starter** workspace (Hello-world frontend, full backend).
- `ex-sequence-inspector-solution/` — the **reference solution** (everything wired up).

You edit `ex-sequence-inspector/` as you go through the four exercises. Each exercise's qmd describes the changes layered on top of the previous one.

## How to run, every day

Two terminals:

```bash
# terminal 1
cargo run -p backend                  # axum on :3000

# terminal 2
cd frontend && trunk serve            # yew on :8080
```

Then open <http://127.0.0.1:8080>. See the [workspace README](ex-sequence-inspector/README.md) for details and one-off setup instructions.
