use crate::game::payout::payout;
use crate::game::symbol::Symbol;
use serde_derive::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;

pub mod payout;
pub mod symbol;

/// Количество барабанов в слот машине
pub const NUM_REELS: usize = 3;

#[derive(Debug, Clone)]
pub struct InvalidBet;

impl Error for InvalidBet {}

impl fmt::Display for InvalidBet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Недопустимый размер ставки!")
    }
}

#[derive(Debug, Clone)]
pub struct LowBalance;

impl Error for LowBalance {}

impl fmt::Display for LowBalance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Недостаточно средств на балансе!")
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Game {
    credits: u32,
    bet: Bet,
    win: u32,
    stops: Vec<Symbol>,
}

impl Game {
    /// Game constructor
    ///
    /// # Examples
    /// ```
    /// let game = Game::new(1000, Bet::new(1, 1, 100));
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

    /// Устанавливает размер ставки
    ///
    /// # Examples
    /// ```
    /// let mut game = Game::new(1000, Bet::new(10, 1, 100));
    /// game.set_bet_size(15);
    ///
    /// assert_eq!(game.bet_size(), 15)
    /// ```
    pub fn set_bet_size(&mut self, bet_size: u32) {
        self.bet.set_size(bet_size)
    }

    /// Возвращает размер ставки
    pub fn bet_size(&self) -> u32 {
        self.bet.size
    }

    /// Возврашает количество кредитов на счету
    pub fn credits(&self) -> u32 {
        self.credits
    }

    /// Возврашает размер последнего выигрыша
    pub fn win(&self) -> u32 {
        self.win
    }

    /// Символы на барабанах
    pub fn symbols(&self) -> Vec<Symbol> {
        self.stops.clone()
    }

    /// Симулирует вращение барабанов слот машины. Результатом вращения является изменение
    /// количества кредитов, размера выигрыша и символов,
    ///
    /// # Examples
    ///
    /// ```
    /// let mut game = Game::new(1000, Bet::new(1, 1, 100));
    /// game.spin().unwrap();
    ///
    /// assert_eq!(game.credits(), 999);
    /// ```
    ///
    /// # Errors
    ///
    /// Возвращает [`LowBalance`] если количество кредитов на счету [`credits`] меньше размера ставки [`bet_size`].
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

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Bet {
    /// Размер ставки
    pub size: u32,
    min: u32,
    max: u32,
}

impl Bet {
    /// Конструктор
    ///
    /// # Panics
    ///
    /// Паникует если `size < min` или `size > max` или `min > max`, проверяется функцией [`is_valid`].
    ///
    /// [`is_valid`]: #method.is_valid
    pub fn new(size: u32, min: u32, max: u32) -> Bet {
        assert!(Bet::is_valid(size, min, max), InvalidBet.to_string());

        Bet { size, min, max }
    }

    /// Устанавливает размер ставки
    ///
    /// # Panics
    ///
    /// Паникует если `size` меньше [`min`] или больше [`max`], проверяется функцией [`is_valid`].
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

    /// Возвращает минимально допустимый размер ставки
    pub fn min(&self) -> u32 {
        self.min
    }

    /// Возвращает максимально допустимый размер ставки
    pub fn max(&self) -> u32 {
        self.max
    }

    /// Проверяет значения на валидность.
    ///
    /// Возвращает `true` если удовлетворены все 3 условия: `bet >= min`, `bet <= max`, `min <= max`.
    /// В противном случае вернет `false`.
    ///
    /// # Examples
    ///
    /// ```
    /// assert!(Bet::is_valid(1, 1, 1));
    ///
    /// // bet > max
    /// assert!(! Bet::is_valid(10, 1, 5));
    /// ```
    pub fn is_valid(bet: u32, min: u32, max: u32) -> bool {
        if bet >= min && bet <= max && min <= max {
            return true;
        }

        false
    }
}
