use std::io;

use rand::Rng;

enum GameResult {
    Win,
    Lose,
    Draw
}

fn main() {
    println!("Welcome to Rock-Paper-Scissors");
    println!("Instructions : Enter 'rock', 'paper', or 'scissors'. Type quit to exit");

    loop {
        println!("Make you choice: ");
        
        let user_choice = get_user_choice();
        if(user_choice) == "quit"{
            println!("Thank you for playing. Good bye!");
            break;
        }


        let computer_choice = get_computer_choice();

        println!("Computer choice is: {} ", computer_choice);

        match determine_winner(&user_choice, &computer_choice) {
            GameResult::Win => println!("You win!"), 
            GameResult::Lose => println!("You lose!"), 
            GameResult::Draw => println!("It's a draw!"), 
        }
    }

}

fn determine_winner(user_choice: &str, computer_choice: &str) -> GameResult {
    match (user_choice, computer_choice) {
        ("rock", "scissors") => GameResult::Win,
        ("paper", "rock") => GameResult::Win,
        ("scissors", "paper") => GameResult::Win,
        (a, b ) if a==b => GameResult::Draw,
        _ => GameResult::Lose

    }
}



fn get_computer_choice() -> String {
    let chices = ["rock", "paper","scissors"];
    let index = rand::thread_rng().gen_range(0..chices.len());
    chices[index].to_string()
}

fn get_user_choice() -> String {
    let mut choice = String::new();

    io::stdin().read_line(&mut choice).expect("Failed to read the line.");

    let choice = choice.trim().to_lowercase();

    match choice.as_str() {
        "rock"|"paper"|"scissors"|"quit" => choice,
        _=> {
            println!("Invalid choice. Please enter 'rock'. 'paper','scissors'");
            get_user_choice()
        }
    }

}




