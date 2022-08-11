use bevy::{
    prelude::*, //default bevy
    window::PresentMode, // needed to specify window info
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
};
use bevy_obj::*; // immport wavefront obj files

mod systems;

pub const ARENA_HEIGHT: f32 = 1000.0;
pub const ARENA_WIDTH: f32 = 1000.0;
pub const UNIVERSE_SIZE: usize = 30;

enum AppState {
    Menu,
    InGame,
    Paused,
}

pub struct SessionResource {
    pub life: Vec<Vec<Vec<Vec<bevy::prelude::Entity>>>>,
    pub counter: i64,
    pub generation: i64,
    pub life_form_materials: [bevy::prelude::Handle<StandardMaterial>; 6], //stores handles to the 6 life form tetras
    pub life_form_meshes: [bevy::prelude::Handle<Mesh>; 2], //stores handles to the two life form meshes
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    meshes: Res<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    //setting up initial state of life throughout our 3d space
    let universe_life = vec![vec![vec![vec![Entity::from_raw(0); UNIVERSE_SIZE]; UNIVERSE_SIZE]; UNIVERSE_SIZE]; 6];
    
    commands.insert_resource(SessionResource {
        life: universe_life,
        counter: 0,
        generation: 1,
        life_form_materials: [
            materials.add(StandardMaterial {
                base_color: Color::rgb(1.0, 1.0, 1.0), // white
                ..default()
            }),
            materials.add(StandardMaterial {
                base_color: Color::rgb(0.6, 0.2, 1.0), // red
                ..default()
            }),
            materials.add(StandardMaterial {
                base_color: Color::rgb(0.5, 0.5, 1.0), // light blue
                ..default()
            }),
            materials.add(StandardMaterial {
                base_color: Color::rgb(0.2, 0.2, 0.7), // dark blue
                ..default()
            }),
            materials.add(StandardMaterial {
                base_color: Color::rgb(0.5, 0.5, 0.5), // light grey
                ..default()
            }),
            materials.add(StandardMaterial {
                base_color: Color::rgb(0.2, 0.2, 0.2), // dark grey
                ..default()
            })
        ],
        life_form_meshes: [
            asset_server.load("mesh/hill-tetrahedron-mirrored.obj"),
            asset_server.load("mesh/hill-tetrahedron.obj"),
        ]
    });

    // camera
    systems::camera_movement::setup(commands);
}

fn main() {    
    App::new()
    .insert_resource(WindowDescriptor {
        title: "Game of Life".to_string(),
        width: 1500.,
        height: 900.,
        present_mode: PresentMode::AutoVsync,
        ..default()
    })
    .add_plugins(DefaultPlugins)
    .add_plugin(ObjPlugin)
    .add_plugin(LogDiagnosticsPlugin::default())
    .add_plugin(FrameTimeDiagnosticsPlugin::default())
    .add_startup_system(setup)
    .add_system(systems::camera_movement::move_camera_on_keyboard_input)
    .add_system(systems::life::run)
    .run()
}
