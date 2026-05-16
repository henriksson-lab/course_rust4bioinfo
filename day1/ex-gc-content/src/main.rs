use std::env;

/// Compute the GC content of a DNA sequence as a fraction in [0.0, 1.0].
///
/// G and C count toward GC, in both upper and lower case. Any other byte
/// (A, T, N, gaps, …) does not. An empty sequence has GC content 0.0.
fn gc_content(seq: &[u8]) -> f64 {
    // TODO:
    //   1. handle the empty-sequence case
    //   2. count the number of G/C/g/c bytes in `seq`
    //   3. return that count divided by the total length, as f64
    let _ = seq;
    0.0
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: gc-content <DNA-SEQUENCE>");
        std::process::exit(1);
    }
    let seq = args[1].as_bytes();
    let gc = gc_content(seq);
    println!("GC content: {:.2}%", gc * 100.0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_sequence() {
        assert_eq!(gc_content(b""), 0.0);
    }

    #[test]
    fn all_gc() {
        assert_eq!(gc_content(b"GCGC"), 1.0);
    }

    #[test]
    fn half_gc() {
        assert_eq!(gc_content(b"ATGC"), 0.5);
    }

    #[test]
    fn lowercase_handled() {
        assert_eq!(gc_content(b"gcgc"), 1.0);
    }
}
