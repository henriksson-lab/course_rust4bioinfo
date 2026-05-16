use std::env;

/// Return the largest interior slice of a Phred+33-encoded quality string
/// whose endpoints both pass a minimum quality score.
///
/// This trims low-quality bases from the start and the end, but leaves
/// low-quality positions in the middle alone. The returned slice is a
/// view into `qual` — no bytes are copied.
fn quality_trim(qual: &[u8], min_score: u8) -> &[u8] {
    // TODO:
    //   1. compute the byte threshold = min_score + 33
    //   2. walk a `start` index forward while qual[start] < threshold
    //      (and start is still in range!)
    //   3. walk an `end` index backward while qual[end - 1] < threshold
    //      (and end is still greater than start!)
    //   4. return &qual[start..end]
    let _ = min_score;
    qual
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("usage: quality-trim <QUAL-STRING> <MIN-SCORE>");
        std::process::exit(1);
    }
    let qual = args[1].as_bytes();
    let min_score: u8 = args[2].parse().expect("MIN-SCORE must be 0..=93");
    let trimmed = quality_trim(qual, min_score);
    println!(
        "trimmed: {:?}  ({} -> {} bytes)",
        std::str::from_utf8(trimmed).expect("ASCII input"),
        qual.len(),
        trimmed.len(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(quality_trim(b"", 20), b"");
    }

    #[test]
    fn all_pass() {
        // 'I' = ASCII 73 = score 40, well above threshold 20
        assert_eq!(quality_trim(b"IIII", 20), b"IIII");
    }

    #[test]
    fn all_fail() {
        // '!' = ASCII 33 = score 0, below threshold 20
        assert_eq!(quality_trim(b"!!!!", 20), b"");
    }

    #[test]
    fn trim_start_only() {
        assert_eq!(quality_trim(b"!!IIII", 20), b"IIII");
    }

    #[test]
    fn trim_end_only() {
        assert_eq!(quality_trim(b"IIII!!", 20), b"IIII");
    }

    #[test]
    fn trim_both_ends() {
        assert_eq!(quality_trim(b"!!IIII!!", 30), b"IIII");
    }

    #[test]
    fn low_in_middle_is_kept() {
        // The function trims from the ends only. A low-quality base in
        // the middle stays put.
        assert_eq!(quality_trim(b"I!I", 30), b"I!I");
    }
}
