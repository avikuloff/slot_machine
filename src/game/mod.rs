use crate::game::payout::payout;
use crate::game::symbol::Symbol;
use serde_derive::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;

pub mod payout;
pub mod symbol;

/// Number of virtual reels in a slot machine
pub const NUM_REELS: usize = 3;

#[derive(Debug, Clone)]
pub struct InvalidBet;

impl Error for InvalidBet {}

impl fmt::Display for InvalidBet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid bet!")
    }
}

/// This error occurs if there are not enough credits on the balance
#[derive(Debug, Clone)]
pub struct LowBalance;

impl Error for LowBalance {}

impl fmt::Display for LowBalance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Insufficient credits on the balance!")
    }
}

/// Game state
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Game {
    credits: u32,
    bet: Bet,
    win: u32,
    stops: Vec<Symbol>,
}

impl Game {
    /// Creates new [`Game`] instance. Initial symbols are set randomly, the winnings are 0.
    ///
    /// # Examples
    /// ```
    /// # use slot_machine::game::{Game, Bet};
    /// Game::new(1000, Bet::new(1, 1, 100));
    /// ```
    pub fn new(credits: u32, bet: Bet) -> Game {
        let stops = vec![Symbol::random(), Symbol::random(), Symbol::random()];

        Game {
            credits,
            bet,
            win: 0,
            stops,
        }
    }

    /// Bet setter.
    pub fn set_bet(&mut self, bet: Bet) {
        self.bet = bet;
    }

    /// Sets the bet size in credits
    ///
    /// # Examples
    /// ```
    /// # use slot_machine::game::{Game, Bet};
    /// let mut game = Game::new(1000, Bet::new(10, 1, 100));
    /// game.set_bet_size(15);
    ///
    /// assert_eq!(game.bet_size(), 15)
    /// ```
    pub fn set_bet_size(&mut self, bet_size: u32) {
        self.bet.set_size(bet_size)
    }

    /// Returns the bet size in credits
    pub fn bet_size(&self) -> u32 {
        self.bet.size
    }

    /// Returns the number of credits in the balance
    pub fn credits(&self) -> u32 {
        self.credits
    }

    /// Returns the amount of the last win
    pub fn win(&self) -> u32 {
        self.win
    }

    /// Symbols on the reels
    pub fn symbols(&self) -> Vec<Symbol> {
        self.stops.clone()
    }

    /// Simulates the rotation of the reels slot machine.
    /// The result of rotation is a change in the number of credits, the amount of winnings and symbols on the reels.
    ///
    /// # Examples
    ///
    /// ```
    /// # use slot_machine::game::{Game, Bet};
    /// let mut game = Game::new(1000, Bet::new(1, 1, 100));
    /// game.spin().unwrap();
    ///
    /// assert_eq!(game.credits(), game.credits() + game.win());
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`LowBalance`] if the number of credits in the balance [`credits`] is less than the bet size [`bet_size`].
    ///
    /// [`credits`]: #method.credits
    /// [`bet_size`]: #method.bet_size
    pub fn spin(&mut self) -> Result<(), LowBalance> {
        if self.credits() < self.bet_size() {
            return Err(LowBalance);
        }

        let mut stops = Vec::with_capacity(NUM_REELS);

        for _i in 0..NUM_REELS {
            stops.push(Symbol::random());
        }

        self.stops = stops;
        self.credits -= self.bet_size();
        self.win = payout(&self.stops) * self.bet_size();
        self.credits += self.win;

        Ok(())
    }

    /// Converts an instance to a Json object
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Bet {
    /// Bet size
    pub size: u32,
    /// Minimum allowable bet
    min: u32,
    /// Maximum allowable bet
    max: u32,
}

impl Bet {
    /// Creates new [`Bet`] instance.
    ///
    /// # Panics
    ///
    /// Panics if `size < min` or `size > max` or `min > max`, checked by function [`is_valid`].
    ///
    /// [`is_valid`]: #method.is_valid
    pub fn new(size: u32, min: u32, max: u32) -> Bet {
        assert!(Bet::is_valid(size, min, max), InvalidBet.to_string());

        Bet { size, min, max }
    }

    /// Sets the bet size in credits
    ///
    /// # Panics
    ///
    /// Panic if `size` is less than [`min`] or greater than [`max`], checked by function [`is_valid`].
    ///
    /// [`min`]: #method.min
    /// [`max`]: #method.max
    /// [`is_valid`]: #method.is_valid
    pub fn set_size(&mut self, size: u32) {
        assert!(
            Bet::is_valid(size, self.min(), self.max()),
            InvalidBet.to_string()
        );

        self.size = size
    }

    /// Returns the minimum allowable bet
    pub fn min(&self) -> u32 {
        self.min
    }

    /// Returns the maximum allowable bet
    pub fn max(&self) -> u32 {
        self.max
    }

    /// Validates the values of [`Bet`].
    ///
    /// Returns `true` if all 3 conditions are satisfied: `bet >= min`, `bet <= max`, `min <= max`, otherwise it returns `false`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use slot_machine::game::Bet;
    /// // Valid Bet
    /// assert!(Bet::is_valid(1, 1, 1));
    ///
    /// // Invalid Bet (bet > max)
    /// assert!(! Bet::is_valid(10, 1, 5));
    /// ```
    pub fn is_valid(bet: u32, min: u32, max: u32) -> bool {
        if bet >= min && bet <= max && min <= max {
            return true;
        }

        false
    }
}
