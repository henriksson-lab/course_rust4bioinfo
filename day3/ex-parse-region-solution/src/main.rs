use std::env;

#[derive(Debug, Clone, PartialEq, Eq)]
enum ParseRegionError {
    MissingColon,
    MissingDash,
    InvalidStart,
    InvalidEnd,
    EndBeforeStart,
}

fn parse_region(s: &str) -> Result<(String, u64, u64), ParseRegionError> {
    let (chrom, rest) = s.split_once(':').ok_or(ParseRegionError::MissingColon)?;
    let (start_s, end_s) = rest.split_once('-').ok_or(ParseRegionError::MissingDash)?;
    let start: u64 = start_s.parse().map_err(|_| ParseRegionError::InvalidStart)?;
    let end:   u64 = end_s  .parse().map_err(|_| ParseRegionError::InvalidEnd)?;
    if end < start {
        return Err(ParseRegionError::EndBeforeStart);
    }
    Ok((chrom.to_string(), start, end))
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
