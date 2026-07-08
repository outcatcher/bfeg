use crate::random;
use rust_xlsxwriter::Workbook;

const CALIB_ROWS: u64 = 100;
const CALIB_PATH: &str = "/tmp/bfeg_calib.xlsx";

/// Write a small sample file and measure bytes-per-cell.
/// Cleans up the temp file on success.
pub fn calibrate(cols: u16) -> Result<f64, Box<dyn std::error::Error>> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    // header row
    worksheet.write_row(0, 0, (1u16..=cols).map(|i| format!("col_{i}")))?;

    // data rows
    for row_num in 1u64..=CALIB_ROWS {
        let row: Vec<String> = (0..cols).map(|_| random::random_value()).collect();
        worksheet.write_row(row_num as u32, 0, row)?;
    }

    workbook.save(CALIB_PATH)?;

    let file_size = std::fs::metadata(CALIB_PATH)?.len();
    let total_cells = (CALIB_ROWS + 1) * cols as u64; // +1 header
    let per_cell = file_size as f64 / total_cells as f64;

    let _ = std::fs::remove_file(CALIB_PATH);
    Ok(per_cell)
}
