use std::{borrow::Cow, collections::HashMap};

use serde::Serialize;

use crate::{context::Context, shared_strings::SharedStringIndex};

#[derive(Default, Serialize)]
pub(crate) struct Cells {
    next_column_index: ColumnIndex,
    cells: HashMap<ColumnIndex, Cell>,
}

#[derive(Default, Copy, Clone, Eq, PartialEq, Hash, Serialize)]
pub struct ColumnIndex(pub usize);

#[derive(Serialize)]
pub struct Cell {
    pub(crate) column_index: ColumnIndex,
    pub(crate) cell_type: CellType,
}

#[derive(Serialize)]
#[serde(tag = "t", content = "c")]
pub(crate) enum CellType {
    String(SharedStringIndex),
    Number(Number),
}

#[derive(Serialize)]
pub enum Number {
    I8(i8),
    U8(u8),
    I16(i16),
    U16(u16),
    I32(i32),
    U32(u32),
    I64(i64),
    U64(u64),
    Isize(isize),
    Usize(usize),
    I128(i128),
    U128(u128),
    F32(f32),
    F64(f64),
}

impl Cells {
    pub(crate) fn add_str_cell(
        &mut self,
        context: &mut Context,
        cell_value: Cow<'static, str>,
    ) -> &mut Cell {
        self.set_str_cell(context, self.next_column_index, cell_value)
    }

    pub(crate) fn set_str_cell(
        &mut self,
        context: &mut Context,
        column_index: ColumnIndex,
        cell_value: Cow<'static, str>,
    ) -> &mut Cell {
        let shared_string_index = context.add_shared_string(cell_value);

        self.set_cell(
            context,
            Cell {
                column_index,
                cell_type: CellType::String(shared_string_index),
            },
        )
    }

    pub(crate) fn add_num_cell(&mut self, context: &mut Context, cell_value: Number) -> &mut Cell {
        self.set_num_cell(context, self.next_column_index, cell_value)
    }

    pub(crate) fn set_num_cell(
        &mut self,
        context: &mut Context,
        column_index: ColumnIndex,
        cell_value: Number,
    ) -> &mut Cell {
        self.set_cell(
            context,
            Cell {
                column_index,
                cell_type: CellType::Number(cell_value),
            },
        )
    }

    pub(crate) fn set_cell(&mut self, context: &mut Context, cell: Cell) -> &mut Cell {
        let column_index = cell.column_index;
        if column_index.0 >= self.next_column_index.0 {
            self.next_column_index = ColumnIndex(column_index.0 + 1);
        }

        context.add_column_index(column_index);

        self.cells.insert(column_index, cell);
        self.cells.get_mut(&column_index).unwrap()
    }
}
