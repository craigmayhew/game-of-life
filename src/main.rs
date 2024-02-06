#![feature(let_chains)] // allow if something && let Some(blah) =
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use bevy::{
    prelude::*, //default bevy
    window::{PresentMode,PrimaryWindow}, // needed to specify window info
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
};
// these are needed to load an icon
use bevy::winit::WinitWindows;
use winit::window::Icon;
 // used import wavefront obj files
use bevy_obj::*;

mod systems;

// default universe size if none specified
const DEFAULT_UNIVERSE_SIZE: usize = 20;

// Defines the amount of time that should elapse between each physics step.
#[derive(PartialEq, Debug, Resource)]
pub struct GameSpeed {
    ticks_per_second: f64,
    last_tick_stamp: f64,
}

#[derive(Clone, Eq, Default, PartialEq, Debug, Hash, Resource, States)]
pub enum AppState {
    #[default]
    Splash,
    InGame,
    Paused,
    LoadGame,
    NewGame,
    SaveGame,
}

#[derive(Resource)]
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
}

fn set_window_icon(
    // we have to use `NonSend` here
    windows: NonSend<WinitWindows>,
    primary_window: Query<Entity, With<PrimaryWindow>>,
) {
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
    let primary_entity = primary_window.single();
    let primary = windows.get_window(primary_entity).unwrap();
    primary.set_window_icon(Some(icon));
}

fn main() {
    use bevy::time::common_conditions::on_timer;
    use std::time::Duration;
    App::new()
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Game of Life".to_string(),
            resolution: (1500.0,900.0).into(),
            present_mode: PresentMode::AutoVsync,
            ..default()
        }),
        ..default()
    }))
    .add_state::<AppState>()
    .add_plugin(ObjPlugin)
    .add_plugin(LogDiagnosticsPlugin::default())
    .add_plugin(FrameTimeDiagnosticsPlugin::default())
    .add_startup_system(set_window_icon)
    .add_startup_system(systems::camera_movement::setup)
    .insert_resource(ClearColor(Color::BLACK)) //set the background colour of our window (the universe)
    .add_startup_system(setup)
    // keyboard input (excluding camera movement)
    .add_system(systems::keyboard::run)
    // life system
    .add_system(
        systems::life::run.run_if(on_timer(Duration::from_millis(100)))
    )
    // AppState::Splash
    .add_system(systems::menu::setup.in_schedule(OnEnter(AppState::Splash)))
    .add_system(systems::menu::run.in_set(OnUpdate(AppState::Splash)))
    .add_system(systems::menu::cleanup.in_schedule(OnExit(AppState::Splash)))
    // AppState::InGame
    .add_system(systems::hud::enter.in_schedule(OnEnter(AppState::InGame)))
    .add_systems(
        (systems::hud::run,systems::life::place_life_with_keyboard,systems::camera_movement::move_camera_on_keyboard_input).in_set(OnUpdate(AppState::InGame))
    )
    .add_system(systems::hud::cleanup.in_schedule(OnExit(AppState::InGame)))
    // AppState::Paused
    .add_systems(
        (systems::menu_paused::enter,systems::hud::enter).in_schedule(OnEnter(AppState::Paused))
    )
    .add_systems(
        (systems::camera_movement::move_camera_on_keyboard_input,systems::hud::run,systems::life::place_life_with_keyboard).in_set(OnUpdate(AppState::Paused))
    )
    .add_systems((systems::hud::cleanup,systems::menu_paused::cleanup).in_schedule(OnExit(AppState::Paused)))
    // AppState::NewGame
    .add_system(
        systems::life::new_universe.in_schedule(OnEnter(AppState::NewGame)).before(systems::life::run)
    )
    // AppState::LoadGame
    .add_system(
        systems::saves::load.in_schedule(OnEnter(AppState::LoadGame)).before(systems::life::run)
    )
    // AppState::SaveGame
    .add_system(
        systems::saves::save.in_schedule(OnEnter(AppState::SaveGame)).before(systems::life::run)
    )
    .run()
}
