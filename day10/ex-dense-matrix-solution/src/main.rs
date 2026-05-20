use ndarray::{array, Array1, Array2};

/// A 3x4 matrix used by the tests. Don't modify.
fn sample_matrix() -> Array2<f64> {
    array![
        [1.0, 2.0, 3.0, 4.0],
        [5.0, 6.0, 7.0, 8.0],
        [9.0, 10.0, 11.0, 12.0],
    ]
}

/// Return the sum of the diagonal entries A[0][0] + A[1][1] + A[2][2] + ...
/// For a non-square matrix, sum entries A[i][i] for i in 0..min(rows, cols).
fn trace(a: &Array2<f64>) -> f64 {
    let n = a.nrows().min(a.ncols());
    (0..n).map(|i| a[[i, i]]).sum()
}

/// Return the transpose of `a`. The transpose of an MxN matrix is an NxM matrix
/// where entry (i, j) of the output is entry (j, i) of the input.
fn transpose(a: &Array2<f64>) -> Array2<f64> {
    a.t().to_owned()
}

/// Compute the matrix-vector product A · v.
/// A is MxN, v is length N, result is length M.
fn matvec(a: &Array2<f64>, v: &Array1<f64>) -> Array1<f64> {
    a.dot(v)
}

fn main() {
    let a = sample_matrix();
    println!("matrix:\n{}", a);
    println!("trace = {}", trace(&a));
    println!("transpose:\n{}", transpose(&a));
    let v = array![1.0, 2.0, 3.0, 4.0];
    println!("A · v = {}", matvec(&a, &v));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trace_of_sample() {
        // diagonal entries: 1 + 6 + 11 = 18
        assert_eq!(trace(&sample_matrix()), 18.0);
    }

    #[test]
    fn trace_of_identity_3() {
        let id = Array2::<f64>::eye(3);
        assert_eq!(trace(&id), 3.0);
    }

    #[test]
    fn transpose_shape() {
        let t = transpose(&sample_matrix());
        assert_eq!(t.shape(), &[4, 3]);
    }

    #[test]
    fn transpose_value() {
        let t = transpose(&sample_matrix());
        assert_eq!(t[[0, 1]], 5.0); // a[[1, 0]] was 5
        assert_eq!(t[[3, 2]], 12.0); // a[[2, 3]] was 12
    }

    #[test]
    fn matvec_basic() {
        let a = sample_matrix();
        let v = array![1.0, 2.0, 3.0, 4.0];
        // row 0: 1*1 + 2*2 + 3*3 + 4*4 = 30
        // row 1: 5*1 + 6*2 + 7*3 + 8*4 = 70
        // row 2: 9*1 + 10*2 + 11*3 + 12*4 = 110
        let got = matvec(&a, &v);
        assert_eq!(got, array![30.0, 70.0, 110.0]);
    }
}
