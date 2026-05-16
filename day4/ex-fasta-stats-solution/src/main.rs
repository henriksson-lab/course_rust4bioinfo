use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

use noodles_fasta as fasta;

#[derive(Debug, Clone, PartialEq)]
pub struct ContigStat {
    pub name: String,
    pub length: usize,
    pub gc: f64,
}

pub fn fasta_stats<R: BufRead>(reader: R) -> std::io::Result<Vec<ContigStat>> {
    let mut reader = fasta::io::Reader::new(reader);
    let mut out = Vec::new();
    for result in reader.records() {
        let record = result?;
        let name = std::str::from_utf8(record.name())
            .map_err(|_| std::io::Error::other("non-UTF-8 record name"))?
            .to_string();
        let seq: &[u8] = record.sequence().as_ref();
        let length = seq.len();
        let gc_count = seq
            .iter()
            .filter(|&&b| matches!(b, b'G' | b'C' | b'g' | b'c'))
            .count();
        let gc = if length == 0 {
            0.0
        } else {
            gc_count as f64 / length as f64
        };
        out.push(ContigStat { name, length, gc });
    }
    Ok(out)
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: fasta-stats <PATH>");
        std::process::exit(1);
    }
    let reader = BufReader::new(File::open(&args[1])?);
    for stat in fasta_stats(reader)? {
        println!(
            "{}\t{}\t{:.2}%",
            stat.name,
            stat.length,
            stat.gc * 100.0
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn empty_input() {
        let stats = fasta_stats(Cursor::new(b"")).unwrap();
        assert!(stats.is_empty());
    }

    #[test]
    fn single_record() {
        let data: &[u8] = b">seq1\nACGTACGT\n";
        let stats = fasta_stats(Cursor::new(data)).unwrap();
        assert_eq!(stats.len(), 1);
        assert_eq!(stats[0].name, "seq1");
        assert_eq!(stats[0].length, 8);
        assert!((stats[0].gc - 0.5).abs() < 1e-9);
    }

    #[test]
    fn multiline_sequence_is_concatenated() {
        let data: &[u8] = b">seq1\nACGT\nACGT\nACGT\n";
        let stats = fasta_stats(Cursor::new(data)).unwrap();
        assert_eq!(stats[0].length, 12);
        assert!((stats[0].gc - 0.5).abs() < 1e-9);
    }

    #[test]
    fn multiple_records() {
        let data: &[u8] = b">seq1\nACGT\n>seq2\nGGGGCCCC\n>seq3\nAAAATTTT\n";
        let stats = fasta_stats(Cursor::new(data)).unwrap();
        assert_eq!(stats.len(), 3);
        assert_eq!(stats[0].name, "seq1");
        assert_eq!(stats[1].name, "seq2");
        assert_eq!(stats[2].name, "seq3");
        assert!((stats[1].gc - 1.0).abs() < 1e-9);
        assert!((stats[2].gc - 0.0).abs() < 1e-9);
    }
}
