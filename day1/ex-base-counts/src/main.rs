use std::env;

/// Count A, C, G, T, and "other" bases in a DNA sequence, case-insensitive.
///
/// Returns (a, c, g, t, other). "Other" counts every byte that is not a
/// plain A/C/G/T in either case: N, ambiguity codes, gaps, anything else.
fn base_counts(seq: &[u8]) -> (usize, usize, usize, usize, usize) {
    // TODO:
    //   1. declare five mutable usize counters initialised to 0
    //   2. iterate over `seq` and `match` each byte:
    //        - increment the right counter for A/a, C/c, G/g, T/t
    //        - increment `other` for anything else
    //   3. return the tuple (a, c, g, t, other)
    let _ = seq;
    (0, 0, 0, 0, 0)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: base-counts <DNA sequence>");
        std::process::exit(1);
    }
    let (a, c, g, t, other) = base_counts(args[1].as_bytes());
    println!("A: {}", a);
    println!("C: {}", c);
    println!("G: {}", g);
    println!("T: {}", t);
    println!("other: {}", other);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(base_counts(b""), (0, 0, 0, 0, 0));
    }

    #[test]
    fn one_of_each() {
        assert_eq!(base_counts(b"ACGT"), (1, 1, 1, 1, 0));
    }

    #[test]
    fn case_insensitive() {
        assert_eq!(base_counts(b"acgt"), (1, 1, 1, 1, 0));
    }

    #[test]
    fn mixed_case() {
        assert_eq!(base_counts(b"AaCcGgTt"), (2, 2, 2, 2, 0));
    }

    #[test]
    fn with_n() {
        assert_eq!(base_counts(b"ACNGT"), (1, 1, 1, 1, 1));
    }

    #[test]
    fn with_garbage() {
        assert_eq!(base_counts(b"AXYZ"), (1, 0, 0, 0, 3));
    }
}
