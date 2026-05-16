# Design principles

The principles below govern how we build course material in this repository. They exist to keep us honest as the material scales — every new exercise, lecture, and chapter should be checked against them.

See [audience.md](audience.md) for who we are designing for, and [elo.md](elo.md) for what we are trying to teach.

## 1. Exercises over stretch goals

We prefer a **good number of core exercises that every student is expected to complete** over a small number of exercises with long optional "Going further" tails.

- The audience is broad — 4th-year biologists with one prior programming course. Stretch goals systematically benefit the students who are already ahead and widen the spread within the cohort rather than building shared competence.
- If a topic is important enough to mention in an exercise, it is important enough to teach properly somewhere in the course. Move it to a core exercise on the day the prerequisite material is taught, rather than parking it as a stretch.
- A short, neutral **"Where this leads"** section at the end of an exercise is acceptable — it tells students what we deliberately defer and to which day, without inviting them to attempt it now.

## 2. Linked hints over named hints

Every key function, method, macro, or language feature named in a hint must be a **hyperlink to its documentation**.

- Crate items link to docs.rs (e.g. [`noodles_fasta::Reader`](https://docs.rs/noodles-fasta/latest/noodles_fasta/struct.Reader.html)).
- Standard library items link to doc.rust-lang.org/std (e.g. [`<[u8]>::is_empty`](https://doc.rust-lang.org/std/primitive.slice.html#method.is_empty)).
- Language features link to the Rust Book or the Rust Reference, whichever explains the concept at the right level (e.g. [`match`](https://doc.rust-lang.org/book/ch06-02-match.html), [byte literals](https://doc.rust-lang.org/reference/tokens.html#byte-literals)).

A student should never have to guess search terms in order to complete a core exercise. Advanced students are explicitly encouraged to google and explore beyond the linked hints — but the linked hints alone must be sufficient for the rest of the class to finish.

## 3. Tests are the spec

Where it fits, every exercise ships with a `#[cfg(test)] mod tests` block (or `tests/` integration tests) that pins down the expected behaviour. Students run `cargo test` and iterate until it passes.

- The student's job is to make existing tests pass; writing their own tests is itself a learning outcome (day 5) and not assumed earlier.
- A starter crate should compile from day one — even if its function bodies are stubs — so that `cargo test` always produces a real test report rather than a build error.

## 4. Concept ↔ biology pairing

Every Rust concept is introduced together with a biological topic that motivates it (see [elo.md](elo.md)). We do not teach a language feature in the abstract first and apply it to biology later.

- Day-by-day structure is a working draft, not a contract: we will shift, merge, or split days as the pairings become clearer.
- The unit of design is the **pairing**, not the day.

## 5. Real cargo projects, not REPL snippets

Exercises are shipped as real cargo projects (one **starter** crate and one sibling **solution** crate per exercise), referenced from a `.qmd` walkthrough. We do not rely on Quarto's Jupyter engine to execute Rust at render time.

- Learning `rustup`, `cargo`, and how to read real compiler errors is an explicit day-1 outcome. Pasting code into a notebook bypasses that.
- The qmd holds the prose, hints (as collapsible callouts), and reference solution snippets. The crate holds code that the compiler actually verifies.

## 6. Honest about what we defer

When an exercise uses a simplification that we will later replace with a better tool (e.g. `.expect` instead of `?`, ad-hoc parsing instead of `noodles`, a `for` loop instead of an iterator), say so plainly and name the day on which the proper treatment arrives. Students should always know what is a stepping stone and what is idiomatic Rust.
