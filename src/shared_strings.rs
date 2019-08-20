use std::{borrow::Cow, collections::HashMap};

use serde::Serialize;

#[derive(Debug, Copy, Clone, Serialize, Eq, PartialEq, Hash)]
pub(crate) struct SharedStringIndex(pub(crate) usize);

pub(crate) type SharedString = Cow<'static, str>;

#[derive(Default, Serialize)]
pub(crate) struct SharedStrings {
    pub(crate) strings: HashMap<SharedString, SharedStringIndex>,
}

impl SharedStrings {
    pub(crate) fn insert(&mut self, value: Cow<'static, str>) -> SharedStringIndex {
        let len = self.strings.len();

        *self
            .strings
            .entry(value)
            .or_insert_with(|| SharedStringIndex(len))
    }
}
