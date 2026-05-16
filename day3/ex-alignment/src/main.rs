use std::env;

const MATCH: i32 = 1;
const MISMATCH: i32 = -1;
const GAP: i32 = -1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Diag,
    Up,
    Left,
    End,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Alignment {
    score: i32,
    aligned_a: Vec<u8>,
    aligned_b: Vec<u8>,
}

/// Needleman–Wunsch global alignment with fixed scoring (+1 / -1 / -1).
fn align(a: &[u8], b: &[u8]) -> Alignment {
    let n = a.len();
    let m = b.len();

    let mut score: Vec<Vec<i32>> = vec![vec![0; m + 1]; n + 1];
    let mut from: Vec<Vec<Direction>> = vec![vec![Direction::End; m + 1]; n + 1];

    // TODO (phase 1 — initialise):
    //   set score[i][0] = i * GAP, from[i][0] = Direction::Up   for 1..=n
    //   set score[0][j] = j * GAP, from[0][j] = Direction::Left for 1..=m
    //   (score[0][0] stays 0; from[0][0] stays End)

    // TODO (phase 2 — fill):
    //   for i in 1..=n { for j in 1..=m {
    //     let s = MATCH if a[i-1] == b[j-1] else MISMATCH;
    //     diag = score[i-1][j-1] + s
    //     up   = score[i-1][j]   + GAP
    //     left = score[i][j-1]   + GAP
    //     pick the max; record the matching Direction
    //   } }

    // TODO (phase 3 — traceback from (n, m) to (0, 0)):
    //   build aligned_a and aligned_b in reverse, then reverse() them.
    let aligned_a: Vec<u8> = Vec::new();
    let aligned_b: Vec<u8> = Vec::new();

    let _ = (a, b, &mut score, &mut from);

    Alignment {
        score: score[n][m],
        aligned_a,
        aligned_b,
    }
}

fn strip_gaps(s: &[u8]) -> Vec<u8> {
    s.iter().copied().filter(|&b| b != b'-').collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("usage: align <SEQ_A> <SEQ_B>");
        std::process::exit(1);
    }
    let r = align(args[1].as_bytes(), args[2].as_bytes());
    println!("score: {}", r.score);
    println!("{}", std::str::from_utf8(&r.aligned_a).expect("ASCII"));
    println!("{}", std::str::from_utf8(&r.aligned_b).expect("ASCII"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identical_sequences() {
        let r = align(b"ACGT", b"ACGT");
        assert_eq!(r.score, 4);
        assert_eq!(r.aligned_a, b"ACGT");
        assert_eq!(r.aligned_b, b"ACGT");
    }

    #[test]
    fn single_substitution_no_gaps() {
        let r = align(b"ACGT", b"AGGT");
        assert_eq!(r.score, 2);
        assert_eq!(r.aligned_a, b"ACGT");
        assert_eq!(r.aligned_b, b"AGGT");
    }

    #[test]
    fn empty_both() {
        let r = align(b"", b"");
        assert_eq!(r.score, 0);
        assert_eq!(r.aligned_a, b"");
        assert_eq!(r.aligned_b, b"");
    }

    #[test]
    fn empty_a_full_gap_in_a() {
        let r = align(b"", b"ACGT");
        assert_eq!(r.score, -4);
        assert_eq!(r.aligned_a, b"----");
        assert_eq!(r.aligned_b, b"ACGT");
    }

    #[test]
    fn empty_b_full_gap_in_b() {
        let r = align(b"ACGT", b"");
        assert_eq!(r.score, -4);
        assert_eq!(r.aligned_a, b"ACGT");
        assert_eq!(r.aligned_b, b"----");
    }

    #[test]
    fn alignment_lengths_match() {
        let r = align(b"AAACGT", b"ACGTTT");
        assert_eq!(
            r.aligned_a.len(),
            r.aligned_b.len(),
            "aligned sequences must have the same length"
        );
    }

    #[test]
    fn strip_gaps_recovers_input() {
        let r = align(b"AAACGT", b"ACGTTT");
        assert_eq!(strip_gaps(&r.aligned_a), b"AAACGT");
        assert_eq!(strip_gaps(&r.aligned_b), b"ACGTTT");
    }
}
