use std::env;

#[derive(Debug, Clone, PartialEq, Eq)]
enum ParseRegionError {
    MissingColon,
    MissingDash,
    InvalidStart,
    InvalidEnd,
    EndBeforeStart,
}

/// Parse a region string like "chr1:1000-2000" into (chrom, start, end).
fn parse_region(s: &str) -> Result<(String, u64, u64), ParseRegionError> {
    // TODO:
    //   1. split_once(':') to separate chrom from "start-end"
    //   2. split_once('-') to separate start_s from end_s
    //   3. parse start_s and end_s as u64; map their errors to our enum
    //   4. check end >= start
    //   5. return Ok((chrom.to_string(), start, end))
    //
    // Use the `?` operator on Result values to propagate errors.
    let _ = s;
    Err(ParseRegionError::MissingColon)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: parse-region <REGION-STRING>");
        std::process::exit(1);
    }
    match parse_region(&args[1]) {
        Ok((chrom, start, end)) => {
            println!("OK: {} from {} to {} (length {})", chrom, start, end, end - start);
        }
        Err(e) => {
            eprintln!("parse error: {:?}", e);
            std::process::exit(2);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_simple() {
        assert_eq!(
            parse_region("chr1:1000-2000"),
            Ok(("chr1".to_string(), 1000, 2000)),
        );
    }

    #[test]
    fn valid_zero_length() {
        assert_eq!(
            parse_region("chrX:42-42"),
            Ok(("chrX".to_string(), 42, 42)),
        );
    }

    #[test]
    fn valid_mt() {
        assert_eq!(
            parse_region("chrMT:1-16569"),
            Ok(("chrMT".to_string(), 1, 16569)),
        );
    }

    #[test]
    fn missing_colon() {
        assert_eq!(parse_region("chr1-1000-2000"), Err(ParseRegionError::MissingColon));
    }

    #[test]
    fn missing_dash() {
        assert_eq!(parse_region("chr1:1000_2000"), Err(ParseRegionError::MissingDash));
    }

    #[test]
    fn invalid_start() {
        assert_eq!(parse_region("chr1:abc-2000"), Err(ParseRegionError::InvalidStart));
    }

    #[test]
    fn invalid_end() {
        assert_eq!(parse_region("chr1:1000-xyz"), Err(ParseRegionError::InvalidEnd));
    }

    #[test]
    fn end_before_start() {
        assert_eq!(parse_region("chr1:2000-1000"), Err(ParseRegionError::EndBeforeStart));
    }
}
