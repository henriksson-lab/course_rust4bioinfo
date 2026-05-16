use std::env;

/// Convert a Phred+33 quality byte (as found in a FASTQ file) to its
/// numeric quality score.
///
/// Panics if the byte is outside the valid Phred+33 range (33..=126).
fn phred_score(byte: u8) -> u8 {
    // TODO:
    //   1. if `byte` is below 33 or above 126, panic with a clear message
    //      that includes both the numeric value and the character form
    //   2. otherwise, return `byte - 33`
    let _ = byte;
    0
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 || args[1].len() != 1 {
        eprintln!("usage: phred <single ASCII character>");
        std::process::exit(1);
    }
    let byte = args[1].as_bytes()[0];
    let score = phred_score(byte);
    println!("byte {:?} -> Phred score {}", byte as char, score);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lowest_quality() {
        assert_eq!(phred_score(b'!'), 0);
    }

    #[test]
    fn typical_high_quality() {
        assert_eq!(phred_score(b'I'), 40);
    }

    #[test]
    fn highest_quality() {
        assert_eq!(phred_score(b'~'), 93);
    }

    #[test]
    #[should_panic]
    fn below_range_panics() {
        let _ = phred_score(32);
    }

    #[test]
    #[should_panic]
    fn above_range_panics() {
        let _ = phred_score(127);
    }
}
