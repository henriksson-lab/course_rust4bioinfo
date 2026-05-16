use std::env;

/// Complement a single DNA base (from day 1).
fn complement_base(base: u8) -> u8 {
    match base {
        b'A' => b'T',
        b'T' => b'A',
        b'C' => b'G',
        b'G' => b'C',
        b'a' => b't',
        b't' => b'a',
        b'c' => b'g',
        b'g' => b'c',
        b'N' => b'N',
        b'n' => b'n',
        other => panic!("complement_base: unsupported base {:?}", other as char),
    }
}

/// Reverse complement of a DNA sequence.
///
/// Returns a freshly-allocated Vec<u8> containing the complement of each
/// base of `seq` in reverse order. The caller owns the returned vector.
fn reverse_complement(seq: &[u8]) -> Vec<u8> {
    // TODO:
    //   1. allocate a new Vec<u8> with capacity for `seq.len()` bytes
    //   2. iterate from the last index of `seq` down to 0
    //   3. push complement_base(seq[i]) onto the vector at each step
    //   4. return the vector
    let _ = seq;
    Vec::new()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: revcomp <DNA-SEQUENCE>");
        std::process::exit(1);
    }
    let rc = reverse_complement(args[1].as_bytes());
    println!(
        "{}",
        String::from_utf8(rc).expect("DNA bases are always ASCII")
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(reverse_complement(b""), Vec::<u8>::new());
    }

    #[test]
    fn single_a() {
        assert_eq!(reverse_complement(b"A"), b"T");
    }

    #[test]
    fn palindrome_acgt() {
        assert_eq!(reverse_complement(b"ACGT"), b"ACGT");
    }

    #[test]
    fn standard_case() {
        assert_eq!(reverse_complement(b"ATGCATGC"), b"GCATGCAT");
    }

    #[test]
    fn all_a_becomes_all_t() {
        assert_eq!(reverse_complement(b"AAAA"), b"TTTT");
    }

    #[test]
    fn lowercase_preserved() {
        // acgt reversed is tgca; complement of tgca is acgt
        assert_eq!(reverse_complement(b"acgt"), b"acgt");
    }

    #[test]
    fn with_n() {
        // NACG -> reversed: GCAN -> complemented: CGTN
        assert_eq!(reverse_complement(b"NACG"), b"CGTN");
    }
}
