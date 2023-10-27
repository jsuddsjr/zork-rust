use std::io::Error;
mod game;
mod title;

pub fn create_game() -> game::Game {
    game::Game::new()
}

fn main() -> Result<(), Error> {
    title::print();
    let mut game = create_game();
    game.run();
    Ok(())
}
