use crate::random;
use indicatif::{ProgressBar, ProgressStyle};
use rust_xlsxwriter::Workbook;

/// Generate the full .xlsx file in a single streaming pass.
pub fn generate(path: &str, cols: u16, rows: u64) -> Result<(), Box<dyn std::error::Error>> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    // header row
    worksheet.write_row(0, 0, (1u16..=cols).map(|i| format!("col_{i}")))?;

    let pb = ProgressBar::new(rows);
    pb.set_style(
        ProgressStyle::with_template(
            "  [{bar:40.blue}] {pos:>7}/{len} rows {msg}",
        )
        .unwrap()
        .progress_chars("=> "),
    );

    // data rows
    for row_idx in 0u64..rows {
        let row: Vec<String> = (0..cols).map(|_| random::random_value()).collect();
        worksheet.write_row((row_idx + 1) as u32, 0, row)?;
        pb.inc(1);
    }

    pb.finish_and_clear();

    let spinner = ProgressBar::new_spinner();
    spinner.set_message("saving .xlsx, it will take some time");
    spinner.enable_steady_tick(std::time::Duration::from_millis(100));

    let path = path.to_owned();
    let handle = std::thread::spawn(move || workbook.save(&path));

    match handle.join().unwrap() {
        Ok(()) => spinner.finish_with_message("saved"),
        Err(e) => {
            spinner.finish_with_message("save failed");
            return Err(Box::new(e));
        }
    }

    Ok(())
}
