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

    //TODO: Replace rotations with 6 correctly rotated obj files
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
        Neighbour graph implemented by checks():
        - Faces:
            white (Zero) faces: light blue (Two) at y-1, light grey (Four) at z-1
            red (One) faces: dark grey (Five) at y+1, dark blue (Three) at z+1
            light blue (Two) faces: light grey (Four) at x+1, white (Zero) at y+1
            dark blue (Three) faces: dark grey (Five) at x+1, red (One) at z-1
            light grey (Four) faces: light blue (Two) at x-1, white (Zero) at z+1
            dark grey (Five) faces: dark blue (Three) at x-1, red (One) at y-1
        - Edges (unique neighbor indices by edges; single- and double-axis, corners excluded):
            white (Zero) edges: One, Two, Three, Four, Five
            red (One) edges: Zero, Two, Three, Four, Five
            light blue (Two) edges: Zero, One, Three, Four, Five
            dark blue (Three) edges: Zero, One, Two, Four, Five
            light grey (Four) edges: Zero, One, Two, Three, Five
            dark grey (Five) edges: Zero, One, Two, Three, Four
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

    fn initialise_test_universe(save_filename: &str) -> bevy::prelude::App {
        // Setup app
        let mut app = App::new();

        app.add_plugins((MinimalPlugins, AssetPlugin::default()));

        app.init_state::<AppState>()
            .insert_state(AppState::LoadGame);

        app.init_asset::<StandardMaterial>();
        app.init_asset::<bevy::prelude::Mesh>();

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

        // No startup systems in headless tests

        // Add state transition systems
        app.add_systems(OnEnter(AppState::LoadGame), crate::systems::saves::load);
        // Life system will be registered by each test after the first update into InGame

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
        app.add_systems(
            Update,
            run.run_if(bevy::ecs::schedule::common_conditions::in_state(
                AppState::InGame,
            )),
        );
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
        app.add_systems(
            Update,
            run.run_if(bevy::ecs::schedule::common_conditions::in_state(
                AppState::InGame,
            )),
        );
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
        app.add_systems(
            Update,
            run.run_if(bevy::ecs::schedule::common_conditions::in_state(
                AppState::InGame,
            )),
        );
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
    fn test_osc4_period10_persists_and_repeats() {
        // Load the known 4-live period-10 oscillator (3x3x3 universe)
        let mut app = initialise_test_universe("test_osc4_period10");
        check_universe_state(&app.world, &AppState::LoadGame, 1, 0);

        // Run load
        app.update();

        // Add life system and ensure we start from the loaded state
        app.add_systems(
            Update,
            run.run_if(bevy::ecs::schedule::common_conditions::in_state(
                AppState::InGame,
            )),
        );

        // After load, we should be InGame with generation=2 and counter=4 (from the save)
        check_universe_state(&app.world, &AppState::InGame, 2, 4);

        // Snapshot function that ignores entity IDs and captures alive pattern
        fn snapshot(world: &bevy::prelude::World) -> Vec<u8> {
            let session = world.resource::<SessionResource>();
            let us = session.universe_size;
            let mut k = Vec::with_capacity(6 * us * us * us);
            for n in 0..6 {
                for x in 0..us {
                    for y in 0..us {
                        for z in 0..us {
                            let alive = matches!(
                                session.life[n][x][y][z],
                                LifeDataContainer::Alive(_)
                            );
                            k.push(alive as u8);
                        }
                    }
                }
            }
            k
        }

        // Detect the actual cycle period from the loaded seed (which may be in a transient state)
        use std::collections::HashMap;
        let mut seen: HashMap<Vec<u8>, usize> = HashMap::new();
        let mut period_found: Option<usize> = None;

        // Insert the initial signature at step 0
        let mut sig = snapshot(&app.world);
        seen.insert(sig.clone(), 0);

        for step in 1..=200 {
            // advance one generation
            app.update();

            // population non-zero
            let counter = app.world.resource::<SessionResource>().counter;
            assert!(counter > 0, "population reached zero at step {}", step);

            // capture signature and check for repeat
            sig = snapshot(&app.world);
            if let Some(prev_step) = seen.get(&sig) {
                let period = step - prev_step;
                period_found = Some(period);
                break;
            } else {
                seen.insert(sig.clone(), step);
            }
        }

        let period = period_found.expect("no repeat detected within 200 steps");
        assert_eq!(period, 10, "expected period 10, got {}", period);
    }
}

#[cfg(test)]
mod experiments {
    #[test]
    #[ignore = "experiment, not a test; run with --ignored"]
    fn enumerate_oscillations_3x3x3_up_to_2() {
        // Enumerate seeds in a 3x3x3 universe with up to 2 live tetras and
        // detect period-k oscillations under the current rules.
        const SIZE: i32 = 3;
        const MAX_STEPS: usize = 9;

        type State = [[[[bool; 3]; 3]; 3]; 6];

        fn empty() -> State {
            [[[[false; 3]; 3]; 3]; 6]
        }

        fn idx_to_coords(idx: usize) -> (usize, usize, usize, usize) {
            let n = idx / 27;
            let r = idx % 27;
            let x = r / 9;
            let r2 = r % 9;
            let y = r2 / 3;
            let z = r2 % 3;
            (n, x, y, z)
        }

        fn wrap(a: i32) -> usize {
            let mut v = a % SIZE;
            if v < 0 {
                v += SIZE;
            }
            v as usize
        }

        fn next(state: &State, checks_all: &[Vec<super::NeighbourChecks>; 6]) -> State {
            let mut out = empty();
            for (ni, checks_vec) in checks_all.iter().enumerate() {
                for x in 0..3 {
                    for y in 0..3 {
                        for z in 0..3 {
                            let mut neighbours: usize = 0;

                            for check in checks_vec.iter() {
                                let (mut cx, mut cy, mut cz) = (x as i32, y as i32, z as i32);
                                match check.axis {
                                    super::Axis::XPos => cx += 1,
                                    super::Axis::XNeg => cx -= 1,
                                    super::Axis::YPos => cy += 1,
                                    super::Axis::YNeg => cy -= 1,
                                    super::Axis::ZPos => cz += 1,
                                    super::Axis::ZNeg => cz -= 1,
                                    super::Axis::XPosYPos => {
                                        cx += 1;
                                        cy += 1
                                    }
                                    super::Axis::XPosYNeg => {
                                        cx += 1;
                                        cy -= 1
                                    }
                                    super::Axis::XNegYPos => {
                                        cx -= 1;
                                        cy += 1
                                    }
                                    super::Axis::XNegYNeg => {
                                        cx -= 1;
                                        cy -= 1
                                    }
                                    super::Axis::XPosZPos => {
                                        cx += 1;
                                        cz += 1
                                    }
                                    super::Axis::XPosZNeg => {
                                        cx += 1;
                                        cz -= 1
                                    }
                                    super::Axis::XNegZPos => {
                                        cx -= 1;
                                        cz += 1
                                    }
                                    super::Axis::XNegZNeg => {
                                        cx -= 1;
                                        cz -= 1
                                    }
                                    super::Axis::YPosZPos => {
                                        cy += 1;
                                        cz += 1
                                    }
                                    super::Axis::YPosZNeg => {
                                        cy += 1;
                                        cz -= 1
                                    }
                                    super::Axis::YNegZPos => {
                                        cy -= 1;
                                        cz += 1
                                    }
                                    super::Axis::YNegZNeg => {
                                        cy -= 1;
                                        cz -= 1
                                    }
                                }

                                let (cx, cy, cz) = (wrap(cx), wrap(cy), wrap(cz));
                                let nn = check.n as usize;
                                if state[nn][cx][cy][cz] {
                                    neighbours += 1;
                                }
                            }

                            // same-xyz neighbours (all other five tetras)
                            for m in 0..6 {
                                if m != ni && state[m][x][y][z] {
                                    neighbours += 1;
                                }
                            }

                            let alive = state[ni][x][y][z];
                            out[ni][x][y][z] = if alive {
                                neighbours == 2 || neighbours == 3
                            } else {
                                neighbours == 3
                            };
                        }
                    }
                }
            }
            out
        }

        fn key(state: &State) -> Vec<u8> {
            let mut k = Vec::with_capacity(6 * 27);
            for n in 0..6 {
                for x in 0..3 {
                    for y in 0..3 {
                        for z in 0..3 {
                            k.push(state[n][x][y][z] as u8);
                        }
                    }
                }
            }
            k
        }

        let checks_all: [Vec<super::NeighbourChecks>; 6] = [
            super::checks(&super::TetraIndex::Zero),
            super::checks(&super::TetraIndex::One),
            super::checks(&super::TetraIndex::Two),
            super::checks(&super::TetraIndex::Three),
            super::checks(&super::TetraIndex::Four),
            super::checks(&super::TetraIndex::Five),
        ];

        let total_cells = 6 * 27;
        let mut found = 0usize;

        // 0-lives seed
        {
            let mut st = empty();
            let mut seen = std::collections::HashMap::<Vec<u8>, usize>::new();
            seen.insert(key(&st), 0);

            for step in 1..=MAX_STEPS {
                st = next(&st, &checks_all);
                let k = key(&st);
                if let Some(prev) = seen.get(&k) {
                    let period = step - prev;
                    println!("seed 0-lives oscillates with period {}", period);
                    found += 1;
                    break;
                }
                seen.insert(k, step);

                // all dead?
                let all_dead = (0..6).all(|n| {
                    (0..3).all(|x| (0..3).all(|y| (0..3).all(|z| st[n][x][y][z] == false)))
                });
                if all_dead {
                    break;
                }
            }
        }

        // 1-live seeds
        for i in 0..total_cells {
            let mut st = empty();
            let (n, x, y, z) = idx_to_coords(i);
            st[n][x][y][z] = true;
            let mut seen = std::collections::HashMap::<Vec<u8>, usize>::new();
            seen.insert(key(&st), 0);

            let mut cur = st;
            for step in 1..=MAX_STEPS {
                cur = next(&cur, &checks_all);
                let k = key(&cur);
                if let Some(prev) = seen.get(&k) {
                    let period = step - prev;
                    println!("seed 1 at idx {} oscillates period {}", i, period);
                    found += 1;
                    break;
                }
                seen.insert(k, step);

                let all_dead = (0..6).all(|n| {
                    (0..3).all(|x| (0..3).all(|y| (0..3).all(|z| cur[n][x][y][z] == false)))
                });
                if all_dead {
                    break;
                }
            }
        }

        // 2-live seeds
        for i in 0..total_cells {
            for j in (i + 1)..total_cells {
                let mut st = empty();
                let (n1, x1, y1, z1) = idx_to_coords(i);
                let (n2, x2, y2, z2) = idx_to_coords(j);
                st[n1][x1][y1][z1] = true;
                st[n2][x2][y2][z2] = true;

                let mut seen = std::collections::HashMap::<Vec<u8>, usize>::new();
                seen.insert(key(&st), 0);

                let mut cur = st;
                for step in 1..=MAX_STEPS {
                    cur = next(&cur, &checks_all);
                    let k = key(&cur);
                    if let Some(prev) = seen.get(&k) {
                        let period = step - prev;
                        println!("seed 2 at idxs ({}, {}) oscillates period {}", i, j, period);
                        found += 1;
                        break;
                    }
                    seen.insert(k, step);

                    let all_dead = (0..6).all(|n| {
                        (0..3).all(|x| (0..3).all(|y| (0..3).all(|z| cur[n][x][y][z] == false)))
                    });
                    if all_dead {
                        break;
                    }
                }
            }
        }

        println!("Found {} oscillating seeds (period ≤ {})", found, MAX_STEPS);
        // Exploratory enumeration; don't fail CI even if none found
        assert!(true);
    }

    #[test]
    #[ignore = "experiment, not a test; run with --ignored"]
    fn enumerate_oscillations_3x3x3_4_and_5_same_xyz() {
        // Enumerate 4- and 5-live seeds restricted to the same (x,y,z) cell across tetra indices.
        // This keeps the search small while probing rich interactions via same-xyz neighbors.
        const SIZE: i32 = 3;
        const MAX_STEPS: usize = 8;

        type State = [[[[bool; 3]; 3]; 3]; 6];

        fn empty() -> State {
            [[[[false; 3]; 3]; 3]; 6]
        }

        fn wrap(a: i32) -> usize {
            let mut v = a % SIZE;
            if v < 0 {
                v += SIZE;
            }
            v as usize
        }

        fn next(state: &State, checks_all: &[Vec<super::NeighbourChecks>; 6]) -> State {
            let mut out = empty();
            for (ni, checks_vec) in checks_all.iter().enumerate() {
                for x in 0..3 {
                    for y in 0..3 {
                        for z in 0..3 {
                            let mut neighbours: usize = 0;

                            for check in checks_vec.iter() {
                                let (mut cx, mut cy, mut cz) = (x as i32, y as i32, z as i32);
                                match check.axis {
                                    super::Axis::XPos => cx += 1,
                                    super::Axis::XNeg => cx -= 1,
                                    super::Axis::YPos => cy += 1,
                                    super::Axis::YNeg => cy -= 1,
                                    super::Axis::ZPos => cz += 1,
                                    super::Axis::ZNeg => cz -= 1,
                                    super::Axis::XPosYPos => {
                                        cx += 1;
                                        cy += 1;
                                    }
                                    super::Axis::XPosYNeg => {
                                        cx += 1;
                                        cy -= 1;
                                    }
                                    super::Axis::XNegYPos => {
                                        cx -= 1;
                                        cy += 1;
                                    }
                                    super::Axis::XNegYNeg => {
                                        cx -= 1;
                                        cy -= 1;
                                    }
                                    super::Axis::XPosZPos => {
                                        cx += 1;
                                        cz += 1;
                                    }
                                    super::Axis::XPosZNeg => {
                                        cx += 1;
                                        cz -= 1;
                                    }
                                    super::Axis::XNegZPos => {
                                        cx -= 1;
                                        cz += 1;
                                    }
                                    super::Axis::XNegZNeg => {
                                        cx -= 1;
                                        cz -= 1;
                                    }
                                    super::Axis::YPosZPos => {
                                        cy += 1;
                                        cz += 1;
                                    }
                                    super::Axis::YPosZNeg => {
                                        cy += 1;
                                        cz -= 1;
                                    }
                                    super::Axis::YNegZPos => {
                                        cy -= 1;
                                        cz += 1;
                                    }
                                    super::Axis::YNegZNeg => {
                                        cy -= 1;
                                        cz -= 1;
                                    }
                                }

                                let (cx, cy, cz) = (wrap(cx), wrap(cy), wrap(cz));
                                let nn = check.n as usize;
                                if state[nn][cx][cy][cz] {
                                    neighbours += 1;
                                }
                            }

                            // same-xyz neighbours (all other five tetras)
                            for m in 0..6 {
                                if m != ni && state[m][x][y][z] {
                                    neighbours += 1;
                                }
                            }

                            let alive = state[ni][x][y][z];
                            out[ni][x][y][z] = if alive {
                                neighbours == 2 || neighbours == 3
                            } else {
                                neighbours == 3
                            };
                        }
                    }
                }
            }
            out
        }

        fn key(state: &State) -> Vec<u8> {
            let mut k = Vec::with_capacity(6 * 27);
            for n in 0..6 {
                for x in 0..3 {
                    for y in 0..3 {
                        for z in 0..3 {
                            k.push(state[n][x][y][z] as u8);
                        }
                    }
                }
            }
            k
        }

        let checks_all: [Vec<super::NeighbourChecks>; 6] = [
            super::checks(&super::TetraIndex::Zero),
            super::checks(&super::TetraIndex::One),
            super::checks(&super::TetraIndex::Two),
            super::checks(&super::TetraIndex::Three),
            super::checks(&super::TetraIndex::Four),
            super::checks(&super::TetraIndex::Five),
        ];

        let mut found = 0usize;

        // Enumerate all (x,y,z) cells; for each, choose 4-of-6 and 5-of-6 tetra indices at that same cell.
        for x in 0..3 {
            for y in 0..3 {
                for z in 0..3 {
                    // 4-live seeds at same (x,y,z)
                    for a in 0..6 {
                        for b in (a + 1)..6 {
                            for c in (b + 1)..6 {
                                for d in (c + 1)..6 {
                                    let mut st = empty();
                                    st[a][x][y][z] = true;
                                    st[b][x][y][z] = true;
                                    st[c][x][y][z] = true;
                                    st[d][x][y][z] = true;

                                    let mut seen =
                                        std::collections::HashMap::<Vec<u8>, usize>::new();
                                    seen.insert(key(&st), 0);

                                    let mut cur = st;
                                    for step in 1..=MAX_STEPS {
                                        cur = next(&cur, &checks_all);
                                        let k = key(&cur);
                                        if let Some(prev) = seen.get(&k) {
                                            let period = step - prev;
                                            println!(
                                                "4-live seed at cell ({},{},{}) layers [{},{},{},{}] oscillates period {}",
                                                x, y, z, a, b, c, d, period
                                            );
                                            found += 1;
                                            break;
                                        }
                                        seen.insert(k, step);

                                        let all_dead = (0..6).all(|n| {
                                            (0..3).all(|xx| {
                                                (0..3).all(|yy| {
                                                    (0..3).all(|zz| cur[n][xx][yy][zz] == false)
                                                })
                                            })
                                        });
                                        if all_dead {
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // 5-live seeds at same (x,y,z)
                    for a in 0..6 {
                        for b in (a + 1)..6 {
                            for c in (b + 1)..6 {
                                for d in (c + 1)..6 {
                                    for e in (d + 1)..6 {
                                        let mut st = empty();
                                        st[a][x][y][z] = true;
                                        st[b][x][y][z] = true;
                                        st[c][x][y][z] = true;
                                        st[d][x][y][z] = true;
                                        st[e][x][y][z] = true;

                                        let mut seen =
                                            std::collections::HashMap::<Vec<u8>, usize>::new();
                                        seen.insert(key(&st), 0);

                                        let mut cur = st;
                                        for step in 1..=MAX_STEPS {
                                            cur = next(&cur, &checks_all);
                                            let k = key(&cur);
                                            if let Some(prev) = seen.get(&k) {
                                                let period = step - prev;
                                                println!(
                                                    "5-live seed at cell ({},{},{}) layers [{},{},{},{},{}] oscillates period {}",
                                                    x, y, z, a, b, c, d, e, period
                                                );
                                                found += 1;
                                                break;
                                            }
                                            seen.insert(k, step);

                                            let all_dead = (0..6).all(|n| {
                                                (0..3).all(|xx| {
                                                    (0..3).all(|yy| {
                                                        (0..3).all(|zz| cur[n][xx][yy][zz] == false)
                                                    })
                                                })
                                            });
                                            if all_dead {
                                                break;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        println!(
            "Found {} oscillating 4/5-live seeds (period ≤ {})",
            found, MAX_STEPS
        );
        assert!(true);
    }
}
