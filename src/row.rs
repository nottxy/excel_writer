use std::{borrow::Cow, collections::HashMap};

use serde::Serialize;

use crate::{
    cell::{Cell, Cells, ColIndex, Number},
    context::Context,
};

#[derive(Default, Serialize)]
pub(crate) struct Rows {
    pub(crate) next_row_index: RowIndex,
    pub(crate) rows: HashMap<RowIndex, Row>,
}

#[derive(Default, Eq, PartialEq, Hash, Copy, Clone, Serialize)]
pub struct RowIndex(pub usize);

#[derive(Serialize)]
pub struct Row {
    pub(crate) row_index: RowIndex,
    pub(crate) cells: Cells,
}

impl Rows {
    pub(crate) fn add_row(&mut self) -> &mut Row {
        self.get_row(self.next_row_index)
    }

    pub(crate) fn get_row(&mut self, row_index: RowIndex) -> &mut Row {
        if row_index.0 >= self.next_row_index.0 {
            self.next_row_index = RowIndex(row_index.0 + 1);
        }

        self.rows.entry(row_index).or_insert_with(|| Row {
            row_index,
            cells: Cells::default(),
        })
    }
}

impl Row {
    pub fn add_str_cell<V: Into<Cow<'static, str>>>(
        &mut self,
        context: &mut Context,
        cell_value: V,
    ) -> &mut Cell {
        self.cells.add_str_cell(context, cell_value.into())
    }

    pub fn set_str_cell<V: Into<Cow<'static, str>>>(
        &mut self,
        context: &mut Context,
        col_index: ColIndex,
        cell_value: V,
    ) -> &mut Cell {
        self.cells
            .set_str_cell(context, col_index, cell_value.into())
    }

    pub fn add_num_cell(&mut self, context: &mut Context, cell_value: Number) -> &mut Cell {
        self.cells.add_num_cell(context, cell_value)
    }

    pub fn set_num_cell(
        &mut self,
        context: &mut Context,
        col_index: ColIndex,
        cell_value: Number,
    ) -> &mut Cell {
        self.cells.set_num_cell(context, col_index, cell_value)
    }
}
