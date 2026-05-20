//! Day 11 — Exercise 3: GC content over a batch of sequences.

use burn::backend::NdArray;
use burn::tensor::{Device, Int, Tensor, TensorData, backend::Backend};

fn seq_to_ints(seq: &str) -> Vec<i64> {
    seq.chars()
        .map(|c| match c {
            'A' | 'a' => 0,
            'C' | 'c' => 1,
            'G' | 'g' => 2,
            'T' | 't' => 3,
            other => panic!("unknown base {other:?}"),
        })
        .collect()
}

fn build_batch<B: Backend>(seqs: &[&str], device: &Device<B>) -> Tensor<B, 2, Int> {
    let n = seqs.len();
    let l = seqs[0].len();
    assert!(seqs.iter().all(|s| s.len() == l));
    let flat: Vec<i64> = seqs.iter().flat_map(|s| seq_to_ints(s)).collect();
    Tensor::<B, 1, Int>::from_data(TensorData::new(flat, [n * l]), device).reshape([n, l])
}

/// GC fraction of each sequence in a `[N, L]` integer batch.
/// Output shape: `[N]`, each entry in `[0.0, 1.0]`.
fn gc_content<B: Backend>(seqs: Tensor<B, 2, Int>) -> Tensor<B, 1> {
    // TODO:
    //   1. is_c = seqs == 1, as an int tensor
    //   2. is_g = seqs == 2, as an int tensor
    //   3. is_gc = (is_c + is_g).float()        // [N, L] of 0.0 or 1.0
    //   4. gc = is_gc.mean_dim(1).squeeze_dim(1)    // [N]
    let _ = seqs;
    todo!("Hints in 03-gc-content.qmd")
}

fn main() {
    type B = NdArray;
    let device = Default::default();
    let batch = build_batch::<B>(&["AAAA", "ACGT", "GCGC"], &device);
    println!("GC content: {}", gc_content(batch));
}

#[cfg(test)]
mod tests {
    use super::*;
    type B = NdArray;
    fn device() -> Device<B> { Default::default() }

    fn approx_eq(a: &[f32], b: &[f32], tol: f32) -> bool {
        a.len() == b.len() && a.iter().zip(b).all(|(x, y)| (x - y).abs() < tol)
    }

    #[test]
    fn shape_is_n() {
        let batch = build_batch::<B>(&["AAAA", "ACGT", "GCGC"], &device());
        let gc = gc_content(batch);
        assert_eq!(gc.dims(), [3]);
    }

    #[test]
    fn all_at_basics() {
        let batch = build_batch::<B>(&["AAAA"], &device());
        let gc: Vec<f32> = gc_content(batch).to_data().to_vec().unwrap();
        assert!(approx_eq(&gc, &[0.0], 1e-6));
    }

    #[test]
    fn all_gc() {
        let batch = build_batch::<B>(&["GCGCGC"], &device());
        let gc: Vec<f32> = gc_content(batch).to_data().to_vec().unwrap();
        assert!(approx_eq(&gc, &[1.0], 1e-6));
    }

    #[test]
    fn half_and_half() {
        // "ACGT": 1 G + 1 C of 4 bases = 0.5
        let batch = build_batch::<B>(&["ACGT"], &device());
        let gc: Vec<f32> = gc_content(batch).to_data().to_vec().unwrap();
        assert!(approx_eq(&gc, &[0.5], 1e-6));
    }

    #[test]
    fn mixed_batch() {
        let batch = build_batch::<B>(&["AAAA", "ACGT", "GCGC"], &device());
        let gc: Vec<f32> = gc_content(batch).to_data().to_vec().unwrap();
        assert!(approx_eq(&gc, &[0.0, 0.5, 1.0], 1e-6));
    }
}
