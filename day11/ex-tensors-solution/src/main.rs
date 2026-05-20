//! Day 11 — Exercise 1: tensors on any backend. Reference solution.

use burn::backend::NdArray;
use burn::tensor::{Device, Tensor, backend::Backend};

/// Mean of each column. Input [N, D] → output [D].
fn column_means<B: Backend>(x: Tensor<B, 2>) -> Tensor<B, 1> {
    x.mean_dim(0).squeeze_dim(0)
}

/// Subtract column means from every row. Input [N, D] → output [N, D].
fn centre_columns<B: Backend>(x: Tensor<B, 2>) -> Tensor<B, 2> {
    let means: Tensor<B, 2> = x.clone().mean_dim(0);   // [1, D]
    x - means                                          // broadcasts to [N, D]
}

/// L2 norm of each row. Input [N, D] → output [N].
fn row_l2_norms<B: Backend>(x: Tensor<B, 2>) -> Tensor<B, 1> {
    let sq: Tensor<B, 2> = x.powf_scalar(2.0);         // [N, D]
    sq.sum_dim(1).sqrt().squeeze_dim(1)                    // [N]
}

fn sample_matrix<B: Backend>(device: &Device<B>) -> Tensor<B, 2> {
    Tensor::<B, 2>::from_floats(
        [[1.0,  2.0,  3.0,  4.0],
         [5.0,  6.0,  7.0,  8.0],
         [9.0, 10.0, 11.0, 12.0]],
        device,
    )
}

fn main() {
    type Cpu = NdArray;
    let device = Default::default();
    let x = sample_matrix::<Cpu>(&device);

    println!("input matrix (CPU):\n{}", x);
    println!("column means: {}",  column_means(x.clone()));
    println!("centred:\n{}",      centre_columns(x.clone()));
    println!("row L2 norms: {}",  row_l2_norms(x));

    #[cfg(feature = "gpu")]
    {
        use burn::backend::Wgpu;
        let device = burn::backend::wgpu::WgpuDevice::default();
        let x = sample_matrix::<Wgpu>(&device);

        println!("\ninput matrix (GPU):\n{}", x);
        println!("column means: {}", column_means(x.clone()));
        println!("centred:\n{}",     centre_columns(x.clone()));
        println!("row L2 norms: {}", row_l2_norms(x));
    }
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
    fn column_means_basic() {
        let x = sample_matrix::<B>(&device());
        let got: Vec<f32> = column_means(x).to_data().to_vec().unwrap();
        assert!(approx_eq(&got, &[5.0, 6.0, 7.0, 8.0], 1e-5));
    }

    #[test]
    fn column_means_shape() {
        let m = column_means(sample_matrix::<B>(&device()));
        assert_eq!(m.dims(), [4]);
    }

    #[test]
    fn centre_columns_shape() {
        let c = centre_columns(sample_matrix::<B>(&device()));
        assert_eq!(c.dims(), [3, 4]);
    }

    #[test]
    fn centre_columns_zero_mean() {
        let c = centre_columns(sample_matrix::<B>(&device()));
        let sums: Vec<f32> = c.sum_dim(0).to_data().to_vec().unwrap();
        assert!(approx_eq(&sums, &[0.0; 4], 1e-5));
    }

    #[test]
    fn row_l2_norms_shape() {
        let n = row_l2_norms(sample_matrix::<B>(&device()));
        assert_eq!(n.dims(), [3]);
    }

    #[test]
    fn row_l2_norms_values() {
        let got: Vec<f32> = row_l2_norms(sample_matrix::<B>(&device())).to_data().to_vec().unwrap();
        assert!(approx_eq(&got, &[30.0f32.sqrt(), 174.0f32.sqrt(), 446.0f32.sqrt()], 1e-3));
    }
}
