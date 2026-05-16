use std::collections::HashMap;
use std::env;
use std::time::Instant;

/// Tiny deterministic pseudo-random generator so the workload is the same
/// in debug and release runs (and across machines).
struct Lcg(u64);
impl Lcg {
    fn new(seed: u64) -> Self { Self(seed) }
    fn next_u64(&mut self) -> u64 {
        // Numerical Recipes LCG.
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        self.0
    }
}

fn random_sequence(rng: &mut Lcg, len: usize) -> Vec<u8> {
    const BASES: &[u8; 4] = b"ACGT";
    (0..len).map(|_| BASES[(rng.next_u64() % 4) as usize]).collect()
}

fn kmer_counts(seq: &[u8], k: usize) -> HashMap<Vec<u8>, usize> {
    let mut counts: HashMap<Vec<u8>, usize> = HashMap::new();
    if k == 0 || k > seq.len() {
        return counts;
    }
    for window in seq.windows(k) {
        *counts.entry(window.to_vec()).or_insert(0) += 1;
    }
    counts
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: release-mode <N_SEQS>");
        std::process::exit(1);
    }
    let n: usize = args[1].parse().expect("N_SEQS must be a non-negative integer");
    let k = 7;
    let len = 1000;

    // Generate `n` random sequences once.
    let mut rng = Lcg::new(0xC0FFEE);
    let sequences: Vec<Vec<u8>> = (0..n).map(|_| random_sequence(&mut rng, len)).collect();

    // Count k-mers across all of them and time it.
    let start = Instant::now();
    let mut total_distinct: usize = 0;
    for seq in &sequences {
        let counts = kmer_counts(seq, k);
        total_distinct += counts.len();
    }
    let elapsed = start.elapsed();

    println!("sequences: {}", n);
    println!("sequence length: {}", len);
    println!("k: {}", k);
    println!("total distinct k-mers (summed across sequences): {}", total_distinct);
    println!("elapsed: {:.3?}", elapsed);

    let profile = if cfg!(debug_assertions) { "debug" } else { "release" };
    println!("build profile: {}", profile);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lcg_is_reproducible() {
        let mut a = Lcg::new(42);
        let mut b = Lcg::new(42);
        for _ in 0..1000 {
            assert_eq!(a.next_u64(), b.next_u64());
        }
    }

    #[test]
    fn random_sequence_is_dna() {
        let mut rng = Lcg::new(7);
        let seq = random_sequence(&mut rng, 100);
        assert_eq!(seq.len(), 100);
        for &b in &seq {
            assert!(matches!(b, b'A' | b'C' | b'G' | b'T'), "non-DNA byte {}", b);
        }
    }

    #[test]
    fn small_workload_returns_some_kmers() {
        let mut rng = Lcg::new(1);
        let seq = random_sequence(&mut rng, 50);
        let counts = kmer_counts(&seq, 3);
        assert!(!counts.is_empty());
    }
}
