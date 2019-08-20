use std::{
    collections::HashMap,
    io::{Error, ErrorKind, Result, Seek, Write},
};

use lazy_static::lazy_static;
use tera::{Context, Tera, Value};
use zip::{write::FileOptions, CompressionMethod, ZipWriter};

// _rels/.rels                      Package-relationship item
// xl/_rels/workbook.xml.rels       Part-relationship item
// xl/theme/theme1.xml              Theme part
// xl/worksheets/sheet1.xml         Worksheet part
// xl/sharedStrings.xml             Shared String Table part
// xl/styles.xml                    Styles part
// xl/workbook.xml                  Workbook part
// [Content_Types].xml              Content-type item

use crate::{context, excel::Excel};

lazy_static! {
    pub(crate) static ref TPLS: Tera = {
        let mut tera = Tera::default();

        tera.register_filter("cell_type", cell_type);

        tera.add_raw_template("_rels/.rels", &include_str!("../template/_rels/.rels"))
            .unwrap();
        tera.add_raw_template(
            "xl/_rels/workbook.xml.rels",
            &include_str!("../template/xl/_rels/workbook.xml.rels"),
        )
        .unwrap();
        tera.add_raw_template(
            "xl/theme/theme1.xml",
            &include_str!("../template/xl/theme/theme1.xml"),
        )
        .unwrap();
        tera.add_raw_template(
            "xl/worksheets/sheet1.xml",
            &include_str!("../template/xl/worksheets/sheet1.xml"),
        )
        .unwrap();
        tera.add_raw_template(
            "xl/sharedStrings.xml",
            &include_str!("../template/xl/sharedStrings.xml"),
        )
        .unwrap();
        tera.add_raw_template("xl/styles.xml", &include_str!("../template/xl/styles.xml"))
            .unwrap();
        tera.add_raw_template(
            "xl/workbook.xml",
            &include_str!("../template/xl/workbook.xml"),
        )
        .unwrap();
        tera.add_raw_template(
            "[Content_Types].xml",
            &include_str!("../template/[Content_Types].xml"),
        )
        .unwrap();
        tera
    };
}

pub(crate) fn write_to<W: Write + Seek>(
    excel: &Excel,
    excel_context: &context::Context,
    writer: W,
) -> Result<W> {
    let mut zip_writer = ZipWriter::new(writer);

    let file_options = FileOptions::default().compression_method(CompressionMethod::Deflated);

    let sheets = &excel.workbook.sheets.sheets;

    {
        let context = Context::new();
        write_zip_file("_rels/.rels", None, context, &mut zip_writer, file_options)?;
    }

    {
        let mut context = Context::new();
        context.insert("sheets", sheets);
        write_zip_file(
            "xl/_rels/workbook.xml.rels",
            None,
            context,
            &mut zip_writer,
            file_options,
        )?;
    }

    {
        let context = Context::new();
        write_zip_file(
            "xl/theme/theme1.xml",
            None,
            context,
            &mut zip_writer,
            file_options,
        )?;
    }

    for sheet in sheets {
        let mut context = Context::new();
        context.insert("rows", &sheet.rows);
        context.insert("column_ids_cache", &excel_context.column_ids_cache);

        let zip_file_name = format!("xl/worksheets/sheet{}.xml", sheet.index.0 + 1);
        write_zip_file(
            "xl/worksheets/sheet1.xml",
            Some(&zip_file_name),
            context,
            &mut zip_writer,
            file_options,
        )?;
    }

    {
        let mut context = Context::new();
        let string_len = excel_context.shared_strings.strings.len();
        let mut strings = HashMap::with_capacity(string_len);

        for (string, string_index) in &excel_context.shared_strings.strings {
            strings.insert(string_index, string.as_ref());
        }
        context.insert("string_len", &string_len);
        context.insert("strings", &strings);
        write_zip_file(
            "xl/sharedStrings.xml",
            None,
            context,
            &mut zip_writer,
            file_options,
        )?;
    }

    {
        let context = Context::new();
        write_zip_file(
            "xl/styles.xml",
            None,
            context,
            &mut zip_writer,
            file_options,
        )?;
    }

    {
        let mut context = Context::new();
        context.insert("sheets", sheets);
        write_zip_file(
            "xl/workbook.xml",
            None,
            context,
            &mut zip_writer,
            file_options,
        )?;
    }

    {
        let mut context = Context::new();
        context.insert("sheets", sheets);
        write_zip_file(
            "[Content_Types].xml",
            None,
            context,
            &mut zip_writer,
            file_options,
        )?;
    }

    zip_writer.finish().map_err(Into::into)
}

fn write_zip_file<W: Write + Seek>(
    tpl_file_name: &str,
    zip_file_name: Option<&str>,
    context: Context,
    zip_writer: &mut ZipWriter<W>,
    file_options: FileOptions,
) -> Result<()> {
    let file_content = match TPLS.render(tpl_file_name, &context) {
        Ok(file_content) => file_content,
        Err(err) => {
            return Err(Error::new(ErrorKind::InvalidData, format!("{:?}", err)));
        }
    };

    let file_name = zip_file_name.unwrap_or(tpl_file_name);

    zip_writer.start_file(file_name, file_options)?;
    zip_writer.write_all(file_content.as_bytes())
}

fn cell_type(value: Value, _: HashMap<String, Value>) -> tera::Result<Value> {
    let new_value = match value {
        Value::String(string_value) => match string_value.as_str() {
            "String" => Value::String("s".into()),
            "Number" => Value::String("n".into()),
            _ => Value::String(string_value),
        },
        _ => value,
    };

    Ok(new_value)
}
