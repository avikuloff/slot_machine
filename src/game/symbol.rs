extern crate rand;

use self::rand::distributions::Uniform;
use crate::game::symbol::Symbol::*;
use core::fmt;
use rand::Rng;
use serde_derive::{Deserialize, Serialize};
use std::ops::RangeInclusive;

/// The range of numbers for which there are corresponding symbols.
pub const RANGE: RangeInclusive<u32> = 0..=127;

/// Symbols
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Symbol {
    Blank,
    Cherry,
    Bar,
    DoubleBar,
    TripleBar,
    Seven,
    Jackpot,
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Symbol {
    /// Searches for the corresponding [`Symbol`] in the range [`RANGE`] for `number`.
    ///
    /// # Panics
    ///
    /// Panics if the `numbe`r is not in the [`RANGE`].
    ///
    /// [`RANGE`]: ../symbol/constant.RANGE.html
    ///
    /// # Examples
    /// ```
    /// # use slot_machine::game::symbol::Symbol;
    /// let symbol = Symbol::from_number(125);
    /// assert_eq!(Symbol::Seven, symbol)
    /// ```
    pub fn from_number(number: u32) -> Symbol {
        match number {
            _n @ 0..=72 => Blank,
            _n @ 73..=77 => Cherry,
            _n @ 78..=93 => Bar,
            _n @ 94..=106 => DoubleBar,
            _n @ 107..=117 => TripleBar,
            _n @ 118..=125 => Seven,
            _n @ 126..=127 => Jackpot,
            _ => panic!(
                "The number is not in the range {}..={}!",
                RANGE.start(),
                RANGE.end()
            ),
        }
    }

    /// Returns a random [`Symbol`]
    ///
    /// # Examples
    ///
    /// ```
    /// # use slot_machine::game::symbol::Symbol;
    /// let symbol = Symbol::random();
    /// ```
    pub fn random() -> Symbol {
        let uniform = Uniform::new_inclusive(RANGE.start(), RANGE.end());
        let number = rand::thread_rng().sample(uniform);

        Symbol::from_number(number)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn blank_from_number() {
        assert_eq!(Symbol::from_number(0), Symbol::Blank);
        assert_eq!(Symbol::from_number(72), Symbol::Blank);
    }

    #[test]
    fn cherry_from_number() {
        assert_eq!(Symbol::from_number(73), Symbol::Cherry);
        assert_eq!(Symbol::from_number(77), Symbol::Cherry);
    }

    #[test]
    fn bar_from_number() {
        assert_eq!(Symbol::from_number(78), Symbol::Bar);
        assert_eq!(Symbol::from_number(93), Symbol::Bar);
    }

    #[test]
    fn double_bar_from_number() {
        assert_eq!(Symbol::from_number(94), Symbol::DoubleBar);
        assert_eq!(Symbol::from_number(106), Symbol::DoubleBar);
    }

    #[test]
    fn triple_bar_from_number() {
        assert_eq!(Symbol::from_number(107), Symbol::TripleBar);
        assert_eq!(Symbol::from_number(117), Symbol::TripleBar);
    }

    #[test]
    fn seven_from_number() {
        assert_eq!(Symbol::from_number(118), Symbol::Seven);
        assert_eq!(Symbol::from_number(125), Symbol::Seven);
    }

    #[test]
    fn jackpot_from_number() {
        assert_eq!(Symbol::from_number(126), Symbol::Jackpot);
        assert_eq!(Symbol::from_number(127), Symbol::Jackpot);
    }

    #[test]
    #[should_panic]
    fn invalid_from_number() {
        Symbol::from_number(128);
    }
}
