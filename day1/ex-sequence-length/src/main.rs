use std::env;

/// Return the number of bytes in `seq`.
fn length(seq: &[u8]) -> usize {
    // TODO: return seq.len()
    let _ = seq;
    0
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: sequence-length <DNA-SEQUENCE>");
        std::process::exit(1);
    }
    println!("length: {}", length(args[1].as_bytes()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_is_zero() {
        assert_eq!(length(b""), 0);
    }

    #[test]
    fn acgt_is_four() {
        assert_eq!(length(b"ACGT"), 4);
    }

    #[test]
    fn long_sequence() {
        assert_eq!(length(b"ACGTACGTACGTACGTACGT"), 20);
    }
}
