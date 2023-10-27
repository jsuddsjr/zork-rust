pub mod forest;

use crate::game::Game;

pub fn register_objects(game: &mut Game) {
    game.add(forest::Forest::default());
    game.add(forest::Leaves::default());
    game.add(forest::Key::new());
}
