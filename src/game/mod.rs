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
    bet: u32,
    bet_min: u32,
    bet_max: u32,
}

impl Error for InvalidBet {}

impl fmt::Display for InvalidBet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message: &str;

        if self.bet_min > self.bet_max {
            message = "bet_min > bet_max";
        } else if self.bet < self.bet_min {
            message = "bet < bet_min";
        } else if self.bet > self.bet_max {
            message = "bet > bet_max";
        } else {
            panic!("Unknown error!");
        }

        write!(f, "{}", message)
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
    ///
    /// # Errors
    ///
    /// Returns [`InvalidBet`] if bet_min > bet_max or bet < bet_min or bet > bet_max.
    pub fn new(credits: u32, bet: u32, bet_min: u32, bet_max: u32) -> Result<Self, InvalidBet> {
        if ! Game::validate_bet(bet, bet_min, bet_max) {
            return Err(InvalidBet {bet, bet_min, bet_max})
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
    ///
    /// # Errors
    ///
    /// Returns [`InvalidBet`] if bet < [`Game::bet_min`] or bet > [`Game::bet_max`].
    pub fn set_bet(&mut self, bet: u32) -> Result<(), InvalidBet> {
        if ! Game::validate_bet(bet, self.bet_min, self.bet_max) {
            let bet_min = self.bet_min;
            let bet_max = self.bet_max;

            return Err(InvalidBet {bet, bet_min, bet_max})
        }

        self.bet = bet;

        Ok(())
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

    /// Converts an instance to a Json object.
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    // Returns true if the bet is valid.
    fn validate_bet(bet: u32, bet_min: u32, bet_max: u32) -> bool {
        if bet_min > bet_max || bet < bet_min || bet > bet_max {
            return false;
        }

        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn game_new() {
        let credits = 1000;
        let bet = 1;
        let bet_min = 1;
        let bet_max = 10;

        assert_eq!(
            Game {credits, bet, bet_min, bet_max, win: 0},
            Game::new(credits, bet, bet_min, bet_max).unwrap()
        )
    }

    #[test]
    fn game_spin() {
        let mut game = Game::new(1000, 1, 1, 10).unwrap();

        assert!(game.spin().is_ok())
    }

    #[test]
    fn game_validate_bet() {
        assert!(Game::validate_bet(1, 1, 10))
    }
}
