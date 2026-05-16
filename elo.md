# Expected Learning Outcomes

An intensive course of roughly a working week. Each day has at most two prerecorded ~30-minute lectures (one in the morning, one after lunch). The remaining time is self-paced study material and exercises built around bioinformatics tasks.

The day-by-day structure below is a working draft rather than a fixed plan: the unit of pacing is the **concept–topic pairing**, and we are willing to shift, merge, or split days once we have a clearer picture of how the material flows. The guiding principle: each Rust concept is introduced together with a biological topic that naturally motivates it, so that students never learn a language feature in the abstract.

See [audience.md](audience.md) for the assumed background.

## Overall course outcome

By the end of the week, a student should be able to take a small, well-defined bioinformatics task — for example, parsing a FASTA file, computing per-record statistics, filtering by a criterion, and writing the result back out — and implement it in idiomatic Rust as a `cargo` project, using community crates where appropriate, with basic tests and a readable error story.

Equally important: the student should know **when Rust is the right tool** (and when to stay in Python/R), and have a realistic mental model of what they would need to learn next to tackle larger projects.

## Cross-cutting outcomes (apply throughout the week)

A student who finishes the course can:

- Explain in one or two sentences why Rust is used in modern bioinformatics tooling (e.g. samtools rewrites, single-cell tools, alignment indexes) — performance, safety, single static binary, ecosystem trajectory
- Read a Rust compiler error message and act on the suggested fix without panic
- Use `cargo` to create a project, add a dependency, build, run, test, and produce a release binary
- Read idiomatic Rust well enough to follow examples in crate documentation on docs.rs
- Compare a Rust solution to its R/Python equivalent and articulate the trade-offs (development time, runtime, memory, deployability)

## Day-by-day outcomes

### Day 1 — Orientation and the toolchain

By end of day 1 the student can:

- Install Rust via `rustup` and explain what `rustup`, `cargo`, and `rustc` each do
- Create a new project with `cargo new`, build it, and run it
- Write a `main` function that reads command-line arguments and prints to stdout
- Use the basic scalar types (`i32`, `u64`, `f64`, `bool`, `char`) and understand why Rust has so many integer types (record counts, coordinates, quality scores)
- Declare variables with `let` and `let mut`, and explain why immutability is the default
- Write `if`/`else`, `loop`, `while`, and `for` and recognise that `if` and `match` are expressions, not statements
- Define and call functions with typed parameters and return values

**Biological examples to cover:**
- GC content of a DNA string passed on the command line
- Counting each base (A/C/G/T/N) with a `for` loop and `match`
- Complementing a single nucleotide via `match` on a `u8`
- Converting a Phred quality character to a numeric score (`q = c - 33`) with a sanity check that the result is in the expected range
- Hamming distance between two equal-length sequences — motivating typed function signatures and an early taste of why Rust forces you to handle the "different lengths" case

**Lecture topics (suggested):** (1) Why Rust for bioinformatics; (2) Hello cargo — the toolchain tour.

### Day 2 — Ownership, borrowing, and strings

By end of day 2 the student can:

- State the three ownership rules and predict whether a given snippet will compile
- Distinguish `String` from `&str` and explain when to use each (e.g. holding a parsed sequence vs. inspecting a slice of one)
- Use `&` and `&mut` references and follow the borrow checker's reasoning in simple cases
- Recognise common borrow-checker error messages and apply the standard fixes (clone, take a reference, shorten a borrow's scope)
- Pass slices (`&[u8]`, `&str`) to functions instead of owning data unnecessarily
- Treat DNA sequences as `&[u8]` rather than `&str` where appropriate and explain why

**Biological examples to cover:**
- Reverse complement of a DNA sequence — first as an owned `String` return, then as a function that writes into a caller-provided buffer, to make the cost of allocation visible
- A sliding k-mer window over a long sequence using slice indexing — no copies, just `&seq[i..i+k]`
- Trimming a read down to a high-quality interior region by returning a slice (`&[u8]`) instead of a new `Vec<u8>`
- Why DNA is naturally `&[u8]` and not `&str`: indexable in O(1), no UTF-8 multi-byte surprises, and bytes map directly into codon-table indices
- Demonstrate that the same `gc_content(seq: &[u8])` function works on a literal `b"ACGT"`, on a chunk loaded from a FASTA, and on a slice of a chromosome — without copying

**Lecture topics (suggested):** (1) Ownership and borrowing with sequence data; (2) Strings, slices, and bytes — choosing the right representation.

### Day 3 — Data structures, iterators, and error handling

By end of day 3 the student can:

- Use `Vec<T>`, `HashMap<K, V>`, and arrays appropriately, and pick between them for a given task (e.g. a k-mer counter)
- Define a `struct` to model a domain object (e.g. a FASTA record, a genomic interval) and implement methods with `impl`
- Define and pattern-match on an `enum`, including variants with data (e.g. a strand `enum`, or a parsed VCF field)
- Use `Option<T>` and `Result<T, E>` and propagate errors with `?`
- Write iterator chains using `map`, `filter`, `collect`, `sum`, `count`, and `fold`, and explain why iterators are zero-cost
- Convert a Python/R-style indexed loop into an iterator chain and judge which is more readable
- Define a **recursive** data type — for example a phylogenetic tree node whose children are themselves nodes — and explain why this requires indirection through `Box<T>` (a recursive struct has unknown size on the stack)
- Write recursive functions over such a tree: counting leaves, computing tree depth, summing branch lengths, collecting tip labels, and pretty-printing in Newick-like form
- Recognise when a recursive solution is clearer than an iterative one (tree traversal, divide-and-conquer) and when the reverse is true (long flat sequences, where deep recursion risks stack overflow)

**Biological examples to cover:**
- `struct FastaRecord { id, description, sequence }` and `struct GenomicInterval { chrom, start, end, strand }`
- `enum Strand { Plus, Minus }` and `enum VariantType { Snp, Insertion, Deletion, Complex }`, with `match` driving downstream logic
- A k-mer counter using `HashMap<Vec<u8>, u32>` over a streamed sequence
- A codon → amino acid lookup that returns `Option<AminoAcid>` (`None` for unknown or ambiguous codons containing `N`)
- Parsing a region string like `"chr1:1000-2000"` into a `GenomicInterval`, returning a `Result` with a meaningful error type
- An iterator pipeline that filters reads by length, computes per-read GC content, and aggregates the mean with `fold`
- Phylogenetic trees as the headline recursion example:
  - `enum Node { Leaf { name, branch_length }, Internal { children: Vec<Box<Node>>, branch_length } }`
  - Recursive functions: count tips, compute maximum depth, sum total branch length, collect all tip labels, pretty-print as a Newick string, find the most recent common ancestor of two tips

**Lecture topics (suggested):** (1) Structs, enums, and modelling biological data — including recursive types for phylogenetic trees; (2) Iterators, recursion, and the `?` operator — the everyday Rust control flow.

### Day 4 — Real bioinformatics: I/O, crates, and modules

By end of day 4 the student can:

- Read and write files using `std::fs` and `std::io`, including buffered I/O (`BufReader`, `BufWriter`) and gzip-compressed input where needed
- Add a crate from crates.io (e.g. `clap`, `anyhow`, `serde`, `flate2`, `zip`, `plotters`, `noodles`, `bio`, or `rust-htslib`) and read its documentation on docs.rs
- Use the [`noodles`](https://docs.rs/noodles/latest/noodles/) family of crates as the default toolbox for bioinformatics file formats — recognise that it is a pure-Rust workspace of per-format crates (`noodles-fasta`, `noodles-fastq`, `noodles-sam`, `noodles-bam`, `noodles-vcf`, `noodles-bcf`, `noodles-bed`, `noodles-gff`, `noodles-cram`, …) reached through the `noodles` umbrella crate via feature flags
- Parse a FASTA or FASTQ file with `noodles` and process records in a streaming fashion (without loading the whole file into memory)
- Read a BAM/SAM file with `noodles-bam` / `noodles-sam`: iterate records, inspect flags, filter by mapping quality, and look up reference names via the header
- Read a VCF file with `noodles-vcf`: iterate records, classify variants (SNP vs. indel) using the enums from day 3, and report per-chromosome counts
- Explain when to reach for `noodles` (pure Rust, no system library, broad format coverage, actively maintained) vs. `rust-htslib` (thin FFI wrapper around the canonical C library, useful when you need bit-for-bit parity with `samtools`/`bcftools`)
- Distinguish a gzip stream (`.fa.gz`, single compressed stream — use `flate2`) from a zip archive (`.zip`, multiple named entries — use the `zip` crate), and pick the right tool for each; recognise that `noodles` handles BGZF (the block-gzip variant used in `.bam`, `.vcf.gz`, `.bgz`) transparently
- Read entries from a `.zip` archive by name, and create a new `.zip` archive that bundles several output files (e.g. a per-sample report plus a summary table)
- Split a program across multiple files using `mod` and `pub`, and understand the difference between a binary crate, a library crate, and a workspace
- Write a small CLI tool with subcommands or flags using `clap`
- Produce a publication-style figure (PNG or SVG) from computed data using the [`plotters`](https://docs.rs/plotters/latest/plotters/) crate, and recognise the building blocks of its API (`DrawingArea`, `ChartBuilder`, series such as `LineSeries` and `Histogram`) well enough to adapt examples from its documentation
- Compare `plotters` mentally to `ggplot2` / `matplotlib`: an imperative builder API rather than a layered grammar, but with the same conceptual pieces (axes, scales, series, labels)
- Handle errors at the application boundary using `anyhow` (or equivalent) and emit a useful message instead of a panic

**Biological examples to cover:**
- Streaming a `.fa` or `.fa.gz` reference one record at a time with `noodles-fasta` and computing per-contig length and GC
- Filtering a `.fastq.gz` file by mean read quality with `noodles-fastq` and writing the surviving reads to a new `.fastq.gz`
- Parsing a simple BED file by hand (split-on-tabs, parse coordinates) before reaching for `noodles-bed` — to reinforce slicing and `Result` handling, and then doing the same task with the crate so the trade-off is visible
- A `noodles-bam` walk over a small aligned BAM: count primary alignments per reference, count reads passing a mapping-quality cutoff, and tally how many are secondary or supplementary using the SAM flag bits
- A `noodles-vcf` walk over a small VCF: classify each record as SNP / insertion / deletion / complex using the `VariantType` enum from day 3, and emit a per-chromosome summary table
- A `clap`-driven CLI like `seqtool stats <in>` and `seqtool filter --min-len 100 --min-qual 20 <in> <out>`, organised into multiple modules (`io`, `seq`, `cli`)
- Plotting with `plotters`:
  - A read-length distribution as a histogram from a FASTQ
  - Mean per-base quality vs. read position as a line plot (the classic FastQC-style profile)
  - Per-contig GC vs. length as a scatter plot
  - Saving each figure as both PNG (for the report) and SVG (for embedding)
- Bundling per-sample QC outputs (a small TSV, a JSON summary, the figures produced above) into a single results `.zip` for handover to a collaborator
- Reading entries from a reference dataset distributed as a `.zip` archive (e.g. a bundle of small annotation files), to contrast with the gzip-stream case from earlier

**Lecture topics (suggested):** (1) Cargo, crates.io, and the bioinformatics ecosystem — with `noodles` as the worked example; (2) Streaming I/O — how to process files bigger than RAM.

### Day 5 — Going faster, testing, and what comes next

By end of day 5 the student can:

- Write unit tests with `#[test]` and run them with `cargo test`
- Build a release binary with `cargo build --release` and measure the speed difference against a debug build and against an equivalent Python/R implementation
- Identify obvious performance pitfalls in their own code: unnecessary allocations, cloning in hot loops, repeated hash lookups, unbuffered I/O
- Use `rayon`'s `par_iter` to parallelise an embarrassingly parallel computation (e.g. per-record statistics across FASTQ reads) and reason about whether it is safe to do so
- Explain at a high level what traits and generics are, and recognise common ones (`Display`, `Debug`, `Clone`, `From`, `Iterator`) when reading code
- Name at least two Rust bioinformatics projects in active use and locate their source code
- Articulate, for a future project of their own choosing, whether Rust is a reasonable language choice and what the first concrete step would be

**Biological examples to cover:**
- Unit tests for reverse-complement and codon translation against textbook answers, plus a property-style check that "reverse-complement twice = identity"
- Timing a k-mer counter on a real `.fastq.gz` in debug vs `--release`, and against a one-line Python/Biopython equivalent — concrete numbers, not hand-waving
- Identifying allocation hotspots in a naive implementation (e.g. a per-read `to_string()` inside the inner loop) and removing them
- Parallelising per-read GC content or per-read length statistics over a FASTQ with `rayon::par_iter`, including a brief discussion of why this particular computation is safe to parallelise (no shared mutable state per read)
- A short tour of real Rust bioinformatics projects so students know where to look next: `noodles` (htslib-style I/O), `rust-bio`, `rust-htslib`, `varlociraptor` (variant calling), `alevin-fry` (single-cell RNA-seq), `nanoq` and `chopper` (long-read QC) — and a note that `pyo3` is how many of these expose Python bindings

**Lecture topics (suggested):** (1) Testing and `--release` — the two switches that matter most; (2) Parallelism with `rayon` and a tour of the wider ecosystem.

## Explicit non-outcomes

To keep the course honest and the cognitive load survivable, the following are **not** expected outcomes:

- Writing explicit lifetime annotations beyond what is needed to read elided ones
- Designing trait hierarchies or writing generic library APIs
- Using `async`/`await` or any async runtime
- Writing macros (using `println!`, `vec!`, `assert_eq!` is fine; defining `macro_rules!` is not)
- Writing or reasoning about `unsafe` code
- Foreign function interface (FFI) to C or Python
- Building Python extension modules with `pyo3` / `maturin`
- WebAssembly, embedded Rust, or GPU programming
- Deep knowledge of any one bioinformatics crate's full API — students will know how to *find* what they need, not memorise it

Students who want to pursue these topics will be pointed to follow-up resources on day 5.

## Assessment (informal)

Each day's self-paced material ends with a short hands-on exercise tied to that day's outcomes. The week culminates in a small capstone task on day 5 that touches outcomes from every preceding day: parse a bioinformatics file, compute something useful per record, optionally in parallel, with tests and a clean CLI. Success on the capstone is the working definition of "passed the course."
