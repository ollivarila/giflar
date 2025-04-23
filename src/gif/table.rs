use super::Color;
use std::fmt::Debug;

pub type GlobalColorTable = ColorTable;
pub type LocalColorTable = ColorTable;

#[derive(Clone)]
pub struct ColorTable {
    colors: Vec<Color>,
}

impl Debug for ColorTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GlobalColorTable")
            .field("colors_length", &self.colors.len())
            .field("first_color", &self.colors.get(0))
            .field("last_color", &self.colors.last())
            .finish()
    }
}

impl ColorTable {
    pub fn new(colors: Vec<Color>) -> Self {
        Self { colors }
    }
}
