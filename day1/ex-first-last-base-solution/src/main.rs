use std::env;

/// Return the first byte of `seq`.
///
/// Behaviour on an empty slice is undefined — not tested.
fn first_base(seq: &[u8]) -> u8 {
    seq[0]
}

/// Return the last byte of `seq`.
///
/// Behaviour on an empty slice is undefined — not tested.
fn last_base(seq: &[u8]) -> u8 {
    seq[seq.len() - 1]
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: first-last-base <DNA-SEQUENCE>");
        std::process::exit(1);
    }
    let seq = args[1].as_bytes();
    println!("first: {}, last: {}", first_base(seq) as char, last_base(seq) as char);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_of_acgt_is_a() {
        assert_eq!(first_base(b"ACGT"), b'A');
    }

    #[test]
    fn last_of_acgt_is_t() {
        assert_eq!(last_base(b"ACGT"), b'T');
    }

    #[test]
    fn single_base() {
        assert_eq!(first_base(b"G"), b'G');
        assert_eq!(last_base(b"G"), b'G');
    }
}
