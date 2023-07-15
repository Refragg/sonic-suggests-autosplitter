#![no_std]

use arrayvec::ArrayString;
use asr::print_message;
use numtoa::NumToA;
use super::{GameStatePair, State};

pub fn print_game_state(vars: &GameStatePair, only_print_changed: bool) {
    let mut num_buffer = [0u8; 20];

    let mut final_string: ArrayString<256> = ArrayString::new();

    match only_print_changed {
        true => {
            if vars.is_loading.changed() {
                final_string.push_str("\nis_loading: ");
                final_string.push_str(match vars.is_loading.current {
                    true => { "yes" }
                    false => { "no" }
                });
            }

            if vars.current_state.changed() {
                final_string.push_str("\ncurrent_state: ");
                final_string.push_str(match vars.current_state.current {
                    State::InMenu => "InMenu",
                    State::IntroCutscene => "IntroCutscene",
                    State::InGame => "InGame",
                    State::FightingBoss => "FightingBoss",
                    State::BossBeaten => "BossBeaten",
                    State::EndCutscene => "EndCutscene",
                    State::InitializingGame => "InitializingGame"
                });
            }

            if vars.lives.changed() {
                final_string.push_str("\nlives: ");
                final_string.push_str(vars.lives.current.numtoa_str(10, &mut num_buffer));
            }

            if vars.rings.changed() {
                final_string.push_str("\nrings: ");
                final_string.push_str(vars.rings.current.numtoa_str(10, &mut num_buffer));
            }

            if vars.checkpoint.changed() {
                final_string.push_str("\ncheckpoint: ");
                final_string.push_str(vars.checkpoint.current.numtoa_str(10, &mut num_buffer));
            }

            if vars.boss_health.changed() {
                final_string.push_str("\nboss_health: ");
                final_string.push_str(vars.boss_health.current.numtoa_str(10, &mut num_buffer));
            }
        }
        
        false => {
            final_string.push_str("\nis_loading: ");
            final_string.push_str(match vars.is_loading.current {
                true => { "yes" }
                false => { "no" }
            });

            final_string.push_str("\ncurrent_state: ");
            final_string.push_str(match vars.current_state.current {
                State::InMenu => "InMenu",
                State::IntroCutscene => "IntroCutscene",
                State::InGame => "InGame",
                State::FightingBoss => "FightingBoss",
                State::BossBeaten => "BossBeaten",
                State::EndCutscene => "EndCutscene",
                State::InitializingGame => "InitializingGame"
            });

            final_string.push_str("\nlives: ");
            final_string.push_str(vars.lives.current.numtoa_str(10, &mut num_buffer));

            final_string.push_str("\nrings: ");
            final_string.push_str(vars.rings.current.numtoa_str(10, &mut num_buffer));

            final_string.push_str("\ncheckpoint: ");
            final_string.push_str(vars.checkpoint.current.numtoa_str(10, &mut num_buffer));

            final_string.push_str("\nboss_health: ");
            final_string.push_str(vars.boss_health.current.numtoa_str(10, &mut num_buffer));
        }
    }

    if final_string.len() != 0 {
        final_string.remove(0); // remove the first new line character
        print_message(final_string.as_str());
    }
}