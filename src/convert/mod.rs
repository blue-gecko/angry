pub mod random;
pub mod simple;

use std::iter::Iterator;

pub trait Convertor {
    fn convert(&mut self, s: String) -> String {
        s.chars().map(|c| self.convert_char(c)).flatten().collect()
    }

    fn convert_char(&mut self, c: char) -> Box<dyn Iterator<Item = char>>;
}
