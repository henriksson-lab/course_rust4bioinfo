# Day 5 — Exercises

Tests, `--release`, and parallelism — the three things that separate a working Rust prototype from a deployable bioinformatics tool.

| # | Topic | Pattern | Crate |
|---|---|---|---|
| 1 | [Use tests to find the bug](01-find-the-bug.qmd) | write tests, watch them fail, fix the function | [`ex-find-the-bug/`](ex-find-the-bug/) |
| 2 | [`--release` is a 50× free lunch](02-release-mode.qmd) | measure debug vs release runtime; no code to write | [`ex-release-mode/`](ex-release-mode/) |
| 3 | [Parallel GC with `rayon`](03-parallel-gc.qmd) | turn `.iter()` into `.par_iter()` | [`ex-parallel-gc/`](ex-parallel-gc/) |

Exercise 2 has no `-solution` crate because there is nothing to fill in — the program is provided complete, and the exercise is to **run** it under both build profiles and observe the timing difference.
