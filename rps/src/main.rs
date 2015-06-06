extern crate rand;

use std::io;
use rand::Rng;

fn main() {
    println!("Rock, Paper, or Scissors?");

    let mut user_choice = String::new();

    io::stdin().read_line(&mut user_choice)
        .ok()
        .expect("Failed to read line");

    let user_choice = user_choice.trim();
    println!("Your choice: {}", user_choice);

    if user_choice == "Rock" || user_choice == "Paper" || user_choice == "Scissors" {
        let computer_choice = determine_computer_choice();
        println!("Computer's choice: {}", computer_choice);

        println!("{}", choose_winner(user_choice, computer_choice));
    } else {
        println!("Incorrect Input! Please choose: Rock, Paper, or Scissors");        
    }
}

fn determine_computer_choice() -> &'static str {
    let num = rand::thread_rng().gen_range(1, 4);
    match num {
        1 => "Rock",
        2 => "Paper",
        3 => "Scissors",
        _ => "",
    }
}

fn winning_map(choice: &'static str) -> &'static str {
    match choice {
        "Paper" => "Rock",
        "Scissors" => "Paper",
        "Rock" => "Scissors",
        _ => "",
    }
}

fn choose_winner(user: &str, computer: &'static str) -> &'static str {
    if user == computer {
        "Tie!"
    } else if winning_map(computer) == user {
        "Computer wins!"
    } else {
        "User wins!"
    }
}
