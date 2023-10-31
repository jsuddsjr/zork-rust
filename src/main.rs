use game::{
    objects::{forest, kitchen},
    Game, GameAtlas, GameObject,
};

mod game;

fn main() -> () {
    game::title::print();

    let mut vec = Vec::new() as Vec<Box<dyn GameObject>>;
    forest::create(&mut vec);
    kitchen::create(&mut vec);

    let mut atlas = GameAtlas::new();
    atlas.add_all(vec);

    let loc: String = forest::Forest::default().name();
    let game = Game::new(loc, atlas);
    game.run();
}
