use std::{borrow::Cow, collections::HashMap};

use excel_column_id::ColumnId;

use crate::{
    cell::ColIndex,
    shared_strings::{SharedStringIndex, SharedStrings},
};

#[derive(Default)]
pub struct Context {
    pub(crate) shared_strings: SharedStrings,
    pub(crate) column_ids_cache: HashMap<ColIndex, ColumnId>,
}

impl Context {
    pub(crate) fn add_shared_string(&mut self, value: Cow<'static, str>) -> SharedStringIndex {
        self.shared_strings.insert(value)
    }

    pub(crate) fn add_col_index(&mut self, col_index: ColIndex) {
        self.column_ids_cache
            .entry(col_index)
            .or_insert_with(|| ColumnId::from(col_index.0 + 1));
    }
}
