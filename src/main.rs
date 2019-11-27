extern crate amethyst;
use amethyst::prelude::*;

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

fn main() {
    println!("Hello, world!");
}
