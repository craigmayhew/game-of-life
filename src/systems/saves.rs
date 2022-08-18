use bevy::{
    prelude::*, //default bevy
};
use std::fs::{File,read_to_string};
use std::io::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    AppState,
    SessionResource,
    systems::life::{Life,LifeDataContainer},
};

#[derive(Debug, Deserialize, Serialize)]
pub struct SaveResource {
    pub life: Vec<Vec<Vec<Vec<usize>>>>,
    pub counter: i64,
    pub generation: i64,
    pub universe_size: usize,
}

pub fn load (
    mut life_entities: Query<Entity, With<Life>>,
    mut commands: Commands,
    mut session: ResMut<SessionResource>,
    mut state: ResMut<State<AppState>>,
) {
    match state.current() {
        AppState::LoadGame => {},
        _ => {return},
    }
    
    let contents = read_to_string("saves/test.ron").expect("Failed to load save file");

    let data: SaveResource = ron::from_str(&contents).unwrap();
    
    // unspawn every single life entity
    for ent in life_entities.iter_mut() {
        commands.entity(ent.to_owned()).despawn();
    }

    // so we can spawn new ones form the save file
    for (n, vec1) in data.life.iter().enumerate() {
        for (x, vec2) in vec1.iter().enumerate() {
            for (y, vec3) in vec2.iter().enumerate() {
                for (z, alive_or_not) in vec3.iter().enumerate() {
                    if alive_or_not == &1 {
                        let transform_new_life: bevy::prelude::Transform = crate::systems::life::create_life_xyz(n, x, y, z);
                    
                        // make the life form exist!
                        session.life[n][x][y][z] = LifeDataContainer::Alive(commands.spawn_bundle(PbrBundle {
                            mesh: session.life_form_meshes[n%2].clone(),
                            material: session.life_form_materials[n].clone(),
                            transform: transform_new_life,
                            ..Default::default()
                        }).insert(Life).id());
                    } else {
                        session.life[n][x][y][z] = LifeDataContainer::Dead(true);
                    }
                }
            }
        }
    }

    session.counter = data.counter;
    session.generation = data.generation;
    session.universe_size = data.universe_size;

    // in bevy 0.8 overwrite_set() is needed instead of set() when system is called via on_enter()
    let res = state.overwrite_set(AppState::Paused);
    if let Err(e) = res {
        println!("Saves System, Error changing state to Paused: {}", e);
    }
}

pub fn save (
    session: Res<SessionResource>,
    mut state: ResMut<State<AppState>>,
) {
    match state.current() {
        AppState::SaveGame => {},
        _ => {return},
    }

    // save game state
    let mut save = SaveResource {
        life: vec![vec![vec![vec![0; session.universe_size]; session.universe_size]; session.universe_size]; 6],
        counter: session.counter,
        generation: session.generation,
        universe_size: session.universe_size,
    };

    for (n, vec1) in session.life.iter().enumerate() {
        for (x, vec2) in vec1.iter().enumerate() {
            for (y, vec3) in vec2.iter().enumerate() {
                for (z, alive_or_not) in vec3.iter().enumerate() {
                    if let LifeDataContainer::Alive(_) = alive_or_not {
                        save.life[n][x][y][z] = 1;
                    }
                }
            }
        }
    }

    let mut file = File::create("saves/test.ron").unwrap();
    file.write_all(
        ron::to_string(&save).
        unwrap()
        .as_bytes(),
    )
    .unwrap();
    let result = file.sync_data();

    // in bevy 0.8 overwrite_set() is needed instead of set() when system is called via on_enter()
    let res = state.overwrite_set(AppState::Paused);
    if let Err(e) = res {
        println!("Saves System, Error changing state to Paused: {}", e);
    }
}
