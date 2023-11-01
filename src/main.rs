use game::{
    objects::{forest, kitchen},
    Game, GameAtlas, GameObject,
};

mod game;

fn main() -> () {
    game::title::print();

    let loc: String = forest::Forest::default().name();
    let mut vec = Vec::new() as Vec<Box<dyn GameObject>>;
    forest::create(&mut vec);
    kitchen::create(&mut vec);

    let mut atlas = GameAtlas::new(loc.clone());
    atlas.add_all(vec);

    let mut game = Game::new(atlas);
    game.run();
}
