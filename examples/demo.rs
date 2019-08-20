use std::io::Result;

use excel_writer::{ColIndex, Context, Excel, MergeCell, RowIndex};

fn main() -> Result<()> {
    let mut excel = Excel::default();
    let mut context = Context::default();

    let sheet = excel.workbook().add_sheet(Some("hello"));

    {
        let row = sheet.add_row();

        row.add_str_cell(&mut context, "cell1");
        row.add_str_cell(&mut context, "cell2");
    }

    {
        let row = sheet.get_row(RowIndex(1));
        row.set_str_cell(&mut context, ColIndex(0), "hello");
    }

    sheet.set_merge_cell(MergeCell {
        from_row_index: RowIndex(1),
        from_col_index: ColIndex(0),
        to_row_index: RowIndex(1),
        to_col_index: None,
    });

    excel.write_to_file(&context, "demo.xlsx")?;

    Ok(())
}
