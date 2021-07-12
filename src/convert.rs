use std::{
    convert::Into,
    iter::{once, Iterator},
};

pub trait Convertor {
    fn convert<T: Into<String>>(&self, s: T) -> String {
        s.into()
            .chars()
            .map(|c| self.convert_char(c))
            .flatten()
            .collect()
    }

    fn convert_char(&self, c: char) -> Box<dyn Iterator<Item = char>>;
}

pub struct SimpleConvertor {}

impl SimpleConvertor {
    pub fn new() -> Self {
        SimpleConvertor {}
    }
}

impl Convertor for SimpleConvertor {
    fn convert_char(&self, c: char) -> Box<dyn Iterator<Item = char>> {
        if c.is_alphabetic() {
            if c.is_lowercase() {
                Box::new(c.to_uppercase())
            } else {
                Box::new(c.to_lowercase())
            }
        } else {
            Box::new(once(c))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_string_to_upper() {
        let c = SimpleConvertor {};

        assert_eq!(c.convert("simple string"), "SIMPLE STRING");
    }

    #[test]
    fn convert_char_from_lower() {
        let c = SimpleConvertor {};

        let mut i = c.convert_char('c');
        assert_eq!(i.next(), Some('C'));
        assert_eq!(i.next(), None);
    }

    #[test]
    fn convert_char_from_upper() {
        let c = SimpleConvertor {};

        let mut i = c.convert_char('C');
        assert_eq!(i.next(), Some('c'));
        assert_eq!(i.next(), None);
    }

    #[test]
    fn convert_char_from_numeric() {
        let c = SimpleConvertor {};

        let mut i = c.convert_char('1');
        assert_eq!(i.next(), Some('1'));
        assert_eq!(i.next(), None);
    }

    #[test]
    fn convert_char_from_lower_ligature() {
        let c = SimpleConvertor {};

        let mut i = c.convert_char('ß');
        assert_eq!(i.next(), Some('S'));
        assert_eq!(i.next(), Some('S'));
        assert_eq!(i.next(), None);
    }
}
