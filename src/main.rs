extern crate amethyst;
use amethyst::{
    core::transform::{Transform, TransformBundle},
    //Component is used to attach structs to entities in the game
    ecs::prelude::{Component, DenseVecStorage},
    prelude::*,
    //renderer is used to display a window
    renderer::{
        Camera,
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    //needed for application_root_dir() etc
    utils::application_root_dir,
};

pub const ARENA_HEIGHT: f32 = 100.0;
pub const ARENA_WIDTH: f32 = 100.0;

pub const LIFEFORM_HEIGHT: f32 = 16.0;
pub const LIFEFORM_WIDTH: f32 = 4.0;

struct LifeForm {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl LifeForm {
    fn new() -> LifeForm {
        LifeForm {
            x: 50.0,
            y: 50.0,
            z: 50.0,
        }
    }
}

// By implementing Component for the LifeForm struct, it can now be attached to entities in the game
impl Component for LifeForm {
    type Storage = DenseVecStorage<Self>;
}

fn initialise_lifeforms(world: &mut World) {
    let mut transform = Transform::default();

    // Correctly position the life form.
    let y = ARENA_HEIGHT / 2.0;
    transform.set_translation_xyz(LIFEFORM_WIDTH * 0.5, y, 0.0);

    // Create a life form entity.
    world
        .create_entity()
        .with(LifeForm::new())
        .with(transform)
        .build();
}

struct GameplayState {
    lifeforms: u8,
}

fn initialise_camera(world: &mut World) {
    // Setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom left. 
    let mut transform = Transform::default();
    //initial camera position
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .with(transform)
        .build();
}

//GameData is the internal shared data between states
impl SimpleState for GameplayState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        println!("Number of lifeforms: {}", self.lifeforms);
        let world = data.world;

        world.register::<LifeForm>();
        initialise_lifeforms(world);

        initialise_camera(world);
    }
}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());
    
    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");

    let game_data = GameDataBuilder::default()
    .with_bundle(
        RenderingBundle::<DefaultBackend>::new()
            // The RenderToWindow plugin provides all the scaffolding for opening a window and drawing on it
            .with_plugin(
                RenderToWindow::from_config_path(display_config_path)
                    .with_clear([0.0, 0.0, 0.0, 1.0]),
            )
            // RenderFlat2D plugin is used to render entities with a `SpriteRender` component.
            .with_plugin(RenderFlat2D::default()),
    )?
    // Add the transform bundle which handles tracking entity positions
    // TODO: The manual says to add this instead of running world.register::<LifeForm>(); inside impl SimpleState for GameplayState
    // However this doesnt seem to work as described, maybe try removing it?
    .with_bundle(TransformBundle::new())?;

    let assets_dir = app_root.join("assets");
    let mut game = Application::new(assets_dir, GameplayState{lifeforms: 0}, game_data)?;
    game.run();

    Ok(())
}
