use std::env;

fn base_counts(seq: &[u8]) -> (usize, usize, usize, usize, usize) {
    let mut a: usize = 0;
    let mut c: usize = 0;
    let mut g: usize = 0;
    let mut t: usize = 0;
    let mut other: usize = 0;
    for &b in seq {
        match b {
            b'A' | b'a' => a += 1,
            b'C' | b'c' => c += 1,
            b'G' | b'g' => g += 1,
            b'T' | b't' => t += 1,
            _ => other += 1,
        }
    }
    (a, c, g, t, other)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: base-counts <DNA sequence>");
        std::process::exit(1);
    }
    let (a, c, g, t, other) = base_counts(args[1].as_bytes());
    println!("A: {}", a);
    println!("C: {}", c);
    println!("G: {}", g);
    println!("T: {}", t);
    println!("other: {}", other);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(base_counts(b""), (0, 0, 0, 0, 0));
    }

    #[test]
    fn one_of_each() {
        assert_eq!(base_counts(b"ACGT"), (1, 1, 1, 1, 0));
    }

    #[test]
    fn case_insensitive() {
        assert_eq!(base_counts(b"acgt"), (1, 1, 1, 1, 0));
    }

    #[test]
    fn mixed_case() {
        assert_eq!(base_counts(b"AaCcGgTt"), (2, 2, 2, 2, 0));
    }

    #[test]
    fn with_n() {
        assert_eq!(base_counts(b"ACNGT"), (1, 1, 1, 1, 1));
    }

    #[test]
    fn with_garbage() {
        assert_eq!(base_counts(b"AXYZ"), (1, 0, 0, 0, 3));
    }
}
