//! Day 11 — Exercise 4: pairwise Euclidean distance matrix.

use burn::backend::NdArray;
use burn::tensor::{Device, Tensor, backend::Backend};

/// Compute the [N, N] matrix of pairwise Euclidean distances between rows of `x`.
///
/// Input shape:  [N, D]
/// Output shape: [N, N]
fn pairwise_l2<B: Backend>(x: Tensor<B, 2>) -> Tensor<B, 2> {
    // TODO: see hints 1-4 in 04-distance-matrix.qmd.
    //   1. a = x.unsqueeze_dim::<3>(1)     -> [N, 1, D]
    //   2. b = x.unsqueeze_dim::<3>(0)     -> [1, N, D]
    //   3. diff = a - b                    -> [N, N, D]   (broadcast)
    //   4. sq_sum = (diff^2).sum_dim(2)    -> [N, N, 1]
    //   5. return sq_sum.sqrt().squeeze_dim(2) -> [N, N]
    let _ = x;
    todo!("Hints in 04-distance-matrix.qmd")
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
    println!("points:\n{}", x);
    let d = pairwise_l2(x);
    println!("pairwise L2 distances:\n{}", d);
}

#[cfg(test)]
mod tests {
    use super::*;
    type B = NdArray;
    fn device() -> Device<B> { Default::default() }

    fn data(t: Tensor<B, 2>) -> Vec<f32> {
        t.to_data().to_vec().unwrap()
    }

    #[test]
    fn output_shape_nn() {
        let d = pairwise_l2(sample_points::<B>(&device()));
        assert_eq!(d.dims(), [4, 4]);
    }

    #[test]
    fn diagonal_zero() {
        let d = data(pairwise_l2(sample_points::<B>(&device())));
        for i in 0..4 {
            assert!((d[i * 4 + i]).abs() < 1e-4, "D[{i},{i}] = {}", d[i * 4 + i]);
        }
    }

    #[test]
    fn symmetric() {
        let d = data(pairwise_l2(sample_points::<B>(&device())));
        for i in 0..4 {
            for j in 0..4 {
                let dij = d[i * 4 + j];
                let dji = d[j * 4 + i];
                assert!((dij - dji).abs() < 1e-4);
            }
        }
    }

    #[test]
    fn classic_distances() {
        // points: (0,0), (3,4), (6,0), (3,-4)
        // (0,1): sqrt(9+16)  = 5
        // (0,2): sqrt(36+0)  = 6
        // (0,3): sqrt(9+16)  = 5
        // (1,3): sqrt(0+64)  = 8
        // (1,2): sqrt(9+16)  = 5
        let d = data(pairwise_l2(sample_points::<B>(&device())));
        assert!((d[0 * 4 + 1] - 5.0).abs() < 1e-4);
        assert!((d[0 * 4 + 2] - 6.0).abs() < 1e-4);
        assert!((d[0 * 4 + 3] - 5.0).abs() < 1e-4);
        assert!((d[1 * 4 + 2] - 5.0).abs() < 1e-4);
        assert!((d[1 * 4 + 3] - 8.0).abs() < 1e-4);
    }

    #[test]
    fn single_point() {
        // 1 point => 1x1 zero matrix
        let device = device();
        let x = Tensor::<B, 2>::from_floats([[1.0, 2.0, 3.0]], &device);
        let d = pairwise_l2(x);
        assert_eq!(d.dims(), [1, 1]);
        let v: Vec<f32> = d.to_data().to_vec().unwrap();
        assert!(v[0].abs() < 1e-5);
    }
}
