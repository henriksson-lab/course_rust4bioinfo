use std::env;

/// Return a new `Vec<u8>` containing `motif` repeated `n` times.
///
/// Examples (with bytes shown as text for readability):
/// - `repeat(b"AT", 3)` returns `b"ATATAT"`.
/// - `repeat(b"A", 0)` returns `b""`.
/// - `repeat(b"", 5)` returns `b""`.
fn repeat(motif: &[u8], n: usize) -> Vec<u8> {
    let mut out = Vec::with_capacity(motif.len() * n);
    for _ in 0..n {
        out.extend_from_slice(motif);
    }
    out
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("usage: repeat-motif <MOTIF> <N>");
        std::process::exit(1);
    }
    let motif = args[1].as_bytes();
    let n: usize = args[2].parse().expect("N must be a non-negative integer");
    let out = repeat(motif, n);
    println!("{}", String::from_utf8_lossy(&out));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_motif_yields_empty() {
        assert_eq!(repeat(b"", 5), b"");
    }

    #[test]
    fn n_zero_yields_empty() {
        assert_eq!(repeat(b"AT", 0), b"");
    }

    #[test]
    fn at_three_times() {
        assert_eq!(repeat(b"AT", 3), b"ATATAT");
    }

    #[test]
    fn single_base_five_times() {
        assert_eq!(repeat(b"G", 5), b"GGGGG");
    }
}
