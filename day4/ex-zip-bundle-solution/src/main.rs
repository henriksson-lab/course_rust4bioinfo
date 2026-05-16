use std::env;
use std::fs::File;
use std::io::{Read, Seek, Write};
use std::path::Path;

pub fn write_zip_bundle<W: Write + Seek>(
    writer: W,
    entries: &[(&str, &[u8])],
) -> Result<(), Box<dyn std::error::Error>> {
    let mut zw = zip::ZipWriter::new(writer);
    let options = zip::write::FileOptions::default();
    for (name, body) in entries {
        zw.start_file(*name, options)?;
        zw.write_all(body)?;
    }
    zw.finish()?;
    Ok(())
}

pub fn read_zip_bundle<R: Read + Seek>(
    reader: R,
) -> Result<Vec<(String, Vec<u8>)>, Box<dyn std::error::Error>> {
    let mut archive = zip::ZipArchive::new(reader)?;
    let mut out: Vec<(String, Vec<u8>)> = Vec::with_capacity(archive.len());
    for i in 0..archive.len() {
        let mut entry = archive.by_index(i)?;
        let name = entry.name().to_string();
        let mut body = Vec::new();
        entry.read_to_end(&mut body)?;
        out.push((name, body));
    }
    Ok(out)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: zip-bundle <OUTPUT_ZIP>");
        std::process::exit(1);
    }
    let path = Path::new(&args[1]);
    let file = File::create(path)?;
    let entries: &[(&str, &[u8])] = &[
        ("summary.txt", b"all reads passed QC\n"),
        ("counts.tsv", b"sample\treads\nA\t1000\nB\t2000\n"),
        ("note.md", b"# Sample report\n\nFigures attached.\n"),
    ];
    write_zip_bundle(file, entries)?;
    println!("wrote {}", path.display());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn roundtrip_in_memory() {
        let mut buf: Vec<u8> = Vec::new();
        let entries: &[(&str, &[u8])] = &[
            ("a.txt", b"hello"),
            ("b.tsv", b"x\ty\n1\t2\n"),
        ];
        write_zip_bundle(Cursor::new(&mut buf), entries).unwrap();
        assert!(buf.len() > 22);
        let read_back = read_zip_bundle(Cursor::new(&buf)).unwrap();
        assert_eq!(read_back.len(), 2);
        assert_eq!(read_back[0].0, "a.txt");
        assert_eq!(read_back[0].1, b"hello");
        assert_eq!(read_back[1].0, "b.tsv");
        assert_eq!(read_back[1].1, b"x\ty\n1\t2\n");
    }

    #[test]
    fn zip_signature_present() {
        let mut buf: Vec<u8> = Vec::new();
        let entries: &[(&str, &[u8])] = &[("only.txt", b"!")];
        write_zip_bundle(Cursor::new(&mut buf), entries).unwrap();
        assert_eq!(&buf[..4], b"PK\x03\x04");
    }

    #[test]
    fn empty_bundle_roundtrips_to_zero_entries() {
        let mut buf: Vec<u8> = Vec::new();
        write_zip_bundle(Cursor::new(&mut buf), &[]).unwrap();
        let read_back = read_zip_bundle(Cursor::new(&buf)).unwrap();
        assert!(read_back.is_empty());
    }
}
