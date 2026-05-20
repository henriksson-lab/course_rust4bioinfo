use ndarray::{array, Array1, Array2};

/// A 4-state transition matrix (each row sums to 1.0).
/// States: 0=sunny, 1=cloudy, 2=rain, 3=storm.
fn weather_matrix() -> Array2<f64> {
    array![
        [0.7, 0.2, 0.1, 0.0],  // from sunny
        [0.3, 0.4, 0.2, 0.1],  // from cloudy
        [0.2, 0.3, 0.4, 0.1],  // from rain
        [0.1, 0.2, 0.4, 0.3],  // from storm
    ]
}

/// One step of the Markov chain: p_{t+1} = p_t · P  (row-vector convention).
/// `p` is treated as a row vector of length n; `P` is n×n; result is length n.
fn step(p: &Array1<f64>, transition: &Array2<f64>) -> Array1<f64> {
    transition.t().dot(p)
}

/// Run power iteration: starting from `p_0`, apply `step` `n` times.
/// Return the final distribution.
fn power_iterate(mut p: Array1<f64>, transition: &Array2<f64>, n: usize) -> Array1<f64> {
    for _ in 0..n {
        p = step(&p, transition);
    }
    p
}

/// Solve for the steady state by power iteration until consecutive distributions
/// are within `tol` of each other. Return (steady_state, iterations_used).
fn steady_state(transition: &Array2<f64>, tol: f64, max_iter: usize) -> (Array1<f64>, usize) {
    let n = transition.nrows();
    let mut p = Array1::from_elem(n, 1.0 / n as f64);
    for k in 0..max_iter {
        let next = step(&p, transition);
        let max_change = next
            .iter()
            .zip(p.iter())
            .map(|(a, b)| (a - b).abs())
            .fold(0.0_f64, f64::max);
        p = next;
        if max_change < tol {
            return (p, k + 1);
        }
    }
    (p, max_iter)
}

fn main() {
    let p = weather_matrix();
    let p0 = array![1.0, 0.0, 0.0, 0.0];  // start sunny

    println!("first 5 steps from sunny:");
    let mut q = p0.clone();
    for t in 0..5 {
        println!("t={}: {:.4}", t, q);
        q = step(&q, &p);
    }

    let (ss, iters) = steady_state(&p, 1e-10, 10_000);
    println!("steady state after {} iters: {:.4}", iters, ss);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step_sums_to_one() {
        let p = weather_matrix();
        let p0 = array![0.25, 0.25, 0.25, 0.25];
        let p1 = step(&p0, &p);
        let total: f64 = p1.iter().sum();
        assert!((total - 1.0).abs() < 1e-9, "distribution sum drifted: {}", total);
    }

    #[test]
    fn power_iterate_50_steps_changes_distribution() {
        let p = weather_matrix();
        let p0 = array![1.0, 0.0, 0.0, 0.0];
        let after = power_iterate(p0.clone(), &p, 50);
        // After 50 steps, the distribution should NOT still be all-mass-on-state-0
        assert!(after[0] < 0.6);
        // And it should still sum to ~1
        let total: f64 = after.iter().sum();
        assert!((total - 1.0).abs() < 1e-6);
    }

    #[test]
    fn steady_state_is_stationary() {
        let p = weather_matrix();
        let (ss, _) = steady_state(&p, 1e-10, 10_000);
        let next = step(&ss, &p);
        // Multiplying the steady state by P should give back (approximately) the
        // steady state.
        let max_change = ss
            .iter()
            .zip(next.iter())
            .map(|(a, b)| (a - b).abs())
            .fold(0.0_f64, f64::max);
        assert!(max_change < 1e-6);
    }

    #[test]
    fn power_iterate_50_matches_steady_state() {
        let p = weather_matrix();
        let p0 = array![1.0, 0.0, 0.0, 0.0];
        let after_50 = power_iterate(p0, &p, 200);   // plenty of iterations
        let (ss, _) = steady_state(&p, 1e-10, 10_000);
        // 200 power-iteration steps should be very close to the iterative steady-state
        let max_diff = after_50
            .iter()
            .zip(ss.iter())
            .map(|(a, b)| (a - b).abs())
            .fold(0.0_f64, f64::max);
        assert!(max_diff < 1e-4);
    }
}
