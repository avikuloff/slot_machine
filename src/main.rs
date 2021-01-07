use crate::game::{Bet, Game};

pub mod game;

fn main() {
    let bet = Bet::new(1, 1, 100);
    let mut game = Game::new(10, bet);
    for _i in 0..100 {
        if let Err(e) = game.spin() {
            println!("{}", e);
            break;
        }

        if game.win() > 0 {
            println!("You WIN! {} : {:?}", game.win(), game.symbols());
        }
    }

    println!("Hello, world! {:?}", game.to_json());
}

