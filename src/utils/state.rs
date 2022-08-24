use std::fmt;
use std::fmt::Formatter;

#[derive(PartialEq, Debug)]
pub enum InvalidWordSearch {
    NoMatchingCriteria,
    InvalidStartingLetter(char),
    InvalidWordLength(usize),
}

impl fmt::Display for InvalidWordSearch {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let description = match *self {
            InvalidWordSearch::NoMatchingCriteria => "Neither of the given values match any of the words in the list",
            InvalidWordSearch::InvalidStartingLetter(_char) => "The given character '{char}' is not a letter of the alphabet",
            InvalidWordSearch::InvalidWordLength(_size) => "No word in the list is of length '{size}'",
        };
        f.write_str(description)
    }
}

