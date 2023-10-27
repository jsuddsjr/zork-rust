use std::io::Error;

mod game;
mod objects;
mod title;

use game::{Game, Mediator, NotifyAction};

fn main() -> Result<(), Error> {
    title::print();
    let mut game = Game::default();
    objects::register_objects(&mut game);

    game.notify_action(NotifyAction::SetLocation("forest"));
    // GAME.run();
    Ok(())
}
