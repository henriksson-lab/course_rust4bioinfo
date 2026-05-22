# Expected Learning Outcomes

A two-week intensive course in Rust, framed around bioinformatics problems. Each day pairs a Rust concept with a biological topic that motivates it: students never learn a language feature in the abstract.

See [audience.md](audience.md) for the assumed background.

## Overall course outcome

By the end of the course, a student should be able to take a small, well-defined bioinformatics task — for example, parsing a FASTA file, computing per-record statistics, querying it from SQL, or training a small sequence model — and implement it in idiomatic Rust as a `cargo` project, using community crates where appropriate, with basic tests and a readable error story.

Equally important: the student should know **when Rust is the right tool** (and when to stay in Python/R), and have a realistic mental model of what they would need to learn next to tackle larger projects.

## Cross-cutting outcomes (apply throughout the course)

A student who finishes the course can:

- Explain why Rust is used in modern bioinformatics tooling (samtools rewrites, single-cell tools, alignment indexes) — performance, safety, single static binary, ecosystem trajectory.
- Read a Rust compiler error message and act on the suggested fix without panic.
- Use `cargo` to create a project, add a dependency, build, run, test, and produce a release binary.
- Read idiomatic Rust well enough to follow examples in crate documentation on [docs.rs](https://docs.rs).
- Compare a Rust solution to its R/Python equivalent and articulate the trade-offs (development time, runtime, memory, deployability).

## Day-by-day outcomes

### Day 1 — Toolchain, scalars, control flow, functions

- Install Rust via `rustup`; explain what `rustup`, `cargo`, and `rustc` each do.
- Create, build, run, and test a project with `cargo`.
- Use the scalar types (`u8`, `i32`, `u32`, `u64`, `usize`, `f64`, `bool`, `char`); understand why Rust has so many integer types and pick appropriately for read counts, coordinates, and quality scores ([Quiz: scalar types and ranges](day1/00e-types-quiz.qmd)).
- Declare immutable and mutable bindings; use `if`, `match`, `loop`, `while`, `for`.
- Define and call functions with typed parameters and return values.
- **Bio examples:** GC content; base counts; complementing a single nucleotide; Phred quality decoding; Hamming distance.

### Day 2 — Ownership, borrowing, strings, slices, bytes

- State the three ownership rules and predict whether a snippet compiles ([Quiz: ownership and borrowing](day2/00a-ownership-quiz.qmd)).
- Distinguish `String` from `&str`; pass slices (`&[u8]`, `&str`) instead of owning data.
- Treat DNA sequences as `&[u8]` and explain why.
- **Bio examples:** Reverse complement (owned vs. into a buffer); k-mer sliding window; quality trimming as a slice view.

### Day 3 — Structs, enums, iterators, recursion, error handling

- Use `Vec<T>`, `HashMap<K, V>`, arrays; pick between them for a given task ([Quiz: enums, Option, and Result](day3/00a-quiz.qmd)).
- Define `struct`s and `enum`s; pattern-match exhaustively.
- Use `Option<T>` and `Result<T, E>`; propagate errors with `?`.
- Define a recursive enum (e.g. a phylogenetic tree) and explain why it needs `Box<T>`.
- **Bio examples:** Genomic intervals; codon table; k-mer counter; phylogenetic tree; Needleman–Wunsch alignment (dynamic programming).

### Day 4 — I/O, crates, modules, plotting, zip

- Read and write files with `std::io`, buffered I/O, and `flate2` for gzip ([Quiz: I/O, crates, and error handling](day4/00a-quiz.qmd)).
- Add and use crates from crates.io; navigate docs.rs.
- Use the [`noodles`](https://docs.rs/noodles/) family for FASTA, FASTQ, BAM, VCF, …
- Produce a publication-style figure with [`plotters`](https://docs.rs/plotters/).
- Bundle outputs into a `.zip` archive.
- **Bio examples:** BED parsing by hand; per-contig FASTA stats; FASTQ filtering by quality; read-length histogram; bundled results archive.

### Day 5 — Tests, `--release`, parallelism

- Write unit tests with `#[test]`; run with `cargo test` ([Quiz: tests, `--release`, and parallelism](day5/00a-quiz.qmd)).
- Build release binaries; measure the 10–100× speed-up vs. debug.
- Identify allocation hotspots and remove them.
- Parallelise with `rayon::par_iter`; reason about `Send`/`Sync`.
- **Bio examples:** Find-the-bug via tests; release-mode timing on a k-mer counter; parallel GC content across a FASTQ.

### Day 6 — Rust for the web

- Compile Rust to WebAssembly with `wasm-pack` ([Quiz: Yew and WebAssembly](day6/00a-quiz.qmd)).
- Write a Yew component; understand state, props, and re-renders.
- Talk to a backend via `gloo-net` / `web-sys`.
- Share types between frontend and backend via a workspace `shared` crate.
- **Bio examples:** Reactive sequence statistics; fetching from a Rust backend; GC sliding-window inspector.

### Day 7 — SQL and databases

- Install and use `sqlite3` (see [Install SQLite](intro/install-sqlite.qmd)).
- Write `SELECT`, `JOIN`, `WHERE`, `GROUP BY`, `HAVING`, `COUNT` ([Quiz: SQL and SQLite](day7/00a-quiz.qmd)).
- Use parameter binding (`?1`, `:name`) and prevent SQL injection.
- Read and write SQLite from Rust with [`rusqlite`](https://docs.rs/rusqlite/).
- Enable `PRAGMA foreign_keys = ON`; wrap bulk inserts in transactions.
- **Bio examples:** Strain database with mutations and joins; Rust CLI that reads, writes, and queries it.

### Day 8 — Traits

- Implement `Display` and other named traits ([Quiz: traits, generics, and dispatch](day8/00a-quiz.qmd)).
- Write generic functions with trait bounds; understand monomorphisation.
- Use `Box<dyn Trait>` for heterogeneous collections; compare static vs. dynamic dispatch.
- Derive the standard "mechanical" traits (`Debug`, `Clone`, `PartialEq`, …).
- **Bio examples:** `Display` for `GenomicInterval`; a custom `Annotate` trait; generic `summarize<T: Score>`; heterogeneous readers.

### Day 9 — Algorithms and complexity

- Predict the Big-O of standard-library collection operations ([Quiz: data-structure complexity](day9/00a-ds-complexity-quiz.qmd), [Quiz: complexity from code](day9/00b-code-complexity-quiz.qmd)).
- Recognise complexity from a code snippet (nested loops, hash vs. tree, sort vs. scan).
- Pick `HashMap` vs. `BTreeMap` vs. `Vec` for a given access pattern.

### Day 10 — Working with matrices

- Use `ndarray::Array2` for dense matrices; slicing and matrix-vector multiplication ([Quiz: dense, sparse, and iterative](day10/00a-quiz.qmd)).
- Use `sprs::CsMat` for sparse data (single-cell, k-mer tables, networks).
- Implement Gauss–Seidel as a simple iterative solver.
- Run power iteration on a Markov chain to find a stationary distribution.
- **Bio examples:** Single-cell-style sparse expression matrix; viral-strain Markov chain.

### Day 11 — GPUs and tensors with Burn

- Read a `Tensor<B, D>` signature; recognise rank vs. shape ([Quiz: tensors, backends, and shapes](day11/00a-quiz.qmd)).
- Write functions generic over `<B: Backend>` so the same code runs on CPU and GPU.
- Use broadcasting and reductions to replace explicit loops.
- Compute PCA via [`nalgebra::DMatrix::svd`](https://nalgebra.org/).
- Implement PWM scanning as `Conv1d`; train a tiny classifier with Burn's autodiff.
- **Bio examples:** One-hot DNA; GC content as a tensor reduction; pairwise distance matrix; PCA on expression data; promoter classifier.

### Day 12 — GPU compute kernels with wgpu

- Read and write WGSL compute shaders ([Quiz: wgpu compute kernels](day12/00a-quiz.qmd)).
- Walk through the 6-step wgpu pipeline (adapter → device → buffers → pipeline → bind group → dispatch).
- Use storage and uniform buffers; bound-check inside shaders; size workgroup dispatches.
- Use atomics for counters; workgroup shared memory for fast reductions; 2-D dispatch for matrices.
- Re-implement Day 1 / 5 (GC content), Day 2 (reverse complement), and Day 11 (pairwise distances) as raw GPU kernels and compare.

## Explicit non-outcomes

To keep the course honest and the cognitive load survivable, the following are **not** expected outcomes:

- Writing explicit lifetime annotations beyond what is needed to read elided ones
- Designing trait hierarchies or writing generic library APIs
- Using `async`/`await` or any async runtime
- Writing macros (using `println!`, `vec!`, `assert_eq!` is fine; defining `macro_rules!` is not)
- Writing or reasoning about `unsafe` code
- Foreign function interface (FFI) to C or Python
- Building Python extension modules with `pyo3` / `maturin`
- Deep knowledge of any one bioinformatics crate's full API — students will know how to *find* what they need, not memorise it

## Assessment (informal)

Each day's self-paced material ends with hands-on exercises tied to that day's outcomes, plus a short [quiz](day1/00e-types-quiz.qmd) that drills the core concepts. The course culminates in a capstone task that touches outcomes from across the week — parse a bioinformatics file, compute something useful per record, optionally in parallel or on the GPU, with tests and a clean CLI. Success on the capstone is the working definition of "passed the course."
