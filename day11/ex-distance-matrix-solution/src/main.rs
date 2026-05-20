//! Day 11 — Exercise 4: pairwise Euclidean distance matrix. Reference solution.

use burn::backend::NdArray;
use burn::tensor::{Device, Tensor, backend::Backend};

fn pairwise_l2<B: Backend>(x: Tensor<B, 2>) -> Tensor<B, 2> {
    let a = x.clone().unsqueeze_dim::<3>(1);           // [N, 1, D]
    let b = x.unsqueeze_dim::<3>(0);                   // [1, N, D]
    let diff = a - b;                                  // [N, N, D]
    diff.powf_scalar(2.0).sum_dim(2).sqrt().squeeze_dim(2) // [N, N]
}

fn sample_points<B: Backend>(device: &Device<B>) -> Tensor<B, 2> {
    Tensor::<B, 2>::from_floats(
        [[0.0,  0.0],
         [3.0,  4.0],
         [6.0,  0.0],
         [3.0, -4.0]],
        device,
    )
}

fn main() {
    type B = NdArray;
    let device = Default::default();
    let x = sample_points::<B>(&device);
    println!("pairwise L2 distances:\n{}", pairwise_l2(x));
}

#[cfg(test)]
mod tests {
    use super::*;
    type B = NdArray;
    fn device() -> Device<B> { Default::default() }
    fn data(t: Tensor<B, 2>) -> Vec<f32> { t.to_data().to_vec().unwrap() }

    #[test]
    fn output_shape_nn() {
        assert_eq!(pairwise_l2(sample_points::<B>(&device())).dims(), [4, 4]);
    }

    #[test]
    fn diagonal_zero() {
        let d = data(pairwise_l2(sample_points::<B>(&device())));
        for i in 0..4 { assert!(d[i * 4 + i].abs() < 1e-4); }
    }

    #[test]
    fn symmetric() {
        let d = data(pairwise_l2(sample_points::<B>(&device())));
        for i in 0..4 { for j in 0..4 {
            assert!((d[i * 4 + j] - d[j * 4 + i]).abs() < 1e-4);
        }}
    }

    #[test]
    fn classic_distances() {
        let d = data(pairwise_l2(sample_points::<B>(&device())));
        assert!((d[0 * 4 + 1] - 5.0).abs() < 1e-4);
        assert!((d[0 * 4 + 2] - 6.0).abs() < 1e-4);
        assert!((d[0 * 4 + 3] - 5.0).abs() < 1e-4);
        assert!((d[1 * 4 + 2] - 5.0).abs() < 1e-4);
        assert!((d[1 * 4 + 3] - 8.0).abs() < 1e-4);
    }

    #[test]
    fn single_point() {
        let device = device();
        let x = Tensor::<B, 2>::from_floats([[1.0, 2.0, 3.0]], &device);
        let d = pairwise_l2(x);
        assert_eq!(d.dims(), [1, 1]);
        let v: Vec<f32> = d.to_data().to_vec().unwrap();
        assert!(v[0].abs() < 1e-5);
    }
}
