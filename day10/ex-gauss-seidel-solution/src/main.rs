use ndarray::{array, Array1, Array2};

/// Run one sweep of the Gauss-Seidel update on `x`.
///
/// For each row i:
///   x[i] = (b[i] - sum_{j != i} A[i][j] * x[j]) / A[i][i]
///
/// Note: the sum uses the CURRENT values of x — including any x[j] that
/// was already updated earlier in this sweep (that's what distinguishes
/// Gauss-Seidel from Jacobi).
fn gauss_seidel_step(a: &Array2<f64>, b: &Array1<f64>, x: &mut Array1<f64>) {
    let n = b.len();
    for i in 0..n {
        let s: f64 = (0..n).filter(|&j| j != i).map(|j| a[[i, j]] * x[j]).sum();
        x[i] = (b[i] - s) / a[[i, i]];
    }
}

/// Run Gauss-Seidel until the maximum change between sweeps is below `tol`,
/// or until `max_iter` sweeps have been performed. Returns the final x
/// and the number of iterations actually run.
fn solve(a: &Array2<f64>, b: &Array1<f64>, tol: f64, max_iter: usize) -> (Array1<f64>, usize) {
    let n = b.len();
    let mut x = Array1::<f64>::zeros(n);
    for k in 0..max_iter {
        let prev = x.clone();
        gauss_seidel_step(a, b, &mut x);
        let max_change = x
            .iter()
            .zip(prev.iter())
            .map(|(a, b)| (a - b).abs())
            .fold(0.0_f64, f64::max);
        if max_change < tol {
            return (x, k + 1);
        }
    }
    (x, max_iter)
}

fn main() {
    // A simple 3x3 SPD system with known solution x = [1, 1, 1]:
    //   4 -1  0     x0     3
    //  -1  4 -1     x1 =   2
    //   0 -1  4     x2     3
    let a = array![
        [4.0, -1.0, 0.0],
        [-1.0, 4.0, -1.0],
        [0.0, -1.0, 4.0],
    ];
    let b = array![3.0, 2.0, 3.0];
    let (x, iters) = solve(&a, &b, 1e-8, 1000);
    println!("converged in {} iterations: x = {}", iters, x);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(a: &Array1<f64>, b: &Array1<f64>, tol: f64) -> bool {
        a.iter().zip(b.iter()).all(|(x, y)| (x - y).abs() < tol)
    }

    #[test]
    fn solves_simple_3x3() {
        let a = array![
            [4.0, -1.0, 0.0],
            [-1.0, 4.0, -1.0],
            [0.0, -1.0, 4.0],
        ];
        let b = array![3.0, 2.0, 3.0];
        let (x, _) = solve(&a, &b, 1e-8, 1000);
        assert!(close(&x, &array![1.0, 1.0, 1.0], 1e-6));
    }

    #[test]
    fn solves_2x2() {
        // [2 1] [x0]   [3]   -> x = [1, 1]
        // [1 3] [x1] = [4]
        let a = array![[2.0, 1.0], [1.0, 3.0]];
        let b = array![3.0, 4.0];
        let (x, _) = solve(&a, &b, 1e-8, 1000);
        assert!(close(&x, &array![1.0, 1.0], 1e-6));
    }

    #[test]
    fn converges_in_reasonable_iterations() {
        let a = array![
            [4.0, -1.0, 0.0],
            [-1.0, 4.0, -1.0],
            [0.0, -1.0, 4.0],
        ];
        let b = array![3.0, 2.0, 3.0];
        let (_, iters) = solve(&a, &b, 1e-8, 1000);
        // For a well-conditioned SPD 3x3 this should converge in well under 100 sweeps.
        assert!(iters < 100, "expected fast convergence; took {} iterations", iters);
    }

    #[test]
    fn one_step_changes_x() {
        let a = array![[2.0, 1.0], [1.0, 3.0]];
        let b = array![3.0, 4.0];
        let mut x = array![0.0, 0.0];
        gauss_seidel_step(&a, &b, &mut x);
        // After one sweep, x should not still be zero
        assert!(x[0] != 0.0 || x[1] != 0.0);
    }
}
