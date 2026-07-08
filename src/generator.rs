use crate::random;
use rust_xlsxwriter::Workbook;

/// Generate the full .xlsx file in a single streaming pass.
pub fn generate(path: &str, cols: u16, rows: u64) -> Result<(), Box<dyn std::error::Error>> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    // header row
    worksheet.write_row(0, 0, (1u16..=cols).map(|i| format!("col_{i}")))?;

    // data rows
    let report_every = (rows / 20).max(1);
    for row_idx in 0u64..rows {
        let row: Vec<String> = (0..cols).map(|_| random::random_value()).collect();
        worksheet.write_row((row_idx + 1) as u32, 0, row)?;

        if (row_idx + 1) % report_every == 0 {
            eprintln!("  {}/{} rows written", row_idx + 1, rows);
        }
    }

    workbook.save(path)?;
    Ok(())
}
