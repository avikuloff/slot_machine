use crate::game::symbol::Symbol;
use crate::game::symbol::Symbol::*;
use crate::game::NUM_REELS;

/// Рассчитывает выплату.
///
/// # Panics
///
/// The `payout` function will panic if the number of elements in the vector is not 3.
pub fn payout(symbols: &Vec<Symbol>) -> u32 {
    assert_eq!(
        symbols.len(),
        NUM_REELS,
        "Вектор должен содержать {} символа! Содержит: {}",
        NUM_REELS,
        symbols.len()
    );

    if is_all(symbols, Jackpot) {
        return 1666;
    } else if is_all(symbols, Seven) {
        return 300;
    } else if is_all(symbols, TripleBar) {
        return 100;
    } else if is_all(symbols, DoubleBar) {
        return 50;
    } else if is_all(symbols, Bar) {
        return 25;
    } else if is_all(symbols, Cherry)
        || symbols
            .iter()
            .map(|x| x.to_string())
            .filter(|x| x.contains("Bar"))
            .count()
            == 3
    {
        return 12;
    } else if symbols.iter().filter(|x| x == &&Cherry).count() == 2 {
        return 6;
    } else if symbols.iter().filter(|x| x == &&Cherry).count() == 1 {
        return 3;
    }

    0
}

/// Возвращает `true` если `vec` содержит только `expected`
fn is_all(vec: &Vec<Symbol>, expected: Symbol) -> bool {
    vec.iter().all(|x| x == &expected)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_payout() {
        assert_eq!(payout(&vec![Jackpot; 3]), 1666);
        assert_eq!(payout(&vec![Seven; 3]), 300);
        assert_eq!(payout(&vec![TripleBar; 3]), 100);
        assert_eq!(payout(&vec![DoubleBar; 3]), 50);
        assert_eq!(payout(&vec![Bar; 3]), 25);
        assert_eq!(payout(&vec![Cherry; 3]), 12);
        assert_eq!(payout(&vec![Bar, DoubleBar, TripleBar]), 12);
        assert_eq!(payout(&vec![Cherry, Cherry, Blank]), 6);
        assert_eq!(payout(&vec![Bar, Blank, Cherry]), 3);
        assert_eq!(payout(&vec![Bar, Blank, Seven]), 0);
    }

    #[test]
    #[should_panic]
    fn payout_vec_length_not_3() {
        payout(&vec![Bar, Blank, Blank, Bar]);
    }
}
