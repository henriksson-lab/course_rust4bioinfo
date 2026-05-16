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
        b'N' => b'N',
        b'n' => b'n',
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
        assert_eq!(reverse_complement(b"acgt"), b"acgt");
    }

    #[test]
    fn with_n() {
        assert_eq!(reverse_complement(b"NACG"), b"CGTN");
    }
}
