use std::env;
use std::path::{Path, PathBuf};

use plotters::prelude::*;

/// Plot a histogram of `lengths` to `output_path` (PNG).
///
/// Lengths are binned in fixed-width bins of `bin_width`. The plot has axes,
/// a caption, and one bar per non-empty bin.
pub fn plot_histogram(
    lengths: &[usize],
    bin_width: usize,
    output_path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    // TODO:
    //   1. compute max length, number of bins, count per bin, max count
    //   2. construct a BitMapBackend at output_path, fill with WHITE
    //   3. build a ChartBuilder with caption + axes + cartesian range
    //   4. draw one Rectangle per bin
    //   5. call root.present()
    let _ = (lengths, bin_width, output_path);
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("usage: plot-readlen <BIN_WIDTH> <OUTPUT_PNG>");
        std::process::exit(1);
    }
    let bin_width: usize = args[1].parse()?;
    let out: PathBuf = PathBuf::from(&args[2]);
    // Synthetic demo data — a triangular-ish length distribution.
    let lengths: Vec<usize> = (0..1000)
        .map(|i| 100 + (((i * 37) % 200) as usize))
        .collect();
    plot_histogram(&lengths, bin_width, &out)?;
    println!("wrote {}", out.display());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn tmp_path(name: &str) -> PathBuf {
        let mut p = std::env::temp_dir();
        p.push(format!("ex-plot-readlen-{}-{}.png", std::process::id(), name));
        p
    }

    #[test]
    fn produces_a_non_empty_png() {
        let out = tmp_path("simple");
        let _ = fs::remove_file(&out);
        let lengths: Vec<usize> = (0..200).map(|i| 50 + (i % 100)).collect();
        plot_histogram(&lengths, 10, &out).expect("plot should succeed");
        let bytes = fs::read(&out).expect("output file should exist");
        assert!(!bytes.is_empty(), "output is empty");
        // PNG magic: 89 50 4E 47 0D 0A 1A 0A
        assert_eq!(&bytes[..8], b"\x89PNG\r\n\x1A\n");
        let _ = fs::remove_file(&out);
    }

    #[test]
    fn empty_lengths_is_a_clean_no_crash() {
        let out = tmp_path("empty");
        let _ = fs::remove_file(&out);
        let result = plot_histogram(&[], 10, &out);
        // We don't require a particular output, but the function must
        // not panic or return an error for empty input.
        assert!(result.is_ok());
        let _ = fs::remove_file(&out);
    }
}
