use {
    crate::convert::Convertor,
    rand::{Rng, RngCore},
    std::{
        fmt::{Debug, Formatter, Result},
        iter::{once, Iterator},
    },
};

pub struct RandomConvertor {
    rng: Box<dyn RngCore>,
    percent: u8,
    step: Option<u8>,
    flipped: bool,
    current: u16,
}

#[allow(dead_code)]
impl RandomConvertor {
    pub fn with_rng(rng: Box<dyn RngCore>, percent: u8, step: Option<u8>) -> Box<dyn Convertor> {
        Box::new(RandomConvertor {
            rng,
            percent,
            step,
            flipped: false,
            current: percent as u16,
        })
    }

    pub fn new(percent: u8, step: Option<u8>) -> Box<dyn Convertor> {
        let rng = Box::new(rand::thread_rng());
        RandomConvertor::with_rng(rng, percent, step)
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
impl Debug for RandomConvertor {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("RandomConvertor")
            .field("percent", &self.percent)
            .field("step", &self.step)
            .finish()
    }
}

impl Convertor for RandomConvertor {
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
        let rng = Box::new(StepRng::new(50, 50));
        let mut c = RandomConvertor::with_rng(rng, 50, None);

        assert_eq!(c.convert(String::from("simple string")), "sImPlE sTrInG");
    }

    #[test]
    fn random_convert_no_step_from_caps() {
        let rng = Box::new(StepRng::new(50, 50));
        let mut c = RandomConvertor::with_rng(rng, 50, None);

        assert_eq!(c.convert(String::from("SIMPLE STRING")), "sImPlE sTrInG");
    }

    #[test]
    fn random_convert_no_step_from_mixed() {
        let rng = Box::new(StepRng::new(50, 50));
        let mut c = RandomConvertor::with_rng(rng, 50, None);

        assert_eq!(c.convert(String::from("SiMpLe StRiNg")), "sImPlE sTrInG");
    }

    #[test]
    fn random_convert_with_step() {
        let rng = Box::new(StepRng::new(0, 25));
        let mut c = RandomConvertor::with_rng(rng, 25, Some(25));

        assert_eq!(c.convert(String::from("simple string")), "SimpLe stRing");
    }
}
