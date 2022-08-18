use bevy::{
    prelude::*, //default bevy
    input::{keyboard::KeyCode, Input},
};

use crate::{
    AppState,
    GameSpeed,
};

pub fn run (
    keys: Res<Input<KeyCode>>,
    mut state: ResMut<State<AppState>>,
    mut game_speed: ResMut<GameSpeed>,
) {
    //controls
    if keys.just_pressed(KeyCode::P) {
        // (un)pause game
        match state.current() {
            AppState::InGame => {
                let res = state.set(AppState::Paused);
                if let Err(e) = res {
                    println!("Keyboard System, Error changing state to Paused: {}", e);
                }
            },
            AppState::Paused => {
                let res = state.set(AppState::InGame);
                if let Err(e) = res {
                    println!("Keyboard System, Error changing state to InGame: {}", e);
                }
            },
            _ => {},
        }
    } else if keys.just_pressed(KeyCode::L) {
        // load game
        if state.current() == &AppState::InGame || state.current() == &AppState::Paused {
            let res = state.set(AppState::LoadGame);
            if let Err(e) = res {
                println!("Keyboard System, Error changing state to LoadGame: {}", e);
            }
        }
    } else if keys.just_pressed(KeyCode::K) {
        // save game
        if state.current() == &AppState::InGame || state.current() == &AppState::Paused {
            let res = state.set(AppState::SaveGame);
            if let Err(e) = res {
                println!("Keyboard System, Error changing state to SaveGame: {}", e);
            }
        }
    }
    // game tick speed
    if keys.just_pressed(KeyCode::Plus) || keys.just_pressed(KeyCode::PageUp) {
        game_speed.ticks_per_second += 1.0;
    } else if keys.just_pressed(KeyCode::Minus) || keys.just_pressed(KeyCode::PageDown) {
        if game_speed.ticks_per_second > 1.0 {
            game_speed.ticks_per_second -= 1.0;
        }
    }
}
