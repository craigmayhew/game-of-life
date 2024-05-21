#![feature(let_chains)] // allow if something && let Some(blah) =
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,                           //default bevy
    window::{PresentMode, PrimaryWindow}, // needed to specify window info
};
// these are needed to load an icon
use bevy::winit::WinitWindows;
use winit::window::Icon;
// used import wavefront obj files
use bevy_obj::*;

mod systems;

// default universe size if none specified
const DEFAULT_UNIVERSE_SIZE: usize = 20;

// meshes
const MESH_TETRA_BYTES: &'static [u8] = include_bytes!("../assets/mesh/hill-tetrahedron.obj");
const MESH_TETRA_MIRRORED_BYTES: &'static [u8] =
    include_bytes!("../assets/mesh/hill-tetrahedron-mirrored.obj");
// fonts
const FONT_BYTES: &'static [u8] = include_bytes!("../assets/font/square.ttf");
// application icon
const ICON: &'static [u8] = include_bytes!("../assets/hills-tetrahedron.png");
// sounds
const SOUND_BG_LOOP: &'static [u8] = include_bytes!("../assets/sound/ambient_loop.ogg");
const SOUND_BUTTON_CLICK: &'static [u8] = include_bytes!("../assets/sound/button_click.ogg");
const SOUND_BUTTON_HOVER: &'static [u8] = include_bytes!("../assets/sound/button_over.ogg");

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
    Credits,
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
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let mesh =
        bevy_obj::load_obj_from_bytes(MESH_TETRA_BYTES).expect("load_obj_from_bytes() failed");
    let tetrahedron_mesh_handle = meshes.add(mesh);
    let mesh = bevy_obj::load_obj_from_bytes(MESH_TETRA_MIRRORED_BYTES)
        .expect("load_obj_from_bytes() failed");
    let tetrahedron_mirrored_mesh_handle = meshes.add(mesh);

    //load materials and assets
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
            }),
        ],
        life_form_meshes: [tetrahedron_mesh_handle, tetrahedron_mirrored_mesh_handle],
        universe_size: DEFAULT_UNIVERSE_SIZE,
    });

    //resource used to know which save file to load if/when needed
    commands.insert_resource(systems::saves::string_to_game_file_name(""));

    commands.insert_resource(GameSpeed {
        ticks_per_second: 2.0,
        last_tick_stamp: 0.0,
    });

    //ambient light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.1,
    });

    // a nearby sun!
    commands
        .spawn(PointLightBundle {
            transform: Transform::from_xyz(10_000_000.0, 0.0, 0.0),
            point_light: PointLight {
                intensity: f32::MAX / 1_000_000_000_000.0, // lumens
                color: Color::YELLOW,
                radius: 100.0,
                range: 10_000_000.0,
                shadows_enabled: true,
                ..default()
            },
            ..default()
        })
        .with_children(|builder| {
            builder.spawn(PbrBundle {
                mesh: meshes.add(Sphere::new(900_000.0).mesh().uv(32, 18)),
                material: materials.add(StandardMaterial {
                    base_color: Color::YELLOW,
                    emissive: Color::YELLOW,
                    diffuse_transmission: 1.0,
                    ..default()
                }),
                ..default()
            });
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
        let image = image::load_from_memory(ICON)
            .expect("Error loading logo image")
            .to_rgba8();
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
                resolution: (1500.0, 900.0).into(),
                present_mode: PresentMode::AutoVsync,
                ..default()
            }),
            ..default()
        }))
        .init_state::<AppState>()
        .add_plugins(ObjPlugin)
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_systems(Startup, set_window_icon)
        .add_systems(Startup, systems::camera_movement::setup)
        .add_systems(Startup, systems::sound::setup)
        .insert_resource(ClearColor(Color::BLACK)) //set the background colour of our window (the universe)
        .add_systems(Startup, setup)
        // keyboard input (excluding camera movement)
        .add_systems(Update, systems::keyboard::run)
        // life system
        .add_systems(
            Update,
            systems::life::run
                .run_if(on_timer(Duration::from_millis(100)))
                .run_if(in_state(AppState::InGame)),
        )
        // sound system
        .add_systems(Update, systems::sound::update)
        // AppState::Splash
        .add_systems(OnEnter(AppState::Splash), systems::menu::setup)
        .add_systems(
            Update,
            systems::menu::run.run_if(in_state(AppState::Splash)),
        )
        .add_systems(OnExit(AppState::Splash), systems::menu::cleanup)
        // AppState::InGame
        .add_systems(OnEnter(AppState::InGame), systems::hud::enter)
        .add_systems(
            Update,
            (
                systems::hud::run,
                systems::life::place_life_with_keyboard.run_if(in_state(AppState::InGame)),
                systems::camera_movement::move_camera_on_keyboard_input
                    .run_if(in_state(AppState::InGame)),
            ),
        )
        .add_systems(OnExit(AppState::InGame), systems::hud::cleanup)
        // AppState::Paused
        .add_systems(
            OnEnter(AppState::Paused),
            (systems::menu_paused::enter, systems::hud::enter),
        )
        .add_systems(
            Update,
            (
                systems::camera_movement::move_camera_on_keyboard_input
                    .run_if(in_state(AppState::Paused)),
                systems::hud::run,
                systems::life::place_life_with_keyboard.run_if(in_state(AppState::Paused)),
            ),
        )
        .add_systems(
            OnExit(AppState::Paused),
            (systems::hud::cleanup, systems::menu_paused::cleanup),
        )
        // AppState::Credits
        .add_systems(
            OnEnter(AppState::Credits),
            (systems::menu_credits::enter, systems::hud::enter),
        )
        .add_systems(
            Update,
            (
                systems::camera_movement::move_camera_on_keyboard_input
                    .run_if(in_state(AppState::Credits)),
                systems::hud::run,
            ),
        )
        .add_systems(
            OnExit(AppState::Credits),
            (systems::hud::cleanup, systems::menu_credits::cleanup),
        )
        // AppState::NewGame
        .add_systems(
            OnEnter(AppState::NewGame),
            systems::life::new_universe.before(systems::life::run),
        )
        // AppState::LoadGame
        .add_systems(
            OnEnter(AppState::LoadGame),
            systems::saves::load.before(systems::life::run),
        )
        // AppState::SaveGame
        .add_systems(
            OnEnter(AppState::SaveGame),
            systems::saves::save.before(systems::life::run),
        )
        .run()
}
