# Day 3 — Exercises

Structs, enums, iterators, `Option`/`Result`, and recursion — the toolkit for modelling biological data well.

Start with [**00 — Concepts**](00-concepts.qmd) for a reference overview of the day's Rust features (companion to the slides).

| # | Topic | Rust concepts | Crate |
|---|---|---|---|
| 0 | [**Concepts**](00-concepts.qmd) | structs, enums, `Option`, `Result`, iterators, recursion | — |
| 1 | [Genomic interval](01-genomic-interval.qmd) | `struct`, `enum`, `impl` methods, `#[derive]` | [`ex-genomic-interval/`](ex-genomic-interval/) |
| 2 | [Parsing a region string](02-parse-region.qmd) | custom error `enum`, `Result`, `?`, `split_once`, `parse` | [`ex-parse-region/`](ex-parse-region/) |
| 3 | [Codon table](03-codon-table.qmd) | `Option<u8>`, `match` on `[u8; 3]`, `|` alternatives, `to_ascii_uppercase` | [`ex-codon-table/`](ex-codon-table/) |
| 4 | [k-mer counts](04-kmer-counts.qmd) | `HashMap`, the entry API, `<[T]>::windows` iterator | [`ex-kmer-counts/`](ex-kmer-counts/) |
| 5 | [Phylogenetic tree](05-phylo-tree.qmd) | recursive `enum`, recursive functions, pattern matching with named fields | [`ex-phylo-tree/`](ex-phylo-tree/) |
