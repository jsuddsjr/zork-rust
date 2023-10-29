use game::{Forest, Game, GameAtlas, GameObject, Kitchen};

mod game;

fn main() -> () {
    game::title::print();

    let mut atlas = GameAtlas::new();
    atlas.add(Forest::default());
    atlas.add(Kitchen::default());

    let loc: String = Forest::default().name();
    let game = Game::new(loc, atlas);
    game.run();
}
