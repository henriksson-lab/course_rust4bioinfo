# Day 10 — Exercises

Matrices, dense and sparse.

Start with [**00 — Concepts**](00-concepts.qmd) for a reference overview of matrices, sparse storage formats, and iterative solvers (companion to the slides). Day 10 builds on the data-structure vocabulary from day 9: a matrix is just a 2-D array of numbers, but how you *store* it changes what is fast and what is slow.

| # | Topic | Rust concepts | Crate |
|---|---|---|---|
| 0 | [**Concepts**](00-concepts.qmd) | matrices, dense vs sparse, COO/CSR/CSC, iterative solvers | — |
| 1 | Loading and operating on a dense matrix | `ndarray::Array2`, slicing, matrix-vector multiply | [`ex-dense-matrix/`](ex-dense-matrix/) |
| 2 | Sparse representations — memory comparison | `sprs::CsMat`, triplet → CSR conversion | [`ex-sparse-matrix/`](ex-sparse-matrix/) |
| 3 | Implement Gauss-Seidel | iterative linear-system solver, in-place updates | [`ex-gauss-seidel/`](ex-gauss-seidel/) |
| 4 | Markov chain — power iteration vs steady state | matrix-vector multiplication in a loop, eigenvector | [`ex-markov-chain/`](ex-markov-chain/) |

## Working pattern

For each exercise:

```bash
cd day10/ex-<name>
cargo test           # see what is expected — some tests will fail at first
# edit src/lib.rs (or src/main.rs) until cargo test reports: 0 failed
```

The `ex-<name>-solution/` directories contain working reference implementations. Try not to peek before you have your own version passing the tests.
