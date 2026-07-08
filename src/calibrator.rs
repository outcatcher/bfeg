use crate::random;
use indicatif::{ProgressBar, ProgressStyle};
use rust_xlsxwriter::Workbook;

/// Write a sample file of `sample_rows` and measure bytes-per-cell.
/// Cleans up the temp file on success.
pub fn calibrate(cols: u16, sample_rows: u64) -> Result<f64, Box<dyn std::error::Error>> {
    let calib_path = std::env::temp_dir().join("bfeg_calib.xlsx");

    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    // header row
    worksheet.write_row(0, 0, (1u16..=cols).map(|i| format!("col_{i}")))?;

    let pb = ProgressBar::new(sample_rows);
    pb.set_style(
        ProgressStyle::with_template(
            "  [{bar:40.blue}] {pos:>7}/{len} rows {msg}",
        )
        .unwrap()
        .progress_chars("=> "),
    );
    pb.set_message("calibrating...");

    // data rows
    for row_num in 1u64..=sample_rows {
        let row: Vec<String> = (0..cols).map(|_| random::random_value()).collect();
        worksheet.write_row(row_num as u32, 0, row)?;
        pb.inc(1);
    }

    pb.finish_and_clear();

    let spinner = ProgressBar::new_spinner();
    spinner.set_message("saving calibration sample...");
    spinner.enable_steady_tick(std::time::Duration::from_millis(100));

    let path = calib_path.to_str().ok_or("invalid temp path")?.to_owned();
    let handle = std::thread::spawn(move || workbook.save(&path));

    match handle.join().unwrap() {
        Ok(()) => spinner.finish_with_message("✅ sample saved"),
        Err(e) => {
            spinner.finish_with_message("sample save failed");
            return Err(Box::new(e));
        }
    }

    let file_size = std::fs::metadata(&calib_path)?.len();
    let total_cells = (sample_rows + 1) * cols as u64; // +1 header
    let per_cell = file_size as f64 / total_cells as f64;

    let _ = std::fs::remove_file(&calib_path);
    Ok(per_cell)
}
