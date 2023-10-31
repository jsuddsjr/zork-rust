use game::{
    objects::forest::{Forest, Key, Leaves},
    objects::kitchen::{Bread, BreadBox, Kitchen, Knife, Sink},
    Game, GameAtlas, GameObject,
};

mod game;

fn main() -> () {
    game::title::print();

    let mut vec = Vec::new() as Vec<Box<dyn GameObject>>;
    vec.push(Box::new(Forest::default()));
    vec.push(Box::new(Leaves::default()));
    vec.push(Box::new(Key::new()));
    vec.push(Box::new(Kitchen::default()));
    vec.push(Box::new(Knife::new()));
    vec.push(Box::new(Sink::new()));
    vec.push(Box::new(BreadBox::new()));
    vec.push(Box::new(Bread::new()));

    let mut atlas = GameAtlas::new();
    atlas.add_all(vec);

    let loc: String = Forest::default().name();
    let game = Game::new(loc, atlas);
    game.run();
}
