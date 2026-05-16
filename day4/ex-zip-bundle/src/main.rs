use std::env;
use std::fs::File;
use std::io::{Read, Seek, Write};
use std::path::Path;

/// Write a `.zip` archive at `output_path` containing the given (name, body) entries.
pub fn write_zip_bundle<W: Write + Seek>(
    writer: W,
    entries: &[(&str, &[u8])],
) -> Result<(), Box<dyn std::error::Error>> {
    let mut _zw = zip::ZipWriter::new(writer);
    // TODO:
    //   - default file options: zip::write::FileOptions::default()
    //   - for each (name, body): _zw.start_file(*name, options)?
    //                            _zw.write_all(body)?
    //   - finish the archive with _zw.finish()?
    let _ = entries;
    Ok(())
}

/// Read all entries from a `.zip` archive and return them as (name, body) pairs.
pub fn read_zip_bundle<R: Read + Seek>(
    reader: R,
) -> Result<Vec<(String, Vec<u8>)>, Box<dyn std::error::Error>> {
    let mut _archive = zip::ZipArchive::new(reader)?;
    // TODO:
    //   - loop over 0.._archive.len(), open each entry with by_index(i)
    //   - capture entry.name() and the bytes read with read_to_end
    //   - push (name, body) onto the output Vec
    Ok(Vec::new())
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
        assert!(buf.len() > 22, "zip output looks empty (got {} bytes)", buf.len());

        let read_back = read_zip_bundle(Cursor::new(&buf)).unwrap();
        assert_eq!(read_back.len(), 2);
        assert_eq!(read_back[0].0, "a.txt");
        assert_eq!(read_back[0].1, b"hello");
        assert_eq!(read_back[1].0, "b.tsv");
        assert_eq!(read_back[1].1, b"x\ty\n1\t2\n");
    }

    #[test]
    fn zip_signature_present() {
        // A valid empty zip starts with bytes that are NOT the "PK" signature
        // because there are no entries — but writing one entry should put
        // a "PK\x03\x04" local file header at the start.
        let mut buf: Vec<u8> = Vec::new();
        let entries: &[(&str, &[u8])] = &[("only.txt", b"!")];
        write_zip_bundle(Cursor::new(&mut buf), entries).unwrap();
        assert_eq!(&buf[..4], b"PK\x03\x04", "zip local-file-header magic missing");
    }

    #[test]
    fn empty_bundle_roundtrips_to_zero_entries() {
        let mut buf: Vec<u8> = Vec::new();
        write_zip_bundle(Cursor::new(&mut buf), &[]).unwrap();
        let read_back = read_zip_bundle(Cursor::new(&buf)).unwrap();
        assert!(read_back.is_empty());
    }
}
