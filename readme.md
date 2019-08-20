# Excel Writer

Write to a `std::io::Write` or `std::fs::File` as `.xlsx` File

[crates-url]: https://crates.io/crates/excel_writer


## Usage

To use `excel_writer`, first add this to your `Cargo.toml`:

```toml
[dependencies]
excel_writer = "0.1"
```

Next, add this to your crate:

```rust
use excel_writer::{Context, Excel};

let mut excel = Excel::default();
let mut context = Context::default();

let sheet = excel.workbook().add_sheet(Some("first sheet"));

let row = sheet.add_row();

row.add_str_cell(&mut context, "cell1");
row.add_str_cell(&mut context, "cell2");

excel.write_to_file(&context, "demo.xlsx")?;
```


## Features

* Add Sheet
* Add Row
* Set Cell
* Merge cells

## License

This project is licensed under the [MIT license](LICENSE).

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in `bytes` by you, shall be licensed as MIT, without any additional
terms or conditions.
