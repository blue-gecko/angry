pub mod random;
pub mod simple;

use std::{convert::Into, iter::Iterator};

pub trait Convertor {
    fn convert<T: Into<String>>(&mut self, s: T) -> String {
        s.into()
            .chars()
            .map(|c| self.convert_char(c))
            .flatten()
            .collect()
    }

    fn convert_char(&mut self, c: char) -> Box<dyn Iterator<Item = char>>;
}
