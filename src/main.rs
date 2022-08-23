mod state;
mod utils;

use std::{env, fs};
use std::collections::HashSet;
use std::process::exit;
use std::thread::current;
use utils::{Hangman, WordManager};
use state::{Player, GameState};

fn main() {

    println!("
  ██   ██  █████  ███    ██  ██████  ███    ███  █████  ███    ██
  ██   ██ ██   ██ ████   ██ ██       ████  ████ ██   ██ ████   ██
  ███████ ███████ ██ ██  ██ ██   ███ ██ ████ ██ ███████ ██ ██  ██
  ██   ██ ██   ██ ██  ██ ██ ██    ██ ██  ██  ██ ██   ██ ██  ██ ██
  ██   ██ ██   ██ ██   ████  ██████  ██      ██ ██   ██ ██   ████
    ");

    println!("     _________
    |/      |
    |      (_)
    |      \\|/
    |       |
    |      / \
    |
____|____");

    let mut hangman = Hangman { sprite: "".to_string() };
    println!("Please enter your name: ");
    let mut name = String::new();
    std::io::stdin()
        .read_line(&mut name)
        .expect("Failed to read input");
    let mut player = Player {
        name,
        state: GameState::None,
        hits: 0,
        wins: 0,
        loses: 0
    };
    hangman.reset();

    println!("Menu:\n\
    1: Play Game\n\
    2: Guess Random Word\n\
    3: Quit\n\
    You Pick: ");

    let mut option = String::new();
    std::io::stdin()
        .read_line(&mut option)
        .expect("Failed to read input");

    let mut guess_word : String = String::new();
    let mut letter = String::new();
    let mut length = String::new();

    let list = file_reader("wordlist10000.txt");
    match &*option {
        "1" => {
            println!("What letter does the word have to start with ? : ");
            std::io::stdin().read_line(&mut letter).expect("Failed to read input");
            println!("And what's the length of the word? : ");
            std::io::stdin().read_line(&mut length).expect("Failed to read input");
            let guesses = list.get_word(as_char(letter), as_len(length)).unwrap();
            println!("Which of these words do you choose ? :");
            std::io::stdin().read_line(&mut guess_word).expect("Failed to read input");
            player.set_state(GameState::InGame);
        },
        "2" => {
            println!("What's the length of the word? : ");
            std::io::stdin().read_line(&mut length).expect("Failed to read input");
            guess_word = list.get_random_word(as_len(length)).unwrap();
            player.set_state(GameState::InGame);
        },
        "3" => {
            exit(0)
        }
        _ => {}
    }

    let mut current_guess = String::new();
    let mut guesses: HashSet<String> = vec![].into_iter().collect();

    let mut correct_guesses = vec!["_"; guess_word.len()];
    let letters = guess_word.split("").collect();

    while player.state == GameState::InGame {
        let mut l = String::new();
        println!("Guess a letter: ");
        std::io::stdin().read_line(&mut l).expect("Failed to read input");
        current_guess = l;
        if letters.
        guesses.insert(current_guess.to_owned());

        if player.hits == 6 {
            player.state = GameState::Loser;
        }
    }
    hangman.print();
    println!("{}", hangman.sprite);

    env::set_var("RUST_BACKTRACE", "full");

    println!("{}", list.get_random_word(6).unwrap());
    match list.get_word('z',6) {
        Ok(res) => {
            for word in res {
                println!("{}", word);
            }
        }
        Err(err) => {
            println!("{}", err);
        }
    }
}

fn file_reader(path : &str) -> String {
    let word_list = fs::read_to_string(path)
        .expect("File not found!");
    word_list
}