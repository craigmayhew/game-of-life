use bevy::{
    prelude::*, //default bevy
    input::{keyboard::KeyCode, Input},
};

use crate::{
    AppState,
};

pub fn run (
    keys: Res<Input<KeyCode>>,
    mut state: ResMut<State<AppState>>,
) {
    //controls
    if keys.just_pressed(KeyCode::Space) {
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
    } else if keys.just_pressed(KeyCode::Return) {
        // todo, place life form?
    } else if keys.just_pressed(KeyCode::L) {
        // load game
        if state.current() == &AppState::InGame {
            let res = state.set(AppState::LoadGame);
            if let Err(e) = res {
                println!("Keyboard System, Error changing state to LoadGame: {}", e);
            }
        }
    } else if keys.just_pressed(KeyCode::K) {
        // save game
        if state.current() == &AppState::InGame {
            let res = state.set(AppState::SaveGame);
            if let Err(e) = res {
                println!("Keyboard System, Error changing state to SaveGame: {}", e);
            }
        }
    }
}
