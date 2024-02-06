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
    state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
    mut game_speed: ResMut<GameSpeed>,
) {
    //controls
    if keys.just_pressed(KeyCode::P) {
        // (un)pause game
        match &state.0 {
            AppState::InGame => {
                next_state.set(AppState::Paused);
            },
            AppState::Paused => {
                next_state.set(AppState::InGame);
            },
            _ => {},
        }
    } else if keys.just_pressed(KeyCode::L) {
        // load game
        if &state.0 == &AppState::InGame || &state.0 == &AppState::Paused {
            next_state.set(AppState::LoadGame);
        }
    } else if keys.just_pressed(KeyCode::K) {
        // save game
        if &state.0 == &AppState::InGame || &state.0 == &AppState::Paused {
            next_state.set(AppState::SaveGame);
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
    //access menu
    if keys.just_pressed(KeyCode::Escape) {
        if &state.0 == &AppState::InGame || &state.0 == &AppState::Paused {
            next_state.set(AppState::Splash);
        } else if &state.0 == &AppState::Splash {
            next_state.set(AppState::InGame);
        }
    }
}
