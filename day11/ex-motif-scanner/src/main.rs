//! Day 11 — Exercise 6: 1-D conv motif scanner.

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
    ints.one_hot(4).float().swap_dims(1, 2)            // [N, 4, L]
}

/// Build a single PWM (motif `GATA`) of shape [1, 4, 4].
fn gata_pwm<B: Backend>(device: &Device<B>) -> Tensor<B, 3> {
    // Bases: A=0, C=1, G=2, T=3.
    // Positions:  0=G,   1=A,   2=T,   3=A
    // So weight  A[1]=A[3]=1, G[0]=1, T[2]=1, everything else 0.
    let data: [[[f32; 4]; 4]; 1] = [[
        [0.0, 1.0, 0.0, 1.0],   // A row
        [0.0, 0.0, 0.0, 0.0],   // C row
        [1.0, 0.0, 0.0, 0.0],   // G row
        [0.0, 0.0, 1.0, 0.0],   // T row
    ]];
    Tensor::<B, 3>::from_floats(data, device)
}

/// Scan PWMs across one-hot DNA via 1-D convolution.
///
/// `one_hot` shape: [N, 4, L]
/// `pwm`     shape: [M, 4, w]
/// returns          [N, M, L - w + 1]
fn scan_motifs<B: Backend>(
    one_hot: Tensor<B, 3>,
    pwm: Tensor<B, 3>,
) -> Tensor<B, 3> {
    // TODO: call burn::tensor::module::conv1d with stride=[1], padding=[0],
    // dilation=[1], groups=1.
    let _ = (one_hot, pwm);
    todo!("Hint 1 in 06-motif-scanner.qmd")
}

fn main() {
    type B = NdArray;
    let device = Default::default();

    // 16-bp sequences; the 2nd contains "GATA" starting at position 4.
    let seqs = ["AAAAAAAAAAAAAAAA", "ACGTGATATGCAACGT", "CCCCCCCCCCCCCCCC"];
    let oh = build_one_hot::<B>(&seqs, &device);
    let pwm = gata_pwm::<B>(&device);
    let scores = scan_motifs(oh, pwm);          // [3, 1, 13]
    println!("scores ({:?}):\n{}", scores.dims(), scores);
}

#[cfg(test)]
mod tests {
    use super::*;

    type B = NdArray;
    fn device() -> Device<B> { Default::default() }

    #[test]
    fn output_shape() {
        let seqs = ["AAAAAAAAAAAAAAAA"];     // L = 16, w = 4 → out length 13
        let oh = build_one_hot::<B>(&seqs, &device());
        let pwm = gata_pwm::<B>(&device());
        let s = scan_motifs(oh, pwm);
        assert_eq!(s.dims(), [1, 1, 13]);
    }

    #[test]
    fn perfect_match_scores_4() {
        // "GATA" alone, perfectly matches the PWM at position 0.
        let seqs = ["GATA"];                                // L = 4
        let oh = build_one_hot::<B>(&seqs, &device());      // [1, 4, 4]
        let pwm = gata_pwm::<B>(&device());                 // [1, 4, 4]
        let s = scan_motifs(oh, pwm);                       // [1, 1, 1]
        let v: Vec<f32> = s.to_data().to_vec().unwrap();
        assert!((v[0] - 4.0).abs() < 1e-5);
    }

    #[test]
    fn finds_motif_in_middle_of_sequence() {
        // The motif starts at position 4 in this sequence.
        let seq = "ACGTGATATGCAACGT";                       // L = 16
        let oh = build_one_hot::<B>(&[seq], &device());
        let pwm = gata_pwm::<B>(&device());
        let s = scan_motifs(oh, pwm);                       // [1, 1, 13]
        let v: Vec<f32> = s.to_data().to_vec().unwrap();
        // The max score should be 4.0 (perfect match), at position 4.
        let (best_i, best_v) = v.iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .unwrap();
        assert!((best_v - 4.0).abs() < 1e-5, "expected score 4 at the motif, got {}", best_v);
        assert_eq!(best_i, 4, "expected motif at position 4, found at {}", best_i);
    }

    #[test]
    fn no_match_low_score() {
        // No "GATA" anywhere → maximum score should be at most 2 (rare partial matches).
        let seq = "CCCCCCCCCCCCCCCC";
        let oh = build_one_hot::<B>(&[seq], &device());
        let pwm = gata_pwm::<B>(&device());
        let s = scan_motifs(oh, pwm);
        let v: Vec<f32> = s.to_data().to_vec().unwrap();
        let max = v.iter().cloned().fold(f32::MIN, f32::max);
        assert!(max < 1.0, "unexpected high score in all-C sequence: {}", max);
    }
}
