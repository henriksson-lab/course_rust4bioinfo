//! Day 11 — Exercise 5: PCA with nalgebra.

use nalgebra::DMatrix;

/// Subtract each column's mean from that column.
fn centre_columns(m: &DMatrix<f64>) -> DMatrix<f64> {
    // TODO: hint 1 (column means) and hint 2 (DMatrix::from_fn).
    let _ = m;
    todo!("Hints in 05-pca.qmd")
}

/// Compute the top-`k` principal-component scores of `m`.
///
/// `m` is `[n_samples, n_features]`. The output is `[n_samples, k]`.
fn pca(m: &DMatrix<f64>, k: usize) -> DMatrix<f64> {
    // TODO:
    //   1. centred = centre_columns(m)
    //   2. svd = centred.svd(true, true)
    //   3. take the first k columns of U and the first k singular values
    //   4. scores = U[:, :k] * diag(s[:k])
    let _ = (m, k);
    todo!("Hints in 05-pca.qmd")
}

fn sample_matrix() -> DMatrix<f64> {
    // 8 samples × 3 features (genes). Sample-i values designed so PC1 separates
    // the first four samples from the last four along a clear axis.
    let data: Vec<f64> = vec![
        1.0, 2.0, 1.0,
        2.0, 4.0, 0.5,
        1.5, 3.0, 0.8,
        1.2, 2.4, 1.2,
       10.0, 1.0, 5.0,
       11.0, 0.5, 5.5,
       10.5, 0.8, 5.2,
        9.8, 1.2, 4.8,
    ];
    DMatrix::from_row_slice(8, 3, &data)
}

fn main() {
    let x = sample_matrix();
    println!("X ({}x{}):\n{}", x.nrows(), x.ncols(), x);
    let c = centre_columns(&x);
    println!("centred (column means ≈ 0):\n{}", c);
    let pcs = pca(&x, 2);
    println!("PC scores (2 components):\n{}", pcs);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx_eq(a: f64, b: f64, tol: f64) -> bool { (a - b).abs() < tol }

    #[test]
    fn centre_columns_zero_mean() {
        let c = centre_columns(&sample_matrix());
        for j in 0..c.ncols() {
            assert!(approx_eq(c.column(j).mean(), 0.0, 1e-10),
                    "column {j} mean: {}", c.column(j).mean());
        }
    }

    #[test]
    fn centre_columns_preserves_shape() {
        let c = centre_columns(&sample_matrix());
        assert_eq!(c.nrows(), 8);
        assert_eq!(c.ncols(), 3);
    }

    #[test]
    fn centre_columns_preserves_variance() {
        // After centring, sum of squares per column should equal the original
        // matrix's sum of (x - mean)^2 per column — same thing computed differently.
        let m = sample_matrix();
        let c = centre_columns(&m);
        for j in 0..m.ncols() {
            let mean = m.column(j).mean();
            let expected: f64 = m.column(j).iter().map(|x| (x - mean).powi(2)).sum();
            let got: f64 = c.column(j).iter().map(|x| x.powi(2)).sum();
            assert!(approx_eq(expected, got, 1e-10));
        }
    }

    #[test]
    fn pca_output_shape() {
        let pcs = pca(&sample_matrix(), 2);
        assert_eq!(pcs.nrows(), 8);
        assert_eq!(pcs.ncols(), 2);
    }

    #[test]
    fn pca_pc1_separates_two_groups() {
        // The two halves of sample_matrix are well-separated. After PCA the
        // average of PC1 over rows 0..4 and rows 4..8 should have opposite signs
        // (PCA is unique up to a sign flip, so this is the strongest sign-
        // invariant statement we can make).
        let pcs = pca(&sample_matrix(), 2);
        let g1: f64 = pcs.rows(0, 4).column(0).mean();
        let g2: f64 = pcs.rows(4, 4).column(0).mean();
        assert!(g1 * g2 < 0.0, "PC1 should separate the two halves: {g1}, {g2}");
        assert!(g1.abs() > 1.0 && g2.abs() > 1.0, "magnitudes should be substantial");
    }

    #[test]
    fn pca_pc1_dominates_variance() {
        // For this dataset PC1 should carry far more variance than PC2.
        let pcs = pca(&sample_matrix(), 2);
        let v1: f64 = pcs.column(0).iter().map(|x| x.powi(2)).sum();
        let v2: f64 = pcs.column(1).iter().map(|x| x.powi(2)).sum();
        assert!(v1 > 5.0 * v2, "PC1 should dominate: v1={v1}, v2={v2}");
    }

    #[test]
    fn pca_k_one() {
        let pcs = pca(&sample_matrix(), 1);
        assert_eq!(pcs.nrows(), 8);
        assert_eq!(pcs.ncols(), 1);
    }
}
