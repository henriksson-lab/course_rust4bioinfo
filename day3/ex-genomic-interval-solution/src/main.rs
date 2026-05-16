use std::env;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Strand {
    Plus,
    Minus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct GenomicInterval {
    chrom: String,
    start: u64,
    end: u64,
    strand: Strand,
}

impl GenomicInterval {
    fn length(&self) -> u64 {
        self.end - self.start
    }

    fn overlaps(&self, other: &GenomicInterval) -> bool {
        self.chrom == other.chrom
            && self.start < other.end
            && other.start < self.end
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 5 {
        eprintln!("usage: genomic-interval <CHROM> <START> <END> <+|->");
        std::process::exit(1);
    }
    let strand = match args[4].as_str() {
        "+" => Strand::Plus,
        "-" => Strand::Minus,
        s => {
            eprintln!("strand must be + or -, got {:?}", s);
            std::process::exit(1);
        }
    };
    let interval = GenomicInterval {
        chrom: args[1].clone(),
        start: args[2].parse().expect("START must be a non-negative integer"),
        end:   args[3].parse().expect("END must be a non-negative integer"),
        strand,
    };
    println!("{:?}", interval);
    println!("length: {}", interval.length());
    println!("self-overlap: {}", interval.overlaps(&interval));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn iv(chrom: &str, start: u64, end: u64) -> GenomicInterval {
        GenomicInterval {
            chrom: chrom.to_string(),
            start,
            end,
            strand: Strand::Plus,
        }
    }

    #[test]
    fn length_basic() {
        assert_eq!(iv("chr1", 100, 200).length(), 100);
    }

    #[test]
    fn length_empty() {
        assert_eq!(iv("chr1", 100, 100).length(), 0);
    }

    #[test]
    fn overlap_overlapping() {
        assert!(iv("chr1", 100, 200).overlaps(&iv("chr1", 150, 250)));
    }

    #[test]
    fn overlap_disjoint() {
        assert!(!iv("chr1", 100, 200).overlaps(&iv("chr1", 200, 300)));
    }

    #[test]
    fn overlap_one_contains_other() {
        assert!(iv("chr1", 100, 500).overlaps(&iv("chr1", 200, 300)));
    }

    #[test]
    fn overlap_different_chroms() {
        assert!(!iv("chr1", 100, 200).overlaps(&iv("chr2", 100, 200)));
    }
}
