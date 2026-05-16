use std::env;

fn kmers(seq: &[u8], k: usize) -> Vec<&[u8]> {
    if k == 0 || k > seq.len() {
        return Vec::new();
    }
    let mut out: Vec<&[u8]> = Vec::with_capacity(seq.len() - k + 1);
    for i in 0..=seq.len() - k {
        out.push(&seq[i..i + k]);
    }
    out
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
