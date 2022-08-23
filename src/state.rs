use std::fmt;
use std::fmt::Formatter;

#[derive(PartialEq, Eq, Clone)]
pub(crate) enum GameState {
    Loser,
    Winner,
    InGame,
    None
}

#[derive(Clone)]
pub(crate) struct Player {
    pub(crate) name: String,
    pub(crate) state: GameState,
    pub(crate) hits: i32,
    pub(crate) wins: i32,
    pub(crate) loses: i32,
    pub(crate) high_score: i32
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

    pub(crate) fn hit(&mut self) {
        self.hits += 1;
    }
}

