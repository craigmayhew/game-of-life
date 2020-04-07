extern crate amethyst;
use amethyst::{
    assets::{AssetLoaderSystemData, AssetStorage, Loader, Handle},
    core::ArcThreadPool,
    core::math::Vector3,
    core::transform::{Transform, TransformBundle},
    //Component is used to attach structs to entities in the game
    ecs::prelude::{Component, DenseVecStorage, Dispatcher},
    input::{InputBundle, StringBindings},
    prelude::*,
    //renderer is used to display a window
    renderer::{
        Camera,
        formats::mesh::ObjFormat,
        //needed for sprites
        ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture,
        //
        plugins::{RenderFlat2D, RenderFlat3D, RenderToWindow},
        types::DefaultBackend,
        palette::rgb::LinSrgba,
        RenderingBundle,
        rendy::{
            texture::palette::load_from_linear_rgba,
        },
        Material,
        MaterialDefaults,
        Mesh,
    },
    shred::{DispatcherBuilder},
    //needed for application_root_dir() etc
    utils::application_root_dir,
};

mod systems;
use crate::systems::camera_movement;

pub const ARENA_HEIGHT: f32 = 1000.0;
pub const ARENA_WIDTH: f32 = 1000.0;

pub const LIFEFORM_HEIGHT: f32 = 1.0;
pub const LIFEFORM_WIDTH: f32 = 1.0;

struct LifeForm {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

//todo: is this needed? looks like it was for x,y,z but do we need it now?!
impl LifeForm {
    fn new() -> LifeForm {
        LifeForm {
            x: 5.0,
            y: 5.0,
            z: 5.0,
        }
    }
}

// By implementing Component for the LifeForm struct, it can now be attached to entities in the game
impl Component for LifeForm {
    type Storage = DenseVecStorage<Self>;
}

fn initialise_lifeforms(world: &mut World, sprite_sheet: Handle<SpriteSheet>) {
    //// 3d tetra
     
    //loading tetra mesh 
    let mesh_tetra = world.exec(|loader: AssetLoaderSystemData<'_, Mesh> | {
        loader.load("mesh/tetra.obj", ObjFormat, ())
    });

    //creating a texture
    let yellow = world.exec(|loader: AssetLoaderSystemData<Texture> | {
        loader.load_from_data(
            load_from_linear_rgba(LinSrgba::new(1.0, 1.0, 0.0, 1.0)).into(),
            (),
        )
    });

    //load material
    let default_material = world.read_resource::<MaterialDefaults>().0.clone();

    let mat_yellow = world.exec(|loader: AssetLoaderSystemData<Material> | {
        loader.load_from_data(
            Material {
                albedo: yellow,
                ..default_material.clone()
            },
            (),
        )
    });

    let mut transform = Transform::default();

    //set size of tetrahedrons
    let scale = Vector3::new(150.0, 150.0, 150.0);
    transform.set_scale(scale);

    //render some tetrahedrons!
    for x in 1..5 {
        for y in 1..5 {
            for z in 1..5 {
                transform.set_translation_xyz(150.0 * x as f32, 150.0 * y as f32, 150.0 * z as f32);
                // Create a life form entity.
                world
                    .create_entity()
                    .with(mesh_tetra.clone())
                    .with(mat_yellow.clone())
                    .with(LifeForm::new())//todo this line maybe superflous
                    .with(transform.clone())
                    .build();
            }
        }
    }

    //// 2d square
    let mut transform = Transform::default();

    // Correctly position the life form.
    let x = ARENA_WIDTH / 2.0;
    let y = ARENA_HEIGHT / 2.0;
    transform.set_translation_xyz(x, y, -100.0);
    transform.set_rotation_x_axis(0.2);
    transform.set_rotation_y_axis(0.2);
    transform.set_rotation_z_axis(0.2);

    // Assign the sprites for the lifeform
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 0, // lifeform is the first sprite in the sprite_sheet
    };

    // Create a life form entity.
    world
        .create_entity()
        .with(sprite_render.clone())
        .with(LifeForm::new())//todo this line maybe superflous
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
            "texture/lifeform_sprite.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/lifeform_sprite.ron", // Here we load the associated ron file
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}

struct GameplayState {
    pub dispatcher: Dispatcher<'static, 'static>,
    lifeforms: u8,
}

fn initialise_camera(world: &mut World) {
    // Setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom left. 
    let mut transform = Transform::default();
    //initial camera position
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 200.0);

    world
        .create_entity()
        .named("Main camera")
        .with(Camera::standard_3d(ARENA_WIDTH, ARENA_HEIGHT))
        .with(transform)
        .build();
}

//GameData is the internal shared data between states
impl SimpleState for GameplayState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        println!("Number of lifeforms: {}", self.lifeforms);
        let mut world = data.world;

        // Load the spritesheet necessary to render the graphics.
        let sprite_sheet_handle = load_sprite_sheet(world);

        world.register::<LifeForm>();
        initialise_lifeforms(world, sprite_sheet_handle);

        initialise_camera(world);

        let dispatcher = DispatcherBuilder::new()
        .with_pool((*world.read_resource::<ArcThreadPool>()).clone())
        .with(
            camera_movement::CameraMovementSystem::default(),
            "camera_movement",
            &[],
        )
        .build();

        self.dispatcher = dispatcher;

        self.dispatcher.setup(&mut world);
    }

    fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans {
        //println!("update SimpleState");

        self.dispatcher.dispatch(&data.world);
        
        //data.data.update(&data.world);
        
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

    let game_data = GameDataBuilder::default()
    .with_bundle(
        RenderingBundle::<DefaultBackend>::new()
            // The RenderToWindow plugin provides all the scaffolding for opening a window and drawing on it
            .with_plugin(
                RenderToWindow::from_config_path(display_config_path)?
                    .with_clear([0.0, 0.0, 0.0, 1.0]),
            )
            // RenderFlat2D plugin is used to render entities with a `SpriteRender` component.
            .with_plugin(RenderFlat2D::default())
            // RenderFlat3D plugin is used to render entities with a `SpriteRender` component.
            .with_plugin(RenderFlat3D::default()),
    )?
    .with_bundle(input_bundle)?
    // Add the transform bundle which handles tracking entity positions
    // TODO: The manual says to add this instead of running world.register::<LifeForm>(); inside impl SimpleState for GameplayState
    // However this doesnt seem to work as described, maybe try removing it?
    .with_bundle(TransformBundle::new())?;

    let assets_dir = app_root.join("assets");
    let game_play_start = GameplayState{dispatcher: DispatcherBuilder::new().build(), lifeforms: 0};
    let mut game = Application::new(assets_dir, game_play_start, game_data)?;
    game.run();

    Ok(())
}
