use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Write};

const MAX_ATTEMPTS: u32 = 10;
const MIN_NUMBER: u32 = 1;
const MAX_NUMBER: u32 = 100;

// Represents the result of processing a player's guess.
enum GuessResult {
    TooHigh,
    TooLow,
    Correct,
    Invalid(String),
}

// Main game structure.
struct Game {
    secret_number: u32,
    attempts_left: u32,
}

impl Game {
    // Initialize a new game.
    fn new() -> Self {
        let secret_number = rand::thread_rng().gen_range(MIN_NUMBER..=MAX_NUMBER);
        Game {
            secret_number,
            attempts_left: MAX_ATTEMPTS,
        }
    }
    
    fn process_guess(&mut self, guess: &str) -> GuessResult {
        // Parse the guess and validate it
        match guess.trim().parse::<u32>() {
            Ok(num) if num < MIN_NUMBER || num > MAX_NUMBER => {
                GuessResult::Invalid(format!("Please enter a number between {} and {}", MIN_NUMBER, MAX_NUMBER))
            }
            Ok(num) => {
                self.attempts_left -= 1;
                match num.cmp(&self.secret_number) {
                    Ordering::Less => GuessResult::TooLow,
                    Ordering::Greater => GuessResult::TooHigh,
                    Ordering::Equal => GuessResult::Correct,
                }
            }
            Err(_) => GuessResult::Invalid("Please enter a valid number".to_string()),
        }
    }

    fn is_game_over(&self) -> bool {
        self.attempts_left == 0
    }

    fn attempts_left(&self) -> u32 {
        self.attempts_left
    }

    fn secret_number(&self) -> u32 {
        self.secret_number
    }
}

fn main() {
    println!("Welcome to the Number Guessing Game!");
    println!("I'm thinking of a number between {} and {}", MIN_NUMBER, MAX_NUMBER);
    println!("You have {} attempts to guess it.", MAX_ATTEMPTS);

    let mut game = Game::new();

    loop {
        print!("\nEnter your guess: ");
        io::stdout().flush().unwrap();

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Process the guess and provide feedback.
        match game.process_guess(&guess) {
            GuessResult::TooHigh => {
                println!("Too high! You have {} attempts left.", game.attempts_left());
            }
            GuessResult::TooLow => {
                println!("Too low! You have {} attempts left.", game.attempts_left());
            }
            GuessResult::Correct => {
                println!("\nCongratulations! You've guessed the number!");
                println!("You won with {} attempts remaining!", game.attempts_left());
                break;
            }
            GuessResult::Invalid(msg) => {
                println!("Error: {}", msg);
                continue;
            }
        }

        // Check if player has run out of attempts.
        if game.is_game_over() {
            println!("\nGame Over! You've run out of attempts.");
            println!("The number was: {}", game.secret_number());
            break;
        }
    }

    println!("\nThanks for playing!");
}