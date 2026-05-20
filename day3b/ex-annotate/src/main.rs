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
        // TODO: return e.g. "chr1:1000-2000 (1000 bp)"
        // Use format! with self.chrom, self.start, self.end, and (self.end - self.start)
        String::new()
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
        // TODO: return "forward" / "reverse" / "unknown" depending on the variant
        String::new()
    }
}

/// A DNA sequence — owned bytes.
struct Sequence {
    bases: Vec<u8>,
}

impl Annotate for Sequence {
    fn description(&self) -> String {
        // TODO: return e.g. "ACGT (length 4, GC 50%)"
        // Hint: count bytes that are b'G' or b'C', compute fraction.
        //       If empty, return "(empty sequence)".
        String::new()
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
