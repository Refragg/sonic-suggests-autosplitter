// This autosplitter is build with no_std in mind but we cannot actually enable it due to a bug with the wasi environment
//#![no_std]

use asr::{watcher::{Watcher, Pair}};

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum State {
    InMenu,
    IntroCutscene,
    InGame,
    FightingBoss,
    BossBeaten,
    EndCutscene,
    InitializingGame
}

pub fn get_state_from_int(state: &i32) -> State
{
    match state {
        0 => State::InMenu,
        1 => State::IntroCutscene,
        2 => State::InGame,
        3 => State::FightingBoss,
        4 => State::BossBeaten,
        5 => State::EndCutscene,
        6 => State::InitializingGame,
        _ => State::InitializingGame
    }
}

pub struct GameState {
    pub is_loading: Watcher<bool>,
    pub current_state: Watcher<State>,
    pub lives: Watcher<i32>,
    pub rings: Watcher<i32>,
    pub checkpoint: Watcher<i32>,
    pub boss_health: Watcher<i32>
}

pub struct GameStatePair {
    pub is_loading: Pair<bool>,
    pub current_state: Pair<State>,
    pub lives: Pair<i32>,
    pub rings: Pair<i32>,
    pub checkpoint: Pair<i32>,
    pub boss_health: Pair<i32>
}