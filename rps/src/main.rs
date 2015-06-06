extern crate rand;

use std::io;
use rand::Rng;

fn main() {
    println!("Rock, paper, or scissors?");

    let mut user_choice = String::new();

    io::stdin().read_line(&mut user_choice)
        .ok()
        .expect("Failed to read line");

    println!("Your choice: {}", user_choice);

    let computer_choice = determine_computer_choice();
    println!("Computer's choice: {}", computer_choice);

    println!("{}", choose_winner(user_choice.trim(), computer_choice));
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
    println!("{}", user);
    println!("{}", computer);
    println!("{}", user == computer);
    if user == computer {
        "Tie!"
    } else if winning_map(computer) == user {
        "Computer wins!"
    } else {
        "User wins!"
    }
}
