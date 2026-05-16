# Day 4 — Exercises

Real bioinformatics I/O: streaming records from real file formats with [`noodles`](https://docs.rs/noodles/), plotting with [`plotters`](https://docs.rs/plotters/), and bundling outputs with [`zip`](https://docs.rs/zip/).

Start with [**00 — Concepts**](00-concepts.qmd) for a reference overview of the day's Rust features (companion to the slides).

| # | Topic | Crate | Path |
|---|---|---|---|
| 0 | [**Concepts**](00-concepts.qmd) | cargo dependencies, `std::io` traits, generics, modules | — |
| 1 | [Parsing BED by hand](01-bed-parse.qmd) | std only | [`ex-bed-parse/`](ex-bed-parse/) |
| 2 | [Per-contig stats with `noodles-fasta`](02-fasta-stats.qmd) | `noodles-fasta` | [`ex-fasta-stats/`](ex-fasta-stats/) |
| 3 | [Filter FASTQ](03-fastq-filter.qmd) | `noodles-fastq` | [`ex-fastq-filter/`](ex-fastq-filter/) |
| 4 | [Read-length histogram](04-plot-readlen.qmd) | `plotters` | [`ex-plot-readlen/`](ex-plot-readlen/) |
| 5 | [Zip bundle](05-zip-bundle.qmd) | `zip` | [`ex-zip-bundle/`](ex-zip-bundle/) |
