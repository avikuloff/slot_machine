// Simple text-based user interface

use slot_machine::game::Game;
use std::thread::sleep;
use std::time::Duration;

const BALANCE: u32 = 1000;
const BET_SIZE: u32 = 1;
const BET_MIN: u32 = 1;
const BET_MAX: u32 = 10;

fn main() {
    println!("Greetings!");

    println!("Your balance: {} credits", BALANCE);
    println!("Bet size: {}", BET_SIZE);
    print_help();

    //let bet = Bet::new(BET_SIZE, BET_MIN, BET_MAX);
    let mut game = Game::new(BALANCE, BET_SIZE, BET_MIN, BET_MAX).unwrap();

    loop {
        let mut command = String::new();

        std::io::stdin()
            .read_line(&mut command)
            .expect("Failed to read command!");

        match command.trim().to_uppercase().as_str() {
            "BALANCE" => println!("Your balance: {} credits.", game.credits()),
            "BET" => println!("Current bet: {} credits.", game.bet()),
            "BET PLUS" => match bet_plus(&mut game) {
                Ok(val) => println!("Bet size: {}.", val),
                Err(e) => println!("{}", e),
            },
            "BET MINUS" => match bet_minus(&mut game) {
                Ok(val) => println!("Bet size: {}.", val),
                Err(e) => println!("{}", e),
            },
            "SPIN" => spin(&mut game),
            val if val.starts_with("AUTOSPIN") => {
                let split = val.split(" ");

                let number_spins = split.last().unwrap().parse::<u32>().unwrap();

                for _ in 0..number_spins {
                    spin(&mut game);
                    sleep(Duration::from_secs(1));
                }
            }
            "HELP" => print_help(),
            _ => println!("Invalid command!"),
        }
    }
}

fn spin(game: &mut Game) {
    let symbols = game.spin();
    match symbols {
        Ok(val) => {
            println!("{:?}", val);
            println!("You win {} credits", game.win());
        }
        Err(e) => println!("{}", e.to_owned()),
    }
}

// Increase bet size
fn bet_plus(game: &mut Game) -> Result<u32, String> {
    let bet_size = match game.bet() {
        1 => 2,
        2 => 3,
        3 => 5,
        5 => 10,
        BET_MAX => return Err("Max bet size!".to_owned()),
        _ => return Err("Invalid bet size!".to_owned()),
    };

    game.set_bet(bet_size);

    Ok(bet_size)
}

// Decrease bet size
fn bet_minus(game: &mut Game) -> Result<u32, String> {
    let bet_size = match game.bet() {
        10 => 5,
        5 => 3,
        3 => 2,
        2 => 1,
        BET_MIN => return Err("Min bet size!".to_owned()),
        _ => return Err("Invalid bet size!".to_owned()),
    };

    game.set_bet(bet_size);

    Ok(bet_size)
}

// Prints help text
fn print_help() {
    println!("To get a balance, put the `balance`.");
    println!("To get a bet size, put the `bet`.");
    println!("To increase or decrease the size of the bet, put `bet plus` or `bet minus`.");
    println!("To activate auto-spin, put `autospin <NUMBER>` where NUMBER is the number of spins.")
}
