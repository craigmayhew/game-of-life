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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

pub fn order_life_form_meshes<T>(normal: T, mirrored: T) -> [T; 2] {
    // The even tetra types use the normal mesh and the odd types use the
    // mirrored mesh. This chirality order is required for the six transformed
    // tetrahedra to form a consistent tessellation.
    [normal, mirrored]
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
impl Axis {
    const fn offset(self) -> (i32, i32, i32) {
        match self {
            Axis::XPos => (1, 0, 0),
            Axis::XNeg => (-1, 0, 0),
            Axis::YPos => (0, 1, 0),
            Axis::YNeg => (0, -1, 0),
            Axis::ZPos => (0, 0, 1),
            Axis::ZNeg => (0, 0, -1),
            Axis::XPosYPos => (1, 1, 0),
            Axis::XPosYNeg => (1, -1, 0),
            Axis::XNegYPos => (-1, 1, 0),
            Axis::XNegYNeg => (-1, -1, 0),
            Axis::XPosZPos => (1, 0, 1),
            Axis::XPosZNeg => (1, 0, -1),
            Axis::XNegZPos => (-1, 0, 1),
            Axis::XNegZNeg => (-1, 0, -1),
            Axis::YPosZPos => (0, 1, 1),
            Axis::YPosZNeg => (0, 1, -1),
            Axis::YNegZPos => (0, -1, 1),
            Axis::YNegZNeg => (0, -1, -1),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NeighbourChecks {
    n: TetraIndex,
    axis: Axis,
}

const fn neighbour(n: TetraIndex, axis: Axis) -> NeighbourChecks {
    NeighbourChecks { n, axis }
}

fn wrapped_coordinate(coordinate: usize, offset: i32, universe_size: usize) -> usize {
    (coordinate as i64 + i64::from(offset)).rem_euclid(universe_size as i64) as usize
}

fn checks(n: &TetraIndex) -> [NeighbourChecks; 13] {
    use self::Axis::*;
    use self::TetraIndex::*;

    // These are the 13 external tetrahedra that share a face or edge with the
    // transformed OBJ mesh. The other five neighbours occupy the same logical
    // cube and are counted separately by the simulation.
    match n {
        TetraIndex::Zero => [
            // Faces
            neighbour(Two, ZNeg),
            neighbour(Four, XPos),
            // Edges
            neighbour(One, YNeg),
            neighbour(One, ZNeg),
            neighbour(Two, XPosYNeg),
            neighbour(Three, YPos),
            neighbour(Three, XPos),
            neighbour(Four, YPosZNeg),
            neighbour(Five, ZNeg),
            neighbour(Five, YPosZNeg),
            neighbour(Five, XPosYNeg),
            neighbour(Five, XPosZNeg),
            neighbour(Five, XPos),
        ],
        TetraIndex::One => [
            // Faces
            neighbour(Three, YPos),
            neighbour(Five, XPos),
            // Edges
            neighbour(Zero, ZPos),
            neighbour(Zero, YPos),
            neighbour(Two, ZNeg),
            neighbour(Two, XPos),
            neighbour(Three, XPosZPos),
            neighbour(Four, YPosZNeg),
            neighbour(Four, YPos),
            neighbour(Four, XPos),
            neighbour(Four, XPosZPos),
            neighbour(Four, XPosYPos),
            neighbour(Five, YPosZNeg),
        ],
        TetraIndex::Two => [
            // Faces
            neighbour(Zero, ZPos),
            neighbour(Four, YPos),
            // Edges
            neighbour(Zero, XNegYPos),
            neighbour(One, XNeg),
            neighbour(One, ZPos),
            neighbour(Three, XNegYPos),
            neighbour(Three, ZPos),
            neighbour(Three, YPos),
            neighbour(Three, YPosZPos),
            neighbour(Three, XPosZPos),
            neighbour(Four, XPosZPos),
            neighbour(Five, YPos),
            neighbour(Five, XPos),
        ],
        TetraIndex::Three => [
            // Faces
            neighbour(One, YNeg),
            neighbour(Five, ZNeg),
            // Edges
            neighbour(Zero, XNeg),
            neighbour(Zero, YNeg),
            neighbour(One, XNegZNeg),
            neighbour(Two, XNegZNeg),
            neighbour(Two, YNegZNeg),
            neighbour(Two, YNeg),
            neighbour(Two, ZNeg),
            neighbour(Two, XPosYNeg),
            neighbour(Four, ZNeg),
            neighbour(Four, XPos),
            neighbour(Five, XPosYNeg),
        ],
        TetraIndex::Four => [
            // Faces
            neighbour(Zero, XNeg),
            neighbour(Two, YNeg),
            // Edges
            neighbour(Zero, YNegZPos),
            neighbour(One, XNegYNeg),
            neighbour(One, XNegZNeg),
            neighbour(One, XNeg),
            neighbour(One, YNeg),
            neighbour(One, YNegZPos),
            neighbour(Two, XNegZNeg),
            neighbour(Three, XNeg),
            neighbour(Three, ZPos),
            neighbour(Five, YNeg),
            neighbour(Five, ZNeg),
        ],
        TetraIndex::Five => [
            // Faces
            neighbour(One, XNeg),
            neighbour(Three, ZPos),
            // Edges
            neighbour(Zero, XNeg),
            neighbour(Zero, XNegZPos),
            neighbour(Zero, XNegYPos),
            neighbour(Zero, YNegZPos),
            neighbour(Zero, ZPos),
            neighbour(One, YNegZPos),
            neighbour(Two, XNeg),
            neighbour(Two, YNeg),
            neighbour(Three, XNegYPos),
            neighbour(Four, ZPos),
            neighbour(Four, YPos),
        ],
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

    // Random seeding belongs to new-game creation. Simulation ticks must only
    // evolve the current universe so loaded generation-one saves remain intact.
    for tetra_index in TETRA_INDEXES {
        let n = tetra_index as usize;
        for x in 0..session.universe_size {
            for y in 0..session.universe_size {
                for z in 0..session.universe_size {
                    if rand::random::<bool>() {
                        continue;
                    }

                    let entity = commands
                        .spawn(PbrBundle {
                            mesh: session.life_form_meshes[n % 2].clone(),
                            material: session.life_form_materials[n].clone(),
                            transform: create_life_xyz(&tetra_index, x, y, z),
                            ..default()
                        })
                        .insert(Life)
                        .id();

                    session.life[n][x][y][z] = LifeDataContainer::Alive(entity);
                    session.counter += 1;
                }
            }
        }
    }

    next_state.set(AppState::InGame);
}

pub fn run(mut commands: Commands, mut session: ResMut<SessionResource>) {
    println!("running life!");
    if session.counter > 0 {
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
                            let (offset_x, offset_y, offset_z) = check.axis.offset();
                            let check_x = wrapped_coordinate(x, offset_x, session.universe_size);
                            let check_y = wrapped_coordinate(y, offset_y, session.universe_size);
                            let check_z = wrapped_coordinate(z, offset_z, session.universe_size);

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
    use std::collections::HashSet;

    use bevy::render::mesh::VertexAttributeValues;

    use super::*;

    const VERTEX_EPSILON: f32 = 0.01;

    fn mesh_vertices(bytes: &[u8]) -> Vec<Vec3> {
        let mesh = bevy_obj::load_obj_from_bytes(bytes).expect("load_obj_from_bytes() failed");
        let positions = match mesh.attribute(Mesh::ATTRIBUTE_POSITION) {
            Some(VertexAttributeValues::Float32x3(positions)) => positions,
            _ => panic!("expected Float32x3 mesh positions"),
        };

        let mut vertices = Vec::new();
        for position in positions {
            let vertex = Vec3::from_array(*position);
            if !vertices
                .iter()
                .any(|existing: &Vec3| existing.distance_squared(vertex) < f32::EPSILON)
            {
                vertices.push(vertex);
            }
        }

        assert_eq!(vertices.len(), 4, "a tetrahedron must have four vertices");
        vertices
    }

    fn transformed_vertices(
        tetra_index: TetraIndex,
        offset: IVec3,
        ordered_meshes: &[Vec<Vec3>; 2],
    ) -> Vec<Vec3> {
        let coordinate = IVec3::splat(3) + offset;
        let transform = create_life_xyz(
            &tetra_index,
            coordinate.x as usize,
            coordinate.y as usize,
            coordinate.z as usize,
        )
        .compute_matrix();

        ordered_meshes[tetra_index as usize % 2]
            .iter()
            .map(|vertex| transform.transform_point3(*vertex))
            .collect()
    }

    fn shared_vertex_count(left: &[Vec3], right: &[Vec3]) -> usize {
        left.iter()
            .filter(|left_vertex| {
                right.iter().any(|right_vertex| {
                    left_vertex.distance_squared(*right_vertex) <= VERTEX_EPSILON * VERTEX_EPSILON
                })
            })
            .count()
    }

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
            life_form_meshes: order_life_form_meshes(
                tetrahedron_mesh_handle.clone(),
                tetrahedron_mirrored_mesh_handle.clone(),
            ),
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
    fn neighbour_table_matches_transformed_mesh_geometry() {
        let normal_vertices = mesh_vertices(crate::MESH_TETRA_BYTES);
        let mirrored_vertices = mesh_vertices(crate::MESH_TETRA_MIRRORED_BYTES);
        let ordered_meshes = order_life_form_meshes(normal_vertices, mirrored_vertices);

        for tetra_index in TETRA_INDEXES {
            let base_vertices = transformed_vertices(tetra_index, IVec3::ZERO, &ordered_meshes);
            let expected_external: HashSet<_> = checks(&tetra_index)
                .iter()
                .map(|check| (check.n as usize, check.axis.offset()))
                .collect();
            let mut actual_external = HashSet::new();
            let mut same_cube_neighbours = 0;
            let mut same_cube_face_neighbours = 0;
            let mut external_face_neighbours = 0;

            for candidate_index in TETRA_INDEXES {
                for x in -2..=2 {
                    for y in -2..=2 {
                        for z in -2..=2 {
                            let offset = IVec3::new(x, y, z);
                            if candidate_index == tetra_index && offset == IVec3::ZERO {
                                continue;
                            }

                            let candidate_vertices =
                                transformed_vertices(candidate_index, offset, &ordered_meshes);
                            let shared_vertices =
                                shared_vertex_count(&base_vertices, &candidate_vertices);

                            if offset == IVec3::ZERO {
                                if shared_vertices >= 2 {
                                    same_cube_neighbours += 1;
                                    if shared_vertices == 3 {
                                        same_cube_face_neighbours += 1;
                                    }
                                }
                            } else if shared_vertices >= 2 {
                                assert!(
                                    shared_vertices <= 3,
                                    "distinct tetrahedra must not overlap"
                                );
                                actual_external.insert((
                                    candidate_index as usize,
                                    (offset.x, offset.y, offset.z),
                                ));
                                if shared_vertices == 3 {
                                    external_face_neighbours += 1;
                                }
                            }
                        }
                    }
                }
            }

            assert_eq!(same_cube_neighbours, 5, "{tetra_index:?}");
            assert_eq!(actual_external.len(), 13, "{tetra_index:?}");
            assert_eq!(
                same_cube_face_neighbours + external_face_neighbours,
                4,
                "{tetra_index:?}"
            );
            assert_eq!(actual_external, expected_external, "{tetra_index:?}");
        }
    }

    #[test]
    fn test_loaded_generation_one_is_evolved_without_reseeding() {
        let mut app = initialise_test_universe("test_01");
        app.update();
        check_universe_state(&app.world, &AppState::InGame, 2, 2);

        app.world.resource_mut::<SessionResource>().generation = 1;
        app.add_systems(
            Update,
            run.run_if(bevy::ecs::schedule::common_conditions::in_state(
                AppState::InGame,
            )),
        );
        app.update();

        check_universe_state(&app.world, &AppState::InGame, 2, 0);
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
           Start State: 3 tetras indexed 0,1,2 in the same cube.
           Expect: evolution according to the rendered face/edge topology.
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
        check_universe_state(&app.world, &AppState::InGame, 3, 8);
        app.update();
        check_universe_state(&app.world, &AppState::InGame, 4, 15);
        app.update();
        check_universe_state(&app.world, &AppState::InGame, 5, 35);
        app.update();
        check_universe_state(&app.world, &AppState::InGame, 6, 29);
        app.update();
        check_universe_state(&app.world, &AppState::InGame, 7, 55);
    }
    #[test]
    fn test_life_345_in_same_cube_breeds() {
        /* TEST DESCRIPTION
           Start State: 3 tetras indexed 3,4,5 in the same cube.
           Expect: the same evolution as the symmetric 0,1,2 fixture.
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
        check_universe_state(&app.world, &AppState::InGame, 3, 8);
        app.update();
        check_universe_state(&app.world, &AppState::InGame, 4, 15);
        app.update();
        check_universe_state(&app.world, &AppState::InGame, 5, 35);
        app.update();
        check_universe_state(&app.world, &AppState::InGame, 6, 29);
        app.update();
        check_universe_state(&app.world, &AppState::InGame, 7, 55);
    }
}
