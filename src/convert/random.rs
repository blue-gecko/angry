use {
    crate::convert::Convertor,
    rand::{Rng, RngCore},
    std::{
        fmt::{Debug, Formatter, Result},
        iter::{once, Iterator},
    },
};

pub struct RandomConvertor<'a> {
    rng: &'a mut dyn RngCore,
    percent: u8,
    step: Option<u8>,
    flipped: bool,
    current: u16,
}

#[allow(dead_code)]
impl<'a> RandomConvertor<'a> {
    pub fn new(rng: &'a mut dyn RngCore, percent: u8, step: Option<u8>) -> Self {
        RandomConvertor {
            rng,
            percent,
            step,
            flipped: false,
            current: percent as u16,
        }
    }

    fn current(&mut self) -> u16 {
        let current = self.current;
        match self.step {
            Some(step) if !self.flipped => {
                self.current += step as u16;
            }
            _ => {}
        }

        current
    }

    fn flip(&mut self, flipped: bool) {
        self.flipped = flipped;
        if flipped {
            self.current = self.percent as u16;
        }
    }

    fn rng(&mut self) -> u16 {
        self.rng.gen::<u16>() % 100
    }
}

/// Manual debug implementation required, to skip the Rng field
impl<'a> Debug for RandomConvertor<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("RandomConvertor")
            .field("percent", &self.percent)
            .field("step", &self.step)
            .finish()
    }
}

impl<'a> Convertor for RandomConvertor<'a> {
    fn convert_char(&mut self, c: char) -> Box<dyn Iterator<Item = char>> {
        if c.is_alphabetic() {
            if self.rng() < self.current() {
                self.flip(true);

                if c.is_lowercase() {
                    Box::new(c.to_uppercase())
                } else {
                    Box::new(once(c))
                }
            } else {
                self.flip(false);

                if c.is_uppercase() {
                    Box::new(c.to_lowercase())
                } else {
                    Box::new(once(c))
                }
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
    fn random_convert_no_step() {
        let rng = &mut StepRng::new(50, 50);
        let mut c = RandomConvertor::new(rng, 50, None);

        assert_eq!(c.convert("simple string"), "sImPlE sTrInG");
    }

    #[test]
    fn random_convert_no_step_from_caps() {
        let rng = &mut StepRng::new(50, 50);
        let mut c = RandomConvertor::new(rng, 50, None);

        assert_eq!(c.convert("SIMPLE STRING"), "sImPlE sTrInG");
    }

    #[test]
    fn random_convert_no_step_from_mixed() {
        let rng = &mut StepRng::new(50, 50);
        let mut c = RandomConvertor::new(rng, 50, None);

        assert_eq!(c.convert("SiMpLe StRiNg"), "sImPlE sTrInG");
    }

    #[test]
    fn random_convert_with_step() {
        let rng = &mut StepRng::new(0, 25);
        let mut c = RandomConvertor::new(rng, 25, Some(25));
        println!("{:?}", c);

        assert_eq!(c.convert("simple string"), "SimpLe stRing");
    }
}
