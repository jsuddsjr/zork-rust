pub mod game;
pub mod objects;
pub mod parser;
pub mod title;

pub use game::{Game, GameAtlas, GameContext};
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

pub enum NotifyAction {
    Set(Location),          // game location
    Move(String, Location), // object name, new location
}

// Mediator has notification methods.
pub trait Mediator<'a> {
    fn notify(&'a mut self, action: NotifyAction) -> Handled;
}

#[allow(unused_variables)]
pub trait GameObject {
    fn name(&self) -> String;

    fn loc(&self) -> String {
        String::from("nowhere")
    }

    fn set_loc(&mut self, loc: String) {}

    fn can_do(&self, action: &Action) -> bool {
        false
    }

    fn act(&mut self, action: Action) -> Handled {
        false
    }

    fn act_react<'me, 'a>(
        &'me mut self,
        mediator: &'a mut dyn Mediator<'a>,
        action: Action,
    ) -> Handled
    where
        'me: 'a,
    {
        false
    }
}
