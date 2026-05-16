use std::env;
use std::time::Instant;

use rayon::prelude::*;

fn gc_content(seq: &[u8]) -> f64 {
    if seq.is_empty() {
        return 0.0;
    }
    let gc = seq
        .iter()
        .filter(|&&b| matches!(b, b'G' | b'C' | b'g' | b'c'))
        .count();
    gc as f64 / seq.len() as f64
}

/// Sequential reference implementation — provided as a baseline.
pub fn per_seq_gc_sequential(sequences: &[Vec<u8>]) -> Vec<f64> {
    sequences.iter().map(|seq| gc_content(seq)).collect()
}

/// Parallel implementation — TODO: write the rayon version.
pub fn per_seq_gc_parallel(sequences: &[Vec<u8>]) -> Vec<f64> {
    // TODO:
    //   Same as the sequential version, but use rayon's parallel iterator
    //   (.par_iter() in place of .iter()). The rest of the line is identical.
    let _ = sequences;
    Vec::new()
}

// --- Random workload generator (same as ex-release-mode) ---

struct Lcg(u64);
impl Lcg {
    fn new(seed: u64) -> Self { Self(seed) }
    fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        self.0
    }
}

fn random_sequence(rng: &mut Lcg, len: usize) -> Vec<u8> {
    const BASES: &[u8; 4] = b"ACGT";
    (0..len).map(|_| BASES[(rng.next_u64() % 4) as usize]).collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: parallel-gc <N_SEQS>");
        std::process::exit(1);
    }
    let n: usize = args[1].parse().expect("N_SEQS must be a non-negative integer");
    let mut rng = Lcg::new(0xFEEDFACE);
    let sequences: Vec<Vec<u8>> = (0..n).map(|_| random_sequence(&mut rng, 1000)).collect();

    let t0 = Instant::now();
    let seq_result = per_seq_gc_sequential(&sequences);
    let t_seq = t0.elapsed();

    let t1 = Instant::now();
    let par_result = per_seq_gc_parallel(&sequences);
    let t_par = t1.elapsed();

    println!("sequences: {}", n);
    println!("sequential: {:.3?}", t_seq);
    println!("parallel:   {:.3?}", t_par);
    let speedup = t_seq.as_secs_f64() / t_par.as_secs_f64().max(1e-9);
    println!("speedup:    {:.2}x", speedup);
    assert_eq!(
        seq_result.len(),
        par_result.len(),
        "sequential and parallel produced different number of results"
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_sequences() -> Vec<Vec<u8>> {
        vec![
            b"ACGTACGT".to_vec(),
            b"GGGGCCCC".to_vec(),
            b"AAAATTTT".to_vec(),
            b"".to_vec(),
            b"ATGCATGCATGC".to_vec(),
        ]
    }

    #[test]
    fn sequential_known_answers() {
        let r = per_seq_gc_sequential(&sample_sequences());
        assert!((r[0] - 0.5).abs() < 1e-9);
        assert!((r[1] - 1.0).abs() < 1e-9);
        assert!((r[2] - 0.0).abs() < 1e-9);
        assert!((r[3] - 0.0).abs() < 1e-9);
        assert!((r[4] - 0.5).abs() < 1e-9);
    }

    #[test]
    fn parallel_matches_sequential_small() {
        let s = sample_sequences();
        let seq = per_seq_gc_sequential(&s);
        let par = per_seq_gc_parallel(&s);
        assert_eq!(seq, par);
    }

    #[test]
    fn parallel_matches_sequential_large() {
        let mut rng = Lcg::new(99);
        let s: Vec<Vec<u8>> = (0..1000).map(|_| random_sequence(&mut rng, 200)).collect();
        let seq = per_seq_gc_sequential(&s);
        let par = per_seq_gc_parallel(&s);
        assert_eq!(seq, par);
    }
}
