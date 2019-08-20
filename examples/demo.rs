use std::io::Result;

use excel_writer::{Context, Excel};

fn main() -> Result<()> {
    let mut excel = Excel::default();
    let mut context = Context::default();

    let sheet = excel.workbook().add_sheet(Some("hello"));

    let row = sheet.add_row();

    row.add_str_cell(&mut context, "cell1");
    row.add_str_cell(&mut context, "cell2");

    excel.write_to_file(&context, "demo.xlsx")?;

    Ok(())
}
