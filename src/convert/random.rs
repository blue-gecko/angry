use {
    crate::convert::Convertor,
    rand::{Rng, RngCore},
    std::iter::{once, Iterator},
};

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
            // this just reverses the current case
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
    fn random_convert() {
        let rng = &mut StepRng::new(1, 1);
        let mut c = RandomConvertor::new(rng);

        assert_eq!(c.convert("simple string"), "sImPlE sTrInG");
    }
}
