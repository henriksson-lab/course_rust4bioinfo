/// A trait that any biological "record" (FASTA, FASTQ, BED, ...) can implement.
trait BioRecord {
    /// The record's name or identifier.
    fn name(&self) -> &str;

    /// The record's length in bases (or sequence bytes for BED).
    fn length(&self) -> usize;
}

struct FastaRecord {
    name: String,
    sequence: String,
}

impl BioRecord for FastaRecord {
    fn name(&self) -> &str {
        // TODO: return &self.name
        ""
    }

    fn length(&self) -> usize {
        // TODO: return self.sequence.len()
        0
    }
}

struct FastqRecord {
    name: String,
    sequence: String,
    quality: String,
}

impl BioRecord for FastqRecord {
    fn name(&self) -> &str {
        // TODO: return &self.name
        ""
    }

    fn length(&self) -> usize {
        // TODO: return self.sequence.len()
        0
    }
}

struct BedRecord {
    chrom: String,
    start: u64,
    end: u64,
}

impl BioRecord for BedRecord {
    fn name(&self) -> &str {
        // TODO: return &self.chrom
        ""
    }

    fn length(&self) -> usize {
        // TODO: return (self.end - self.start) as usize
        0
    }
}

/// Produce a one-line summary for each record. (Provided — do not modify.)
fn summarize(records: &[Box<dyn BioRecord>]) -> Vec<String> {
    records.iter().map(|r| format!("{} ({} bp)", r.name(), r.length())).collect()
}

fn main() {
    let records: Vec<Box<dyn BioRecord>> = vec![
        Box::new(FastaRecord {
            name: "chr1".to_string(),
            sequence: "ACGTACGTACGT".to_string(),
        }),
        Box::new(FastqRecord {
            name: "read42".to_string(),
            sequence: "ACGTACGT".to_string(),
            quality: "IIIIIIII".to_string(),
        }),
        Box::new(BedRecord {
            chrom: "chr1".to_string(),
            start: 100,
            end: 200,
        }),
    ];

    for line in summarize(&records) {
        println!("{}", line);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_records() -> Vec<Box<dyn BioRecord>> {
        vec![
            Box::new(FastaRecord {
                name: "chr1".to_string(),
                sequence: "ACGTACGTACGT".to_string(),
            }),
            Box::new(FastqRecord {
                name: "read42".to_string(),
                sequence: "ACGTACGT".to_string(),
                quality: "IIIIIIII".to_string(),
            }),
            Box::new(BedRecord {
                chrom: "chr1".to_string(),
                start: 100,
                end: 200,
            }),
        ]
    }

    #[test]
    fn fasta_name() {
        let r = FastaRecord { name: "chr1".to_string(), sequence: "ACGT".to_string() };
        assert_eq!(r.name(), "chr1");
    }

    #[test]
    fn fasta_length() {
        let r = FastaRecord { name: "chr1".to_string(), sequence: "ACGTACGT".to_string() };
        assert_eq!(r.length(), 8);
    }

    #[test]
    fn fastq_length() {
        let r = FastqRecord {
            name: "read1".to_string(),
            sequence: "ACGT".to_string(),
            quality: "IIII".to_string(),
        };
        assert_eq!(r.length(), 4);
    }

    #[test]
    fn bed_length() {
        let r = BedRecord { chrom: "chr1".to_string(), start: 100, end: 200 };
        assert_eq!(r.length(), 100);
    }

    #[test]
    fn bed_name_is_chrom() {
        let r = BedRecord { chrom: "chrX".to_string(), start: 0, end: 5 };
        assert_eq!(r.name(), "chrX");
    }

    #[test]
    fn summarize_mixed() {
        let summaries = summarize(&sample_records());
        assert_eq!(summaries, vec![
            "chr1 (12 bp)".to_string(),
            "read42 (8 bp)".to_string(),
            "chr1 (100 bp)".to_string(),
        ]);
    }
}
