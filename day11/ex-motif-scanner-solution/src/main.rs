//! Day 11 — Exercise 6: motif scanner. Reference solution.

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

fn build_one_hot<B: Backend>(seqs: &[&str], device: &Device<B>) -> Tensor<B, 3> {
    let n = seqs.len();
    let l = seqs[0].len();
    let flat: Vec<i64> = seqs.iter().flat_map(|s| seq_to_ints(s)).collect();
    let ints: Tensor<B, 2, Int> =
        Tensor::<B, 1, Int>::from_data(TensorData::new(flat, [n * l]), device).reshape([n, l]);
    ints.one_hot(4).float().swap_dims(1, 2)
}

fn gata_pwm<B: Backend>(device: &Device<B>) -> Tensor<B, 3> {
    let data: [[[f32; 4]; 4]; 1] = [[
        [0.0, 1.0, 0.0, 1.0],
        [0.0, 0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
    ]];
    Tensor::<B, 3>::from_floats(data, device)
}

fn scan_motifs<B: Backend>(one_hot: Tensor<B, 3>, pwm: Tensor<B, 3>) -> Tensor<B, 3> {
    use burn::tensor::module::conv1d;
    use burn::tensor::ops::ConvOptions;
    conv1d(one_hot, pwm, None, ConvOptions::new([1], [0], [1], 1))
}

fn main() {
    type B = NdArray;
    let device = Default::default();
    let seqs = ["AAAAAAAAAAAAAAAA", "ACGTGATATGCAACGT", "CCCCCCCCCCCCCCCC"];
    let oh = build_one_hot::<B>(&seqs, &device);
    let pwm = gata_pwm::<B>(&device);
    let scores = scan_motifs(oh, pwm);
    println!("scores ({:?}):\n{}", scores.dims(), scores);
}

#[cfg(test)]
mod tests {
    use super::*;
    type B = NdArray;
    fn device() -> Device<B> { Default::default() }

    #[test]
    fn output_shape() {
        let oh = build_one_hot::<B>(&["AAAAAAAAAAAAAAAA"], &device());
        let pwm = gata_pwm::<B>(&device());
        assert_eq!(scan_motifs(oh, pwm).dims(), [1, 1, 13]);
    }

    #[test]
    fn perfect_match_scores_4() {
        let oh = build_one_hot::<B>(&["GATA"], &device());
        let pwm = gata_pwm::<B>(&device());
        let v: Vec<f32> = scan_motifs(oh, pwm).to_data().to_vec().unwrap();
        assert!((v[0] - 4.0).abs() < 1e-5);
    }

    #[test]
    fn finds_motif_in_middle_of_sequence() {
        let seq = "ACGTGATATGCAACGT";
        let oh = build_one_hot::<B>(&[seq], &device());
        let pwm = gata_pwm::<B>(&device());
        let v: Vec<f32> = scan_motifs(oh, pwm).to_data().to_vec().unwrap();
        let (best_i, best_v) = v.iter().enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap()).unwrap();
        assert!((best_v - 4.0).abs() < 1e-5);
        assert_eq!(best_i, 4);
    }

    #[test]
    fn no_match_low_score() {
        let oh = build_one_hot::<B>(&["CCCCCCCCCCCCCCCC"], &device());
        let pwm = gata_pwm::<B>(&device());
        let v: Vec<f32> = scan_motifs(oh, pwm).to_data().to_vec().unwrap();
        let max = v.iter().cloned().fold(f32::MIN, f32::max);
        assert!(max < 1.0);
    }
}
