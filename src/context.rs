use std::{borrow::Cow, collections::HashMap};

use excel_column_id::ColumnId;

use crate::{
    cell::ColumnIndex,
    shared_strings::{SharedStringIndex, SharedStrings},
};

#[derive(Default)]
pub struct Context {
    pub(crate) shared_strings: SharedStrings,
    pub(crate) column_ids_cache: HashMap<ColumnIndex, ColumnId>,
}

impl Context {
    pub(crate) fn add_shared_string(&mut self, value: Cow<'static, str>) -> SharedStringIndex {
        self.shared_strings.insert(value)
    }

    pub(crate) fn add_column_index(&mut self, column_index: ColumnIndex) {
        self.column_ids_cache
            .entry(column_index)
            .or_insert_with(|| ColumnId::from(column_index.0 + 1));
    }
}
