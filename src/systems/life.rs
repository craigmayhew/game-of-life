use bevy::{
    prelude::*, //default bevy
};

use crate::{AppState, SessionResource};

pub const LIFE_FORM_SIZE: f32 = 150.0;

pub fn create_life_xyz(n: &TetraIndex, x: usize, y: usize, z: usize) -> bevy::prelude::Transform {
    // position the life form in 3d space
    let mut transform_new_life: Transform;
    match n {
        TetraIndex::Two | TetraIndex::Three => {
            // position the life form in 3d space
            transform_new_life = Transform::from_xyz(
                (x as f32 - 1.0) * LIFE_FORM_SIZE,
                (y as f32 + 1.0) * LIFE_FORM_SIZE,
                (z as f32 - 1.0) * LIFE_FORM_SIZE,
            );
        }
        TetraIndex::Zero | TetraIndex::One | TetraIndex::Four | TetraIndex::Five => {
            transform_new_life = Transform::from_xyz(
                (x as f32) * LIFE_FORM_SIZE,
                (y as f32) * LIFE_FORM_SIZE,
                (z as f32) * LIFE_FORM_SIZE,
            );
        }
    }
    //TODO consider if n == 0 and n == 1 could/should actually be identical blocks
    //NOTES: We seem to be doing all of this in eigths of a turn i.e. 0.25 PI
    //       This suggests our shape starts out at an angle. Confirmed by viewing obj file.
    //BETTER TODO: Replace most of this code with 6 correctly rotated obj files
    match n {
        TetraIndex::Zero => {
            //white
            transform_new_life.rotate_x(3.0 * std::f32::consts::FRAC_PI_4);
            transform_new_life.rotate_y(std::f32::consts::FRAC_PI_2);
            transform_new_life.rotate_z(std::f32::consts::PI);
        }
        TetraIndex::One => {
            //red
            transform_new_life.rotate_x(-std::f32::consts::FRAC_PI_4);
            transform_new_life.rotate_y(0.0);
            transform_new_life.rotate_z(std::f32::consts::FRAC_PI_2);
        }
        TetraIndex::Two => {
            //light blue and dark blue
            transform_new_life.rotate_x(std::f32::consts::FRAC_PI_4);
            transform_new_life.rotate_y(-std::f32::consts::FRAC_PI_2);
            transform_new_life.rotate_z(std::f32::consts::PI);
        }
        TetraIndex::Three => {
            //light blue and dark blue
            transform_new_life.rotate_x(std::f32::consts::FRAC_PI_4);
            transform_new_life.rotate_y(0.0);
            transform_new_life.rotate_z(-std::f32::consts::FRAC_PI_2);
        }
        TetraIndex::Four | TetraIndex::Five => {
            //light grey and dark grey
            transform_new_life.rotate_x(std::f32::consts::FRAC_PI_4);
            transform_new_life.rotate_y(std::f32::consts::PI);
            transform_new_life.rotate_z(0.0);
        }
    }

    //set size of tetrahedrons and return
    transform_new_life.with_scale(Vec3::new(LIFE_FORM_SIZE, LIFE_FORM_SIZE, LIFE_FORM_SIZE))
}

#[derive(Component)]
pub struct Life;

// we use an enum to get around the fact that even entity id 0 is valid
#[derive(Clone, Copy)]
pub enum LifeDataContainer {
    Alive(Entity),
    Dead,
}

#[derive(Clone, Copy)]
pub enum TetraIndex {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
}
pub const TETRA_INDEXES: [TetraIndex; 6] = [
    TetraIndex::Zero,
    TetraIndex::One,
    TetraIndex::Two,
    TetraIndex::Three,
    TetraIndex::Four,
    TetraIndex::Five,
];

use std::ops::Index;
impl Index<TetraIndex> for TetraIndex {
    type Output = usize;

    fn index(&self, tetraindex: TetraIndex) -> &Self::Output {
        match tetraindex {
            TetraIndex::Zero => &0,
            TetraIndex::One => &1,
            TetraIndex::Two => &2,
            TetraIndex::Three => &3,
            TetraIndex::Four => &4,
            TetraIndex::Five => &5,
        }
    }
}
impl Index<TetraIndex> for Vec<Vec<Vec<LifeDataContainer>>> {
    type Output = usize;

    fn index(&self, tetraindex: TetraIndex) -> &Self::Output {
        match tetraindex {
            TetraIndex::Zero => &0,
            TetraIndex::One => &1,
            TetraIndex::Two => &2,
            TetraIndex::Three => &3,
            TetraIndex::Four => &4,
            TetraIndex::Five => &5,
        }
    }
}

pub enum Axis {
    XPos,
    XNeg,
    YPos,
    YNeg,
    ZPos,
    ZNeg,
    XPosYPos,
    XPosYNeg,
    XNegYPos,
    XNegYNeg,
    XPosZPos,
    XPosZNeg,
    XNegZPos,
    XNegZNeg,
    YPosZPos,
    YPosZNeg,
    YNegZPos,
    YNegZNeg,
}
pub struct NeighbourChecks {
    n: TetraIndex,
    axis: Axis,
}

fn checks(n: &TetraIndex) -> Vec<NeighbourChecks> {
    // Q: WHY ARE THERE NO TRIPLE AXIS CHECKS?
    // A: We only check faces and sides of tetras!
    //    Three axis checks are only a requirement of corner checks
    // NOTE: z goes down as you move forward from the start position
    match n {
        TetraIndex::Zero => {
            vec![
                // 2 FACE CHECKS
                NeighbourChecks {
                    n: TetraIndex::Two,
                    axis: Axis::YNeg,
                }, // touches 2 in y-1
                NeighbourChecks {
                    n: TetraIndex::Four,
                    axis: Axis::ZNeg,
                }, // touches 4 in z-1
                // 6 SINGLE AXIS EDGE CHECKS
                NeighbourChecks {
                    n: TetraIndex::Three,
                    axis: Axis::XNeg,
                }, // touches 3 in x-1
                NeighbourChecks {
                    n: TetraIndex::Five,
                    axis: Axis::XPos,
                }, // touches 5 in x+1
                NeighbourChecks {
                    n: TetraIndex::One,
                    axis: Axis::YNeg,
                }, // touches 1 in y-1
                NeighbourChecks {
                    n: TetraIndex::Three,
                    axis: Axis::YNeg,
                }, // touches 3 in y-1
                NeighbourChecks {
                    n: TetraIndex::One,
                    axis: Axis::ZNeg,
                }, // touches 1 in z-1
                NeighbourChecks {
                    n: TetraIndex::Five,
                    axis: Axis::ZNeg,
                }, // touches 5 in z-1
                // 5 DOUBLE AXIS EDGE CHECKS
                NeighbourChecks {
                    n: TetraIndex::One,
                    axis: Axis::YNegZNeg,
                }, // touches 1 in y-1 z-1
                NeighbourChecks {
                    n: TetraIndex::One,
                    axis: Axis::XNegZNeg,
                }, // touches 1 in x-1 z-1
                NeighbourChecks {
                    n: TetraIndex::Two,
                    axis: Axis::XNegZNeg,
                }, // touches 2 in x-1 z-1
                NeighbourChecks {
                    n: TetraIndex::One,
                    axis: Axis::XPosYNeg,
                }, // touches 1 in x+1 y-1
                NeighbourChecks {
                    n: TetraIndex::Four,
                    axis: Axis::XPosYNeg,
                }, // touches 4 in x+1 y-1
            ]
        }
        TetraIndex::One => {
            vec![
                // 2 FACE CHECKS
                NeighbourChecks {
                    n: TetraIndex::Five,
                    axis: Axis::YPos,
                }, // touches 5 in y+1
                NeighbourChecks {
                    n: TetraIndex::Three,
                    axis: Axis::ZPos,
                }, // touches 3 in z+1
                // 6 SINGLE AXIS EDGE CHECKS
                NeighbourChecks {
                    n: TetraIndex::Two,
                    axis: Axis::XNeg,
                }, // touches 2 in x-1
                NeighbourChecks {
                    n: TetraIndex::Four,
                    axis: Axis::XPos,
                }, // touches 4 in x+1
                NeighbourChecks {
                    n: TetraIndex::Zero,
                    axis: Axis::YPos,
                }, // touches 0 in y+1
                NeighbourChecks {
                    n: TetraIndex::Four,
                    axis: Axis::YPos,
                }, // touches 4 in y+1
                NeighbourChecks {
                    n: TetraIndex::Two,
                    axis: Axis::ZPos,
                }, // touches 2 in z+1
                NeighbourChecks {
                    n: TetraIndex::Zero,
                    axis: Axis::ZPos,
                }, // touches 0 in z+1
                // 5 DOUBLE AXIS EDGE CHECKS
                NeighbourChecks {
                    n: TetraIndex::Zero,
                    axis: Axis::YPosZPos,
                }, // touches 0 in y+1 z+1
                NeighbourChecks {
                    n: TetraIndex::Zero,
                    axis: Axis::XPosZPos,
                }, // touches 0 in x+1 z+1
                NeighbourChecks {
                    n: TetraIndex::Five,
                    axis: Axis::XPosZPos,
                }, // touches 5 in x+1 z+1
                NeighbourChecks {
                    n: TetraIndex::Zero,
                    axis: Axis::XNegYPos,
                }, // touches 0 in x-1 y+1
                NeighbourChecks {
                    n: TetraIndex::Three,
                    axis: Axis::XNegYPos,
                }, // touches 3 in x-1 y+1
            ]
        }
        TetraIndex::Two => {
            vec![
                // 2 FACE CHECKS
                NeighbourChecks {
                    n: TetraIndex::Four,
                    axis: Axis::XPos,
                }, // touches 4 in x+1
                NeighbourChecks {
                    n: TetraIndex::Zero,
                    axis: Axis::YPos,
                }, // touches 0 in y+1
                // 6 SINGLE AXIS EDGE CHECKS
                NeighbourChecks {
                    n: TetraIndex::One,
                    axis: Axis::XPos,
                }, // touches 1 in x+1
                NeighbourChecks {
                    n: TetraIndex::Five,
                    axis: Axis::XPos,
                }, // touches 5 in x+1
                NeighbourChecks {
                    n: TetraIndex::Three,
                    axis: Axis::YPos,
                }, // touches 3 in y+1
                NeighbourChecks {
                    n: TetraIndex::Five,
                    axis: Axis::YPos,
                }, // touches 5 in y+1
                NeighbourChecks {
                    n: TetraIndex::Three,
                    axis: Axis::ZPos,
                }, // touches 3 in z+1
                NeighbourChecks {
                    n: TetraIndex::One,
                    axis: Axis::ZNeg,
                }, // touches 1 in z-1
                // 5 DOUBLE AXIS EDGE CHECKS
                NeighbourChecks {
                    n: TetraIndex::Four,
                    axis: Axis::YPosZNeg,
                }, // touches 4 in y+1 z-1
                NeighbourChecks {
                    n: TetraIndex::Five,
                    axis: Axis::YPosZNeg,
                }, // touches 5 in y+1 z-1
                NeighbourChecks {
                    n: TetraIndex::Zero,
                    axis: Axis::XPosZPos,
                }, // touches 0 in x+1 z+1
                NeighbourChecks {
                    n: TetraIndex::Five,
                    axis: Axis::XPosZPos,
                }, // touches 5 in x+1 z+1
                NeighbourChecks {
                    n: TetraIndex::Five,
                    axis: Axis::XPosYPos,
                }, // touches 5 in x+1 y+1
            ]
        }
        TetraIndex::Three => {
            vec![
                // 2 FACE CHECKS
                NeighbourChecks {
                    n: TetraIndex::Five,
                    axis: Axis::XPos,
                }, // touches 5 in x+1
                NeighbourChecks {
                    n: TetraIndex::One,
                    axis: Axis::ZNeg,
                }, // touches 1 in z-1
                // 6 SINGLE AXIS EDGE CHECKS
                NeighbourChecks {
                    n: TetraIndex::Zero,
                    axis: Axis::XPos,
                }, // touches 0 in x+1
                NeighbourChecks {
                    n: TetraIndex::Four,
                    axis: Axis::XPos,
                }, // touches 4 in x+1
                NeighbourChecks {
                    n: TetraIndex::Zero,
                    axis: Axis::YPos,
                }, // touches 0 in y+1
                NeighbourChecks {
                    n: TetraIndex::Two,
                    axis: Axis::YNeg,
                }, // touches 2 in y-1
                NeighbourChecks {
                    n: TetraIndex::Two,
                    axis: Axis::ZNeg,
                }, // touches 2 in z-1
                NeighbourChecks {
                    n: TetraIndex::Four,
                    axis: Axis::ZNeg,
                }, // touches 4 in z-1
                // 5 DOUBLE AXIS EDGE CHECKS
                NeighbourChecks {
                    n: TetraIndex::One,
                    axis: Axis::XPosYNeg,
                }, // touches 1 in x+1 y-1
                NeighbourChecks {
                    n: TetraIndex::Four,
                    axis: Axis::XPosYNeg,
                }, // touches 4 in x+1 y-1
                NeighbourChecks {
                    n: TetraIndex::Four,
                    axis: Axis::XPosZNeg,
                }, // touches 4 in x+1 z-1
                NeighbourChecks {
                    n: TetraIndex::Four,
                    axis: Axis::YPosZNeg,
                }, // touches 4 in y+1 z-1
                NeighbourChecks {
                    n: TetraIndex::Five,
                    axis: Axis::YPosZNeg,
                }, // touches 5 in y+1 z-1
            ]
        }
        TetraIndex::Four => {
            vec![
                // 2 FACE CHECKS
                NeighbourChecks {
                    n: TetraIndex::Two,
                    axis: Axis::XNeg,
                }, // touches face of dark blue
                NeighbourChecks {
                    n: TetraIndex::Zero,
                    axis: Axis::ZPos,
                }, // touches face of white
                // 6 SINGLE AXIS EDGE CHECKS
                NeighbourChecks {
                    n: TetraIndex::One,
                    axis: Axis::XNeg,
                }, // touches 1 in x-1
                NeighbourChecks {
                    n: TetraIndex::Three,
                    axis: Axis::XNeg,
                }, // touches 3 in x-1
                NeighbourChecks {
                    n: TetraIndex::Five,
                    axis: Axis::YPos,
                }, // touches 5 in y+1
                NeighbourChecks {
                    n: TetraIndex::One,
                    axis: Axis::YNeg,
                }, // touches 1 in y-1
                NeighbourChecks {
                    n: TetraIndex::Three,
                    axis: Axis::ZPos,
                }, // touches 3 in z+1
                NeighbourChecks {
                    n: TetraIndex::Five,
                    axis: Axis::ZPos,
                }, // touches 5 in z+1
                // 5 DOUBLE AXIS EDGE CHECKS
                NeighbourChecks {
                    n: TetraIndex::Three,
                    axis: Axis::XNegZPos,
                }, // touches 3 in x-1 z+1
                NeighbourChecks {
                    n: TetraIndex::Zero,
                    axis: Axis::XNegYPos,
                }, // touches 0 in x-1 y+1
                NeighbourChecks {
                    n: TetraIndex::Three,
                    axis: Axis::XNegYPos,
                }, // touches 3 in x-1 y+1
                NeighbourChecks {
                    n: TetraIndex::Two,
                    axis: Axis::YNegZPos,
                }, // touches 2 y-1 z+1
                NeighbourChecks {
                    n: TetraIndex::Three,
                    axis: Axis::YNegZPos,
                }, // touches 3 y-1 z+1
            ]
        }
        TetraIndex::Five => {
            vec![
                // 2 FACE CHECKS
                NeighbourChecks {
                    n: TetraIndex::Three,
                    axis: Axis::XNeg,
                }, // touches 3 in x-1
                NeighbourChecks {
                    n: TetraIndex::One,
                    axis: Axis::YNeg,
                }, // touches 1 in y-1
                // 6 SINGLE AXIS EDGE CHECKS
                NeighbourChecks {
                    n: TetraIndex::Two,
                    axis: Axis::XNeg,
                }, // touches 2 in x-1
                NeighbourChecks {
                    n: TetraIndex::Zero,
                    axis: Axis::XNeg,
                }, // touches 0 in x-1
                NeighbourChecks {
                    n: TetraIndex::Two,
                    axis: Axis::YNeg,
                }, // touches 2 in y-1
                NeighbourChecks {
                    n: TetraIndex::Four,
                    axis: Axis::YNeg,
                }, // touches 4 in y-1
                NeighbourChecks {
                    n: TetraIndex::Zero,
                    axis: Axis::ZPos,
                }, // touches 0 in z+1
                NeighbourChecks {
                    n: TetraIndex::Four,
                    axis: Axis::ZNeg,
                }, // touches 4 in z-1
                // 5 DOUBLE AXIS EDGE CHECKS
                NeighbourChecks {
                    n: TetraIndex::Two,
                    axis: Axis::XNegYNeg,
                }, // touches 2 in x-1 y-1
                NeighbourChecks {
                    n: TetraIndex::One,
                    axis: Axis::XNegZNeg,
                }, // touches 1 in x-1 z-1
                NeighbourChecks {
                    n: TetraIndex::Two,
                    axis: Axis::XNegZNeg,
                }, // touches 2 in x-1 z-1
                NeighbourChecks {
                    n: TetraIndex::Two,
                    axis: Axis::YNegZPos,
                }, // touches 2 y-1 z+1
                NeighbourChecks {
                    n: TetraIndex::Three,
                    axis: Axis::YNegZPos,
                }, // touches 3 y-1 z+1
            ]
        }
    }
}

pub fn place_life_with_keyboard(
    camera: Query<&mut Transform, With<Camera>>,
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    mut session: ResMut<SessionResource>,
) {
    // if we hit the right key(s) then generate life in a specific spot in front of the camera
    if keys.any_just_pressed([
        KeyCode::Digit1,
        KeyCode::Digit2,
        KeyCode::Digit3,
        KeyCode::Digit4,
        KeyCode::Digit5,
        KeyCode::Digit6,
        KeyCode::Space,
    ]) {
        for transform in camera.iter() {
            let spawn_at =
                (transform.translation + (transform.forward() * 1500.0)) / LIFE_FORM_SIZE;

            // TODO we need a way of detecting which of 6 tetras needs to be created, for now just use the number keys
            let (n, tetra_index) = if keys.just_pressed(KeyCode::Digit1) {
                (0, TetraIndex::Zero)
            } else if keys.just_pressed(KeyCode::Digit2) {
                (1, TetraIndex::One)
            } else if keys.just_pressed(KeyCode::Digit3) {
                (2, TetraIndex::Two)
            } else if keys.just_pressed(KeyCode::Digit4) {
                (3, TetraIndex::Three)
            } else if keys.just_pressed(KeyCode::Digit5) {
                (4, TetraIndex::Four)
            } else {
                (5, TetraIndex::Five)
            };

            if spawn_at.x > 0.0
                && spawn_at.x < session.universe_size as f32
                && spawn_at.y > 0.0
                && spawn_at.y < session.universe_size as f32
                && spawn_at.z > 0.0
                && spawn_at.z < session.universe_size as f32
            {
                match session.life[n][spawn_at.x as usize][spawn_at.y as usize][spawn_at.z as usize]
                {
                    LifeDataContainer::Alive(ent) => {
                        //if alive currently
                        commands.entity(ent.to_owned()).despawn();
                        session.life[n][spawn_at.x as usize][spawn_at.y as usize]
                            [spawn_at.z as usize] = LifeDataContainer::Dead;
                        session.counter -= 1;
                    }
                    LifeDataContainer::Dead => {
                        // if dead currently
                        // Place a life form
                        let transform_new_life: bevy::prelude::Transform = create_life_xyz(
                            &tetra_index,
                            spawn_at.x as usize,
                            spawn_at.y as usize,
                            spawn_at.z as usize,
                        );
                        session.life[n][spawn_at.x as usize][spawn_at.y as usize]
                            [spawn_at.z as usize] = LifeDataContainer::Alive(
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
                        session.counter += 1;
                    }
                }
            }
        }
    }
}

pub fn dead_universe() -> Vec<Vec<Vec<Vec<LifeDataContainer>>>> {
    vec![
        vec![
            vec![
                vec![LifeDataContainer::Dead; crate::DEFAULT_UNIVERSE_SIZE];
                crate::DEFAULT_UNIVERSE_SIZE
            ];
            crate::DEFAULT_UNIVERSE_SIZE
        ];
        6
    ]
}

pub fn new_universe(
    mut life_entities: Query<Entity, With<Life>>,
    mut commands: Commands,
    mut session: ResMut<SessionResource>,
    state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    match state.get() {
        AppState::NewGame => {}
        s => {
            println!("WARNING App State was not NewGame, it was {:?}", s);
            return;
        }
    }
    session.counter = 0;
    session.generation = 1;
    session.life = dead_universe();
    session.universe_size = crate::DEFAULT_UNIVERSE_SIZE;
    // unspawn every single life entity
    for ent in life_entities.iter_mut() {
        commands.entity(ent.to_owned()).despawn();
    }

    next_state.set(AppState::InGame);
}

pub fn run(mut commands: Commands, mut session: ResMut<SessionResource>) {
    println!("running life!");
    // first generation, generate random life
    if session.generation == 1 {
        let life_to_create: Vec<Vec<Vec<Vec<LifeDataContainer>>>> = session.life.clone();
        for tetra_index in TETRA_INDEXES {
            let n: usize = tetra_index as usize;
            for (x, vec2) in life_to_create[n].iter().enumerate() {
                for (y, vec3) in vec2.iter().enumerate() {
                    for (z, _empty_entity_id) in vec3.iter().enumerate() {
                        //randomly generate initial life in the universe
                        if rand::random::<bool>() {
                            //create no life here
                            continue;
                        }

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

                        //increment life counter
                        session.counter += 1;
                    }
                }
            }
        }
    } else if session.counter > 1 {
        // while there is life
        let last_gen: Vec<Vec<Vec<Vec<LifeDataContainer>>>> = session.life.clone();

        /*
        white touches dark blue and dark grey in the same xyz and light blue in the y below
        red touches light grey and light blue in same xyz and the dark grey in the y above
        light blue touches red and dark blue in the same xyz and white in the y above
        dark blue touches light blue and white in same xyz and red and dark grey either side (need to check if thats x or z)
        light grey touches dark grey and red in the same xyz and light blue and white either side (need to check if thats x or z)
        dark grey touches light grey and white in the same xyz and red in the y below
        */

        for tetra_index in [
            TetraIndex::Zero,
            TetraIndex::One,
            TetraIndex::Two,
            TetraIndex::Three,
            TetraIndex::Four,
            TetraIndex::Five,
        ] {
            let n: usize = tetra_index as usize;
            for (x, vec2) in last_gen[n].iter().enumerate() {
                for (y, vec3) in vec2.iter().enumerate() {
                    for (z, entity_life) in vec3.iter().enumerate() {
                        let mut neighbours: usize = 0;

                        for check in checks(&tetra_index).iter() {
                            let mut check_x = x;
                            let mut check_y = y;
                            let mut check_z = z;

                            match &check.axis {
                                Axis::XPos => check_x += 1,
                                Axis::XNeg => check_x = check_x.wrapping_sub(1),
                                Axis::YPos => check_y += 1,
                                Axis::YNeg => check_y = check_y.wrapping_sub(1),
                                Axis::ZPos => check_z += 1,
                                Axis::ZNeg => check_z = check_z.wrapping_sub(1),
                                Axis::XPosYPos => {
                                    check_x += 1;
                                    check_y += 1
                                }
                                Axis::XPosYNeg => {
                                    check_x += 1;
                                    check_y = check_y.wrapping_sub(1)
                                }
                                Axis::XNegYPos => {
                                    check_x = check_x.wrapping_sub(1);
                                    check_y += 1
                                }
                                Axis::XNegYNeg => {
                                    check_x = check_x.wrapping_sub(1);
                                    check_y = check_y.wrapping_sub(1)
                                }
                                Axis::XPosZPos => {
                                    check_x += 1;
                                    check_z += 1
                                }
                                Axis::XPosZNeg => {
                                    check_x += 1;
                                    check_z = check_z.wrapping_sub(1)
                                }
                                Axis::XNegZPos => {
                                    check_x = check_x.wrapping_sub(1);
                                    check_z += 1
                                }
                                Axis::XNegZNeg => {
                                    check_x = check_x.wrapping_sub(1);
                                    check_z = check_z.wrapping_sub(1)
                                }
                                Axis::YPosZPos => {
                                    check_y += 1;
                                    check_z += 1
                                }
                                Axis::YPosZNeg => {
                                    check_y += 1;
                                    check_z = check_z.wrapping_sub(1)
                                }
                                Axis::YNegZPos => {
                                    check_y = check_y.wrapping_sub(1);
                                    check_z += 1
                                }
                                Axis::YNegZNeg => {
                                    check_y = check_y.wrapping_sub(1);
                                    check_z = check_z.wrapping_sub(1)
                                }
                            }

                            // handle overflow
                            //TODO: in universe size 256 this may not be needed
                            if check_x == session.universe_size {
                                check_x = 0;
                            }
                            if check_y == session.universe_size {
                                check_y = 0;
                            }
                            if check_z == session.universe_size {
                                check_z = 0;
                            }

                            // handle underflow
                            //TODO: in universe size 256 this may not be needed
                            if check_x > session.universe_size {
                                check_x = session.universe_size - 1;
                            }
                            if check_y > session.universe_size {
                                check_y = session.universe_size - 1;
                            }
                            if check_z > session.universe_size {
                                check_z = session.universe_size - 1;
                            }

                            // check if the neighbour is alive, and if so increment neighbours!
                            if let LifeDataContainer::Alive(_) =
                                last_gen[check.n as usize][check_x][check_y][check_z]
                            {
                                neighbours += 1;
                            }
                        }

                        // CHECK 5 NEIGHBOURS IN SAME CUBE
                        for m in 0..=5 {
                            if n != m
                                && let LifeDataContainer::Alive(_) = last_gen[m][x][y][z]
                            {
                                neighbours += 1
                            }
                        }

                        match entity_life {
                            LifeDataContainer::Alive(ent) => {
                                //if alive in last gen
                                if neighbours > 3 || neighbours == 1 || neighbours == 0 {
                                    commands.entity(ent.to_owned()).despawn();
                                    session.life[n][x][y][z] = LifeDataContainer::Dead;

                                    session.counter -= 1;
                                } else {
                                    // continue to be alive
                                }
                            }
                            LifeDataContainer::Dead => {
                                // if dead in last gen
                                //if neighbours = 3 then become alive
                                if neighbours == 3 {
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

                                    //increment life counter
                                    session.counter += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    session.generation += 1;
}

#[cfg(test)]
mod tests {
    use super::*;

    use bevy::core_pipeline::CorePipelinePlugin;
    use bevy::render::RenderPlugin;
    use bevy_obj::*; // used import wavefront obj files

    fn initialise_test_universe(save_filename: &str) -> bevy::prelude::App {
        // Setup app
        let mut app = App::new();

        app.add_plugins((
            MinimalPlugins,
            AssetPlugin::default(),
            WindowPlugin::default(),
            RenderPlugin::default(),
        ));

        app.init_state::<AppState>()
            .insert_state(AppState::LoadGame);

        app.init_asset::<StandardMaterial>();

        // Asset server for meshes
        let asset_server = app
            .world
            .get_resource::<AssetServer>()
            .expect("expected asset server");

        // Load meshes
        let mesh = bevy_obj::load_obj_from_bytes(crate::MESH_TETRA_MIRRORED_BYTES)
            .expect("load_obj_from_bytes() failed");
        let tetrahedron_mirrored_mesh_handle = asset_server.add(mesh);
        let mesh = bevy_obj::load_obj_from_bytes(crate::MESH_TETRA_BYTES)
            .expect("load_obj_from_bytes() failed");
        let tetrahedron_mesh_handle = asset_server.add(mesh);

        // Materials
        let material_handles = [
            asset_server
                .add(StandardMaterial {
                    base_color: Color::rgb(0.0, 1.0, 0.0),
                    ..default()
                })
                .clone(),
            asset_server
                .add(StandardMaterial {
                    base_color: Color::rgb(0.6, 0.2, 0.2),
                    ..default()
                })
                .clone(),
            asset_server
                .add(StandardMaterial {
                    base_color: Color::rgb(0.5, 0.5, 1.0),
                    ..default()
                })
                .clone(),
            asset_server
                .add(StandardMaterial {
                    base_color: Color::rgb(0.1, 0.1, 0.7),
                    ..default()
                })
                .clone(),
            asset_server
                .add(StandardMaterial {
                    base_color: Color::rgb(1.0, 1.0, 0.0),
                    ..default()
                })
                .clone(),
            asset_server
                .add(StandardMaterial {
                    base_color: Color::rgb(0.2, 0.2, 0.2),
                    ..default()
                })
                .clone(),
        ];

        // New session resource
        let session = SessionResource {
            life: dead_universe(),
            counter: 0,
            generation: 1,
            life_form_materials: material_handles,
            life_form_meshes: [
                tetrahedron_mirrored_mesh_handle.clone(),
                tetrahedron_mesh_handle.clone(),
            ],
            universe_size: 10,
        };

        // New load game resource
        let game_file_to_load =
            crate::systems::saves::GameFileToLoad::Some(save_filename.to_string());

        // Add resources
        app.insert_resource(session);
        app.insert_resource(game_file_to_load);

        /*let sound_resources = crate::systems::sound::SoundResource {
            music: asset_server.add(AudioSource {
                bytes: crate::SOUND_BG_LOOP.into(),
            }),
            button_click: asset_server.add(AudioSource {
                bytes: crate::SOUND_BUTTON_CLICK.into(),
            }),
            button_hover: asset_server.add(AudioSource {
                bytes: crate::SOUND_BUTTON_HOVER.into(),
            }),
        };

        app.insert_resource(sound_resources);*/

        // Setup systems
        app.add_systems(Startup, crate::setup);
        app.add_systems(Startup, crate::systems::camera_movement::setup);
        //app.add_systems(Startup, crate::systems::sound::setup);

        // Add state transition systems
        app.add_systems(OnEnter(AppState::Splash), crate::systems::saves::load);
        app.add_systems(
            OnEnter(AppState::LoadGame),
            crate::systems::saves::load.before(run),
        );
        app.add_systems(OnEnter(AppState::InGame), run);
        app.add_systems(Update, run);

        // Debugging statements
        println!(
            "App initialized with state: {:?}",
            app.world.get_resource::<State<AppState>>()
        );
        app
    }

    fn check_universe_state(
        world: &World,
        expected_app_state: &AppState,
        expected_generation: i64,
        expected_counter: i64,
    ) {
        assert_eq!(world.resource::<State<AppState>>(), expected_app_state);
        assert_eq!(
            world.resource::<SessionResource>().generation,
            expected_generation
        );
        assert_eq!(
            world.resource::<SessionResource>().counter,
            expected_counter
        );
    }

    #[test]
    fn test_life_two_in_same_cube_dies() {
        /* TEST DESCRIPTION
           Start State: 2 tetras indexed ?,? in the same cube
           Expect: universe to die off
        */
        let mut app = initialise_test_universe("test_01");
        check_universe_state(&app.world, &AppState::LoadGame, 1, 0);
        app.update();
        check_universe_state(&app.world, &AppState::InGame, 2, 2);
        app.update();
        check_universe_state(&app.world, &AppState::InGame, 3, 0);
    }
    #[test]
    fn test_life_012_in_same_cube_breeds() {
        /* TEST DESCRIPTION
           Start State: 3 tetras indexed 0,1,2 in the same cube
           Expect: 3 to become a cube of 6t.
                   6 to die and create 12t.
        */
        let mut app = initialise_test_universe("test_02");
        check_universe_state(&app.world, &AppState::LoadGame, 1, 0);
        app.update();
        check_universe_state(&app.world, &AppState::InGame, 2, 3);
        app.update();
        // at this point we have one solid cube of 6 lifeforms
        check_universe_state(&app.world, &AppState::InGame, 3, 6);
        app.update();
        // at this point we have twelve lifeforms that exist from the faces of the starting cube
        check_universe_state(&app.world, &AppState::InGame, 4, 12);
        app.update();
        //
        check_universe_state(&app.world, &AppState::InGame, 5, 36);
        app.update();
        //
        check_universe_state(&app.world, &AppState::InGame, 6, 12);
        app.update();
        //
        check_universe_state(&app.world, &AppState::InGame, 7, 0);
    }
    #[test]
    fn test_life_345_in_same_cube_breeds() {
        /* TEST DESCRIPTION
           Start State: 3 tetras indexed 3,4,5 in the same cube
           Expect: 3 to become a cube of 6t.
                   6 to die and create 12t.
        */
        let mut app = initialise_test_universe("test_03");
        check_universe_state(&app.world, &AppState::LoadGame, 1, 0);
        app.update();
        check_universe_state(&app.world, &AppState::InGame, 2, 3);
        app.update();
        // at this point we have one solid cube of 6 lifeforms
        check_universe_state(&app.world, &AppState::InGame, 3, 6);
        app.update();
        // at this point we have twelve lifeforms that exist from the faces of the starting cube
        check_universe_state(&app.world, &AppState::InGame, 4, 12);
        app.update();
        //
        check_universe_state(&app.world, &AppState::InGame, 5, 36);
        app.update();
        //
        check_universe_state(&app.world, &AppState::InGame, 6, 12);
        app.update();
        //
        check_universe_state(&app.world, &AppState::InGame, 7, 0);
    }
}
