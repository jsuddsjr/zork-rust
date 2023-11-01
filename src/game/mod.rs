pub mod game;
pub mod objects;
pub mod parser;
pub mod title;

pub use game::{Game, GameAtlas, GameContext, NOWHERE};
pub use objects::forest::{Forest, Key, Leaves};
pub use objects::kitchen::{Bread, BreadBox, Kitchen, Knife, Sink};
pub use parser::Token;

pub type Handled = bool;

#[derive(Clone, Debug, PartialEq)]

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

#[derive(Clone, Debug, PartialEq)]
#[allow(dead_code)]
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
    To(String),
}

pub enum Notify {
    Handled,                 // handled, don't do anything else
    Unhandled,               // not handled, try other objects
    Set(Location),           // update game location
    Move(String, Location),  // object name, new location
    Replace(String, String), // old object name, new object name, same location
}

#[allow(unused_variables)]
pub trait GameObject {
    /// Get the name of this object. Required.
    fn name(&self) -> String;

    /// Get the location of this object. Default is NOWHERE.
    fn loc(&self) -> String {
        String::from(NOWHERE)
    }

    /// Set a new location. Default is to do nothing.
    fn set_loc(&mut self, loc: String) {}

    /// Can this object do this action? Default is true.
    fn can_do(&self, action: &Action) -> bool {
        true
    }

    /// Handle an action. Default is unhandled.
    fn act(&mut self, action: Action) -> Notify {
        Notify::Unhandled
    }
}
