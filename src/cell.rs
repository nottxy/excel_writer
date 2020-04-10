use std::{borrow::Cow, collections::HashMap};

use serde::Serialize;

use crate::{context::Context, shared_strings::SharedStringIndex};

#[derive(Default, Serialize)]
pub(crate) struct Cells {
    pub(crate) next_col_index: ColIndex,
    pub(crate) cells: HashMap<ColIndex, Cell>,
}

#[derive(Default, Copy, Clone, Eq, PartialEq, Hash, Serialize)]
pub struct ColIndex(pub usize);

#[derive(Serialize)]
pub struct Cell {
    pub(crate) col_index: ColIndex,
    pub(crate) cell_type: CellType,
}

#[derive(Serialize)]
#[serde(tag = "t", content = "c")]
pub(crate) enum CellType {
    String(SharedStringIndex),
    Number(Number),
}

#[derive(Serialize)]
#[serde(untagged)]
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
        self.set_str_cell(context, self.next_col_index, cell_value)
    }

    pub(crate) fn set_str_cell(
        &mut self,
        context: &mut Context,
        col_index: ColIndex,
        cell_value: Cow<'static, str>,
    ) -> &mut Cell {
        let shared_string_index = context.add_shared_string(cell_value);

        self.set_cell(
            context,
            Cell {
                col_index,
                cell_type: CellType::String(shared_string_index),
            },
        )
    }

    pub(crate) fn add_num_cell(&mut self, context: &mut Context, cell_value: Number) -> &mut Cell {
        self.set_num_cell(context, self.next_col_index, cell_value)
    }

    pub(crate) fn set_num_cell(
        &mut self,
        context: &mut Context,
        col_index: ColIndex,
        cell_value: Number,
    ) -> &mut Cell {
        self.set_cell(
            context,
            Cell {
                col_index,
                cell_type: CellType::Number(cell_value),
            },
        )
    }

    pub(crate) fn set_cell(&mut self, context: &mut Context, cell: Cell) -> &mut Cell {
        let col_index = cell.col_index;
        if col_index.0 >= self.next_col_index.0 {
            self.next_col_index = ColIndex(col_index.0 + 1);
        }

        context.add_col_index(col_index);

        self.cells.insert(col_index, cell);
        self.cells.get_mut(&col_index).unwrap()
    }
}
