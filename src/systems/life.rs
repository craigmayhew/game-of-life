use bevy::{
    prelude::*, //default bevy
    input::{keyboard::KeyCode, Input},
};

use crate::{
    AppState,
    SessionResource,
};

pub const LIFE_FORM_SIZE: f32 = 150.0;

#[derive(Default)]
pub struct LifeTag;

#[derive(Default)]
pub struct LifeSystem {}

pub fn create_life_xyz(
    n:usize,
    x:usize,
    y:usize,
    z:usize,
) -> bevy::prelude::Transform {
    let mut transform_new_life: Transform;
    //rotate it if it's every third life form (todo: as this rotations only have 4 variants they could exist outside this loop!)
    //TODO consider if n == 0 and n == 1 could/should actually be identical blocks
    if n == 0 {//white
        // position the life form in 3d space
        transform_new_life = Transform::from_xyz(
            (x as f32) * LIFE_FORM_SIZE,
            (y as f32) * LIFE_FORM_SIZE,
            (z as f32) * LIFE_FORM_SIZE
        );
        
        transform_new_life.rotate_x(std::f32::consts::PI*0.75);
        transform_new_life.rotate_y(std::f32::consts::FRAC_PI_2);
        transform_new_life.rotate_z(std::f32::consts::PI);
    } else if n == 1 {//red
        // position the life form in 3d space
        transform_new_life = Transform::from_xyz(
            (x as f32) * LIFE_FORM_SIZE,
            (y as f32) * LIFE_FORM_SIZE,
            (z as f32) * LIFE_FORM_SIZE
        );
        
        transform_new_life.rotate_x(std::f32::consts::PI*1.75);
        transform_new_life.rotate_y(0.0);
        transform_new_life.rotate_z(std::f32::consts::FRAC_PI_2);
    } else if n == 2 || n == 3 {//light blue and dark blue
        // position the life form in 3d space
        transform_new_life = Transform::from_xyz(
            (x as f32-1.0) * LIFE_FORM_SIZE,
            (y as f32+1.0) * LIFE_FORM_SIZE,
            (z as f32-1.0) * LIFE_FORM_SIZE
        );
        
        transform_new_life.rotate_x(std::f32::consts::PI*0.75);
        transform_new_life.rotate_y(0.0);
        transform_new_life.rotate_z(0.0);
    } else {//light grey and dark grey
        // position the life form in 3d space
        transform_new_life = Transform::from_xyz(
            x as f32 * LIFE_FORM_SIZE,
            y as f32 * LIFE_FORM_SIZE,
            z as f32 * LIFE_FORM_SIZE
        );

        transform_new_life.rotate_x(std::f32::consts::FRAC_PI_4);
        transform_new_life.rotate_y(std::f32::consts::PI);
        transform_new_life.rotate_z(0.0);
    }

    //set size of tetrahedrons and return
    transform_new_life.with_scale(Vec3::new(LIFE_FORM_SIZE, LIFE_FORM_SIZE, LIFE_FORM_SIZE))
}

#[derive(Component)]
pub struct Life;

// we use an enum to get around the fact that even entity id 0 is valid
#[derive(Clone,Copy)]
pub enum LifeDataContainer {
    Alive(Entity),
    Dead(bool),
}

pub fn place_life_with_keyboard(
    camera: Query<&mut Transform, With<Camera>>,
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    mut session: ResMut<SessionResource>,
) {
    // if we hit the right key(s) then generate life in a specific spot in front of the camera
    if keys.any_just_pressed([KeyCode::Key1, KeyCode::Key2, KeyCode::Key3, KeyCode::Key4, KeyCode::Key5, KeyCode::Key6, KeyCode::Space]) {
        for transform in camera.iter() {
            let xyz_in_front_of_cam = (transform.translation + (transform.forward()*1500.0)) / LIFE_FORM_SIZE;

            let x = xyz_in_front_of_cam.x;
            let y = xyz_in_front_of_cam.y;
            let z = xyz_in_front_of_cam.z;
            // TODO we need a way of detecting which of 6 tetras needs to created, for now just use the number keys
            let n = if keys.just_pressed(KeyCode::Key1) {
                0
            } else if keys.just_pressed(KeyCode::Key2) {
                1
            } else if keys.just_pressed(KeyCode::Key3) {
                2
            } else if keys.just_pressed(KeyCode::Key4) {
                3
            } else if keys.just_pressed(KeyCode::Key5) {
                4
            } else {
                5
            };
            
            if x > 0.0 && x < session.universe_size as f32 &&
            y > 0.0 && y < session.universe_size as f32 &&
            z > 0.0 && z < session.universe_size as f32 {
                match session.life[n][x as usize][y as usize][z as usize] {
                    LifeDataContainer::Alive(ent) => {//if alive currently
                        commands.entity(ent.to_owned()).despawn();
                        session.life[n][x as usize][y as usize][z as usize] = LifeDataContainer::Dead(true);
                        session.counter -= 1;
                    },
                    LifeDataContainer::Dead(_) => {// if dead currently
                        // Place a life form
                        let transform_new_life: bevy::prelude::Transform = create_life_xyz(n, x as usize, y as usize, z as usize);
                        session.life[n][x as usize][y as usize][z as usize] = LifeDataContainer::Alive(commands.spawn_bundle(PbrBundle {
                            mesh: session.life_form_meshes[n%2].clone(),
                            material: session.life_form_materials[n].clone(),
                            transform: transform_new_life,
                            ..Default::default()
                        }).insert(Life).id());
                        session.counter += 1;
                    }
                }
            }
        }
    }
}

pub fn dead_universe() -> Vec<Vec<Vec<Vec<LifeDataContainer>>>>{
    vec![vec![vec![vec![LifeDataContainer::Dead(true); crate::DEFAULT_UNIVERSE_SIZE]; crate::DEFAULT_UNIVERSE_SIZE]; crate::DEFAULT_UNIVERSE_SIZE]; 6]
}

pub fn new_universe(
    mut life_entities: Query<Entity, With<Life>>,
    mut commands: Commands,
    mut session: ResMut<SessionResource>,
    mut state: ResMut<State<AppState>>,
) {
    match state.current() {
        AppState::NewGame => {},
        _ => {return},
    }
    session.counter = 0;
    session.generation = 1;
    session.life = dead_universe();
    session.universe_size = crate::DEFAULT_UNIVERSE_SIZE;
    // unspawn every single life entity
    for ent in life_entities.iter_mut() {
        commands.entity(ent.to_owned()).despawn();
    }

    // in bevy 0.8 overwrite_set() is needed instead of set() when system is called via on_enter()
    let res = state.overwrite_set(AppState::InGame);
    if let Err(e) = res {
        println!("Life System, Error changing state to InGame from NewGame: {}", e);
    }
}

pub fn run(
    mut commands: Commands,
    mut session: ResMut<SessionResource>,
    state: Res<State<AppState>>,
) {
    // only run code after this point when the state is InGame i.e. not paused
    match state.current() {
        AppState::InGame => {},
        _ => {return},
    }

    // first generation, generate random life
    if session.generation == 1 {
        let life_to_create: Vec<Vec<Vec<Vec<LifeDataContainer>>>> = session.life.clone();
        for (n, vec1) in life_to_create.iter().enumerate() {
            for (x, vec2) in vec1.iter().enumerate() {
                for (y, vec3) in vec2.iter().enumerate() {
                    for (z, _empty_entity_id) in vec3.iter().enumerate() {
                        //randomly generate initial life in the universe
                        if rand::random::<bool>() {
                            //create no life here
                            continue;
                        }

                        let transform_new_life: bevy::prelude::Transform = create_life_xyz(n, x, y, z);
                    
                        // make the life form exist!
                        session.life[n][x][y][z] = LifeDataContainer::Alive(commands.spawn_bundle(PbrBundle {
                            mesh: session.life_form_meshes[n%2].clone(),
                            material: session.life_form_materials[n].clone(),
                            transform: transform_new_life,
                            ..Default::default()
                        }).insert(Life).id());
                        
                        //increment life counter
                        session.counter += 1;
                    }
                }
            }
        }
        session.generation = 2;
    } else if session.counter > 1 { // while there is life
        let last_gen: Vec<Vec<Vec<Vec<LifeDataContainer>>>> = session.life.clone();
        let mut next_gen = vec![vec![vec![vec![LifeDataContainer::Dead(true); session.universe_size]; session.universe_size]; session.universe_size]; 6];
        /*
        white touches dark blue and dark grey in the same xyz and light blue in the y below
        red touches light grey and light blue in same xyz and the dark grey in the y above
        light blue touches red and dark blue in the same xyz and white in the y above
        dark blue touches light blue and white in same xyz and red and dark grey either side (need to check if thats x or z)
        light grey touches dark grey and red in the same xyz and light blue and white either side (need to check if thats x or z)
        dark grey touches light grey and white in the same xyz and red in the y below
        */
        
        for (n, vec1) in last_gen.iter().enumerate() {
            for (x, vec2) in vec1.iter().enumerate() {
                for (y, vec3) in vec2.iter().enumerate() {
                    for (z, entity_life) in vec3.iter().enumerate() {
                        let mut neighbours: usize = 0;
                        /*
                          // the x>0 y>0 and z>0 are for checking if we are not needing to wrap the universe
                          // the x==0 y==0 z==0 checks are if we are wrapping the universe to find out neighbour
                        */
                        
                        //remember z goes down as you move forward from the start position
                        if n == 0 {
                            //CHECK 5 NEIGHBOURS IN SAME CUBE
                            if let LifeDataContainer::Alive(_) = last_gen[1][x][y][z] {neighbours += 1}
                            if let LifeDataContainer::Alive(_) = last_gen[2][x][y][z] {neighbours += 1}
                            if let LifeDataContainer::Alive(_) = last_gen[3][x][y][z] {neighbours += 1}
                            if let LifeDataContainer::Alive(_) = last_gen[4][x][y][z] {neighbours += 1}
                            if let LifeDataContainer::Alive(_) = last_gen[5][x][y][z] {neighbours += 1}
                            // 2 FACE CHECKS
                            // white touches dark blue and dark grey in the same xyz and light blue in the y below
                            if y > 0 && let LifeDataContainer::Alive(_) = last_gen[2][x][y-1][z] {neighbours += 1} // touches light blue below
                            if y == 0 && let LifeDataContainer::Alive(_) = last_gen[2][x][session.universe_size-1][z] {neighbours += 1} // touches light blue below (on the other side of the universe)
                            if z > 0 && let LifeDataContainer::Alive(_) = last_gen[4][x][y][z-1] {neighbours += 1} // touches light grey
                            if z == 0 && let LifeDataContainer::Alive(_) = last_gen[4][x][y][session.universe_size-1] {neighbours += 1} // touches light grey (on the other side of the universe)
                            
                            // 11 EDGE CHECKS
                            //touches 3 in x-1
                            if x == 0 && let LifeDataContainer::Alive(_) = last_gen[3][session.universe_size-1][y][z] {neighbours += 1;}
                            if x > 0 && let LifeDataContainer::Alive(_) = last_gen[3][x-1][y][z] {neighbours += 1;}
                            //touches 5 in x+1
                            if session.universe_size >  x+1 && let LifeDataContainer::Alive(_) = last_gen[5][x+1][y][z] {neighbours += 1;}
                            if session.universe_size == x+1 && let LifeDataContainer::Alive(_) = last_gen[5][0  ][y][z] {neighbours += 1;}
                            //touches 1 in y-1
                            if y == 0 && let LifeDataContainer::Alive(_) = last_gen[1][x][session.universe_size-1][z] {neighbours += 1;}
                            if y > 0 && let LifeDataContainer::Alive(_) = last_gen[1][x][y-1][z] {neighbours += 1;}
                            //touches 3 in y-1
                            if y == 0 && let LifeDataContainer::Alive(_) = last_gen[3][x][session.universe_size-1][z] {neighbours += 1;}
                            if y > 0 && let LifeDataContainer::Alive(_) = last_gen[3][x][y-1][z] {neighbours += 1;}
                            //touches 1 in z-1
                            if z == 0 && let LifeDataContainer::Alive(_) = last_gen[1][x][y][session.universe_size-1] {neighbours += 1;}
                            if z > 0 && let LifeDataContainer::Alive(_) = last_gen[1][x][y][z-1] {neighbours += 1;}
                            //touches 5 in z-1
                            if z == 0 && let LifeDataContainer::Alive(_) = last_gen[5][x][y][session.universe_size-1] {neighbours += 1;}
                            if z > 0 && let LifeDataContainer::Alive(_) = last_gen[5][x][y][z-1] {neighbours += 1;}
                            // touches 1 y-1 z-1
                            if z > 0 {
                                if y > 0 {
                                    if let LifeDataContainer::Alive(_) = last_gen[1][x][y-1][z-1] {neighbours += 1;}
                                } else {
                                    if let LifeDataContainer::Alive(_) = last_gen[1][x][session.universe_size-1][z-1] {neighbours += 1;}
                                }
                            } else if y > 0 {
                                if let LifeDataContainer::Alive(_) = last_gen[1][x][y-1][session.universe_size-1] {neighbours += 1;}
                            } else {
                                if let LifeDataContainer::Alive(_) = last_gen[1][x][session.universe_size-1][session.universe_size-1] {neighbours += 1;}
                            }
                            //touches 1 and 2 in x-1 z-1
                            if x > 0 {
                                if z > 0 {
                                    if let LifeDataContainer::Alive(_) = last_gen[1][x-1][y][z-1] {neighbours += 1;}
                                    if let LifeDataContainer::Alive(_) = last_gen[2][x-1][y][z-1] {neighbours += 1;}
                                } else {
                                    if let LifeDataContainer::Alive(_) = last_gen[1][x-1][y][session.universe_size-1] {neighbours += 1;}
                                    if let LifeDataContainer::Alive(_) = last_gen[2][x-1][y][session.universe_size-1] {neighbours += 1;}
                                }
                            } else if z > 0 {
                                if let LifeDataContainer::Alive(_) = last_gen[1][session.universe_size-1][y][z-1] {neighbours += 1;}
                                if let LifeDataContainer::Alive(_) = last_gen[2][session.universe_size-1][y][z-1] {neighbours += 1;}
                            } else {
                                if let LifeDataContainer::Alive(_) = last_gen[1][session.universe_size-1][y][session.universe_size-1] {neighbours += 1;}
                                if let LifeDataContainer::Alive(_) = last_gen[2][session.universe_size-1][y][session.universe_size-1] {neighbours += 1;}
                            }
                            // touches 1 and 4 in x+1 y-1
                            if session.universe_size > x+1 {
                                if y > 0 {
                                    if let LifeDataContainer::Alive(_) = last_gen[1][x+1][y-1][z] {neighbours += 1;}
                                    if let LifeDataContainer::Alive(_) = last_gen[4][x+1][y-1][z] {neighbours += 1;}
                                } else {
                                    if let LifeDataContainer::Alive(_) = last_gen[1][x+1][y][session.universe_size-1] {neighbours += 1;}
                                    if let LifeDataContainer::Alive(_) = last_gen[4][x+1][y][session.universe_size-1] {neighbours += 1;}
                                }
                            } else if y > 0 {
                                if let LifeDataContainer::Alive(_) = last_gen[1][0][y-1][z] {neighbours += 1;}
                                if let LifeDataContainer::Alive(_) = last_gen[4][0][y-1][z] {neighbours += 1;}
                            } else {
                                if let LifeDataContainer::Alive(_) = last_gen[1][0][session.universe_size-1][z] {neighbours += 1;}
                                if let LifeDataContainer::Alive(_) = last_gen[4][0][session.universe_size-1][z] {neighbours += 1;}
                            }
                        } else if n == 1 {// red touches light grey and light blue in same xyz and the dark grey in the y above and dark blue in z-1
                            //CHECK 5 NEIGHBOURS IN SAME CUBE
                            if let LifeDataContainer::Alive(_) = last_gen[0][x][y][z] {neighbours += 1}
                            if let LifeDataContainer::Alive(_) = last_gen[2][x][y][z] {neighbours += 1}
                            if let LifeDataContainer::Alive(_) = last_gen[3][x][y][z] {neighbours += 1}
                            if let LifeDataContainer::Alive(_) = last_gen[4][x][y][z] {neighbours += 1}
                            if let LifeDataContainer::Alive(_) = last_gen[5][x][y][z] {neighbours += 1}
                            // 2 FACE CHECKS
                            //the y>0 and z>0 checks if we are the edge of the univ
                            if session.universe_size > y+1 && let LifeDataContainer::Alive(_) = last_gen[5][x][y+1][z  ] {neighbours += 1;} // touches dark grey above
                            if session.universe_size == y+1 && let LifeDataContainer::Alive(_) = last_gen[5][x][0][z  ] {neighbours += 1;} // touches dark grey above (on the other side of the universe)
                            if session.universe_size > z+1 && let LifeDataContainer::Alive(_) = last_gen[3][x][y][z+1] {neighbours += 1;}
                            if session.universe_size == z+1 && let LifeDataContainer::Alive(_) = last_gen[3][x][y][0  ] {neighbours += 1;}
                            // 11 EDGE CHECKS
                            //touches 2 in x-1
                            if x == 0 && let LifeDataContainer::Alive(_) = last_gen[2][session.universe_size-1][y][z] {neighbours += 1;}
                            if x > 0 && let LifeDataContainer::Alive(_) = last_gen[2][x-1][y][z] {neighbours += 1;}
                            //touches 4 in x+1
                            if session.universe_size >  x+1 && let LifeDataContainer::Alive(_) = last_gen[4][x+1][y][z] {neighbours += 1;}
                            if session.universe_size == x+1 && let LifeDataContainer::Alive(_) = last_gen[4][0  ][y][z] {neighbours += 1;}
                            //touches 0 in y+1
                            if session.universe_size >  y+1 && let LifeDataContainer::Alive(_) = last_gen[0][x][y+1][z] {neighbours += 1;}
                            if session.universe_size == y+1 && let LifeDataContainer::Alive(_) = last_gen[0][x][0  ][z] {neighbours += 1;}
                            //touches 4 in y+1
                            if session.universe_size >  y+1 && let LifeDataContainer::Alive(_) = last_gen[4][x][y+1][z] {neighbours += 1;}
                            if session.universe_size == y+1 && let LifeDataContainer::Alive(_) = last_gen[4][x][0  ][z] {neighbours += 1;}
                            //touches 2 in z+1
                            if session.universe_size >  z+1 && let LifeDataContainer::Alive(_) = last_gen[2][x][y][z+1] {neighbours += 1;}
                            if session.universe_size == z+1 && let LifeDataContainer::Alive(_) = last_gen[2][x][y][0  ] {neighbours += 1;}
                            //touches 0 in z+1
                            if session.universe_size >  z+1 && let LifeDataContainer::Alive(_) = last_gen[0][x][y][z+1] {neighbours += 1;}
                            if session.universe_size == z+1 && let LifeDataContainer::Alive(_) = last_gen[0][x][y][0  ] {neighbours += 1;}
                            //touches 0 in y+1 z+1
                            if session.universe_size > z+1 {
                                if session.universe_size > y+1 {
                                    if let LifeDataContainer::Alive(_) = last_gen[0][x][y+1][z+1] {neighbours += 1;}
                                } else if let LifeDataContainer::Alive(_) = last_gen[0][x][0][z+1] {
                                    neighbours += 1;
                                }
                            } else if session.universe_size > y+1 {
                                if let LifeDataContainer::Alive(_) = last_gen[0][x][y+1][0] {neighbours += 1;}
                            } else if let LifeDataContainer::Alive(_) = last_gen[0][x][0][0] {
                                neighbours += 1;
                            }
                            //touches 0 and 5 in x+1 z+1
                            if session.universe_size > x+1 {
                                if session.universe_size > z+1 {
                                    if let LifeDataContainer::Alive(_) = last_gen[0][x+1][y][z+1] {neighbours += 1;}
                                    if let LifeDataContainer::Alive(_) = last_gen[5][x+1][y][z+1] {neighbours += 1;}
                                } else {
                                    if let LifeDataContainer::Alive(_) = last_gen[0][x+1][y][0] {neighbours += 1;}
                                    if let LifeDataContainer::Alive(_) = last_gen[5][x+1][y][0] {neighbours += 1;}
                                }
                            } else if session.universe_size > z+1 {
                                if let LifeDataContainer::Alive(_) = last_gen[0][0][y][z+1] {neighbours += 1;}
                                if let LifeDataContainer::Alive(_) = last_gen[5][0][y][z+1] {neighbours += 1;}
                            } else {
                                if let LifeDataContainer::Alive(_) = last_gen[0][0][y][0] {neighbours += 1;}
                                if let LifeDataContainer::Alive(_) = last_gen[5][0][y][0] {neighbours += 1;}
                            }
                            //touches 0 and 3 in x-1 y+1
                            if session.universe_size > y+1 {
                                if x > 0 {
                                    if let LifeDataContainer::Alive(_) = last_gen[0][x-1][y+1][z] {neighbours += 1;}
                                    if let LifeDataContainer::Alive(_) = last_gen[3][x-1][y+1][z] {neighbours += 1;}
                                } else {
                                    if let LifeDataContainer::Alive(_) = last_gen[0][session.universe_size-1][y+1][z] {neighbours += 1;}
                                    if let LifeDataContainer::Alive(_) = last_gen[3][session.universe_size-1][y+1][z] {neighbours += 1;}
                                }
                            } else if x > 0 {
                                if let LifeDataContainer::Alive(_) = last_gen[0][x-1][0][z] {neighbours += 1;}
                                if let LifeDataContainer::Alive(_) = last_gen[3][x-1][0][z] {neighbours += 1;}
                            } else {
                                if let LifeDataContainer::Alive(_) = last_gen[0][session.universe_size-1][0][z] {neighbours += 1;}
                                if let LifeDataContainer::Alive(_) = last_gen[3][session.universe_size-1][0][z] {neighbours += 1;}
                            }
                        } else if n == 2 {// light blue touches red and dark blue in the same xyz and white in the y above
                            // CHECK 5 NEIGHBOURS IN SAME CUBE
                            if let LifeDataContainer::Alive(_) = last_gen[0][x][y][z] {neighbours += 1}
                            if let LifeDataContainer::Alive(_) = last_gen[1][x][y][z] {neighbours += 1}
                            if let LifeDataContainer::Alive(_) = last_gen[3][x][y][z] {neighbours += 1}
                            if let LifeDataContainer::Alive(_) = last_gen[4][x][y][z] {neighbours += 1}
                            if let LifeDataContainer::Alive(_) = last_gen[5][x][y][z] {neighbours += 1}
                            // 2 FACE CHECKS
                            //the y >0 checks if we are the edge of the univ
                            if session.universe_size > y+1 && let LifeDataContainer::Alive(_) = last_gen[0][x][y+1][z  ] {neighbours += 1;} // touches white above
                            if session.universe_size == y+1 && let LifeDataContainer::Alive(_) = last_gen[0][x][0][z  ] {neighbours += 1;} // touches white above (on the other side of the universe)
                            if session.universe_size > x+1 && let LifeDataContainer::Alive(_) = last_gen[4][x+1][y  ][z] {neighbours += 1;} // touches in x+1
                            if session.universe_size == x+1 && let LifeDataContainer::Alive(_) = last_gen[4][0][y  ][z] {neighbours += 1;} // touches in x+1 (on the other side of the universe)
                            // 11 EDGE CHECKS
                            //touches 1 in x+1
                            if session.universe_size >  x+1 && let LifeDataContainer::Alive(_) = last_gen[1][x+1][y][z] {neighbours += 1;}
                            if session.universe_size == x+1 && let LifeDataContainer::Alive(_) = last_gen[1][0  ][y][z] {neighbours += 1;}
                            //touches 5 in x+1
                            if session.universe_size >  x+1 && let LifeDataContainer::Alive(_) = last_gen[5][x+1][y][z] {neighbours += 1;}
                            if session.universe_size == x+1 && let LifeDataContainer::Alive(_) = last_gen[5][0  ][y][z] {neighbours += 1;}
                            //touches 3 in y+1
                            if session.universe_size >  y+1 && let LifeDataContainer::Alive(_) = last_gen[3][x][y+1][z] {neighbours += 1;}
                            if session.universe_size == y+1 && let LifeDataContainer::Alive(_) = last_gen[3][x][0  ][z] {neighbours += 1;}
                            //touches 5 in y+1
                            if session.universe_size >  y+1 && let LifeDataContainer::Alive(_) = last_gen[5][x][y+1][z] {neighbours += 1;}
                            if session.universe_size == y+1 && let LifeDataContainer::Alive(_) = last_gen[5][x][0  ][z] {neighbours += 1;}
                            //touches 3 in z+1
                            if session.universe_size >  z+1 && let LifeDataContainer::Alive(_) = last_gen[3][x][y][z+1] {neighbours += 1;}
                            if session.universe_size == z+1 && let LifeDataContainer::Alive(_) = last_gen[3][x][y][0  ] {neighbours += 1;}
                            //touches 1 in z-1
                            if z == 0 && let LifeDataContainer::Alive(_) = last_gen[1][x][y][session.universe_size-1] {neighbours += 1;}
                            if z > 0  && let LifeDataContainer::Alive(_) = last_gen[1][x][y][z-1] {neighbours += 1;}
                            //touches 4 and 5 y+1 z-1
                            if session.universe_size > y+1 {
                                if z > 0 {
                                    if let LifeDataContainer::Alive(_) = last_gen[4][x][y+1][z-1] {neighbours += 1;}
                                    if let LifeDataContainer::Alive(_) = last_gen[5][x][y+1][z-1] {neighbours += 1;}
                                } else if let LifeDataContainer::Alive(_) = last_gen[3][session.universe_size-1][y+1][z] {
                                    neighbours += 1;
                                }
                            } else if z > 0 {
                                if let LifeDataContainer::Alive(_) = last_gen[4][x][0][z-1] {neighbours += 1;}
                                if let LifeDataContainer::Alive(_) = last_gen[5][x][0][z-1] {neighbours += 1;}
                            } else {
                                if let LifeDataContainer::Alive(_) = last_gen[4][x][0][session.universe_size-1] {neighbours += 1;}
                                if let LifeDataContainer::Alive(_) = last_gen[5][x][0][session.universe_size-1] {neighbours += 1;}
                            }
                            //touches 0 and 5 in x+1 z+1
                            if session.universe_size > x+1 {
                                if session.universe_size > z+1 {
                                    if let LifeDataContainer::Alive(_) = last_gen[0][x+1][y][z+1] {neighbours += 1;}
                                    if let LifeDataContainer::Alive(_) = last_gen[5][x+1][y][z+1] {neighbours += 1;}
                                } else {
                                    if let LifeDataContainer::Alive(_) = last_gen[0][x+1][y][0] {neighbours += 1;}
                                    if let LifeDataContainer::Alive(_) = last_gen[5][x+1][y][0] {neighbours += 1;}
                                }
                            } else if session.universe_size > z+1 {
                                if let LifeDataContainer::Alive(_) = last_gen[0][0][y][z+1] {neighbours += 1;}
                                if let LifeDataContainer::Alive(_) = last_gen[5][0][y][z+1] {neighbours += 1;}
                            } else {
                                if let LifeDataContainer::Alive(_) = last_gen[0][0][y][0] {neighbours += 1;}
                                if let LifeDataContainer::Alive(_) = last_gen[5][0][y][0] {neighbours += 1;}
                            }
                            //touches 5 x+1 y+1
                            if session.universe_size > x+1 {
                                if session.universe_size > y+1 {
                                    if let LifeDataContainer::Alive(_) = last_gen[5][x+1][y+1][z] {neighbours += 1;}
                                } else if let LifeDataContainer::Alive(_) = last_gen[5][x+1][0][z] {
                                    neighbours += 1;
                                }
                            } else if session.universe_size > y+1 {
                                if let LifeDataContainer::Alive(_) = last_gen[5][0][y+1][z] {neighbours += 1;}
                            } else if let LifeDataContainer::Alive(_) = last_gen[5][0][0][z] {
                                neighbours += 1;
                            }
                        } else if n == 3 {// dark blue touches light blue and white in same xyz and red and dark grey either side (need to check if thats x or z)
                            // CHECK 5 NEIGHBOURS IN SAME CUBE
                            if let LifeDataContainer::Alive(_) = last_gen[0][x][y][z] {neighbours += 1}
                            if let LifeDataContainer::Alive(_) = last_gen[1][x][y][z] {neighbours += 1}
                            if let LifeDataContainer::Alive(_) = last_gen[2][x][y][z] {neighbours += 1}
                            if let LifeDataContainer::Alive(_) = last_gen[4][x][y][z] {neighbours += 1}
                            if let LifeDataContainer::Alive(_) = last_gen[5][x][y][z] {neighbours += 1}
                            // 2 FACE CHECKS
                            //the y >0 checks if we are the edge of the univ
                            if session.universe_size > x+1 && let LifeDataContainer::Alive(_) = last_gen[5][x+1][y  ][z] {neighbours += 1;} // touches dark grey
                            if session.universe_size == x+1 && let LifeDataContainer::Alive(_) = last_gen[5][0][y  ][z] {neighbours += 1;} // touches dark grey (on the other side of the universe)
                            if z > 0 && let LifeDataContainer::Alive(_) = last_gen[1][x][y  ][z-1] {neighbours += 1;} // touches dark blue in z-1
                            if z == 0 && let LifeDataContainer::Alive(_) = last_gen[1][x][y  ][session.universe_size-1] {neighbours += 1;} // touches dark blue in z-1 (on the other side of the universe)
                            // 11 EDGE CHECKS
                            //touches 0 in x+1
                            if session.universe_size >  x+1 && let LifeDataContainer::Alive(_) = last_gen[0][x+1][y][z] {neighbours += 1;}
                            if session.universe_size == x+1 && let LifeDataContainer::Alive(_) = last_gen[0][0  ][y][z] {neighbours += 1;}
                            //touches 4 in x+1
                            if session.universe_size >  x+1 && let LifeDataContainer::Alive(_) = last_gen[4][x+1][y][z] {neighbours += 1;}
                            if session.universe_size == x+1 && let LifeDataContainer::Alive(_) = last_gen[4][0  ][y][z] {neighbours += 1;}
                            //touches 0 in y+1
                            if session.universe_size >  y+1 && let LifeDataContainer::Alive(_) = last_gen[0][x][y+1][z] {neighbours += 1;}
                            if session.universe_size == y+1 && let LifeDataContainer::Alive(_) = last_gen[0][x][0  ][z] {neighbours += 1;}
                            //touches 2 in y-1
                            if y == 0 && let LifeDataContainer::Alive(_) = last_gen[2][x][session.universe_size-1][z] {neighbours += 1;}
                            if y > 0  && let LifeDataContainer::Alive(_) = last_gen[2][x][y-1][z] {neighbours += 1;}
                            //touches 2 in z-1
                            if z == 0 && let LifeDataContainer::Alive(_) = last_gen[2][x][y][session.universe_size-1] {neighbours += 1;}
                            if z > 0  && let LifeDataContainer::Alive(_) = last_gen[2][x][y][z-1] {neighbours += 1;}
                            //touches 4 in z-1
                            if z == 0 && let LifeDataContainer::Alive(_) = last_gen[4][x][y][session.universe_size-1] {neighbours += 1;}
                            if z > 0  && let LifeDataContainer::Alive(_) = last_gen[4][x][y][z-1] {neighbours += 1;}
                            // touches 1 and 4 in x+1 y-1
                            if session.universe_size > x+1 {
                                if y > 0 {
                                    if let LifeDataContainer::Alive(_) = last_gen[1][x+1][y-1][z] {neighbours += 1;}
                                    if let LifeDataContainer::Alive(_) = last_gen[4][x+1][y-1][z] {neighbours += 1;}
                                } else {
                                    if let LifeDataContainer::Alive(_) = last_gen[1][x+1][y][session.universe_size-1] {neighbours += 1;}
                                    if let LifeDataContainer::Alive(_) = last_gen[4][x+1][y][session.universe_size-1] {neighbours += 1;}
                                }
                            } else if y > 0 {
                                if let LifeDataContainer::Alive(_) = last_gen[1][0][y-1][z] {neighbours += 1;}
                                if let LifeDataContainer::Alive(_) = last_gen[4][0][y-1][z] {neighbours += 1;}
                            } else {
                                if let LifeDataContainer::Alive(_) = last_gen[1][0][session.universe_size-1][z] {neighbours += 1;}
                                if let LifeDataContainer::Alive(_) = last_gen[4][0][session.universe_size-1][z] {neighbours += 1;}
                            }
                            //touches 4 in x+1 z-1
                            if session.universe_size > x+1 {
                                if z > 0 {
                                    if let LifeDataContainer::Alive(_) = last_gen[4][x+1][y][z-1] {neighbours += 1;}
                                } else if let LifeDataContainer::Alive(_) = last_gen[4][x+1][y][session.universe_size-1] {
                                    neighbours += 1;
                                }
                            } else if z > 0 {
                                if let LifeDataContainer::Alive(_) = last_gen[4][0][y][z-1] {neighbours += 1;}
                            } else if let LifeDataContainer::Alive(_) = last_gen[4][0][y][session.universe_size-1] {neighbours += 1;}
                            //touches 4 and 5 y+1 z-1
                            if session.universe_size > y+1 {
                                if z > 0 {
                                    if let LifeDataContainer::Alive(_) = last_gen[4][x][y+1][z-1] {neighbours += 1;}
                                    if let LifeDataContainer::Alive(_) = last_gen[5][x][y+1][z-1] {neighbours += 1;}
                                } else {
                                    if let LifeDataContainer::Alive(_) = last_gen[4][x][y+1][session.universe_size-1] {neighbours += 1;}
                                    if let LifeDataContainer::Alive(_) = last_gen[5][x][y+1][session.universe_size-1] {neighbours += 1;}
                                }
                            } else if z > 0 {
                                if let LifeDataContainer::Alive(_) = last_gen[4][x][0][z-1] {neighbours += 1;}
                                if let LifeDataContainer::Alive(_) = last_gen[5][x][0][z-1] {neighbours += 1;}
                            } else {
                                if let LifeDataContainer::Alive(_) = last_gen[4][x][0][session.universe_size-1] {neighbours += 1;}
                                if let LifeDataContainer::Alive(_) = last_gen[5][x][0][session.universe_size-1] {neighbours += 1;}
                            }
                        } else if n == 4 {// light grey touches dark grey and red in the same xyz and light blue and white either side (need to check if thats x or z)
                            // CHECK 5 NEIGHBOURS IN SAME CUBE
                            if let LifeDataContainer::Alive(_) = last_gen[0][x][y][z] {neighbours += 1}
                            if let LifeDataContainer::Alive(_) = last_gen[1][x][y][z] {neighbours += 1}
                            if let LifeDataContainer::Alive(_) = last_gen[2][x][y][z] {neighbours += 1}
                            if let LifeDataContainer::Alive(_) = last_gen[3][x][y][z] {neighbours += 1}
                            if let LifeDataContainer::Alive(_) = last_gen[5][x][y][z] {neighbours += 1}
                            // 2 FACE CHECKS
                            //the y >0 checks if we are the edge of the univ
                            if x > 0 && let LifeDataContainer::Alive(_) = last_gen[3][x-1][y  ][z  ] {neighbours += 1;} // touches dark blue
                            if x == 0 && let LifeDataContainer::Alive(_) = last_gen[3][session.universe_size-1][y  ][z  ] {neighbours += 1;} // touches dark blue (on the other side of the universe)
                            if session.universe_size > z+1 && let LifeDataContainer::Alive(_) = last_gen[0][x  ][y  ][z+1] {neighbours += 1;} // touches white
                            if session.universe_size == z+1 && let LifeDataContainer::Alive(_) = last_gen[0][x  ][y  ][0] {neighbours += 1;} // touches white (on the other side of the universe)
                            // 11 EDGE CHECKS
                            // touches 3 in z+1
                            if session.universe_size >  z+1 && let LifeDataContainer::Alive(_) = last_gen[3][x][y][z+1] {neighbours += 1;}
                            if session.universe_size == z+1 && let LifeDataContainer::Alive(_) = last_gen[3][x][y][0  ] {neighbours += 1;}
                            // touches 5 in z+1
                            if session.universe_size >  z+1 && let LifeDataContainer::Alive(_) = last_gen[5][x][y][z+1] {neighbours += 1;}
                            if session.universe_size == z+1 && let LifeDataContainer::Alive(_) = last_gen[5][x][y][0  ] {neighbours += 1;}
                            // touches 1 in x-1
                            if x == 0 && let LifeDataContainer::Alive(_) = last_gen[1][session.universe_size-1][y][z] {neighbours += 1;}
                            if x > 0 && let LifeDataContainer::Alive(_) = last_gen[1][x-1][y][z] {neighbours += 1;}
                            // touches 3 in x-1
                            if x == 0 && let LifeDataContainer::Alive(_) = last_gen[3][session.universe_size-1][y][z] {neighbours += 1;}
                            if x > 0 && let LifeDataContainer::Alive(_) = last_gen[3][x-1][y][z] {neighbours += 1;}
                            // touches 5 in y+1
                            if session.universe_size >  y+1 && let LifeDataContainer::Alive(_) = last_gen[5][x][y+1][z] {neighbours += 1;}
                            if session.universe_size == y+1 && let LifeDataContainer::Alive(_) = last_gen[5][x][0  ][z] {neighbours += 1;}
                            // touches 1 in y-1
                            if y == 0 && let LifeDataContainer::Alive(_) = last_gen[1][x][session.universe_size-1][z] {neighbours += 1;}
                            if y > 0  && let LifeDataContainer::Alive(_) = last_gen[1][x][y-1][z] {neighbours += 1;}
                            // touches 3 in x-1 z+1
                            if session.universe_size > z+1 {
                                if x > 0 {
                                    if let LifeDataContainer::Alive(_) = last_gen[3][x-1][y][z+1] {neighbours += 1;}
                                } else {
                                    if let LifeDataContainer::Alive(_) = last_gen[3][session.universe_size-1][y][z+1] {neighbours += 1;}
                                }
                            } else if x > 0 {
                                if let LifeDataContainer::Alive(_) = last_gen[3][x-1][0][0] {neighbours += 1;}
                            } else {
                                if let LifeDataContainer::Alive(_) = last_gen[3][session.universe_size-1][y][0] {neighbours += 1;}
                            }
                            // touches 0 and 3 in x-1 y+1
                            if session.universe_size > y+1 {
                                if x > 0 {
                                    if let LifeDataContainer::Alive(_) = last_gen[0][x-1][y+1][z] {neighbours += 1;}
                                    if let LifeDataContainer::Alive(_) = last_gen[3][x-1][y+1][z] {neighbours += 1;}
                                } else {
                                    if let LifeDataContainer::Alive(_) = last_gen[0][session.universe_size-1][y+1][z] {neighbours += 1;}
                                    if let LifeDataContainer::Alive(_) = last_gen[3][session.universe_size-1][y+1][z] {neighbours += 1;}
                                }
                            } else if x > 0 {
                                if let LifeDataContainer::Alive(_) = last_gen[0][x-1][0][z] {neighbours += 1;}
                                if let LifeDataContainer::Alive(_) = last_gen[3][x-1][0][z] {neighbours += 1;}
                            } else {
                                if let LifeDataContainer::Alive(_) = last_gen[0][session.universe_size-1][0][z] {neighbours += 1;}
                                if let LifeDataContainer::Alive(_) = last_gen[3][session.universe_size-1][0][z] {neighbours += 1;}
                            }
                            // touches 2 and 3 y-1 z+1
                            if session.universe_size > z+1 {
                                if y > 0 {
                                    if let LifeDataContainer::Alive(_) = last_gen[2][x][y-1][z+1] {neighbours += 1;}
                                    if let LifeDataContainer::Alive(_) = last_gen[3][x][y-1][z+1] {neighbours += 1;}
                                } else {
                                    if let LifeDataContainer::Alive(_) = last_gen[2][x][session.universe_size-1][z+1] {neighbours += 1;}
                                    if let LifeDataContainer::Alive(_) = last_gen[3][x][session.universe_size-1][z+1] {neighbours += 1;}
                                }
                            } else if y > 0 {
                                if let LifeDataContainer::Alive(_) = last_gen[2][x][y-1][0] {neighbours += 1;}
                                if let LifeDataContainer::Alive(_) = last_gen[3][x][y-1][0] {neighbours += 1;}
                            } else {
                                if let LifeDataContainer::Alive(_) = last_gen[2][x][session.universe_size-1][0] {neighbours += 1;}
                                if let LifeDataContainer::Alive(_) = last_gen[3][x][session.universe_size-1][0] {neighbours += 1;}
                            }
                        } else if n == 5 {// dark grey touches light grey and white in the same xyz and red in the y below and dark blue in x+1
                            // CHECK 5 NEIGHBOURS IN SAME CUBE
                            if let LifeDataContainer::Alive(_) = last_gen[0][x][y][z] {neighbours += 1}
                            if let LifeDataContainer::Alive(_) = last_gen[1][x][y][z] {neighbours += 1}
                            if let LifeDataContainer::Alive(_) = last_gen[2][x][y][z] {neighbours += 1}
                            if let LifeDataContainer::Alive(_) = last_gen[3][x][y][z] {neighbours += 1}
                            if let LifeDataContainer::Alive(_) = last_gen[4][x][y][z] {neighbours += 1}
                            // 2 FACE CHECKS
                            //the y >0 checks if we are the edge of the univ
                            if y > 0 && let LifeDataContainer::Alive(_) = last_gen[1][x  ][y-1][z] {neighbours += 1;} // touches red below
                            if y == 0 && let LifeDataContainer::Alive(_) = last_gen[1][x  ][session.universe_size-1][z] {neighbours += 1;} // touches red below (on the other side of the universe)
                            if x > 0 && let LifeDataContainer::Alive(_) = last_gen[3][x-1][y  ][z] {neighbours += 1;} // touches dark blue in x-1
                            if x == 0 && let LifeDataContainer::Alive(_) = last_gen[3][session.universe_size-1][y  ][z] {neighbours += 1;} // touches dark blue in x-1 (on the other side of the universe)
                            // 11 EDGE CHECKS
                            // touches 2 in x-1
                            if x == 0 && let LifeDataContainer::Alive(_) = last_gen[2][session.universe_size-1][y][z] {neighbours += 1;}
                            if x > 0  && let LifeDataContainer::Alive(_) = last_gen[2][x-1][y][z] {neighbours += 1;}
                            // touches 5 in x-1
                            if x == 0 && let LifeDataContainer::Alive(_) = last_gen[5][session.universe_size-1][y][z] {neighbours += 1;}
                            if x > 0  && let LifeDataContainer::Alive(_) = last_gen[5][x-1][y][z] {neighbours += 1;}
                            // touches 2 in y-1
                            if y == 0 && let LifeDataContainer::Alive(_) = last_gen[2][x][session.universe_size-1][z] {neighbours += 1;}
                            if y > 0  && let LifeDataContainer::Alive(_) = last_gen[2][x][y-1][z] {neighbours += 1;}
                            // touches 4 in y-1
                            if y == 0 && let LifeDataContainer::Alive(_) = last_gen[4][x][session.universe_size-1][z] {neighbours += 1;}
                            if y > 0  && let LifeDataContainer::Alive(_) = last_gen[4][x][y-1][z] {neighbours += 1;}
                            // touches 0 in z+1
                            if session.universe_size >  z+1 && let LifeDataContainer::Alive(_) = last_gen[0][x][y][z+1] {neighbours += 1;}
                            if session.universe_size == z+1 && let LifeDataContainer::Alive(_) = last_gen[0][x][y][0  ] {neighbours += 1;}
                            // touches 4 in z-1
                            if z == 0 && let LifeDataContainer::Alive(_) = last_gen[4][x][y][session.universe_size-1] {neighbours += 1;}
                            if z > 0  && let LifeDataContainer::Alive(_) = last_gen[4][x][y][z-1] {neighbours += 1;}
                            // touches 2 in x-1 y-1
                            if y > 0 {
                                if x > 0 {
                                    if let LifeDataContainer::Alive(_) = last_gen[2][x-1][y-1][z] {neighbours += 1;}
                                } else if let LifeDataContainer::Alive(_) = last_gen[2][session.universe_size-1][y-1][z] {neighbours += 1;}
                            } else if x > 0 {
                                if let LifeDataContainer::Alive(_) = last_gen[2][x-1][session.universe_size-1][z] {neighbours += 1;}
                            } else if let LifeDataContainer::Alive(_) = last_gen[2][session.universe_size-1][session.universe_size-1][z] {neighbours += 1;}
                            // touches 1 and 2 in x-1 z-1
                            if z > 0 {
                                if x > 0 {
                                    if let LifeDataContainer::Alive(_) = last_gen[1][x-1][y][z-1] {neighbours += 1;}
                                    if let LifeDataContainer::Alive(_) = last_gen[2][x-1][y][z-1] {neighbours += 1;}
                                } else {
                                    if let LifeDataContainer::Alive(_) = last_gen[1][session.universe_size-1][y][z-1] {neighbours += 1;}
                                    if let LifeDataContainer::Alive(_) = last_gen[2][session.universe_size-1][y][z-1] {neighbours += 1;}
                                }
                            } else if x > 0 {
                                if let LifeDataContainer::Alive(_) = last_gen[1][x-1][y][session.universe_size-1] {neighbours += 1;}
                                if let LifeDataContainer::Alive(_) = last_gen[2][x-1][y][session.universe_size-1] {neighbours += 1;}
                            } else {
                                if let LifeDataContainer::Alive(_) = last_gen[1][session.universe_size-1][y][session.universe_size-1] {neighbours += 1;}
                                if let LifeDataContainer::Alive(_) = last_gen[2][session.universe_size-1][y][session.universe_size-1] {neighbours += 1;}
                            }
                            // touches 2 and 3 y-1 z+1
                            if session.universe_size > z+1 {
                                if y > 0 {
                                    if let LifeDataContainer::Alive(_) = last_gen[2][x][y-1][z+1] {neighbours += 1;}
                                    if let LifeDataContainer::Alive(_) = last_gen[3][x][y-1][z+1] {neighbours += 1;}
                                } else {
                                    if let LifeDataContainer::Alive(_) = last_gen[2][x][session.universe_size-1][z+1] {neighbours += 1;}
                                    if let LifeDataContainer::Alive(_) = last_gen[3][x][session.universe_size-1][z+1] {neighbours += 1;}
                                }
                            } else if y > 0 {
                                if let LifeDataContainer::Alive(_) = last_gen[2][x][y-1][0] {neighbours += 1;}
                                if let LifeDataContainer::Alive(_) = last_gen[3][x][y-1][0] {neighbours += 1;}
                            } else {
                                if let LifeDataContainer::Alive(_) = last_gen[2][x][session.universe_size-1][0] {neighbours += 1;}
                                if let LifeDataContainer::Alive(_) = last_gen[3][x][session.universe_size-1][0] {neighbours += 1;}
                            }
                        } else {
                            println!("Error: n was not in 0-5");
                        }
                        match entity_life {
                            LifeDataContainer::Alive(ent) => {//if alive in last gen
                                if neighbours > 3 || neighbours == 1 || neighbours == 0 {
                                    commands.entity(ent.to_owned()).despawn();
                                    next_gen[n][x][y][z] = LifeDataContainer::Dead(true);

                                    session.counter -= 1;
                                } else {
                                    //continue to be alive
                                    next_gen[n][x][y][z] = last_gen[n][x][y][z];
                                }
                            },
                            LifeDataContainer::Dead(_) => {// if dead in last gen
                                //if neighbours = 3 then become alive
                                if neighbours == 3 {
                                    let transform_new_life: bevy::prelude::Transform = create_life_xyz(n, x, y, z);
                                
                                    // make the life form exist!
                                    next_gen[n][x][y][z] = LifeDataContainer::Alive(commands.spawn_bundle(PbrBundle {
                                        mesh: session.life_form_meshes[n%2].clone(),
                                        material: session.life_form_materials[n].clone(),
                                        transform: transform_new_life,
                                        ..Default::default()
                                    }).insert(Life).id());
                                    
                                    //increment life counter
                                    session.counter += 1;
                                }
                            },
                        }
                    }
                }
            }
        }

        session.life = next_gen;
        session.generation += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use bevy_obj::*;// used import wavefront obj files
    use bevy::{
        asset::AssetPlugin,
        core::CorePlugin,
        core_pipeline::CorePipelinePlugin,
        pbr::PbrPlugin,
        render::RenderPlugin,
        window::WindowPlugin,
    };

    fn initialise_test_universe(save_filename: &str) -> bevy::prelude::App {
        // Setup app
        let mut app = App::new();
            
        app.add_plugins(MinimalPlugins);
        app.add_plugin(CorePlugin::default());

        app.add_plugin(AssetPlugin::default());
        app.add_plugin(WindowPlugin::default());
        app.add_plugin(RenderPlugin::default());
        app.add_plugin(CorePipelinePlugin::default());
        app.add_plugin(PbrPlugin::default());
        
        app.add_plugin(ObjPlugin);
        app.add_state(crate::AppState::LoadGame);

        //asset server for meshes
        let asset_server = app.world.get_resource::<AssetServer>().expect("expected asset server");

        //load meshes
        let tetrahedron_mirrored = asset_server.load("mesh/hill-tetrahedron-mirrored.obj");
        let tetrahedron = asset_server.load("mesh/hill-tetrahedron.obj");

        //materials
        let mut materials = app.world.get_resource_mut::<Assets<StandardMaterial>>().expect("expected standard materials");
        let material_handles = [
            materials.add(StandardMaterial {
                base_color: Color::rgb(0.0, 1.0, 0.0), // white -> green
                ..default()
            }).clone(),
            materials.add(StandardMaterial {
                base_color: Color::rgb(0.6, 0.2, 0.2), // red
                ..default()
            }).clone(),
            materials.add(StandardMaterial {
                base_color: Color::rgb(0.5, 0.5, 1.0), // light blue
                ..default()
            }).clone(),
            materials.add(StandardMaterial {
                base_color: Color::rgb(0.1, 0.1, 0.7), // dark blue
                ..default()
            }).clone(),
            materials.add(StandardMaterial {
                base_color: Color::rgb(1.0, 1.0, 0.0), // light grey -> yellow
                ..default()
            }).clone(),
            materials.add(StandardMaterial {
                base_color: Color::rgb(0.2, 0.2, 0.2), // dark grey
                ..default()
            }).clone(),
        ];

        //new session resource
        let session = SessionResource {
            life: dead_universe(),
            counter: 0,
            generation: 1,
            life_form_materials: material_handles,
            life_form_meshes: [
                tetrahedron_mirrored.clone(),
                tetrahedron.clone(),
            ],
            universe_size: 10,
        };

        //new load game resource so we can load our test universe
        let game_file_to_load = crate::systems::saves::GameFileToLoad::Some(save_filename.to_string());

        // Add session resource
        app.insert_resource(session);
        // add our test case save file to be loaded up
        app.insert_resource(game_file_to_load);

        // Add our systems
        app.add_system(run);
        app.add_system_set(
            SystemSet::on_enter(AppState::LoadGame)
            .with_system(crate::systems::saves::load)
            .before(run)
        );
        app
    }

    #[test]
    fn test_life_two_in_same_cube_dies() {
        /* TEST DESCRIPTION
           Start State: 2 tetras indexed ?,? in the same cube
           Expect: universe to die off
        */
        let mut app = initialise_test_universe("test_01");

        //state = LoadGame at this point as we are about to load a game save on the next tic
        assert_eq!(app.world.resource::<State<AppState>>().current(), &AppState::LoadGame);

        // Check we have the right number of life forms on generation 1
        assert_eq!(app.world.resource::<SessionResource>().generation, 1);
        assert_eq!(app.world.resource::<SessionResource>().counter, 0);

        // Run systems (run one tick of time)
        app.update();
        assert_eq!(app.world.resource::<State<AppState>>().current(), &AppState::InGame);

        // Check we have the right number of life forms on generation 2
        assert_eq!(app.world.resource::<SessionResource>().generation, 2);
        assert_eq!(app.world.resource::<SessionResource>().counter, 2);

        // Run systems (run one tick of time)
        app.update();
        assert_eq!(app.world.resource::<State<AppState>>().current(), &AppState::InGame);

        // Check we have the right number of life forms on generation 3
        assert_eq!(app.world.resource::<SessionResource>().generation, 3);
        assert_eq!(app.world.resource::<SessionResource>().counter, 0);
    }
    #[test]
    fn test_life_012_in_same_cube_breeds() {
        /* TEST DESCRIPTION
           Start State: 3 tetras indexed 0,1,2 in the same cube
           Expect: 3 to become a cube of 6t.
                   6 to die and create 12t.
        */

        let mut app = initialise_test_universe("test_02");

        //state = LoadGame at this point as we are about to load a game save on the next tic
        assert_eq!(app.world.resource::<State<AppState>>().current(), &AppState::LoadGame);

        // Check we have the right number of life forms on generation 1 before we load the save
        assert_eq!(app.world.resource::<SessionResource>().generation, 1);
        assert_eq!(app.world.resource::<SessionResource>().counter, 0);

        // Run systems (run one tick of time)
        app.update();
        // test save file has now loaded
        assert_eq!(app.world.resource::<State<AppState>>().current(), &AppState::InGame);

        // Check we have the right number of life forms on generation 2
        assert_eq!(app.world.resource::<SessionResource>().generation, 2);
        assert_eq!(app.world.resource::<SessionResource>().counter, 3);

        // Run systems (run one tick of time)
        app.update();
        assert_eq!(app.world.resource::<State<AppState>>().current(), &AppState::InGame);

        // Check we have the right number of life forms on generation 3
        assert_eq!(app.world.resource::<SessionResource>().generation, 3);
        assert_eq!(app.world.resource::<SessionResource>().counter, 6);
        // at this point we have one solid cube of 6 lifeforms

        // Run systems (run one tick of time)
        app.update();
        assert_eq!(app.world.resource::<State<AppState>>().current(), &AppState::InGame);

        // Check we have the right number of life forms on generation 4
        assert_eq!(app.world.resource::<SessionResource>().generation, 4);
        assert_eq!(app.world.resource::<SessionResource>().counter, 12);
        // at this point we have twelve lifeforms that exist from the faces of the starting cube
    }
}
