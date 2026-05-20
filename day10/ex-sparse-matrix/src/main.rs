use sprs::{CsMat, TriMat};

/// Triplets `(row, col, value)` for a 5x5 matrix with 6 non-zero entries.
/// All other positions are implicitly zero.
fn sample_triplets() -> Vec<(usize, usize, f64)> {
    vec![
        (0, 0, 1.0),
        (0, 4, 2.0),
        (1, 1, 3.0),
        (2, 3, 4.0),
        (4, 0, 5.0),
        (4, 4, 6.0),
    ]
}

/// Build a CSR sparse matrix from a list of triplets.
/// Use `sprs::TriMat` as the intermediate builder, then `.to_csr()`.
fn build_sparse(triplets: &[(usize, usize, f64)], shape: (usize, usize)) -> CsMat<f64> {
    // TODO:
    //   1. Construct a TriMat with the given shape:
    //        let mut tri = TriMat::new(shape);
    //   2. For each (row, col, value) in `triplets`, call tri.add_triplet(row, col, value).
    //   3. Convert to CSR: tri.to_csr().
    let _ = (triplets, shape);
    CsMat::zero((0, 0))
}

/// Return the density of `m`: nnz / (rows * cols).
fn density(m: &CsMat<f64>) -> f64 {
    // TODO: return m.nnz() as f64 / (m.rows() * m.cols()) as f64
    let _ = m;
    0.0
}

fn main() {
    let m = build_sparse(&sample_triplets(), (5, 5));
    println!("shape: {:?}", (m.rows(), m.cols()));
    println!("nnz: {}", m.nnz());
    println!("density: {:.2}%", density(&m) * 100.0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shape_is_5x5() {
        let m = build_sparse(&sample_triplets(), (5, 5));
        assert_eq!(m.rows(), 5);
        assert_eq!(m.cols(), 5);
    }

    #[test]
    fn six_nonzeros() {
        let m = build_sparse(&sample_triplets(), (5, 5));
        assert_eq!(m.nnz(), 6);
    }

    #[test]
    fn density_value() {
        let m = build_sparse(&sample_triplets(), (5, 5));
        // 6 nnz out of 25 entries = 0.24
        assert!((density(&m) - 0.24).abs() < 1e-9);
    }

    #[test]
    fn entry_values() {
        let m = build_sparse(&sample_triplets(), (5, 5));
        // CsMat::get returns Option<&T>
        assert_eq!(m.get(0, 0), Some(&1.0));
        assert_eq!(m.get(2, 3), Some(&4.0));
        assert_eq!(m.get(4, 4), Some(&6.0));
        // missing entry
        assert_eq!(m.get(3, 3), None);
    }

    #[test]
    fn empty_matrix_density_zero() {
        let m = build_sparse(&[], (5, 5));
        assert_eq!(density(&m), 0.0);
    }
}
