pub mod atlas;
pub mod context;
pub mod game;
pub mod objects;
pub mod parser;
pub mod title;

pub use atlas::{GameAtlas, INVENTORY, NOWHERE};
pub use context::GameContext;
pub use game::Game;
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

#[allow(dead_code)]
impl Action {
    pub fn get_object_or_here(&self, here: &str) -> String {
        match self.get_object() {
            Some(obj) => obj,
            None => here.to_string(),
        }
    }

    pub fn get_direction(&self) -> Option<Direction> {
        match self {
            Action::Go(dir) => Some(dir.clone()),
            _ => None,
        }
    }

    pub fn get_object(&self) -> Option<String> {
        match self {
            Action::Attack(obj, _)
            | Action::Drop(obj, _)
            | Action::Light(obj, _)
            | Action::Open(obj, _)
            | Action::Read(obj, _)
            | Action::Say(obj, _)
            | Action::Use(obj, _) => {
                if obj.as_str() != "" {
                    Some(obj.clone())
                } else {
                    None
                }
            }
            Action::Describe(obj)
            | Action::Climb(obj)
            | Action::Examine(obj)
            | Action::Follow(obj)
            | Action::Listen(obj)
            | Action::Take(obj) => obj.clone(),
            _ => None,
        }
    }

    pub fn set_object(&self, prso: String) -> Action {
        match self {
            Action::Attack(_, obj) => Action::Attack(prso, obj.clone()),
            Action::Drop(_, obj) => Action::Drop(prso, obj.clone()),
            Action::Light(_, obj) => Action::Light(prso, obj.clone()),
            Action::Open(_, obj) => Action::Open(prso, obj.clone()),
            Action::Read(_, obj) => Action::Read(prso, obj.clone()),
            Action::Say(_, obj) => Action::Say(prso, obj.clone()),
            Action::Use(_, obj) => Action::Use(prso, obj.clone()),
            Action::Describe(_) => Action::Describe(Some(prso)),
            Action::Climb(_) => Action::Climb(Some(prso)),
            Action::Examine(_) => Action::Examine(Some(prso)),
            Action::Follow(_) => Action::Follow(Some(prso)),
            Action::Listen(_) => Action::Listen(Some(prso)),
            Action::Take(_) => Action::Take(Some(prso)),
            _ => self.clone(),
        }
    }

    pub fn get_indirect_object(&self) -> Option<String> {
        match self {
            Action::Attack(_, obj)
            | Action::Drop(_, obj)
            | Action::Light(_, obj)
            | Action::Open(_, obj)
            | Action::Read(_, obj)
            | Action::Say(_, obj)
            | Action::Use(_, obj) => obj.clone(),
            _ => None,
        }
    }

    pub fn set_indirect_object(&self, prsi: String) -> Action {
        match self {
            Action::Attack(obj, _) => Action::Attack(obj.clone(), Some(prsi)),
            Action::Drop(obj, _) => Action::Drop(obj.clone(), Some(prsi)),
            Action::Light(obj, _) => Action::Light(obj.clone(), Some(prsi)),
            Action::Open(obj, _) => Action::Open(obj.clone(), Some(prsi)),
            Action::Read(obj, _) => Action::Read(obj.clone(), Some(prsi)),
            Action::Say(obj, _) => Action::Say(obj.clone(), Some(prsi)),
            Action::Use(obj, _) => Action::Use(obj.clone(), Some(prsi)),
            _ => self.clone(),
        }
    }

    fn is_error(&self) -> bool {
        match self {
            Action::UnknownAction(_) => true,
            Action::UnknownObject(_) => true,
            Action::UnknownDirection(_) => true,
            Action::MissingTarget(_) => true,
            Action::AmbiguousObject(_) => true,
            _ => false,
        }
    }

    fn print(&self) {
        println!("Action: {:?}", self)
    }

    fn unpack_action(&self) -> (String, Option<String>, Option<String>) {
        match self {
            Action::Go(d) => match d {
                Direction::North => (String::from("go"), Some("north".to_string()), None),
                Direction::South => (String::from("go"), Some("south".to_string()), None),
                Direction::East => (String::from("go"), Some("east".to_string()), None),
                Direction::West => (String::from("go"), Some("west".to_string()), None),
                Direction::Up => (String::from("go"), Some("up".to_string()), None),
                Direction::Down => (String::from("go"), Some("down".to_string()), None),
                Direction::Exit => (String::from("go"), Some("exit".to_string()), None),
                Direction::Enter => (String::from("go"), Some("enter".to_string()), None),
            },

            Action::Climb(o) => (String::from("climb"), o.clone(), None),
            Action::Describe(o) => (String::from("describe"), o.clone(), None),
            Action::Examine(o) => (String::from("examine"), o.clone(), None),
            Action::Follow(o) => (String::from("follow"), o.clone(), None),
            Action::Listen(o) => (String::from("listen"), o.clone(), None),
            Action::Take(o) => (String::from("take"), o.clone(), None),

            Action::Attack(o, i) => (String::from("attack"), Some(o.clone()), i.clone()),
            Action::Drop(o, i) => (String::from("drop"), Some(o.clone()), i.clone()),
            Action::Light(o, i) => (String::from("light"), Some(o.clone()), i.clone()),
            Action::Open(o, i) => (String::from("open"), Some(o.clone()), i.clone()),
            Action::Read(o, i) => (String::from("read"), Some(o.clone()), i.clone()),
            Action::Say(o, i) => (String::from("say"), Some(o.clone()), i.clone()),
            Action::Use(o, i) => (String::from("use"), Some(o.clone()), i.clone()),

            Action::Die => (String::from("die"), None, None),
            Action::Help => (String::from("help"), None, None),
            Action::Inventory => (String::from("inventory"), None, None),
            Action::Wait => (String::from("wait"), None, None),
            Action::Quit => (String::from("quit"), None, None),

            Action::Arrive(o) => (String::from("arrive"), Some(o.clone()), None),
            Action::Leave(o) => (String::from("leave"), Some(o.clone()), None),

            Action::AmbiguousObject(v) => (
                String::from("ambiguousObj"),
                Some(v.first().unwrap().clone()),
                Some(v.last().unwrap().clone()),
            ),

            Action::MissingTarget(a) => (String::from("missingTarget"), Some(a.clone()), None),
            Action::UnknownAction(a) => (String::from("unknownAction"), Some(a.clone()), None),
            Action::UnknownDirection(d) => (String::from("unknownDir"), Some(d.clone()), None),

            _ => (String::from("other"), None, None),
        }
    }
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
