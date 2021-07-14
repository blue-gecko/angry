use {
    rand::{Rng, RngCore},
    std::{
        convert::Into,
        iter::{once, Iterator},
    },
};

pub trait Convertor {
    // fn convert_with_fn<F, T>(&self, s: T, f: F) -> String
    // where
    //     T: Into<String>,
    //     F: Fn(char) -> Box<dyn Iterator<Item = char>>,
    // {
    //     s.into().chars().map(|c| f(c)).flatten().collect()
    // }

    fn convert<T: Into<String>>(&mut self, s: T) -> String {
        s.into()
            .chars()
            .map(|c| self.convert_char(c))
            .flatten()
            .collect()
    }

    fn convert_char(&mut self, c: char) -> Box<dyn Iterator<Item = char>>;
}

pub struct SimpleConvertor<'a> {
    filter: &'a dyn Fn(char) -> bool,
    convert: &'a dyn Fn(char) -> Box<dyn Iterator<Item = char>>,
}

#[allow(dead_code)]
impl<'a> SimpleConvertor<'a> {
    fn new(
        filter: &'a dyn Fn(char) -> bool,
        convert: &'a dyn Fn(char) -> Box<dyn Iterator<Item = char>>,
    ) -> Self {
        Self { filter, convert }
    }

    pub fn uppercase() -> Self {
        Self::new(
            &|c: char| c.is_alphabetic() && c.is_lowercase(),
            &|c: char| Box::new(c.to_uppercase()),
        )
    }

    pub fn lowercase() -> Self {
        Self::new(
            &|c: char| c.is_alphabetic() && c.is_uppercase(),
            &|c: char| Box::new(c.to_lowercase()),
        )
    }

    pub fn reverse() -> Self {
        Self::new(&|c: char| c.is_alphabetic(), &|c: char| {
            if c.is_uppercase() {
                Box::new(c.to_lowercase())
            } else {
                Box::new(c.to_uppercase())
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

pub struct RandomConvertor<'a> {
    rng: &'a mut dyn RngCore,
}

#[allow(dead_code)]
impl<'a> RandomConvertor<'a> {
    pub fn new(rng: &'a mut dyn RngCore) -> Self {
        RandomConvertor { rng }
    }
}

impl<'a> Convertor for RandomConvertor<'a> {
    fn convert_char(&mut self, c: char) -> Box<dyn Iterator<Item = char>> {
        if c.is_alphabetic() && &self.rng.gen::<usize>() % 2 == 0 {
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
    use {super::*, rand::rngs::mock::StepRng};

    #[test]
    fn convert_string_to_upper() {
        let mut c = SimpleConvertor::uppercase();

        assert_eq!(c.convert("simple string"), "SIMPLE STRING");
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

    #[test]
    fn random_convert() {
        let rng = &mut StepRng::new(1, 1);
        let mut c = RandomConvertor::new(rng);

        assert_eq!(c.convert("simple string"), "sImPlE sTrInG");
    }
}
