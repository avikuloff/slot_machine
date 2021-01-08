use crate::game::{Bet, Game};

pub mod game;

fn main() {
    let balance = 1000;
    let bet_size = 1;
    let bet_min = 1;
    let bet_max = 100;

    println!("Greetings!");

    println!("Your balance: {} credits", balance);
    println!("Bet size: {}", bet_size);
    println!("To get a balance, put the `balance`");
    println!("To get a bet size, put the `bet`");
    println!("To increase or decrease the size of the bet, put `bet plus` or `bet minus`.");

    let bet = Bet::new(bet_size, bet_min, bet_max);
    let mut game = Game::new(balance, bet);

    loop {
        let mut command = String::new();

        std::io::stdin()
            .read_line(&mut command)
            .expect("Failed to read command!");

        match command.trim().to_uppercase().as_str() {
            "BALANCE" => println!("Your balance: {} credits.", game.credits()),
            "BET" => println!("Current bet: {} credits.", game.bet_size()),
            "BET PLUS" => {
                bet_plus(&mut game);
                println!("Bet size: {}.", game.bet_size());
            },
            "BET MINUS" => {
                bet_minus(&mut game);
                println!("Bet size: {}.", game.bet_size());
            },
            "SPIN" => {
                if let Ok(_) = game.spin() {
                    println!("{:?}", game.symbols());
                    println!("You win {} credits", game.win());
                }
            }
            _ => println!("Invalid command!")
        }
    }
}

fn bet_plus(game: &mut Game) {
    match game.bet_size() {
        1 => game.set_bet_size(2),
        2 => game.set_bet_size(3),
        3 => game.set_bet_size(5),
        5 => game.set_bet_size(10),
        _ => println!("Invalid bet size!")
    }
}

fn bet_minus(game: &mut Game) {
    match game.bet_size() {
        10 => game.set_bet_size(5),
        5 => game.set_bet_size(3),
        3 => game.set_bet_size(2),
        2 => game.set_bet_size(1),
        _ => println!("Invalid bet size!")
    }
}

