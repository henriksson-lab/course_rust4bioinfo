use std::env;
use std::path::{Path, PathBuf};

use plotters::prelude::*;

pub fn plot_histogram(
    lengths: &[usize],
    bin_width: usize,
    output_path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let max = *lengths.iter().max().unwrap_or(&0);
    let n_bins = if max == 0 { 1 } else { max / bin_width + 1 };
    let mut counts: Vec<u32> = vec![0; n_bins];
    for &l in lengths {
        counts[l / bin_width] += 1;
    }
    let max_count = *counts.iter().max().unwrap_or(&0);

    let root = BitMapBackend::new(output_path, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("Read length distribution", ("sans-serif", 30))
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(50)
        .build_cartesian_2d(
            0u32..((n_bins * bin_width).max(1) as u32),
            0u32..(max_count + 1),
        )?;
    chart
        .configure_mesh()
        .x_desc("read length")
        .y_desc("count")
        .draw()?;
    chart.draw_series(counts.iter().enumerate().map(|(b, &c)| {
        let x0 = (b * bin_width) as u32;
        let x1 = ((b + 1) * bin_width) as u32;
        Rectangle::new([(x0, 0u32), (x1, c)], BLUE.mix(0.6).filled())
    }))?;
    root.present()?;
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
        assert_eq!(&bytes[..8], b"\x89PNG\r\n\x1A\n");
        let _ = fs::remove_file(&out);
    }

    #[test]
    fn empty_lengths_is_a_clean_no_crash() {
        let out = tmp_path("empty");
        let _ = fs::remove_file(&out);
        let result = plot_histogram(&[], 10, &out);
        assert!(result.is_ok());
        let _ = fs::remove_file(&out);
    }
}
