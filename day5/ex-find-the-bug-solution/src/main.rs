use std::env;

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
        b'N' => b'N', // fixed
        b'n' => b'n', // fixed
        other => panic!("complement_base: unsupported base {:?}", other as char),
    }
}

fn reverse_complement(seq: &[u8]) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::with_capacity(seq.len());
    for i in (0..seq.len()).rev() {
        out.push(complement_base(seq[i]));
    }
    out
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: find-the-bug <DNA-SEQUENCE>");
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
    fn empty_input() {
        assert_eq!(reverse_complement(b""), Vec::<u8>::new());
    }

    #[test]
    fn palindrome_acgt() {
        assert_eq!(reverse_complement(b"ACGT"), b"ACGT");
    }

    #[test]
    fn single_base() {
        assert_eq!(reverse_complement(b"A"), b"T");
    }

    #[test]
    fn known_answer() {
        assert_eq!(reverse_complement(b"ATGCATGC"), b"GCATGCAT");
    }

    #[test]
    fn n_passthrough() {
        assert_eq!(reverse_complement(b"NACG"), b"CGTN");
    }

    #[test]
    fn round_trip_identity() {
        for seq in [
            b"A".as_slice(),
            b"ACGT",
            b"ATGCATGC",
            b"NACGTN",
            b"",
        ] {
            let rc = reverse_complement(seq);
            let rc2 = reverse_complement(&rc);
            assert_eq!(rc2, seq.to_vec(), "round-trip failed for {:?}", seq);
        }
    }
}
