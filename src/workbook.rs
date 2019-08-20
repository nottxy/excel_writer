use std::borrow::Cow;

use crate::sheet::{Sheet, Sheets};

#[derive(Default)]
pub struct Workbook {
    pub(crate) sheets: Sheets,
}

impl Workbook {
    pub fn add_sheet<S: Into<Cow<'static, str>>>(&mut self, sheet_name: Option<S>) -> &mut Sheet {
        self.sheets.add_sheet(sheet_name)
    }
}
