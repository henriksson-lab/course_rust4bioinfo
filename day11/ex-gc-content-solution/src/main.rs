//! Day 11 — Exercise 3: GC content. Reference solution.

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

fn gc_content<B: Backend>(seqs: Tensor<B, 2, Int>) -> Tensor<B, 1> {
    let is_c = seqs.clone().equal_elem(1).int();
    let is_g = seqs.equal_elem(2).int();
    let is_gc = (is_c + is_g).float();
    is_gc.mean_dim(1).squeeze_dim(1)
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
        assert_eq!(gc_content(batch).dims(), [3]);
    }

    #[test]
    fn all_at() {
        let v: Vec<f32> = gc_content(build_batch::<B>(&["AAAA"], &device())).to_data().to_vec().unwrap();
        assert!(approx_eq(&v, &[0.0], 1e-6));
    }

    #[test]
    fn all_gc() {
        let v: Vec<f32> = gc_content(build_batch::<B>(&["GCGCGC"], &device())).to_data().to_vec().unwrap();
        assert!(approx_eq(&v, &[1.0], 1e-6));
    }

    #[test]
    fn half_and_half() {
        let v: Vec<f32> = gc_content(build_batch::<B>(&["ACGT"], &device())).to_data().to_vec().unwrap();
        assert!(approx_eq(&v, &[0.5], 1e-6));
    }

    #[test]
    fn mixed_batch() {
        let v: Vec<f32> = gc_content(build_batch::<B>(&["AAAA", "ACGT", "GCGC"], &device())).to_data().to_vec().unwrap();
        assert!(approx_eq(&v, &[0.0, 0.5, 1.0], 1e-6));
    }
}
