use bevy::{
    prelude::*, //default bevy
};

use crate::{AppState, GameSpeed};

pub fn run(
    keys: Res<ButtonInput<KeyCode>>,
    state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
    mut game_speed: ResMut<GameSpeed>,
) {
    //controls
    if keys.just_pressed(KeyCode::KeyP) {
        // (un)pause game
        match state.get() {
            AppState::InGame => {
                next_state.set(AppState::Paused);
            }
            AppState::Paused => {
                next_state.set(AppState::InGame);
            }
            _ => {}
        }
    } else if keys.just_pressed(KeyCode::KeyL) {
        // load game
        if state.get() == &AppState::InGame || state.get() == &AppState::Paused {
            next_state.set(AppState::LoadGame);
        }
    } else if keys.just_pressed(KeyCode::KeyK) {
        // save game
        if state.get() == &AppState::InGame || state.get() == &AppState::Paused {
            next_state.set(AppState::SaveGame);
        }
    }
    // game tick speed
    if keys.just_pressed(KeyCode::NumpadAdd) || keys.just_pressed(KeyCode::PageUp) {
        game_speed.ticks_per_second += 1.0;
    } else if keys.just_pressed(KeyCode::Minus)
        || keys.just_pressed(KeyCode::NumpadSubtract)
        || keys.just_pressed(KeyCode::PageDown)
    {
        if game_speed.ticks_per_second > 1.0 {
            game_speed.ticks_per_second -= 1.0;
        }
    }
    //access menu
    if keys.just_pressed(KeyCode::Escape) {
        match state.get() {
            &AppState::InGame | &AppState::Paused => {
                next_state.set(AppState::Splash);
            }
            &AppState::Splash => {
                next_state.set(AppState::InGame);
            }
            _ => {}
        }
    }
}
