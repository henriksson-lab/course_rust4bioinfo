# Day 11 — GPUs and tensors with Burn

One day on how to use the GPU from Rust. We use [**Burn**](https://burn.dev/) — a PyTorch-style tensor framework that compiles to several GPU backends (CUDA, wgpu, Metal) and a CPU fallback. Every exercise pairs the Rust code with the equivalent PyTorch / NumPy line so you can read the new syntax against something familiar.

Start with [**00 — Concepts**](00-concepts.qmd) for the reference overview (companion to the slides). Day 11 builds on the matrix vocabulary from day 10 (`ndarray` for dense data, matrix-vector multiplication as the workhorse) and the trait vocabulary from day 8 (`impl Trait for Type`, generic functions with bounds). The new ingredient is the **tensor**: an N-dimensional array that lives either in CPU RAM or in GPU memory, with the same code working on both.

| # | Topic | Rust concepts | Crate |
|---|---|---|---|
| 0 | [**Concepts**](00-concepts.qmd) | tensors, devices, backends, autodiff | — |
| 1 | Tensors on any backend | `Tensor<B, D>`, `B: Backend` trait bound | [`ex-tensors/`](ex-tensors/) |
| 2 | One-hot encoding DNA | `Tensor::arange`, `.one_hot(...)`, integer tensors | [`ex-one-hot/`](ex-one-hot/) |
| 3 | GC content across a batch | reductions, boolean masking, `.mean_dim` | [`ex-gc-content/`](ex-gc-content/) |
| 4 | Pairwise distance matrix | broadcasting, `.unsqueeze`, generic `B: Backend` | [`ex-distance-matrix/`](ex-distance-matrix/) |
| 5 | PCA on an expression matrix | `nalgebra::DMatrix`, SVD | [`ex-pca/`](ex-pca/) |
| 6 | 1D conv motif scanner | `Conv1d`, PWM as a weight tensor | [`ex-motif-scanner/`](ex-motif-scanner/) |
| 7 | Train a promoter classifier | `Module`, autodiff, `Adam` | [`ex-promoter-classifier/`](ex-promoter-classifier/) |

## Working pattern

For each exercise:

```bash
cd day11/ex-<name>
cargo test           # see what is expected — some tests will fail at first
# edit src/lib.rs (or src/main.rs) until cargo test reports: 0 failed
```

The first build downloads Burn and its backends and takes several minutes. After that, incremental builds are fast. By default we use the `ndarray` backend (CPU, no driver needed) so everything runs on a laptop. Where a `wgpu` backend feature flag exists, you can opt in with:

```bash
cargo test --features gpu
```

The `ex-<name>-solution/` directories contain working reference implementations. Try not to peek before you have your own version passing the tests.

## Note on backends

Burn is built around a `Backend` trait. Every exercise in this directory writes its tensor functions as `fn op<B: Backend>(x: Tensor<B, D>) -> ...` — the function is generic over the backend, so the same code runs on CPU (`NdArray`) and GPU (`Wgpu`, `Cuda`, `Metal`). Exercise 1 makes this explicit; later exercises use the same pattern without remarking on it.
