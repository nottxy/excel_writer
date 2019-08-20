use std::borrow::Cow;

use serde::Serialize;

use crate::row::{Row, RowIndex, Rows};

#[derive(Default)]
pub(crate) struct Sheets {
    pub(crate) sheets: Vec<Sheet>,
}

#[derive(Serialize)]
pub struct SheetIndex(pub usize);

#[derive(Serialize)]
pub struct Sheet {
    pub(crate) index: SheetIndex,
    pub(crate) name: Cow<'static, str>,
    pub(crate) rows: Rows,
}

impl Sheets {
    pub fn add_sheet<S: Into<Cow<'static, str>>>(&mut self, sheet_name: Option<S>) -> &mut Sheet {
        let index = self.sheets.len();

        let name = sheet_name
            .map(Into::into)
            .unwrap_or_else(|| Cow::Owned(format!("Sheet{}", index + 1)));

        self.sheets.push(Sheet {
            index: SheetIndex(index),
            name,
            rows: Rows::default(),
        });

        self.sheets.last_mut().unwrap()
    }
}

impl Sheet {
    pub fn add_row(&mut self) -> &mut Row {
        self.rows.add_row()
    }

    pub fn get_row(&mut self, row_index: RowIndex) -> &mut Row {
        self.rows.get_row(row_index)
    }
}
