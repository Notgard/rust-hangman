use std::fmt;
use std::fmt::Formatter;

#[derive(PartialEq, Eq)]
pub(crate) enum GameState {
    Loser,
    Winner,
    InGame,
    None
}

pub(crate) struct Player {
    pub(crate) name: String,
    pub(crate) state: GameState,
    pub(crate) hits: i32,
    pub(crate) wins: i32,
    pub(crate) loses: i32,
}

impl Player {
    fn reset_game(&mut self) {
        self.state = GameState::None;
        self.hits = 0;
    }

    fn reset_score(&mut self) {
        self.wins = 0;
        self.loses = 0;
    }

    pub(crate) fn set_state(&mut self, state: GameState) {
        self.state = state;
    }
}

#[derive(PartialEq, Debug)]
enum InvalidWordSearch {
    NoMatchingCriteria,
    InvalidStartingLetter(char),
    InvalidWordLength(usize),
}

impl fmt::Display for InvalidWordSearch {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let description = match *self {
            InvalidWordSearch::NoMatchingCriteria => "Neither of the given values match any of the words in the list",
            InvalidWordSearch::InvalidStartingLetter(char) => "The given character '{char}' is not a letter of the alphabet",
            InvalidWordSearch::InvalidWordLength(size) => "No word in the list is of length '{size}'",
        };
        f.write_str(description)
    }
}

