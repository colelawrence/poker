use std::{fmt::Display, iter::Sum, thread};

/// [Chips] is intentionally non-[Clone] for security
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct Chips(usize);
// pub struct Chips(pub usize);

#[derive(Debug, PartialEq)]
pub enum ChipsError {
    OutOfChips,
}

pub struct Cashier(Chips);

impl Cashier {
    pub fn new(amount_to_start: usize) -> Self {
        Cashier(Chips(amount_to_start))
    }

    /// Buy chips is the only way to get [Chips] without constructing another [Cashier]
    pub fn buy_chips(&mut self, count: usize) -> Result<Chips, ChipsError> {
        self.0.take(count)
    }

    /// Buy chips is the only way to get [Chips] without constructing another [Cashier]
    pub fn exchange_chips(&mut self, mut give_chips: Chips) {
        give_chips.0 = 0;
    }
}

impl Drop for Cashier {
    fn drop(&mut self) {
        (self.0).0 = 0;
    }
}

impl Chips {
    /// Separate out some chips to be used for betting or exchanging.
    pub fn take(&mut self, count: usize) -> Result<Chips, ChipsError> {
        if self.0 < count {
            return Err(ChipsError::OutOfChips);
        }
        self.0 -= count;
        Ok(Chips(count))
    }

    /// Separate out some chips to be used for betting or exchanging.
    pub fn take_all(&mut self) -> Chips {
        let all = Chips(self.0);
        self.0 = 0;
        all
    }

    /// Add a chip stack into this one, totalling their values
    pub fn add(&mut self, mut chips: Chips) {
        self.0 += chips.0;
        chips.0 = 0;
    }

    /// See how many chips are in this stack
    pub fn count(&self) -> usize {
        self.0
    }

    /// Create a new stack of chips
    pub fn new() -> Self {
        Chips(0)
    }
}

impl Sum for Chips {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut total = Chips(0);
        for chip_stack in iter {
            total.add(chip_stack);
        }
        total
    }
}

impl Display for Chips {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            0 => write!(f, "no chips"),
            1 => write!(f, "one chip"),
            2 => write!(f, "two chips"),
            3 => write!(f, "three chips"),
            4 => write!(f, "four chips"),
            5 => write!(f, "five chips"),
            6 => write!(f, "six chips"),
            7 => write!(f, "seven chips"),
            8 => write!(f, "eight chips"),
            9 => write!(f, "nine chips"),
            10 => write!(f, "ten chips"),
            20 => write!(f, "twenty chips"),
            30 => write!(f, "thirty chips"),
            40 => write!(f, "forty chips"),
            50 => write!(f, "fifty chips"),
            other => write!(f, "{} chips", other),
        }
    }
}

impl Drop for Chips {
    fn drop(&mut self) {
        if self.0 > 0 {
            if !thread::panicking() {
                panic!("Dropped {} could mean that Chips were lost!", self);
            }
        }
    }
}

#[cfg(tests)]
mod tests {
    #[test]
    fn test_chip_sum() {
        assert_eq!(
            vec![Chips(1), Chips(2), Chips(3)].into_iter().sum()
            Chips(6),
        );
        assert_eq!(
            Vec::<Chips>::new().into_iter().sum()
            Chips(0),
        );
        assert_eq!(
            vec![Chips(256)].into_iter().sum()
            Chips(256),
        );
    }

    #[test]
    fn test_chip_add() {
        let mut sum_chips = Chips(0);
        sum_chips.add(Chips(6));
        assert_eq!(sum_chips, Chips(6));
        sum_chips.add(Chips(12));
        assert_eq!(sum_chips, Chips(18));
        sum_chips.add(Chips(100));
        assert_eq!(sum_chips, Chips(118));
    }

    #[test]
    fn test_chip_take() {
        {
            let mut empty_chips = Chips(0);
            assert_err!(empty_chips.take(1));
        }
        {
            let mut some_chips = Chips(100);
            assert_err!(some_chips.take(101));
            assert_eq!(some_chips.take(100), Chips(100));
            assert_eq!(some_chips, Chips(0));
        }
    }
}
