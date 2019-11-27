extern crate amethyst;
use amethyst::{
    prelude::*,
    /*renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },*/
    //needed for application_root_dir() etc
    utils::application_root_dir,
};

struct GameplayState {
    lifeforms: u8,
}

//GameData is the internal shared data between states
impl SimpleState for GameplayState {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        println!("Number of lifeforms: {}", self.lifeforms);
    }
}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());
    
    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");

    let game_data = GameDataBuilder::default();

    let assets_dir = app_root.join("assets");
    let mut world = World::new();
    let mut game = Application::new(assets_dir, GameplayState{lifeforms: 0}, game_data)?;
    game.run();

    Ok(())
}
