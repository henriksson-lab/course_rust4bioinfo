# Day 2 — Exercises

Ownership, borrowing, and the difference between owning data and viewing it.

Start with [**00 — Concepts**](00-concepts.qmd) for a reference overview of the day's Rust features (companion to the slides).

| # | Topic | Rust concepts | Crate |
|---|---|---|---|
| 0 | [**Concepts**](00-concepts.qmd) | ownership, references, `Vec`, slices, lifetime elision | — |
| 1 | [Reverse complement](01-reverse-complement.qmd) | `Vec<u8>`, owned return, `Vec::with_capacity`, indexed reverse loop | [`ex-reverse-complement/`](ex-reverse-complement/) |
| 2 | [Reverse complement into a buffer](02-revcomp-buf.qmd) | `&mut Vec<u8>`, `clear` + `reserve`, allocation reuse | [`ex-revcomp-buf/`](ex-revcomp-buf/) |
| 3 | [k-mers as slice views](03-kmers.qmd) | `Vec<&[u8]>`, slice borrowing, lifetime elision | [`ex-kmers/`](ex-kmers/) |
| 4 | [Quality trimming as a slice view](04-quality-trim.qmd) | returning `&[u8]`, `while` loops, slice indexing | [`ex-quality-trim/`](ex-quality-trim/) |
