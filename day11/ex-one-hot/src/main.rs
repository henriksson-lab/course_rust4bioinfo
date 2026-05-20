//! Day 11 — Exercise 2: one-hot encode DNA.

use burn::backend::NdArray;
use burn::tensor::{Device, Int, Tensor, TensorData, backend::Backend};

/// A=0, C=1, G=2, T=3. Other characters panic. Already implemented for you.
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

/// One-hot encode a batch of integer-encoded DNA sequences.
///
/// Input shape:  [N, L]   integer tensor (A=0, C=1, G=2, T=3)
/// Output shape: [N, 4, L] float tensor (1.0 in the row of the base, 0.0 elsewhere)
fn one_hot_dna<B: Backend>(seqs: Tensor<B, 2, Int>) -> Tensor<B, 3> {
    // TODO: see hints 1-3 in 02-one-hot.qmd.
    //   1. call `.one_hot(4)` — adds a length-4 trailing axis
    //   2. call `.float()` — cast Int to Float
    //   3. call `.swap_dims(1, 2)` — put the channel axis in the middle
    let _ = seqs;
    todo!("Hints in 02-one-hot.qmd")
}

fn build_batch<B: Backend>(seqs: &[&str], device: &Device<B>) -> Tensor<B, 2, Int> {
    let n = seqs.len();
    let l = seqs[0].len();
    assert!(seqs.iter().all(|s| s.len() == l), "all sequences must have the same length");
    let flat: Vec<i64> = seqs.iter().flat_map(|s| seq_to_ints(s)).collect();
    Tensor::<B, 1, Int>::from_data(TensorData::new(flat, [n * l]), device).reshape([n, l])
}

fn main() {
    type B = NdArray;
    let device = Default::default();

    let seqs = ["ACGTA", "TTTGC", "GGGGG"];
    let batch = build_batch::<B>(&seqs, &device);
    println!("integer batch ({:?}):\n{}", batch.dims(), batch);

    let oh = one_hot_dna(batch);
    println!("one-hot ({:?}):\n{}", oh.dims(), oh);
}

#[cfg(test)]
mod tests {
    use super::*;

    type B = NdArray;
    fn device() -> Device<B> { Default::default() }

    #[test]
    fn shape_n4l() {
        let batch = build_batch::<B>(&["ACGTA", "TTTGC", "GGGGG"], &device());
        let oh = one_hot_dna(batch);
        assert_eq!(oh.dims(), [3, 4, 5]);
    }

    #[test]
    fn one_per_position() {
        // Each position should have exactly one 1 across the 4 channels.
        let batch = build_batch::<B>(&["ACGTA"], &device());
        let oh = one_hot_dna(batch);
        let per_pos: Vec<f32> = oh.sum_dim(1).to_data().to_vec().unwrap();
        // sum_dim(1) collapses channels to size 1 → shape [1, 1, 5] = 5 entries all 1.0.
        assert!(per_pos.iter().all(|v| (v - 1.0).abs() < 1e-6));
    }

    #[test]
    fn correct_channels() {
        // For sequence "ACGT" (length 4):
        //   position 0 → A → channel 0 hot
        //   position 1 → C → channel 1 hot
        //   position 2 → G → channel 2 hot
        //   position 3 → T → channel 3 hot
        let batch = build_batch::<B>(&["ACGT"], &device());
        let oh = one_hot_dna(batch);
        let data: Vec<f32> = oh.to_data().to_vec().unwrap();
        // Shape is [1, 4, 4]. Layout is contiguous [n, c, l]:
        //   data[c * 4 + l] for n = 0
        // The 1s should sit at (c, l) = (0, 0), (1, 1), (2, 2), (3, 3).
        for c in 0..4 {
            for l in 0..4 {
                let expected = if c == l { 1.0 } else { 0.0 };
                let got = data[c * 4 + l];
                assert!((got - expected).abs() < 1e-6,
                        "channel {c} pos {l}: expected {expected}, got {got}");
            }
        }
    }

    #[test]
    fn sums_to_total_bases() {
        // Sum of all entries in a [N, 4, L] one-hot is N*L (one hot per position).
        let batch = build_batch::<B>(&["ACGTA", "TTTGC", "GGGGG"], &device());
        let oh = one_hot_dna(batch);
        let total: Vec<f32> = oh.sum().to_data().to_vec().unwrap();
        assert!((total[0] - 15.0).abs() < 1e-5);   // 3 sequences × 5 bases
    }
}
