//! Day 11 — Exercise 5: PCA with nalgebra. Reference solution.

use nalgebra::DMatrix;

fn centre_columns(m: &DMatrix<f64>) -> DMatrix<f64> {
    let means: Vec<f64> = (0..m.ncols()).map(|j| m.column(j).mean()).collect();
    DMatrix::from_fn(m.nrows(), m.ncols(), |i, j| m[(i, j)] - means[j])
}

fn pca(m: &DMatrix<f64>, k: usize) -> DMatrix<f64> {
    let centred = centre_columns(m);
    let svd = centred.svd(true, true);
    let u = svd.u.unwrap();
    let s = svd.singular_values;
    let u_k = u.columns(0, k).into_owned();
    let s_k = s.rows(0, k).into_owned();
    u_k * DMatrix::from_diagonal(&s_k)
}

fn sample_matrix() -> DMatrix<f64> {
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
    println!("centred:\n{}", centre_columns(&x));
    println!("PC scores (2 components):\n{}", pca(&x, 2));
}

#[cfg(test)]
mod tests {
    use super::*;
    fn approx_eq(a: f64, b: f64, tol: f64) -> bool { (a - b).abs() < tol }

    #[test]
    fn centre_columns_zero_mean() {
        let c = centre_columns(&sample_matrix());
        for j in 0..c.ncols() {
            assert!(approx_eq(c.column(j).mean(), 0.0, 1e-10));
        }
    }

    #[test]
    fn centre_columns_preserves_shape() {
        let c = centre_columns(&sample_matrix());
        assert_eq!((c.nrows(), c.ncols()), (8, 3));
    }

    #[test]
    fn centre_columns_preserves_variance() {
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
        assert_eq!((pcs.nrows(), pcs.ncols()), (8, 2));
    }

    #[test]
    fn pca_pc1_separates_two_groups() {
        let pcs = pca(&sample_matrix(), 2);
        let g1: f64 = pcs.rows(0, 4).column(0).mean();
        let g2: f64 = pcs.rows(4, 4).column(0).mean();
        assert!(g1 * g2 < 0.0);
        assert!(g1.abs() > 1.0 && g2.abs() > 1.0);
    }

    #[test]
    fn pca_pc1_dominates_variance() {
        let pcs = pca(&sample_matrix(), 2);
        let v1: f64 = pcs.column(0).iter().map(|x| x.powi(2)).sum();
        let v2: f64 = pcs.column(1).iter().map(|x| x.powi(2)).sum();
        assert!(v1 > 5.0 * v2);
    }

    #[test]
    fn pca_k_one() {
        let pcs = pca(&sample_matrix(), 1);
        assert_eq!((pcs.nrows(), pcs.ncols()), (8, 1));
    }
}
