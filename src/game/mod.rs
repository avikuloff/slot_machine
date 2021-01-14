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
pub struct InvalidBet {
    message: String,
}

impl Error for InvalidBet {}

impl fmt::Display for InvalidBet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
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
    bet: u32,
    bet_min: u32,
    bet_max: u32,
    win: u32,
}

impl Game {
    /// Creates new [`Game`] instance. The winnings are 0.
    ///
    /// # Examples
    /// ```
    /// # use slot_machine::game::Game;
    /// Game::new(1000, 1, 1, 100);
    /// ```
    pub fn new(credits: u32, bet: u32, bet_min: u32, bet_max: u32) -> Result<Self, InvalidBet> {
        if bet_min > bet_max {
            return Err(InvalidBet {
                message: "bet_min > bet_max".to_owned(),
            });
        }
        if bet < bet_min {
            return Err(InvalidBet {
                message: "bet < bet_min".to_owned(),
            });
        }
        if bet > bet_max {
            return Err(InvalidBet {
                message: "bet > bet_max".to_owned(),
            });
        }

        Ok(Game {
            credits,
            bet,
            bet_min,
            bet_max,
            win: 0,
        })
    }

    /// Bet setter.
    pub fn set_bet(&mut self, bet: u32) {
        self.bet = bet;
    }

    /// Returns the bet size in credits
    pub fn bet(&self) -> u32 {
        self.bet
    }

    /// Returns the minimum allowable bet
    pub fn min(&self) -> u32 {
        self.bet_min
    }

    /// Returns the maximum allowable bet
    pub fn max(&self) -> u32 {
        self.bet_max
    }

    /// Returns the number of credits in the balance
    pub fn credits(&self) -> u32 {
        self.credits
    }

    /// Returns the amount of the last win
    pub fn win(&self) -> u32 {
        self.win
    }

    /// Simulates the rotation of the reels slot machine.
    ///
    /// Returns symbols on the reels.
    /// Also changes state of the [`Game`] depending on the size of the bet and winnings.
    ///
    /// # Errors
    ///
    /// Returns [`LowBalance`] if the number of credits in the balance [`credits`] is less than the bet size [`bet_size`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use slot_machine::game::Game;
    /// let mut game = Game::new(1000, 1, 1, 100).unwrap();
    /// game.spin().unwrap();
    ///
    /// assert_eq!(game.credits(), game.credits() + game.win());
    /// ```
    ///
    /// [`credits`]: #method.credits
    /// [`bet_size`]: #method.bet_size
    pub fn spin(&mut self) -> Result<Vec<Symbol>, LowBalance> {
        if self.credits() < self.bet() {
            return Err(LowBalance);
        }

        let mut stops = Vec::with_capacity(NUM_REELS);

        for _i in 0..NUM_REELS {
            stops.push(Symbol::random());
        }

        self.credits -= self.bet();
        self.win = payout(&stops) * self.bet();
        self.credits += self.win;

        Ok(stops)
    }

    /// Converts an instance to a Json object
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
