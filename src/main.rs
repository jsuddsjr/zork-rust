use std::io::Error;

mod game;

fn main() -> Result<(), Error> {
    game::title::print();
    let game = game::create_game();
    game.run();
    Ok(())
}
