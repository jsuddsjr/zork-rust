use game::{
    objects::{
        forest::{self, FOREST},
        kitchen,
    },
    Game, GameAtlas, GameObject,
};

mod game;

fn main() -> () {
    game::title::print();

    let mut vec = Vec::new() as Vec<Box<dyn GameObject>>;
    forest::create(&mut vec);
    kitchen::create(&mut vec);

    let mut atlas = GameAtlas::new(String::from(FOREST));
    atlas.add_all(vec);

    let mut game = Game::new(atlas);
    game.run();
}
