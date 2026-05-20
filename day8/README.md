# Day 3b — Exercises

Traits — Rust's interface mechanism.

Start with [**00 — Concepts**](00-concepts.qmd) for a reference overview of traits (companion to the slides). Day 3b sits between day 3 (structs, enums, recursion) and day 4 (I/O with `noodles`): it introduces the one missing piece that ties them together.

| # | Topic | Rust concepts | Crate |
|---|---|---|---|
| 0 | [**Concepts**](00-concepts.qmd) | `trait`, `impl Trait for Type`, `derive`, trait bounds, `Box<dyn Trait>` | — |
| 1 | [Display for a struct](01-display.qmd) | `impl Display for ...`, the `Display` trait, `format!` | [`ex-display/`](ex-display/) |
| 2 | [A custom trait with multiple impls](02-annotate.qmd) | `trait Annotate`, multiple impls | [`ex-annotate/`](ex-annotate/) |
| 3 | [A generic function with a trait bound](03-generic-function.qmd) | `fn f<T: Trait>(...)`, trait bounds | [`ex-generic-function/`](ex-generic-function/) |
| 4 | [Heterogeneous collection with `Box<dyn>`](04-trait-objects.qmd) | `Box<dyn Trait>`, dynamic dispatch, vtable mental model | [`ex-trait-objects/`](ex-trait-objects/) |

## Working pattern

For each exercise:

```bash
cd day8/ex-<name>
cargo test           # see what is expected — some tests will fail at first
# edit src/lib.rs (or src/main.rs) until cargo test reports: 0 failed
```

The `ex-<name>-solution/` directories contain working reference implementations. Try not to peek before you have your own version passing the tests.
