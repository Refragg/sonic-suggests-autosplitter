// This autosplitter is build with no_std in mind but we cannot actually enable it due to a bug with the wasi environment
//#![no_std]

use asr::{print_message, timer, timer::TimerState, Settings};
use super::{GameStatePair, State};

macro_rules! split_if_true {
    ( $e:expr ) => {
        if $e { return true }
    };
}

#[derive(Settings)]
pub struct Settings {
    /// Start timer on main menu
    #[default = true]
    start_main_menu: bool,

    /// Reset timer on main menu
    #[default = true]
    reset_main_menu: bool,
    
    /// Split when triggering Eggmans fight
    #[default = true]
    split_boss_entrance: bool,

    /// Split when Eggman dies
    #[default = true]
    split_boss_death: bool,

    /// Split on each hit on Eggman
    #[default = false]
    split_boss_hit: bool,

    /// Split when a cutscene starts
    #[default = false]
    split_cutscene_start: bool,

    /// Split when a cutscene ends
    #[default = false]
    split_cutscene_end: bool,
    
    /// Split when triggering a checkpoint
    #[default = false]
    split_checkpoint: bool,

    /// Split when Sonic dies
    #[default = false]
    split_sonic_death: bool,

    /// Split when Sonic collects a ring
    #[default = false]
    split_sonic_ring_collect: bool,

    /// Split when Sonic loses rings
    #[default = false]
    split_sonic_ring_loss: bool,
}

pub fn autosplitter_startup() {
    print_message("Sonic Suggests Autosplitter - Loaded, waiting for game process")
}

pub fn autosplitter_init(settings: &Settings) {
    print_message("Sonic Suggests Autosplitter - Attached to process, beginning main autosplitter loop")
}

pub fn autosplitter_update(vars: &GameStatePair, settings: &Settings) {

}

pub fn autosplitter_is_loading(vars: &GameStatePair, settings: &Settings) -> bool {
    return vars.is_loading.current;
}

pub fn autosplitter_reset(vars: &GameStatePair, settings: &Settings) -> bool {
    settings.reset_main_menu && vars.current_state.changed_to(&State::InMenu) 
}

pub fn autosplitter_split(vars: &GameStatePair, settings: &Settings) -> bool {
    split_if_true!(settings.split_boss_entrance && vars.current_state.changed_to(&State::FightingBoss));
    split_if_true!(settings.split_boss_death && vars.current_state.changed_to(&State::BossBeaten));
    split_if_true!(settings.split_boss_hit && vars.boss_health.decreased() && vars.boss_health.current >= 0);
    
    split_if_true!(settings.split_checkpoint && vars.checkpoint.increased());
    
    split_if_true!(settings.split_cutscene_start &&
        (vars.current_state.changed_to(&State::IntroCutscene) ||
        vars.current_state.changed_to(&State::EndCutscene)));
    split_if_true!(settings.split_cutscene_end &&
        (vars.current_state.changed_to(&State::InGame) ||
        vars.current_state.changed_to(&State::InMenu)));
    
    split_if_true!(settings.split_sonic_death && vars.lives.decreased());
    split_if_true!(settings.split_sonic_ring_loss && vars.rings.decreased());
    split_if_true!(settings.split_sonic_ring_collect && vars.rings.increased());

    false
}

pub fn autosplitter_start(vars: &GameStatePair, settings: &Settings) -> bool {
    settings.start_main_menu && vars.current_state.changed_to(&State::IntroCutscene)
}

// Taken from the Sonic Colors Ultimate autosplitter by Jujstme
// https://github.com/SonicSpeedrunning/LiveSplit.SonicColorsUltimate/blob/831198961735fc204431c0365c4a7884456a108a/src/lib.rs#L649
pub fn autosplitter_loop(vars: &GameStatePair, settings: &Settings) {

    // Splitting logic. Adapted from OG LiveSplit:
    // Order of execution
    // 1. update() will always be run first. There are no conditions on the execution of this action.
    autosplitter_update(&vars, &settings);

    // 2. If the timer is currently either running or paused,
    if timer::state() == TimerState::Running || timer::state() == TimerState::Paused {
        // 2.1 then the isLoading and reset actions will be run
        if autosplitter_is_loading(&vars, &settings) {
            timer::pause_game_time()
        }
        else {
            timer::resume_game_time()
        }

        if autosplitter_reset(&vars, &settings) {
            timer::reset()
        }
        // 3. If reset does not return true, then the split action will be run.
        else if autosplitter_split(&vars, &settings) {
            timer::split()
        }
    }

    // 4. If the timer is currently not running (and not paused), then the start action will be run.
    if timer::state() == TimerState::NotRunning {
        if autosplitter_start(&vars, &settings) {
            timer::start();

            if autosplitter_is_loading(&vars, &settings) {
                timer::pause_game_time()
            }
            else {
                timer::resume_game_time()
            }
        }
    }
}