# Sequence inspector — reference solution

This is the fully wired-up version of the day-6 capstone: the same three-crate workspace as [`ex-sequence-inspector/`](../ex-sequence-inspector/), but with the frontend completed.

What the frontend does:

- Loads the sample list from `GET /api/samples` on mount and renders one button per sample.
- Clicking a button fetches `GET /api/samples/{id}` and drops the returned sequence into the textarea.
- The textarea is bound to state with `use_state`; any keystroke or fetch result re-runs the statistics functions and re-renders the table.
- Statistics (length, GC content, per-base counts) are computed in the **browser** using the same pure functions students wrote on days 1 and 2 — no server round-trip per keystroke.

Run it the same way as the starter (two terminals: `cargo run -p backend` and `cd frontend && trunk serve`), then open <http://127.0.0.1:8080>.
