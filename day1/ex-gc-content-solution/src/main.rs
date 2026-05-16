use std::env;

fn gc_content(seq: &[u8]) -> f64 {
    if seq.is_empty() {
        return 0.0;
    }
    let mut gc: usize = 0;
    for &b in seq {
        match b {
            b'G' | b'C' | b'g' | b'c' => gc += 1,
            _ => {}
        }
    }
    gc as f64 / seq.len() as f64
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
