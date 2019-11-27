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
    /// The `State`-local data. Usually you will not have anything.
    /// In this case, we have the number of players here.
    player_count: u8,
}

//GameData is the internal shared data between states
impl State<GameData<'static, 'static>, ()> for GameplayState {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        println!("Number of players: {}", self.player_count);
    }
}

fn main() -> amethyst::Result<()> {
    //amethyst::start_logger(Default::default());
    
    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");

    Ok(())
}
