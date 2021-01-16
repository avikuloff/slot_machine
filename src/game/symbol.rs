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
    /// # Errors
    ///
    /// Returns [`OutOfRange`] if the `number` is not in the [`RANGE`].
    ///
    /// [`RANGE`]: ../symbol/constant.RANGE.html
    ///
    /// # Examples
    /// ```
    /// # use slot_machine::game::symbol::Symbol;
    /// let symbol = Symbol::from_number(125);
    ///
    /// assert_eq!(symbol, Some(Symbol::Seven))
    /// ```
    pub fn from_number(number: u32) -> Option<Self> {
        let symbol = match number {
            0..=72 => Blank,
            73..=77 => Cherry,
            78..=93 => Bar,
            94..=106 => DoubleBar,
            107..=117 => TripleBar,
            118..=125 => Seven,
            126..=127 => Jackpot,
            _ => return None,
        };

        Some(symbol)
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

        Symbol::from_number(number).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn blank_from_number() {
        assert_eq!(Symbol::from_number(0).unwrap(), Symbol::Blank);
        assert_eq!(Symbol::from_number(72).unwrap(), Symbol::Blank);
    }

    #[test]
    fn cherry_from_number() {
        assert_eq!(Symbol::from_number(73).unwrap(), Symbol::Cherry);
        assert_eq!(Symbol::from_number(77).unwrap(), Symbol::Cherry);
    }

    #[test]
    fn bar_from_number() {
        assert_eq!(Symbol::from_number(78).unwrap(), Symbol::Bar);
        assert_eq!(Symbol::from_number(93).unwrap(), Symbol::Bar);
    }

    #[test]
    fn double_bar_from_number() {
        assert_eq!(Symbol::from_number(94).unwrap(), Symbol::DoubleBar);
        assert_eq!(Symbol::from_number(106).unwrap(), Symbol::DoubleBar);
    }

    #[test]
    fn triple_bar_from_number() {
        assert_eq!(Symbol::from_number(107).unwrap(), Symbol::TripleBar);
        assert_eq!(Symbol::from_number(117).unwrap(), Symbol::TripleBar);
    }

    #[test]
    fn seven_from_number() {
        assert_eq!(Symbol::from_number(118).unwrap(), Symbol::Seven);
        assert_eq!(Symbol::from_number(125).unwrap(), Symbol::Seven);
    }

    #[test]
    fn jackpot_from_number() {
        assert_eq!(Symbol::from_number(126).unwrap(), Symbol::Jackpot);
        assert_eq!(Symbol::from_number(127).unwrap(), Symbol::Jackpot);
    }

    #[test]
    fn from_number_assert_error() {
        assert_eq!(Symbol::from_number(128), None);
    }
}
