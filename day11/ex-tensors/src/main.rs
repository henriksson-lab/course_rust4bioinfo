//! Day 11 — Exercise 1: tensors on any backend.
//!
//! Implement the three TODO functions so that all six tests pass. Each function
//! is generic over `<B: Backend>` — the same function body will run on the CPU
//! (`NdArray`) and the GPU (`Wgpu`) without changing one line.

use burn::backend::NdArray;
use burn::tensor::{Device, Tensor, backend::Backend};

/// Return the mean of each column of `x`.
///
/// Input shape:  [n_rows, n_cols]
/// Output shape: [n_cols]
fn column_means<B: Backend>(x: Tensor<B, 2>) -> Tensor<B, 1> {
    // TODO: replace this body. Use `.mean_dim(0)` and `.squeeze_dim(0)`.
    let _ = x;
    todo!("Hint 1 in 01-tensors.qmd")
}

/// Subtract the column means from every row of `x`.
///
/// Input shape:  [n_rows, n_cols]
/// Output shape: [n_rows, n_cols]
fn centre_columns<B: Backend>(x: Tensor<B, 2>) -> Tensor<B, 2> {
    // TODO: compute `x.mean_dim(0)` (shape [1, n_cols]) and let broadcasting
    // do the work when you subtract.
    let _ = x;
    todo!("Hint 2 in 01-tensors.qmd")
}

/// Return the L2 norm of each row of `x`.
///
/// Input shape:  [n_rows, n_cols]
/// Output shape: [n_rows]
fn row_l2_norms<B: Backend>(x: Tensor<B, 2>) -> Tensor<B, 1> {
    // TODO: square element-wise, sum along the column axis, square-root, squeeze.
    let _ = x;
    todo!("Hint 3 in 01-tensors.qmd")
}

fn sample_matrix<B: Backend>(device: &Device<B>) -> Tensor<B, 2> {
    // 3x4 matrix — column means [5, 6, 7, 8], row L2 norms ≈ [5.48, 13.19, 21.12]
    Tensor::<B, 2>::from_floats(
        [[1.0,  2.0,  3.0,  4.0],
         [5.0,  6.0,  7.0,  8.0],
         [9.0, 10.0, 11.0, 12.0]],
        device,
    )
}

fn main() {
    // ---- Run on the CPU (NdArray backend) ----
    type Cpu = NdArray;
    let device = Default::default();
    let x = sample_matrix::<Cpu>(&device);

    println!("input matrix (CPU):\n{}", x);
    println!("column means: {}",  column_means(x.clone()));
    println!("centred:\n{}",      centre_columns(x.clone()));
    println!("row L2 norms: {}",  row_l2_norms(x));

    // ---- Same code, GPU backend ----
    // Build with: cargo run --features gpu
    #[cfg(feature = "gpu")]
    {
        use burn::backend::Wgpu;
        let device = burn::backend::wgpu::WgpuDevice::default();
        let x = sample_matrix::<Wgpu>(&device);

        println!("\ninput matrix (GPU):\n{}", x);
        println!("column means: {}", column_means(x.clone()));
        println!("centred:\n{}",     centre_columns(x.clone()));
        println!("row L2 norms: {}", row_l2_norms(x));
        // Notice: the function definitions never changed. The whole difference
        // between CPU and GPU lives in the type parameter `<B: Backend>`.
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
        let got = column_means(x).to_data();
        let got: Vec<f32> = got.to_vec().unwrap();
        assert!(approx_eq(&got, &[5.0, 6.0, 7.0, 8.0], 1e-5));
    }

    #[test]
    fn column_means_shape() {
        let x = sample_matrix::<B>(&device());
        let m = column_means(x);
        assert_eq!(m.dims(), [4]);
    }

    #[test]
    fn centre_columns_shape() {
        let x = sample_matrix::<B>(&device());
        let c = centre_columns(x);
        assert_eq!(c.dims(), [3, 4]);
    }

    #[test]
    fn centre_columns_zero_mean() {
        let x = sample_matrix::<B>(&device());
        let c = centre_columns(x);
        // After centring, each column sums to (approximately) zero.
        let sums: Vec<f32> = c.sum_dim(0).to_data().to_vec().unwrap();
        assert!(approx_eq(&sums, &[0.0; 4], 1e-5));
    }

    #[test]
    fn row_l2_norms_shape() {
        let x = sample_matrix::<B>(&device());
        let n = row_l2_norms(x);
        assert_eq!(n.dims(), [3]);
    }

    #[test]
    fn row_l2_norms_values() {
        let x = sample_matrix::<B>(&device());
        let got: Vec<f32> = row_l2_norms(x).to_data().to_vec().unwrap();
        // row 0: sqrt(1 + 4 + 9 + 16)   = sqrt(30)  ≈ 5.4772
        // row 1: sqrt(25 + 36 + 49 + 64) = sqrt(174) ≈ 13.1909
        // row 2: sqrt(81 + 100 + 121 + 144) = sqrt(446) ≈ 21.1187
        assert!(approx_eq(&got, &[30.0f32.sqrt(), 174.0f32.sqrt(), 446.0f32.sqrt()], 1e-3));
    }
}
