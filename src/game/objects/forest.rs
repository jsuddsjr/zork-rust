use crate::game::{objects::kitchen::KITCHEN, Action, Direction, GameObject, Location, Notify};

pub fn create(vec: &mut Vec<Box<dyn GameObject>>) {
    vec.push(Box::new(Forest::default()));
    vec.push(Box::new(Leaves::new()));
    vec.push(Box::new(Key::new()));
}

pub static FOREST: &str = "forest";
pub static LEAVES: &str = "leaves";
pub static KEY: &str = "key";

#[derive(Default)]
pub struct Forest;

impl GameObject for Forest {
    fn name(&self) -> String {
        FOREST.to_string()
    }

    fn act(&mut self, action: Action) -> Notify {
        match action {
            Action::Go(Direction::North) | Action::Go(Direction::Exit) => {
                println!("You follow the path north.");
                Notify::Set(Location::To(KITCHEN.to_string()))
            }
            Action::Describe(_) => {
                println!("You find yourself standing in a forest clearing, surrounded by trees. There is a path to the north.");
                Notify::Handled
            }
            Action::Arrive(_) => {
                println!("The fog clears...");
                Notify::Handled
            }
            Action::Leave(_) => {
                println!("The peaceful rustling leaves recede into the distance...");
                Notify::Handled
            }
            Action::Examine(_) => {
                println!("One of the trees nearby has been carved with the inscription: C+J. You wonder what it means.");
                Notify::Handled
            }
            _ => Notify::Unhandled,
        }
    }
}

#[derive(Default)]
pub struct Leaves {
    contains_key: bool,
}

impl Leaves {
    pub fn new() -> Self {
        Self { contains_key: true }
    }
}

impl GameObject for Leaves {
    fn name(&self) -> String {
        LEAVES.to_string()
    }

    fn loc(&self) -> String {
        FOREST.to_string()
    }

    fn can_do(&self, action: &Action) -> bool {
        match action {
            Action::Describe(_) => true,
            Action::Attack(_, _) => true,
            Action::Take(_) => true,
            _ => false,
        }
    }

    fn act(&mut self, action: Action) -> Notify {
        match action {
            Action::Describe(_) => {
                println!("There's a pile of leaves here.");
                Notify::Handled
            }
            Action::Attack(_, _) => {
                println!("The leaves flutter and fly as you kick through them.");
                if self.contains_key {
                    self.contains_key = false;
                    Notify::Move(KEY.to_string(), Location::Local)
                } else {
                    Notify::Handled
                }
            }
            Action::Take(_) => {
                println!("You take a handful of leaves and throw them in the air. Feel better?");
                Notify::Handled
            }
            _ => Notify::Unhandled,
        }
    }
}

#[derive(Default)]
pub struct Key {
    loc: String,
}

impl Key {
    pub fn new() -> Self {
        Self {
            loc: LEAVES.to_string(),
        }
    }
}

impl GameObject for Key {
    fn name(&self) -> String {
        KEY.to_string()
    }

    fn loc(&self) -> String {
        self.loc.clone()
    }

    fn set_loc(&mut self, loc: String) {
        self.loc = loc;
    }

    fn can_do(&self, action: &Action) -> bool {
        match action {
            Action::Describe(_) => true,
            Action::Take(_) => true,
            _ => false,
        }
    }

    fn act(&mut self, action: Action) -> Notify {
        match action {
            Action::Describe(_) => {
                println!("A shiny key glints in the grass.");
                Notify::Handled
            }
            Action::Take(_) => {
                println!("You take the key.");
                Notify::Move(self.name(), Location::Inventory)
            }
            _ => Notify::Unhandled,
        }
    }
}
