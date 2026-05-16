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

fn align(a: &[u8], b: &[u8]) -> Alignment {
    let n = a.len();
    let m = b.len();

    let mut score: Vec<Vec<i32>> = vec![vec![0; m + 1]; n + 1];
    let mut from: Vec<Vec<Direction>> = vec![vec![Direction::End; m + 1]; n + 1];

    // Phase 1: initialise borders.
    for i in 1..=n {
        score[i][0] = (i as i32) * GAP;
        from[i][0] = Direction::Up;
    }
    for j in 1..=m {
        score[0][j] = (j as i32) * GAP;
        from[0][j] = Direction::Left;
    }

    // Phase 2: fill interior.
    for i in 1..=n {
        for j in 1..=m {
            let s = if a[i - 1] == b[j - 1] { MATCH } else { MISMATCH };
            let diag = score[i - 1][j - 1] + s;
            let up = score[i - 1][j] + GAP;
            let left = score[i][j - 1] + GAP;
            if diag >= up && diag >= left {
                score[i][j] = diag;
                from[i][j] = Direction::Diag;
            } else if up >= left {
                score[i][j] = up;
                from[i][j] = Direction::Up;
            } else {
                score[i][j] = left;
                from[i][j] = Direction::Left;
            }
        }
    }

    // Phase 3: traceback.
    let mut aligned_a: Vec<u8> = Vec::new();
    let mut aligned_b: Vec<u8> = Vec::new();
    let (mut i, mut j) = (n, m);
    while i > 0 || j > 0 {
        match from[i][j] {
            Direction::Diag => {
                aligned_a.push(a[i - 1]);
                aligned_b.push(b[j - 1]);
                i -= 1;
                j -= 1;
            }
            Direction::Up => {
                aligned_a.push(a[i - 1]);
                aligned_b.push(b'-');
                i -= 1;
            }
            Direction::Left => {
                aligned_a.push(b'-');
                aligned_b.push(b[j - 1]);
                j -= 1;
            }
            Direction::End => break,
        }
    }
    aligned_a.reverse();
    aligned_b.reverse();

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
        assert_eq!(r.aligned_a.len(), r.aligned_b.len());
    }

    #[test]
    fn strip_gaps_recovers_input() {
        let r = align(b"AAACGT", b"ACGTTT");
        assert_eq!(strip_gaps(&r.aligned_a), b"AAACGT");
        assert_eq!(strip_gaps(&r.aligned_b), b"ACGTTT");
    }
}
