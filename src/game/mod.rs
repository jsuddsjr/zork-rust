pub mod game;
pub mod objects;
pub mod parser;
pub mod title;

pub use game::Game;
pub use objects::forest::{Forest, Key, Leaves};
pub use objects::kitchen::{Kitchen, Knife};

use self::game::GameAtlas;
use self::objects::kitchen::{Bread, BreadBox};
pub type Handled = bool;

pub enum Direction {
    North,
    South,
    East,
    West,
    Up,
    Down,
    Exit, // exit the room in the direction you came from
    Enter,
}

// Actions are created by the parser.
pub enum Action {
    Go(Direction), // handled by "here" object, which calls SetLocation on mediator if successful.

    // Actions that use "here" object if not specified.
    Climb(Option<String>),
    Describe(Option<String>),
    Examine(Option<String>),
    Follow(Option<String>),
    Listen(Option<String>),
    Take(Option<String>),

    // Actions with optional indirect object.
    // If Indirect object is not specified, the game will choose one.
    Attack(String, Option<String>),
    Drop(String, Option<String>),
    Light(String, Option<String>),
    Open(String, Option<String>),
    Read(String, Option<String>),
    Say(String, Option<String>), // say something to an object
    Use(String, Option<String>),

    Die,
    Help,
    Inventory,
    Wait,
    Quit,

    // TDOO: get hints from the game.
    // Hint,

    // These events are sent when player moves between locations. (Not commands.)
    Arrive(String),
    Leave(String),

    // Error actions handled by game.
    UnknownAction(String),
    UnknownObject(String),
    UnknownDirection(String),
    MissingTarget(String),

    // Select from multiple objects
    AmbiguousObject(Vec<String>),
}

pub enum Location {
    Local,
    Inventory,
    To(&'static str),
}

pub enum NotifyAction {
    Set(Location),                // game location
    Move(&'static str, Location), // object name, new location
}

// Mediator has notification methods.
pub trait Mediator {
    fn notify(&'static mut self, action: NotifyAction) -> Handled;
}

#[allow(unused_variables)]
pub trait GameObject {
    fn name(&self) -> String;
    fn loc(&self) -> String {
        String::from("nowhere")
    }
    fn set_loc(&mut self, loc: String) {}
    fn act(&mut self, mediator: &'static mut dyn Mediator, action: Action) -> Handled;
    fn can_do(&self, action: &Action) -> bool {
        false
    }
}

pub fn create_game() -> Box<Game> {
    let mut atlas = GameAtlas::new();
    atlas.add(Forest::default());
    atlas.add(Leaves::default());
    atlas.add(Key::new());
    atlas.add(Kitchen::default());
    atlas.add(Knife::new());
    atlas.add(Bread::new());
    atlas.add(BreadBox::new());

    let loc: String = Forest::default().name();
    let game = Box::new(Game::new(loc, atlas));
    game
}
