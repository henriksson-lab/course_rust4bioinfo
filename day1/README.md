# Day 1 — Exercises

Work through these in order. Each `.qmd` is a self-contained walkthrough with hints (collapsible) and a reference solution; each `ex-<name>/` directory is a real cargo project that the compiler will check. Make `cargo test` pass before moving on.

| # | Topic | Rust concepts | Crate |
|---|---|---|---|
| 1 | [GC content](01-gc-content.qmd) | `for` loops, `match` on bytes, integer-to-float casts, formatted printing | [`ex-gc-content/`](ex-gc-content/) |
| 2 | [Complement a single base](02-complement.qmd) | `match` as an expression, byte literals, `panic!` on invalid input | [`ex-complement/`](ex-complement/) |
| 3 | [Count A, C, G, T, and other](03-base-counts.qmd) | mutable counters, tuple return values, alternative `match` arms | [`ex-base-counts/`](ex-base-counts/) |
| 4 | [Phred quality scores](04-phred.qmd) | `u8` arithmetic, range checks, the FASTQ quality encoding | [`ex-phred/`](ex-phred/) |
| 5 | [Hamming distance](05-hamming.qmd) | two-slice signatures, precondition panics, indexed loops | [`ex-hamming/`](ex-hamming/) |

## Working pattern

For each exercise:

```bash
cd day1/ex-<name>
cargo test           # see what is expected — some tests will fail at first
# edit src/main.rs until cargo test reports: 0 failed
cargo run -- <args>  # try it on real input
```

Every hint in the qmds links to the Rust documentation for whatever new thing it introduces — open them one at a time when you get stuck.

The `ex-<name>-solution/` directories contain working reference implementations. Try not to peek before you have your own version passing the tests.
