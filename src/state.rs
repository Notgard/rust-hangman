use std::collections::HashMap;

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
    pub(crate) fn set_state(&mut self, state: GameState) {
        self.state = state;
    }

    pub(crate) fn hit(&mut self) {
        self.hits += 1;
    }

    pub(crate) fn lose(&mut self) {
        self.state = GameState::Loser;
        self.loses += 1;
        self.high_score -= 100;
    }

    pub(crate) fn win(&mut self) {
        self.state = GameState::Winner;
        self.wins += 1;
        self.high_score += 500;
    }

    pub(crate) fn get_name(self) {
        self.name;
    }
}

pub(crate) fn print_high_scores(players: &HashMap<String, Player>) {
    println!(
        "{0: <10} | {1: <10} | {2: <10} | {3: <10}",
        "Player", "Score", "Wins", "Losses"
    );
    for (key, value) in players.iter() {
        println!("{0: <10} | {1: <10} | {2: <10} | {3: <10}", key.trim(), value.high_score, value.wins, value.loses);
    }
}

