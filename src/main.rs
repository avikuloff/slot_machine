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
    print_help();

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
                match bet_plus(&mut game) {
                    Ok(val) => println!("Bet size: {}.", val),
                    Err(e) => println!("{}", e)
                }
            },
            "BET MINUS" => {
                match bet_minus(&mut game) {
                    Ok(val) => println!("Bet size: {}.", val),
                    Err(e) => println!("{}", e)
                }
            },
            "SPIN" => {
                match game.spin() {
                    Ok(_) => {
                        println!("{:?}", game.symbols());
                        println!("You win {} credits", game.win());
                    }
                    Err(e) => println!("{}", e.to_owned())
                }
            }
            "HELP" => print_help(),
            _ => println!("Invalid command!")
        }
    }
}

// Increase bet size
fn bet_plus(game: &mut Game) -> Result<u32, String> {
    let bet_size = match game.bet_size() {
        1 => 2,
        2 => 3,
        3 => 5,
        5 => 10,
        10 => return Err("Max bet size!".to_owned()),
        _ => return Err("Invalid bet size!".to_owned())
    };

    game.set_bet_size(bet_size);

    Ok(bet_size)
}

// Decrease bet size
fn bet_minus(game: &mut Game) -> Result<u32, String> {
    let bet_size = match game.bet_size() {
        10 => 5,
        5 => 3,
        3 => 2,
        2 => 1,
        1 => return Err("Min bet size!".to_owned()),
        _ => return Err("Invalid bet size!".to_owned())
    };

    game.set_bet_size(bet_size);

    Ok(bet_size)
}

// Prints help text
fn print_help() {
    println!("To get a balance, put the `balance`");
    println!("To get a bet size, put the `bet`");
    println!("To increase or decrease the size of the bet, put `bet plus` or `bet minus`.");
}

