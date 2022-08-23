mod state;
//use std::thread;
//use std::time::Duration;
#[allow(unused_imports)]
use std::sync::{Arc, mpsc, Mutex};
#[allow(unused_imports)]
use std::sync::mpsc::{Receiver, Sender};
use rand::seq::SliceRandom;
use rand::Rng;
use state::InvalidWordSearch;

static ASCII_LOWER: [char; 26] = [
    'a', 'b', 'c', 'd', 'e',
    'f', 'g', 'h', 'i', 'j',
    'k', 'l', 'm', 'n', 'o',
    'p', 'q', 'r', 's', 't',
    'u', 'v', 'w', 'x', 'y',
    'z',
];

pub struct Hangman {
    pub(crate) sprite: String
}

impl Hangman {
    pub(crate) fn update(&mut self, life: usize) {
        let body_parts = vec!["O", "|", "|", "\\", "/", "\\", "/"];
        self.sprite = self.sprite.replacen("*", body_parts[life], 1);
    }

    pub(crate) fn reset(&mut self) {
        self.sprite = "
             ___________
            |	    |
            |	    |
            |	    *
            |	   ***
            |	    *
            |	   * *
            |
            |
     _______|_______".to_owned();
    }

    pub(crate) fn print(&self) {
        println!("{}", self.sprite.replace("*", " "))
    }
}

pub trait WordManager {
    //TODO: word search must be threaded
    fn get_word(&self, start_letter: char, word_len: usize) -> Result<Vec<String>, InvalidWordSearch>;

    fn get_random_word(&self, word_len: usize) -> Option<String>;
}

impl WordManager for String {

    /* Tried to use thread for this method. TODO: Will have to rework on this for 1.1
    fn get_word(&self, start_letter: char, word_len: usize) -> Option<Vec<String>> {
        println!("{}", self);
        let (tx, rx): (Sender<Vec<&str>>, Receiver<Vec<&str>>) = mpsc::channel();

        let current_letter = Arc::new(Mutex::new('a'));
        let letter = current_letter.clone();
        //thread::spawn(move || {
            let words: Vec<&str> = self.split(|c| ASCII_LOWER.contains(&c))
                .collect();
            for sorted_word in words {
                let split = sorted_word.split("\n").collect();
                tx.send(split).unwrap();
                //thread::sleep(Duration::from_secs(1));
                let mut current_letter = letter.lock().unwrap();
                *current_letter = char::from_u32(*current_letter as u32 + 1).unwrap();
            }
        //});
        let mut filtered: Vec<String> = vec![];
        for collection in rx {
            let letter = *current_letter.lock().unwrap();
            if letter == start_letter {
                filtered = collection.iter().map(|&w|
                    String::from(w)
                ).filter(|word| word.len() == word_len).collect();
                if !filtered.is_empty() {
                    return Some(filtered);
                }
            }
        }
        return Some(filtered);
    }
*/
    fn get_word(&self, start_letter: char, word_len: usize) -> Result<Vec<String>, InvalidWordSearch> {
        let format_char = start_letter.to_lowercase().next().unwrap();
        if !ASCII_LOWER.contains(&format_char) {
            return Err(InvalidWordSearch::InvalidStartingLetter(format_char));
        }
        let words = self.split("\n").filter(|&word| {
            word.chars().next().unwrap() == format_char
                && word.len() == word_len
        }).map(|word| {
            String::from(word)
        }).collect::<Vec<String>>();

        if words.is_empty() {
            return Err(InvalidWordSearch::NoMatchingCriteria);
        }
        return Ok(words);
    }

    fn get_random_word(&self, word_len: usize) -> Option<String> {
        let words =  self.split("\n");
        let mut filtered_words: Vec<&str> = words.filter(|&word| {
            word.len() == word_len
        }).collect();
        if filtered_words.is_empty() {
            return None;
        }
        let mut rng = rand::thread_rng();
        filtered_words.shuffle(&mut rand::thread_rng());
        let rn: usize = rng.gen_range(0..filtered_words.len());
        Some(filtered_words[rn].into())
    }
}

fn as_char(string: &String) -> char {
    string.chars().next().unwrap()
}

fn as_len(string: &String) -> usize {
    string.parse::<usize>().unwrap()
}