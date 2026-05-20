use std::env;
use std::fmt;

/// A half-open genomic interval `chrom:start-end` (BED-style, end exclusive).
struct GenomicInterval {
    chrom: String,
    start: u64,
    end: u64,
}

impl fmt::Display for GenomicInterval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: use write! to format self as "chrom:start-end"
        // Example: write!(f, "{}:{}-{}", self.chrom, self.start, self.end)
        let _ = f;
        Ok(())
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!("usage: display <CHROM> <START> <END>");
        std::process::exit(1);
    }
    let iv = GenomicInterval {
        chrom: args[1].clone(),
        start: args[2].parse().expect("START must be u64"),
        end:   args[3].parse().expect("END must be u64"),
    };
    println!("{}", iv);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chr1_simple() {
        let iv = GenomicInterval { chrom: "chr1".to_string(), start: 1000, end: 2000 };
        assert_eq!(format!("{}", iv), "chr1:1000-2000");
    }

    #[test]
    fn chrX_short() {
        let iv = GenomicInterval { chrom: "chrX".to_string(), start: 0, end: 1 };
        assert_eq!(format!("{}", iv), "chrX:0-1");
    }

    #[test]
    fn long_name() {
        let iv = GenomicInterval { chrom: "scaffold_007".to_string(), start: 12_345, end: 67_890 };
        assert_eq!(format!("{}", iv), "scaffold_007:12345-67890");
    }

    #[test]
    fn empty_chrom() {
        let iv = GenomicInterval { chrom: "".to_string(), start: 0, end: 0 };
        assert_eq!(format!("{}", iv), ":0-0");
    }
}
