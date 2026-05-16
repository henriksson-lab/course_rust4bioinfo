use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

use noodles_fastq as fastq;

pub fn filter_fastq<R: BufRead, W: Write>(
    reader: R,
    writer: W,
    min_len: usize,
) -> std::io::Result<(usize, usize)> {
    let mut reader = fastq::io::Reader::new(reader);
    let mut writer = fastq::io::Writer::new(writer);
    let mut total = 0;
    let mut kept = 0;
    for result in reader.records() {
        let record = result?;
        total += 1;
        if record.sequence().len() >= min_len {
            writer.write_record(&record)?;
            kept += 1;
        }
    }
    Ok((kept, total))
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!("usage: fastq-filter <INPUT> <OUTPUT> <MIN_LEN>");
        std::process::exit(1);
    }
    let min_len: usize = args[3].parse().expect("MIN_LEN must be a non-negative integer");
    let reader = BufReader::new(File::open(&args[1])?);
    let writer = BufWriter::new(File::create(&args[2])?);
    let (kept, total) = filter_fastq(reader, writer, min_len)?;
    eprintln!("kept {} / {} records (min_len = {})", kept, total, min_len);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn keeps_long_drops_short() {
        let input: &[u8] = b"@a\nACGTACGT\n+\nIIIIIIII\n@b\nAC\n+\n!!\n@c\nGGGGGGGG\n+\nIIIIIIII\n";
        let mut out: Vec<u8> = Vec::new();
        let (kept, total) = filter_fastq(Cursor::new(input), &mut out, 5).unwrap();
        assert_eq!(total, 3);
        assert_eq!(kept, 2);
        let s = String::from_utf8(out).unwrap();
        assert!(s.contains("@a"));
        assert!(s.contains("@c"));
        assert!(!s.contains("@b"));
    }

    #[test]
    fn keeps_everything_when_min_zero() {
        let input: &[u8] = b"@a\nA\n+\n!\n@b\nAC\n+\n!!\n";
        let mut out: Vec<u8> = Vec::new();
        let (kept, total) = filter_fastq(Cursor::new(input), &mut out, 0).unwrap();
        assert_eq!(kept, 2);
        assert_eq!(total, 2);
    }

    #[test]
    fn empty_input_empty_output() {
        let input: &[u8] = b"";
        let mut out: Vec<u8> = Vec::new();
        let (kept, total) = filter_fastq(Cursor::new(input), &mut out, 10).unwrap();
        assert_eq!(kept, 0);
        assert_eq!(total, 0);
        assert!(out.is_empty());
    }
}
