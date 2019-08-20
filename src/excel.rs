use std::{
    fs::{File, OpenOptions},
    io::{self, Cursor, Result, Seek, Write},
    path::Path,
};

use crate::{context::Context, excel_writer, workbook::Workbook};

#[derive(Default)]
pub struct Excel {
    pub(crate) workbook: Workbook,
}

impl Excel {
    pub fn workbook(&mut self) -> &mut Workbook {
        &mut self.workbook
    }

    pub fn write_to<W: Write + Seek>(&self, excel_content: &Context, writer: W) -> Result<W> {
        excel_writer::write_to(self, excel_content, writer)
    }

    pub fn write_to_file<P: AsRef<Path>>(
        self,
        excel_content: &Context,
        file_path: P,
    ) -> Result<File> {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(file_path)?;

        let buffer = self
            .write_to(excel_content, Cursor::new(Vec::default()))?
            .into_inner();

        let mut read_buffer = Cursor::new(buffer);

        io::copy(&mut read_buffer, &mut file)?;

        Ok(file)
    }
}
