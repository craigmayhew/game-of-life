extern crate amethyst;
extern crate rand;
use amethyst::{
    assets::{AssetStorage, Loader, Handle},
    core::ArcThreadPool,
    core::transform::{Transform, TransformBundle},
    //Component is used to attach structs to entities in the game
    ecs::prelude::{Dispatcher},
    input::{InputBundle, StringBindings},
    prelude::*,
    //renderer is used to display a window
    renderer::{
        //needed for sprites
        ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture,
        //
        plugins::{RenderFlat2D, RenderFlat3D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    shred::{DispatcherBuilder},
    //needed for application_root_dir() etc
    utils::application_root_dir,
};

mod systems;
use crate::systems::camera_movement;
use crate::systems::life;

pub const ARENA_HEIGHT: f32 = 1000.0;
pub const ARENA_WIDTH: f32 = 1000.0;
pub const LIFE_FORM_SIZE: f32 = 150.0;
pub const UNIVERSE_SIZE: usize = 30;

fn initialise_stars(world: &mut World, sprite_sheet: Handle<SpriteSheet>) {
    //// 2D square
    let mut transform = Transform::default();

    // Correctly position the 2D square
    let x = ARENA_WIDTH / 2.0;
    let y = ARENA_HEIGHT / 2.0;
    transform.set_translation_xyz(x, y, -100.0);
    transform.set_rotation_x_axis(0.2);
    transform.set_rotation_y_axis(0.2);
    transform.set_rotation_z_axis(0.2);

    // Assign the sprites for the 2D square
    let sprite_render = SpriteRender {
        sprite_sheet,
        sprite_number: 0, // 0 means it's the first sprite in the sprite_sheet
    };

    // Create a 2D square
    world
        .create_entity()
        .with(sprite_render)
        .with(transform)
        .build();
}

fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    // Load the sprite sheet necessary to render the graphics.
    // The texture is the pixel data
    // `texture_handle` is a cloneable reference to the texture
    let texture_handle = {
        // loader is a resource
        let loader = world.read_resource::<Loader>();
        //texture storage is a resource
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/2d_square_sprite.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/2d_square_sprite.ron", // Here we load the associated ron file
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}

struct GameplayState {
    pub dispatcher: Dispatcher<'static, 'static>,
}

struct SessionResource {
    pub life: Vec<Vec<Vec<Vec<bool>>>>,
    pub counter: i64,
}

fn initialise_camera(world: &mut World) {
    // Setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom left. 
    let mut transform = Transform::default();
    //initial camera position
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 200.0);

    let camera = amethyst::renderer::camera::Camera::perspective(
        ARENA_WIDTH / ARENA_HEIGHT,
        std::f32::consts::FRAC_PI_3,
        0.1,
    );

    world
        .create_entity()
        .named("Main camera")
        .with(camera)
        .with(transform)
        .build();
}

//GameData is the internal shared data between states
impl SimpleState for GameplayState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let mut world = data.world;

        // Load the spritesheet necessary to render the graphics.
        let sprite_sheet_handle = load_sprite_sheet(world);

        initialise_stars(world, sprite_sheet_handle);

        initialise_camera(world);

        //setting up initial state of life throughout our 3d space
        let mut universe_life = vec![vec![vec![vec![false; UNIVERSE_SIZE]; UNIVERSE_SIZE]; UNIVERSE_SIZE]; 6];
        
        //TODO Could be used to load a "save"
        for n in 0..6 {
            for x in 0..UNIVERSE_SIZE {
                for y in 0..UNIVERSE_SIZE {
                    for z in 0..UNIVERSE_SIZE {
                        universe_life[n][x][y][z] = rand::random::<bool>();
                    }
                }
            }
        }                    
        let session_resource = SessionResource {
            life: universe_life,
            counter: 0,
        };
        world.insert(session_resource);

        let dispatcher = DispatcherBuilder::new()
        .with_pool((*world.read_resource::<ArcThreadPool>()).clone())
        .with(
            camera_movement::CameraMovementSystem::default(),
            "camera_movement",
            &[],
        )
        .with(
            life::LifeSystem::default(),
            "life",
            &[],
        )
        .build();

        self.dispatcher = dispatcher;

        self.dispatcher.setup(&mut world);
    }

    fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans {
        self.dispatcher.dispatch(&data.world);
        
        Trans::None
    }
}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());
    
    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");

    let binding_path = app_root.join("config").join("keybindings.ron");
    let input_bundle = InputBundle::<StringBindings>::new()
    .with_bindings_from_file(binding_path)?;

    let black = [0.0, 0.0, 0.0, 1.0];

    let game_data = GameDataBuilder::default()
    .with_bundle(
        RenderingBundle::<DefaultBackend>::new()
            // The RenderToWindow plugin provides all the scaffolding for opening a window and drawing on it
            .with_plugin(
                RenderToWindow::from_config_path(display_config_path)?
                    .with_clear(black),
            )
            // RenderFlat2D plugin is used to render entities with a `SpriteRender` component.
            .with_plugin(RenderFlat2D::default())
            // RenderFlat3D plugin is used to render meshes
            .with_plugin(RenderFlat3D::default()),
    )?
    .with_bundle(input_bundle)?
    // Add the transform bundle which handles tracking entity positions
    .with_bundle(TransformBundle::new())?;

    let assets_dir = app_root.join("assets");
    let game_play_start = GameplayState{dispatcher: DispatcherBuilder::new().build()};
    let mut game = Application::new(assets_dir, game_play_start, game_data)?;
    game.run();

    Ok(())
}
