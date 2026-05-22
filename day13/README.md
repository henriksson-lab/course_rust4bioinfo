# Day 13 — Bits and 2-bit DNA encoding

Number systems, bitwise operations, and barcode analysis.

Start with [**00 — Concepts**](00-concepts.qmd) for a reference overview of binary, hex, bitwise operators, popcount, and 2-bit DNA encoding (companion to the slides). Day 13 takes a step down from the high-level numerical tooling of days 10-12 and looks at how the machine actually represents numbers — and how that representation, applied to DNA, makes single-cell barcode matching and k-mer hashing fast.

| # | Topic | Concepts | Crate |
|---|---|---|---|
| 0 | [**Concepts**](00-concepts.qmd) | bits, bases, bitwise operators, popcount, 2-bit DNA | — |
| 1 | Numbers and bits | binary/decimal/hex, AND/OR/XOR/NOT, shifts, formatting | [`ex-numbers-and-bits/`](ex-numbers-and-bits/) |
| 2 | Pack DNA into 2 bits per base | bit packing, indexing into packed data | [`ex-pack-dna-2bit/`](ex-pack-dna-2bit/) |
| 3 | Hamming distance via XOR + popcount | bitwise XOR, `u64::count_ones`, masking | [`ex-hamming-popcount/`](ex-hamming-popcount/) |
| 4 | Reverse complement on 2-bit DNA | bit reversal, XOR-complement | [`ex-revcomp-2bit/`](ex-revcomp-2bit/) |
| 5 | Barcode whitelist matching | linear scan, Hamming distance, single-cell context | [`ex-barcode-whitelist/`](ex-barcode-whitelist/) |

## Working pattern

For each exercise:

```bash
cd day13/ex-<name>
cargo test           # see what is expected — some tests will fail at first
# edit src/lib.rs (or src/main.rs) until cargo test reports: 0 failed
```

The `ex-<name>-solution/` directories contain working reference implementations. Try not to peek before you have your own version passing the tests.
