use {
    crate::convert::Convertor,
    std::iter::{once, Iterator},
};

pub struct SimpleConvertor<'a> {
    filter: &'a dyn Fn(char) -> bool,
    convert: &'a dyn Fn(char) -> Box<dyn Iterator<Item = char>>,
}

#[allow(dead_code)]
impl<'a> SimpleConvertor<'a> {
    fn new(
        filter: &'a dyn Fn(char) -> bool,
        convert: &'a dyn Fn(char) -> Box<dyn Iterator<Item = char>>,
    ) -> Box<dyn Convertor + 'a> {
        Box::new(SimpleConvertor { filter, convert })
    }

    pub fn uppercase() -> Box<dyn Convertor + 'a> {
        Self::new(
            &|c: char| c.is_alphabetic() && c.is_lowercase(),
            &|c: char| Box::new(c.to_uppercase()),
        )
    }

    pub fn lowercase() -> Box<dyn Convertor + 'a> {
        Self::new(
            &|c: char| c.is_alphabetic() && c.is_uppercase(),
            &|c: char| Box::new(c.to_lowercase()),
        )
    }

    pub fn reverse() -> Box<dyn Convertor + 'a> {
        Self::new(&|c: char| c.is_alphabetic(), &|c: char| {
            if c.is_lowercase() {
                Box::new(c.to_uppercase())
            } else {
                Box::new(c.to_lowercase())
            }
        })
    }
}

impl<'a> Convertor for SimpleConvertor<'a> {
    fn convert_char(&mut self, c: char) -> Box<dyn Iterator<Item = char>> {
        if (self.filter)(c) {
            (self.convert)(c)
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
        let mut c = SimpleConvertor::uppercase();

        assert_eq!(c.convert(String::from("simple string")), "SIMPLE STRING");
    }

    #[test]
    fn upper_char_from_lower() {
        let mut c = SimpleConvertor::uppercase();

        let mut i = c.convert_char('c');
        assert_eq!(i.next(), Some('C'));
        assert_eq!(i.next(), None);
    }

    #[test]
    fn upper_char_from_upper() {
        let mut c = SimpleConvertor::uppercase();

        let mut i = c.convert_char('C');
        assert_eq!(i.next(), Some('C'));
        assert_eq!(i.next(), None);
    }

    #[test]
    fn upper_char_from_numeric() {
        let mut c = SimpleConvertor::uppercase();

        let mut i = c.convert_char('1');
        assert_eq!(i.next(), Some('1'));
        assert_eq!(i.next(), None);
    }

    #[test]
    fn upper_char_from_ligature() {
        let mut c = SimpleConvertor::uppercase();

        let mut i = c.convert_char('æ');
        assert_eq!(i.next(), Some('Æ'));
        assert_eq!(i.next(), None);
    }

    #[test]
    fn upper_char_from_fixed_ligature() {
        let mut c = SimpleConvertor::uppercase();

        let mut i = c.convert_char('ǅ');
        assert_eq!(i.next(), Some('ǅ'));
        assert_eq!(i.next(), None);
    }

    #[test]
    fn upper_char_from_lower_ligature() {
        let mut c = SimpleConvertor::uppercase();

        let mut i = c.convert_char('ß');
        assert_eq!(i.next(), Some('S'));
        assert_eq!(i.next(), Some('S'));
        assert_eq!(i.next(), None);
    }

    #[test]
    fn lower_char_from_lower() {
        let mut c = SimpleConvertor::lowercase();

        let mut i = c.convert_char('c');
        assert_eq!(i.next(), Some('c'));
        assert_eq!(i.next(), None);
    }

    #[test]
    fn lower_char_from_upper() {
        let mut c = SimpleConvertor::lowercase();

        let mut i = c.convert_char('C');
        assert_eq!(i.next(), Some('c'));
        assert_eq!(i.next(), None);
    }

    #[test]
    fn lower_char_from_numeric() {
        let mut c = SimpleConvertor::lowercase();

        let mut i = c.convert_char('1');
        assert_eq!(i.next(), Some('1'));
        assert_eq!(i.next(), None);
    }

    #[test]
    fn lower_char_from_ligature() {
        let mut c = SimpleConvertor::lowercase();

        let mut i = c.convert_char('Æ');
        assert_eq!(i.next(), Some('æ'));
        assert_eq!(i.next(), None);
    }

    #[test]
    fn lower_char_from_fixed_ligature() {
        let mut c = SimpleConvertor::lowercase();

        let mut i = c.convert_char('ǅ');
        assert_eq!(i.next(), Some('ǅ'));
        assert_eq!(i.next(), None);
    }

    #[test]
    fn reverse_char_from_lower() {
        let mut c = SimpleConvertor::reverse();

        let mut i = c.convert_char('c');
        assert_eq!(i.next(), Some('C'));
        assert_eq!(i.next(), None);
    }

    #[test]
    fn reverse_char_from_upper() {
        let mut c = SimpleConvertor::reverse();

        let mut i = c.convert_char('C');
        assert_eq!(i.next(), Some('c'));
        assert_eq!(i.next(), None);
    }

    #[test]
    fn reverse_char_from_numeric() {
        let mut c = SimpleConvertor::reverse();

        let mut i = c.convert_char('1');
        assert_eq!(i.next(), Some('1'));
        assert_eq!(i.next(), None);
    }
}
