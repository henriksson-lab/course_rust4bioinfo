//! Day 11 — Exercise 2: one-hot encode DNA. Reference solution.

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

fn one_hot_dna<B: Backend>(seqs: Tensor<B, 2, Int>) -> Tensor<B, 3> {
    seqs.one_hot(4).float().swap_dims(1, 2)        // [N, L, 4] → [N, 4, L]
}

fn build_batch<B: Backend>(seqs: &[&str], device: &Device<B>) -> Tensor<B, 2, Int> {
    let n = seqs.len();
    let l = seqs[0].len();
    assert!(seqs.iter().all(|s| s.len() == l));
    let flat: Vec<i64> = seqs.iter().flat_map(|s| seq_to_ints(s)).collect();
    Tensor::<B, 1, Int>::from_data(TensorData::new(flat, [n * l]), device).reshape([n, l])
}

fn main() {
    type B = NdArray;
    let device = Default::default();
    let seqs = ["ACGTA", "TTTGC", "GGGGG"];
    let batch = build_batch::<B>(&seqs, &device);
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
        let oh = one_hot_dna(build_batch::<B>(&["ACGTA", "TTTGC", "GGGGG"], &device()));
        assert_eq!(oh.dims(), [3, 4, 5]);
    }

    #[test]
    fn one_per_position() {
        let oh = one_hot_dna(build_batch::<B>(&["ACGTA"], &device()));
        let per_pos: Vec<f32> = oh.sum_dim(1).to_data().to_vec().unwrap();
        assert!(per_pos.iter().all(|v| (v - 1.0).abs() < 1e-6));
    }

    #[test]
    fn correct_channels() {
        let oh = one_hot_dna(build_batch::<B>(&["ACGT"], &device()));
        let data: Vec<f32> = oh.to_data().to_vec().unwrap();
        for c in 0..4 {
            for l in 0..4 {
                let expected = if c == l { 1.0 } else { 0.0 };
                let got = data[c * 4 + l];
                assert!((got - expected).abs() < 1e-6);
            }
        }
    }

    #[test]
    fn sums_to_total_bases() {
        let oh = one_hot_dna(build_batch::<B>(&["ACGTA", "TTTGC", "GGGGG"], &device()));
        let total: Vec<f32> = oh.sum().to_data().to_vec().unwrap();
        assert!((total[0] - 15.0).abs() < 1e-5);
    }
}
