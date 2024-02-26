use bevy::{
    prelude::*, //default bevy
};
use serde::{Deserialize, Serialize};
use std::fs::{read_to_string, File};
use std::io::prelude::*;
use std::time::SystemTime;

use crate::{
    systems::life::{create_life_xyz, Life, LifeDataContainer, TETRA_INDEXES},
    AppState, SessionResource,
};

#[derive(Resource)]
pub enum GameFileToLoad {
    Some(String),
    None(),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SaveResource {
    pub life: Vec<Vec<Vec<Vec<usize>>>>,
    pub counter: i64,
    pub generation: i64,
    pub universe_size: usize,
}

pub fn load(
    mut life_entities: Query<Entity, With<Life>>,
    mut commands: Commands,
    mut session: ResMut<SessionResource>,
    mut next_state: ResMut<NextState<AppState>>,
    mut game_to_be_loaded: ResMut<GameFileToLoad>,
) {
    let name_of_load_file: String;
    match game_to_be_loaded.as_mut() {
        GameFileToLoad::Some(file_name) => {
            name_of_load_file = file_name.to_string();
        }
        GameFileToLoad::None() => {
            name_of_load_file = "latest".to_string();
        }
    }

    let contents = read_to_string("saves/".to_string() + &name_of_load_file + ".ron")
        .expect("Failed to load save file");

    let result = ron::from_str::<SaveResource>(&contents);

    match result {
        Ok(data) => {
            // unspawn every single life entity
            for ent in life_entities.iter_mut() {
                commands.entity(ent.to_owned()).despawn();
            }

            // resize the universe to match the load file
            session.life = vec![
                vec![
                    vec![
                        vec![LifeDataContainer::Dead(true); data.universe_size];
                        data.universe_size
                    ];
                    data.universe_size
                ];
                6
            ];

            // so we can spawn new ones form the save file
            for tetra_index in TETRA_INDEXES {
                let n: usize = tetra_index as usize;
                for (x, vec2) in data.life[n].iter().enumerate() {
                    for (y, vec3) in vec2.iter().enumerate() {
                        for (z, alive_or_not) in vec3.iter().enumerate() {
                            if alive_or_not == &1 {
                                let transform_new_life: bevy::prelude::Transform =
                                    create_life_xyz(&tetra_index, x, y, z);

                                // make the life form exist!
                                session.life[n][x][y][z] = LifeDataContainer::Alive(
                                    commands
                                        .spawn(PbrBundle {
                                            mesh: session.life_form_meshes[n % 2].clone(),
                                            material: session.life_form_materials[n].clone(),
                                            transform: transform_new_life,
                                            ..Default::default()
                                        })
                                        .insert(Life)
                                        .id(),
                                );
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
        }
        Err(e) => {
            println!("Saves System, Error loading save file: {}", e);
        }
    }

    if &name_of_load_file[0..5] == "test_" {
        next_state.set(AppState::InGame)
    } else {
        next_state.set(AppState::Paused)
    };
}

pub fn save(session: Res<SessionResource>, mut next_state: ResMut<NextState<AppState>>) {
    // save game state
    let mut save = SaveResource {
        life: vec![
            vec![
                vec![vec![0; session.universe_size]; session.universe_size];
                session.universe_size
            ];
            6
        ],
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

    let dawn = SystemTime::from(SystemTime::UNIX_EPOCH);
    for filename in [
        "latest",
        &dawn
            .elapsed()
            .expect("Elapsed time has errored when saving")
            .as_secs()
            .to_string(),
    ] {
        let mut file = File::create("saves/".to_owned() + filename + ".ron").unwrap();
        file.write_all(ron::to_string(&save).unwrap().as_bytes())
            .unwrap();
        let result = file.sync_data();
        if let Err(e) = result {
            println!("Saves System, Error trying to write save file: {}", e);
        }
    }

    next_state.set(AppState::Paused);
}
