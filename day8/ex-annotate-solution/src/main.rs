/// A trait whose implementers know how to describe themselves in one line.
trait Annotate {
    fn description(&self) -> String;
}

/// A half-open genomic interval `chrom:start-end`.
struct GenomicInterval {
    chrom: String,
    start: u64,
    end: u64,
}

impl Annotate for GenomicInterval {
    fn description(&self) -> String {
        format!(
            "{}:{}-{} ({} bp)",
            self.chrom,
            self.start,
            self.end,
            self.end - self.start
        )
    }
}

#[derive(Clone, Copy)]
enum Strand {
    Forward,
    Reverse,
    Unknown,
}

impl Annotate for Strand {
    fn description(&self) -> String {
        match self {
            Strand::Forward => "forward".to_string(),
            Strand::Reverse => "reverse".to_string(),
            Strand::Unknown => "unknown".to_string(),
        }
    }
}

/// A DNA sequence — owned bytes.
struct Sequence {
    bases: Vec<u8>,
}

impl Annotate for Sequence {
    fn description(&self) -> String {
        if self.bases.is_empty() {
            "(empty sequence)".to_string()
        } else {
            let gc = self
                .bases
                .iter()
                .filter(|&&b| b == b'G' || b == b'C')
                .count();
            let pct = (gc * 100) / self.bases.len();
            let dna = std::str::from_utf8(&self.bases).unwrap_or("(non-ASCII)");
            format!("{} (length {}, GC {}%)", dna, self.bases.len(), pct)
        }
    }
}

fn main() {
    let iv = GenomicInterval { chrom: "chr1".to_string(), start: 1000, end: 2000 };
    let s = Strand::Forward;
    let seq = Sequence { bases: b"ACGT".to_vec() };

    println!("{}", iv.description());
    println!("{}", s.description());
    println!("{}", seq.description());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interval_basic() {
        let iv = GenomicInterval { chrom: "chr1".to_string(), start: 1000, end: 2000 };
        assert_eq!(iv.description(), "chr1:1000-2000 (1000 bp)");
    }

    #[test]
    fn interval_singleton() {
        let iv = GenomicInterval { chrom: "chrX".to_string(), start: 5, end: 6 };
        assert_eq!(iv.description(), "chrX:5-6 (1 bp)");
    }

    #[test]
    fn strand_forward() {
        assert_eq!(Strand::Forward.description(), "forward");
    }

    #[test]
    fn strand_reverse() {
        assert_eq!(Strand::Reverse.description(), "reverse");
    }

    #[test]
    fn strand_unknown() {
        assert_eq!(Strand::Unknown.description(), "unknown");
    }

    #[test]
    fn sequence_acgt() {
        let seq = Sequence { bases: b"ACGT".to_vec() };
        assert_eq!(seq.description(), "ACGT (length 4, GC 50%)");
    }

    #[test]
    fn sequence_empty() {
        let seq = Sequence { bases: Vec::new() };
        assert_eq!(seq.description(), "(empty sequence)");
    }
}
