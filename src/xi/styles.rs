use crate::xi::Style;

use std::collections::HashMap;

#[derive(Default)]
pub struct StyleCache(HashMap<u64, Style>);

impl StyleCache {
    pub fn insert(&mut self, id: u64, style: Style) {
        self.0.insert(id, style);
    }

    pub fn get(&self, id: u64) -> Option<&Style> {
        self.0.get(&id)
    }

    pub fn remove(&mut self, id: u64) {
        self.0.remove(&id);
    }
}
