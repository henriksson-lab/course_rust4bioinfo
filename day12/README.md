# Day 12 — GPU compute kernels with wgpu

Day 11 used [Burn](https://burn.dev/) — a high-level tensor framework that hides the GPU behind one familiar interface. Day 12 goes one level lower: we write **compute shaders by hand in WGSL** [WebGPU Shading Language — the cross-vendor shader language used by wgpu] and dispatch them with [`wgpu`](https://docs.rs/wgpu/) [the pure-Rust implementation of the WebGPU standard; runs on Vulkan, Metal, DX12, and the browser]. Same hardware, manual control — the *how* behind Burn's *what*.

Several of today's examples revisit problems from earlier days (GC content, reverse complement, pairwise distances) and re-implement them as GPU kernels. Comparing the two implementations is the point.

Start with [**Concept summary**](00-concepts.qmd) for the reference overview, and the [**Quiz**](00a-quiz.qmd) once you've watched the lecture.

| # | Topic | wgpu / WGSL concepts | Crate |
|---|---|---|---|
| 0 | [**Concept summary**](00-concepts.qmd) | adapter, device, queue, buffer, pipeline, bind group, WGSL | — |
| — | [**Quiz**](00a-quiz.qmd) | dispatch sizes, workgroup memory, buffer types | — |
| 1 | Vector add — your first kernel | the full 6-step pipeline; `@workgroup_size` | [`ex-vector-add/`](ex-vector-add/) |
| 2 | SAXPY — scalars via uniforms | uniform buffer, scalar broadcast | [`ex-saxpy/`](ex-saxpy/) |
| 3 | GC content on the GPU (revisit Day 1 / 5) | byte storage buffer; atomic counter | [`ex-gc-content/`](ex-gc-content/) |
| 4 | Reverse complement on the GPU (revisit Day 2) | byte mapping; in-place vs. swap-array | [`ex-revcomp/`](ex-revcomp/) |
| 5 | Parallel reduction (sum) | workgroup shared memory; tree reduction | [`ex-reduction/`](ex-reduction/) |
| 6 | Pairwise distance matrix (revisit Day 11 ex 4) | 2-D dispatch; output indexing | [`ex-pairwise-distance/`](ex-pairwise-distance/) |

## Working pattern

For each exercise:

```bash
cd day12/ex-<name>
cargo test           # runs the kernel and checks the result against a CPU reference
cargo run --release  # the same thing, with timing output
```

The first `cargo build` will download and compile `wgpu` and its backend (Vulkan/Metal/DX12 driver), which takes a few minutes. After that, incremental rebuilds are seconds.

The `ex-<name>-solution/` directories contain working reference implementations. Try not to peek before your own version passes the tests.

## Hardware notes

- **NVIDIA / AMD / Intel discrete GPU** — wgpu uses the Vulkan backend (Linux/Windows) or DX12 (Windows).
- **Apple Silicon / Intel Mac** — wgpu uses Metal.
- **No discrete GPU** — wgpu can fall back to a software adapter (very slow) or you can run on a CPU backend via `wgpu`'s `Backends::PRIMARY | Backends::SECONDARY`.

You do not need to think about which backend you have — that is the whole point of wgpu.
