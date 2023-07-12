// This autosplitter is build with no_std in mind but we cannot actually enable it due to a bug with the wasi environment
//#![no_std]
//
//asr::panic_handler!();
asr::async_main!(stable);

mod autosplitter;
mod debug;
mod state;

use asr::{
    future::{next_tick, sleep},
    print_message,
    signature::Signature,
    watcher::Watcher,
    Address, Process,
};
use autosplitter::*;
use core::{str, time::Duration};
use debug::print_game_state;
use state::{get_state_from_int, GameState, GameStatePair, State};

const WINDOWS_PROCESS_NAME: &str = "SonicSuggests.exe";
const LINUX_PROCESS_NAME: &str = "SonicSuggests.x";
const MACOS_PROCESS_NAME: &str = "SonicSuggests";

const SIGNATURE: Signature<84> = Signature::new(
    "53 6F 6E 69 63 2D 53 75 67 67 65 73 74 73 5F 47 61 6D 65 53 74 61 74 65 5F 76 ?? ?? ?? \
?? ?? ?? \
?? ?? ?? ?? \
?? ?? ?? ?? \
?? ?? ?? ?? \
?? ?? ?? ?? \
?? ?? ?? ?? \
?? ?? ?? ?? \
53 6F 6E 69 63 2D 53 75 67 67 65 73 74 73 5F 47 61 6D 65 53 74 61 74 65 5F 45 4E 44",
);

const VERSIONED_SIGNATURE_STRING_SIZE: usize = 29;
const SIGNATURE_STRING_PADDING: usize = 32 - VERSIONED_SIGNATURE_STRING_SIZE;
const END_SIGNATURE_STRING_SIZE: usize = 28;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct RawGameState {
    signature: [u8; VERSIONED_SIGNATURE_STRING_SIZE],
    _padding: [u8; SIGNATURE_STRING_PADDING],
    is_loading: i32,
    current_state: i32,
    lives: i32,
    rings: i32,
    checkpoint: i32,
    boss_health: i32,
    end_signature: [u8; END_SIGNATURE_STRING_SIZE],
}

macro_rules! unwrap_or_continue {
    ( $e:expr ) => {
        match $e {
            Some(x) => x,
            None => continue,
        }
    };
}

async fn main() {
    let process_name = match asr::get_os() {
        Ok(result) => match result.as_str() {
            "windows" => WINDOWS_PROCESS_NAME,
            "linux" => LINUX_PROCESS_NAME,
            "macos" => MACOS_PROCESS_NAME,
            _ => WINDOWS_PROCESS_NAME,
        },
        Err(_) => {
            panic!("Could not get operating system!");
        }
    };

    let settings = Settings::register();

    autosplitter_startup();

    loop {
        let process = Process::wait_attach(process_name).await;
        process
            .until_closes(async {
                let mut struct_address: Option<Address> = None;

                while struct_address.is_none() {
                    // FIXME: Figure out a way to just detect if we're on the settings screen or the game
                    // to avoid useless iterations because the game state isn't initialized yet.
                    sleep(Duration::from_millis(2000)).await;
                    struct_address = get_game_state_struct_address(&process);
                }

                autosplitter_init(&settings);

                let mut game_state = GameState {
                    is_loading: Watcher::new(),
                    current_state: Watcher::new(),
                    lives: Watcher::new(),
                    rings: Watcher::new(),
                    checkpoint: Watcher::new(),
                    boss_health: Watcher::new(),
                };

                loop {
                    let raw_game_state = match process.read::<RawGameState>(struct_address.unwrap())
                    {
                        Ok(state) => state,
                        Err(_) => {
                            print_message("Could not read the GameState");
                            next_tick().await;
                            break;
                        }
                    };

                    let game_signature = str::from_utf8(raw_game_state.signature.as_slice());

                    if game_signature.is_err() {
                        print_message("GameState signature was invalid");
                        break;
                    }

                    let current_state = get_state_from_int(&raw_game_state.current_state);

                    let vars = GameStatePair {
                        is_loading: *unwrap_or_continue!(game_state.is_loading.update(Some(raw_game_state.is_loading == 1))),
                        current_state: *unwrap_or_continue!(game_state.current_state.update(Some(current_state))),
                        lives: *unwrap_or_continue!(game_state.lives.update(Some(raw_game_state.lives))),
                        rings: *unwrap_or_continue!(game_state.rings.update(Some(raw_game_state.rings))),
                        checkpoint: *unwrap_or_continue!(game_state.checkpoint.update(Some(raw_game_state.checkpoint))),
                        boss_health: *unwrap_or_continue!(game_state.boss_health.update(Some(raw_game_state.boss_health))),
                    };

                    autosplitter_loop(&vars, &settings);
                    print_game_state(&vars, true);

                    next_tick().await;
                }
            })
            .await;
    }
}

fn get_game_state_struct_address(process: &Process) -> Option<Address> {
    for range in process.memory_ranges() {
        if let (Ok(address), Ok(size)) = (range.address(), range.size()) {
            let struct_address = SIGNATURE.scan_process_range(process, (address, size));

            if struct_address.is_some() {
                return struct_address;
            }
        }
    }

    None
}