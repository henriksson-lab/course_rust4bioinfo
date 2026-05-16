use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BedRecord {
    pub chrom: String,
    pub start: u64,
    pub end: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BedError {
    TooFewFields,
    InvalidStart,
    InvalidEnd,
    EndBeforeStart,
}

/// Parse one tab-separated BED line (first three fields: chrom, start, end).
pub fn parse_bed_line(line: &str) -> Result<BedRecord, BedError> {
    // TODO:
    //   - split on '\t', take the first three pieces with .next() and
    //     turn None into BedError::TooFewFields via ok_or
    //   - parse start and end as u64 with .parse(); map_err to InvalidStart
    //     / InvalidEnd
    //   - reject end < start
    //   - return Ok(BedRecord { chrom: chrom.to_string(), start, end })
    let _ = line;
    Err(BedError::TooFewFields)
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: bed-parse <PATH>");
        std::process::exit(1);
    }
    let file = File::open(&args[1])?;
    let reader = BufReader::new(file);

    let mut total_length: u64 = 0;
    let mut total_records: usize = 0;
    let mut errors: usize = 0;
    for line in reader.lines() {
        let line = line?;
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        match parse_bed_line(&line) {
            Ok(r) => {
                total_length += r.end - r.start;
                total_records += 1;
            }
            Err(e) => {
                eprintln!("parse error: {:?} on line {:?}", e, line);
                errors += 1;
            }
        }
    }
    println!("records: {}", total_records);
    println!("total length: {}", total_length);
    println!("errors: {}", errors);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_three_fields() {
        let got = parse_bed_line("chr1\t100\t200").unwrap();
        assert_eq!(got, BedRecord { chrom: "chr1".to_string(), start: 100, end: 200 });
    }

    #[test]
    fn valid_extra_fields_ignored() {
        // BED files often have extra columns (name, score, strand, ...). Ignore them.
        let got = parse_bed_line("chr2\t500\t600\tmy_gene\t0\t+").unwrap();
        assert_eq!(got, BedRecord { chrom: "chr2".to_string(), start: 500, end: 600 });
    }

    #[test]
    fn zero_length_is_valid() {
        // BED allows zero-length intervals (insertion points).
        let got = parse_bed_line("chrX\t42\t42").unwrap();
        assert_eq!(got, BedRecord { chrom: "chrX".to_string(), start: 42, end: 42 });
    }

    #[test]
    fn too_few_fields() {
        assert_eq!(parse_bed_line("chr1\t100"), Err(BedError::TooFewFields));
        assert_eq!(parse_bed_line(""), Err(BedError::TooFewFields));
    }

    #[test]
    fn invalid_start() {
        assert_eq!(parse_bed_line("chr1\tabc\t200"), Err(BedError::InvalidStart));
    }

    #[test]
    fn invalid_end() {
        assert_eq!(parse_bed_line("chr1\t100\txyz"), Err(BedError::InvalidEnd));
    }

    #[test]
    fn end_before_start() {
        assert_eq!(parse_bed_line("chr1\t200\t100"), Err(BedError::EndBeforeStart));
    }
}
