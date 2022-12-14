mod state;
mod utils;

use std::fs;
use std::collections::{HashMap, HashSet};
#[allow(unused_imports)]
use itertools::Itertools;
use std::process::exit;
use utils::{Hangman, WordManager, as_char, as_len};
use state::{Player, GameState, print_high_scores};

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

    let mut hangman = Hangman { sprite: String::new() };

    let mut reg_player: Player;
    let mut player: Player;

    let mut players: HashMap<String, Player> = HashMap::new();
    /*
            MAIN GAME LOOP
     */
    loop {
        hangman.reset();

        println!("Please enter your name: ");
        let mut name = String::new();
        std::io::stdin()
            .read_line(&mut name)
            .expect("Failed to read input");

        let mut copy = String::new();
        let mut copy2 = String::new();
        // Store active players in memory and change between players
        if !players.contains_key(&name) {
            copy = name.to_owned();
            copy2 = name.to_owned();
            reg_player = Player {
                name: copy,
                state: GameState::None,
                hits: 0,
                wins: 0,
                loses: 0,
                high_score: 0
            };
            players.insert(
                copy2,
                reg_player
            );
        }
        //TODO: NED TO UPDATE PLAYER IN MAP AFTER GAME
        player = players.get(&*name).cloned().unwrap();
        println!("Menu:\n\
    1: Play Game\n\
    2: Guess Random Word\n\
    3: Guess Manual Word\n\
    4: Show high scores\n\
    5: Quit");

        let mut option = String::new();
        std::io::stdin()
            .read_line(&mut option)
            .expect("Failed to read input");

        let mut guess_word : String = String::new();
        let mut letter = String::new();
        let mut length = String::new();

        let list = file_reader("wordlist10000.txt");

        match option.as_str().trim() {
            "1" => {
                println!("What letter does the word have to start with ? : ");
                std::io::stdin().read_line(&mut letter).expect("Failed to read input");
                println!("And what's the length of the word? : ");
                std::io::stdin().read_line(&mut length).expect("Failed to read input");
                let guesses = list.get_word(as_char(&letter), as_len(length.trim())).unwrap();
                println!("Which of these words do you choose ? :");
                let mut print = String::new();
                for (index, word) in guesses.iter().enumerate() {
                    print.push_str(word);
                    print.push_str("  ");
                    if index % 12 == 0 && index != 0{
                        print.push_str("\n");
                    }
                }
                println!("{}", print);
                std::io::stdin().read_line(&mut guess_word).expect("Failed to read input");
                player.set_state(GameState::InGame);
            },
            "2" => {
                println!("What's the length of the word? : ");
                std::io::stdin().read_line(&mut length).expect("Failed to read input");
                guess_word = list.get_random_word(as_len(length.trim())).unwrap();
                player.set_state(GameState::InGame);
            },
            "3" => {
                println!("What word do you want to guess");
                std::io::stdin().read_line(&mut guess_word).expect("Failed to read input");
                player.set_state(GameState::InGame);
            },
            "4" => {
                print_high_scores(&players);
                continue;
            },
            "5" => {
                exit(0)
            }
            _ => {
                println!("Something went wrong...");
            }
        }
        guess_word = String::from(guess_word.trim());
        let mut current_guess = String::new();
        let mut guesses: HashSet<String> = vec![].into_iter().collect();

        let mut correct_guesses = vec!["_"; guess_word.len()];
        let letters: Vec<String> = guess_word.trim().split_terminator("").skip(1).map(|l| {
            String::from(l).to_uppercase()
        }).collect();


        while player.state == GameState::InGame {
            let mut l = String::new();
            println!("Guess a letter: ");
            std::io::stdin().read_line(&mut l).expect("Failed to read input");
            current_guess = l.trim().to_uppercase();

            //Avoid possible error on hashset
            if guesses.contains(&*current_guess) {
                continue;
            }
            guesses.insert(current_guess.to_owned());

            print!("Guessed Letters : {}", itertools::join(&guesses, " "));
            //Check if the letter guess is in the word
            if letters.contains(&current_guess) {
                for (index, letter) in letters.iter().enumerate() {
                    if letter == &current_guess {
                        correct_guesses[index] = letter;
                    }
                }
            } else {
                hangman.update(player.hits as usize);
                player.hit();
            }

            hangman.print();

            println!("{}", correct_guesses.join(" "));


            if player.hits == utils::MAX_MISTAKES {
                player.lose();
                println!("So sorry. You struck out.");
                println!("The word was {}", guess_word.to_uppercase());
            } else if !correct_guesses.contains(&"_") {
                player.win();
                println!("You win in {} guesses!!", guesses.iter().count())
            }
        }
        if player.state == GameState::Winner || player.state == GameState::Loser {
            let mut persist = String::new();
            println!("Do you want to play again ? Y/N");
            std::io::stdin().read_line(&mut persist).expect("Failed to read input");
            match persist.to_uppercase().as_str() {
                "Y" => {
                    println!("Press ENTER.");
                    continue;
                },
                "N" => break,
                _ => {}
            }
        }
    }


    //env::set_var("RUST_BACKTRACE", "full");
/*
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
    */
}

fn file_reader(path : &str) -> String {
    let word_list = fs::read_to_string(path)
        .expect("File not found!");
    word_list
}