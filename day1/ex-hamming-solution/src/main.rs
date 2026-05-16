use std::env;

fn hamming_distance(a: &[u8], b: &[u8]) -> usize {
    if a.len() != b.len() {
        panic!(
            "hamming_distance: sequences have different lengths ({} vs {})",
            a.len(),
            b.len(),
        );
    }
    let mut d: usize = 0;
    for i in 0..a.len() {
        if a[i] != b[i] {
            d += 1;
        }
    }
    d
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("usage: hamming <seq1> <seq2>");
        std::process::exit(1);
    }
    let d = hamming_distance(args[1].as_bytes(), args[2].as_bytes());
    println!("Hamming distance: {}", d);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identical() {
        assert_eq!(hamming_distance(b"ACGT", b"ACGT"), 0);
    }

    #[test]
    fn one_difference() {
        assert_eq!(hamming_distance(b"ACGT", b"AAGT"), 1);
    }

    #[test]
    fn all_different() {
        assert_eq!(hamming_distance(b"ACGT", b"TGCA"), 4);
    }

    #[test]
    fn empty_pair() {
        assert_eq!(hamming_distance(b"", b""), 0);
    }

    #[test]
    #[should_panic]
    fn mismatched_lengths_panic() {
        let _ = hamming_distance(b"AC", b"ACG");
    }
}
