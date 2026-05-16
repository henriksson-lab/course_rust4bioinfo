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

/// Write the reverse complement of `seq` into the caller-owned buffer `out`.
///
/// The buffer is cleared first, then filled with the complement of each
/// base of `seq` in reverse order. The underlying allocation is reused
/// across calls — no new allocation as long as `out.capacity()` is enough.
fn reverse_complement_into(seq: &[u8], out: &mut Vec<u8>) {
    // TODO:
    //   1. clear `out` (sets length to 0 without freeing memory)
    //   2. reserve enough room for `seq.len()` bytes (cheap if already big)
    //   3. iterate from the last index of `seq` down to 0
    //   4. push complement_base(seq[i]) onto `out`
    //   (no return value — this function returns the unit type)
    let _ = (seq, out);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: revcomp-buf <DNA-SEQUENCE>");
        std::process::exit(1);
    }
    let mut buf: Vec<u8> = Vec::new();
    reverse_complement_into(args[1].as_bytes(), &mut buf);
    println!(
        "{}",
        String::from_utf8(buf).expect("DNA bases are always ASCII")
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_in_empty_out() {
        let mut buf = Vec::new();
        reverse_complement_into(b"", &mut buf);
        assert_eq!(buf, Vec::<u8>::new());
    }

    #[test]
    fn standard_case() {
        let mut buf = Vec::new();
        reverse_complement_into(b"ATGCATGC", &mut buf);
        assert_eq!(buf, b"GCATGCAT");
    }

    #[test]
    fn palindrome_acgt() {
        let mut buf = Vec::new();
        reverse_complement_into(b"ACGT", &mut buf);
        assert_eq!(buf, b"ACGT");
    }

    #[test]
    fn buffer_is_cleared_between_calls() {
        let mut buf = Vec::with_capacity(64);
        reverse_complement_into(b"ATGC", &mut buf);
        assert_eq!(buf, b"GCAT");
        // Second call must overwrite, not append.
        reverse_complement_into(b"AAA", &mut buf);
        assert_eq!(buf, b"TTT");
    }

    #[test]
    fn buffer_capacity_reused() {
        // After a "warm-up" call the buffer holds a 100-byte allocation;
        // a follow-up call with a smaller input must not reduce capacity.
        let mut buf = Vec::new();
        reverse_complement_into(&[b'A'; 100], &mut buf);
        let cap_after_warmup = buf.capacity();
        reverse_complement_into(b"ATGC", &mut buf);
        assert_eq!(buf, b"GCAT");
        assert!(
            buf.capacity() >= cap_after_warmup,
            "second call shrunk the allocation"
        );
    }

    #[test]
    fn with_lowercase_and_n() {
        let mut buf = Vec::new();
        reverse_complement_into(b"NacgT", &mut buf);
        // NacgT reversed: TgcaN; complement of each: AcgtN
        assert_eq!(buf, b"AcgtN");
    }
}
