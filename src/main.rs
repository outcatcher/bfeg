mod calibrator;
mod generator;
mod random;

use clap::Parser;
use std::process;

/// Big Fucking Excel Generator — generates .xlsx files of arbitrary size with random data.
#[derive(Parser)]
#[command(version)]
struct Args {
    /// Target file size, e.g. "128M", "512M", "1.5G", "2G"
    #[arg(long, default_value = "128M")]
    size: String,

    /// Output file path
    #[arg(long, default_value = "random_data.xlsx")]
    output: String,

    /// Number of columns (default: auto-picked for decent row count)
    #[arg(long)]
    cols: Option<u16>,
}

fn parse_size(raw: &str) -> Result<u64, String> {
    let s = raw.trim().to_uppercase();
    if s.is_empty() {
        return Err("empty size string".into());
    }

    let (num_part, mult): (&str, u64) = if let Some(rest) = s.strip_suffix('G') {
        (rest, 1_073_741_824)
    } else if let Some(rest) = s.strip_suffix('M') {
        (rest, 1_048_576)
    } else if let Some(rest) = s.strip_suffix('K') {
        (rest, 1_024)
    } else {
        (s.as_str(), 1)
    };

    let num: f64 = num_part
        .parse()
        .map_err(|_| format!("invalid size: '{}'", s))?;

    if num <= 0.0 {
        return Err(format!("size must be positive: '{}'", s));
    }

    Ok((num * mult as f64) as u64)
}

fn pick_cols(cols: Option<u16>) -> u16 {
    match cols {
        Some(c) if c >= 4 => c,
        Some(_) => {
            eprintln!("warning: --cols too small, using 4");
            4
        }
        None => 100,
    }
}

fn main() {
    let args = Args::parse();

    let target_bytes = parse_size(&args.size).unwrap_or_else(|e| {
        eprintln!("error: {e}");
        process::exit(1);
    });

    let cols = pick_cols(args.cols);

    // -- calibration --
    let per_cell = calibrator::calibrate(cols).unwrap_or_else(|e| {
        eprintln!("calibration failed: {e}");
        process::exit(1);
    });

    // pad 5% to guard against random variance
    let padded = (target_bytes as f64 * 1.05) as u64;
    let cells_needed = (padded as f64 / per_cell).ceil() as u64;
    let rows = (cells_needed + cols as u64 - 1) / cols as u64;

    eprintln!(
        "target: {} bytes, {:.1} bytes/cell, {} cols × {} rows",
        target_bytes, per_cell, cols, rows,
    );

    generator::generate(&args.output, cols, rows).unwrap_or_else(|e| {
        eprintln!("generation failed: {e}");
        process::exit(1);
    });

    let actual = std::fs::metadata(&args.output)
        .map(|m| m.len())
        .unwrap_or(0);
    let pct = actual as f64 / target_bytes as f64 * 100.0;
    eprintln!(
        "done: {} ({:.1} MB, {:.1}% of target)",
        args.output,
        actual as f64 / 1_048_576.0,
        pct,
    );

    if actual < target_bytes {
        eprintln!(
            "warning: file is smaller than target ({} < {})",
            actual, target_bytes
        );
    }
}
