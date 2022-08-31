#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use bevy::{
    prelude::*, //default bevy
    window::PresentMode, // needed to specify window info
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
};
// these are needed to load an icon
use bevy::{window::WindowId,winit::WinitWindows};
use winit::window::Icon;
 // used import wavefront obj files
use bevy_obj::*;

mod systems;

// default universe size if none specified
const DEFAULT_UNIVERSE_SIZE: usize = 20;

// Defines the amount of time that should elapse between each physics step.
#[derive(PartialEq, Debug)]
pub struct GameSpeed {
    ticks_per_second: f64,
    last_tick_stamp: f64,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum AppState {
    Splash,
    InGame,
    Paused,
    LoadGame,
    NewGame,
    SaveGame,
}

pub struct SessionResource {
    pub life: Vec<Vec<Vec<Vec<systems::life::LifeDataContainer>>>>,
    pub counter: i64,
    pub generation: i64,
    pub life_form_materials: [bevy::prelude::Handle<StandardMaterial>; 6], //stores handles to the 6 life form tetras
    pub life_form_meshes: [bevy::prelude::Handle<Mesh>; 2], //stores handles to the two life form meshes
    pub universe_size: usize,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.insert_resource(SessionResource {
        life: systems::life::dead_universe(),
        counter: 0,
        generation: 1,
        life_form_materials: [
            materials.add(StandardMaterial {
                base_color: Color::rgb(0.0, 1.0, 0.0), // white -> green
                ..default()
            }),
            materials.add(StandardMaterial {
                base_color: Color::rgb(0.6, 0.2, 0.2), // red
                ..default()
            }),
            materials.add(StandardMaterial {
                base_color: Color::rgb(0.5, 0.5, 1.0), // light blue
                ..default()
            }),
            materials.add(StandardMaterial {
                base_color: Color::rgb(0.1, 0.1, 0.7), // dark blue
                ..default()
            }),
            materials.add(StandardMaterial {
                base_color: Color::rgb(1.0, 1.0, 0.0), // light grey -> yellow
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
        ],
        universe_size: DEFAULT_UNIVERSE_SIZE,
    });

    //resource used to know which save file to load if/when needed
    commands.insert_resource(systems::saves::GameFileToLoad::None());

    commands.insert_resource(GameSpeed {
        ticks_per_second: 2.0,
        last_tick_stamp: 0.0
    });

    //ambient light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 1.0,
    });

    // camera
    systems::camera_movement::setup(commands);
}

fn set_window_icon(
    // we have to use `NonSend` here
    windows: NonSend<WinitWindows>,
) {
    let primary = windows.get_window(WindowId::primary()).unwrap();

    // here we use the `image` crate to load our icon data from a png file
    // this is not a very bevy-native solution, but it will do
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open("assets/hills-tetrahedron.png")
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();

    primary.set_window_icon(Some(icon));
}

use bevy::ecs::schedule::ShouldRun;
fn run_if_timestep(
    mut game_speed: ResMut<GameSpeed>,
    time: Res<Time>,
  ) -> ShouldRun
  {
    if game_speed.last_tick_stamp + (1.0/game_speed.ticks_per_second) < time.seconds_since_startup() {
        game_speed.last_tick_stamp = time.seconds_since_startup();
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
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
    .add_state(AppState::Splash)
    .add_plugins(DefaultPlugins)
    .add_plugin(ObjPlugin)
    .add_plugin(LogDiagnosticsPlugin::default())
    .add_plugin(FrameTimeDiagnosticsPlugin::default())
    .add_startup_system(set_window_icon)
    .insert_resource(ClearColor(Color::BLACK)) //set the background colour of our window (the universe)
    .add_startup_system(setup)
    // camera system (camera movement controls)
    .add_system(systems::camera_movement::move_camera_on_keyboard_input)
    // keyboard input (excluding camera movement)
    .add_system(systems::keyboard::run)
    // life system
    .add_system_set(
        SystemSet::new()
            .with_system(systems::life::run)
            .with_run_criteria(run_if_timestep)
    )
    // AppState::Splash
    .add_system_set(
        SystemSet::on_enter(AppState::Splash)
            .with_system(systems::menu::setup)
    )
    .add_system_set(
        SystemSet::on_update(AppState::Splash)
            .with_system(systems::menu::run)
    )
    .add_system_set(
        SystemSet::on_exit(AppState::Splash)
            .with_system(systems::menu::cleanup)
    )
    // AppState::InGame
    .add_system_set(
        SystemSet::on_enter(AppState::InGame)
            .with_system(systems::hud::enter)
    )
    .add_system_set(
        SystemSet::on_update(AppState::InGame)
            .with_system(systems::hud::run)
            // Place Life with keyboard
            .with_system(systems::life::place_life_with_keyboard)
    )
    .add_system_set(
        SystemSet::on_exit(AppState::InGame)
            .with_system(systems::hud::cleanup)
    )
    // AppState::Paused
    .add_system_set(
        SystemSet::on_enter(AppState::Paused)
        .with_system(systems::menu_paused::enter)
        .with_system(systems::hud::enter)
    )
    .add_system_set(
        SystemSet::on_update(AppState::Paused)
            .with_system(systems::hud::run)
            // Place Life with keyboard
            .with_system(systems::life::place_life_with_keyboard)
    )
    .add_system_set(
        SystemSet::on_exit(AppState::Paused)
            .with_system(systems::menu_paused::cleanup)
            .with_system(systems::hud::cleanup)
    )
    // AppState::NewGame
    .add_system_set(
        SystemSet::on_enter(AppState::NewGame)
        .with_system(systems::life::new_universe)
        .before(systems::life::run)
    )
    // AppState::LoadGame
    .add_system_set(
        SystemSet::on_enter(AppState::LoadGame)
        .with_system(systems::saves::load)
        .before(systems::life::run)
    )
    // AppState::SaveGame
    .add_system_set(
        SystemSet::on_enter(AppState::SaveGame)
        .with_system(systems::saves::save)
        .before(systems::life::run)
    )
    .run()
}
