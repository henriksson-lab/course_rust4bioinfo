use std::env;

/// Return every contiguous k-mer of `seq` as a slice view, in order.
///
/// The returned slices borrow from `seq` — no bytes are copied. If `k` is
/// zero, larger than `seq.len()`, or `seq` is empty, the result is empty.
fn kmers(seq: &[u8], k: usize) -> Vec<&[u8]> {
    // TODO:
    //   1. handle the empty-result edge cases (k == 0 or k > seq.len())
    //   2. allocate an output Vec with the right capacity
    //   3. for each valid start index i (0 .. seq.len() - k + 1),
    //      push &seq[i..i + k] onto the output
    //   4. return the Vec
    let _ = (seq, k);
    Vec::new()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("usage: kmers <SEQ> <K>");
        std::process::exit(1);
    }
    let seq = args[1].as_bytes();
    let k: usize = args[2]
        .parse()
        .expect("K must be a non-negative integer");
    for kmer in kmers(seq, k) {
        println!("{}", std::str::from_utf8(kmer).expect("ASCII input"));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn three_mers_of_acgta() {
        let got = kmers(b"ACGTA", 3);
        let want: Vec<&[u8]> = vec![b"ACG", b"CGT", b"GTA"];
        assert_eq!(got, want);
    }

    #[test]
    fn one_mers() {
        let got = kmers(b"ACGT", 1);
        let want: Vec<&[u8]> = vec![b"A", b"C", b"G", b"T"];
        assert_eq!(got, want);
    }

    #[test]
    fn k_equals_len() {
        let got = kmers(b"ACGT", 4);
        let want: Vec<&[u8]> = vec![b"ACGT"];
        assert_eq!(got, want);
    }

    #[test]
    fn k_too_large() {
        assert_eq!(kmers(b"AC", 3), Vec::<&[u8]>::new());
    }

    #[test]
    fn k_zero() {
        assert_eq!(kmers(b"ACGT", 0), Vec::<&[u8]>::new());
    }

    #[test]
    fn empty_seq() {
        assert_eq!(kmers(b"", 3), Vec::<&[u8]>::new());
    }
}
