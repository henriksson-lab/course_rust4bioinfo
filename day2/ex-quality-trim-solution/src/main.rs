use std::env;

fn quality_trim(qual: &[u8], min_score: u8) -> &[u8] {
    let threshold = min_score + 33;
    let mut start: usize = 0;
    while start < qual.len() && qual[start] < threshold {
        start += 1;
    }
    let mut end: usize = qual.len();
    while end > start && qual[end - 1] < threshold {
        end -= 1;
    }
    &qual[start..end]
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
        assert_eq!(quality_trim(b"IIII", 20), b"IIII");
    }

    #[test]
    fn all_fail() {
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
        assert_eq!(quality_trim(b"I!I", 30), b"I!I");
    }
}
