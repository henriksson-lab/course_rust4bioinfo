use std::collections::HashMap;
use std::env;

/// Count every k-mer in `seq`.
///
/// Returns a map from each distinct k-mer (as an owned Vec<u8>) to the
/// number of times it appears as a contiguous window in `seq`. If `k`
/// is 0 or larger than `seq.len()` the map is empty.
fn kmer_counts(seq: &[u8], k: usize) -> HashMap<Vec<u8>, usize> {
    // TODO:
    //   1. create an empty HashMap<Vec<u8>, usize>
    //   2. if k == 0, return the empty map (otherwise seq.windows(0) panics)
    //   3. iterate seq.windows(k); for each window, use the entry API
    //      to increment the counter (or insert 0 then increment)
    //   4. return the map
    let _ = (seq, k);
    HashMap::new()
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
    // Sort by count desc, then by kmer for stable output.
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
        // ATGATG has 3-mers: ATG, TGA, GAT, ATG. So ATG: 2, TGA: 1, GAT: 1.
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
