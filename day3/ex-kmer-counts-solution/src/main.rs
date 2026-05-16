use std::collections::HashMap;
use std::env;

fn kmer_counts(seq: &[u8], k: usize) -> HashMap<Vec<u8>, usize> {
    let mut counts: HashMap<Vec<u8>, usize> = HashMap::new();
    if k == 0 {
        return counts;
    }
    for window in seq.windows(k) {
        *counts.entry(window.to_vec()).or_insert(0) += 1;
    }
    counts
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("usage: kmer-counts <SEQ> <K>");
        std::process::exit(1);
    }
    let seq = args[1].as_bytes();
    let k: usize = args[2].parse().expect("K must be a non-negative integer");
    let counts = kmer_counts(seq, k);
    let mut items: Vec<_> = counts.into_iter().collect();
    items.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    for (kmer, n) in items {
        println!(
            "{}: {}",
            std::str::from_utf8(&kmer).expect("ASCII input"),
            n
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn three_mers_with_repeat() {
        let counts = kmer_counts(b"ATGATG", 3);
        assert_eq!(counts.get(b"ATG".as_slice()), Some(&2));
        assert_eq!(counts.get(b"TGA".as_slice()), Some(&1));
        assert_eq!(counts.get(b"GAT".as_slice()), Some(&1));
        assert_eq!(counts.len(), 3);
    }

    #[test]
    fn k_equals_one_is_base_composition() {
        let counts = kmer_counts(b"ACGTAC", 1);
        assert_eq!(counts.get(b"A".as_slice()), Some(&2));
        assert_eq!(counts.get(b"C".as_slice()), Some(&2));
        assert_eq!(counts.get(b"G".as_slice()), Some(&1));
        assert_eq!(counts.get(b"T".as_slice()), Some(&1));
    }

    #[test]
    fn k_too_large() {
        assert!(kmer_counts(b"AC", 3).is_empty());
    }

    #[test]
    fn empty_sequence() {
        assert!(kmer_counts(b"", 3).is_empty());
    }

    #[test]
    fn k_zero_is_empty() {
        assert!(kmer_counts(b"ACGT", 0).is_empty());
    }
}
