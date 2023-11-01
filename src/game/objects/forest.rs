use crate::game::{Action, Direction, GameObject, Location, Notify};

pub fn create(vec: &mut Vec<Box<dyn GameObject>>) {
    vec.push(Box::new(Forest::default()));
    vec.push(Box::new(Leaves::new()));
    vec.push(Box::new(Key::new()));
}

#[derive(Default)]
pub struct Forest;

impl GameObject for Forest {
    fn name(&self) -> String {
        "forest".to_string()
    }

    fn can_do(&self, action: &Action) -> bool {
        match action {
            Action::Go(Direction::North) => true,
            Action::Describe(_) => true,
            Action::Arrive(_) => true,
            Action::Examine(_) => true,
            _ => false,
        }
    }

    fn act(&mut self, action: Action) -> Notify {
        match action {
            Action::Go(Direction::North) => {
                println!("You follow the path north.");
                Notify::Set(Location::To("kitchen".to_string()))
            }
            Action::Describe(_) => {
                println!("You find yourself standing in a forest clearing, surrounded by trees. There is a path to the north.");
                Notify::Unhandled
            }
            Action::Arrive(_) => {
                println!("The fog clears...");
                Notify::Handled
            }
            Action::Examine(_) => {
                println!("One of the trees nearby has been carved with the inscription: C+J.");
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
        "leaves".to_string()
    }

    fn loc(&self) -> String {
        Forest::default().name()
    }

    fn can_do(&self, action: &Action) -> bool {
        match action {
            Action::Describe(_) => true,
            Action::Attack(_, _) => true,
            _ => false,
        }
    }

    fn act(&mut self, action: Action) -> Notify {
        match action {
            Action::Describe(_) => {
                println!("There's a pile of leaves here.");
                Notify::Unhandled
            }
            Action::Attack(_, _) => {
                if self.contains_key {
                    println!("You uncover a key hidden in the leaves.");
                    self.contains_key = false;
                    Notify::Move("key".to_string(), Location::Local)
                } else {
                    println!("The leaves flutter and fly as you kick through them.");
                    Notify::Handled
                }
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
            loc: Leaves::default().name(),
        }
    }
}

impl GameObject for Key {
    fn name(&self) -> String {
        "key".to_string()
    }

    fn loc(&self) -> String {
        Leaves::default().name()
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
                Notify::Unhandled
            }
            Action::Take(_) => {
                println!("You take the key.");
                Notify::Move(self.name(), Location::Inventory)
            }
            _ => Notify::Unhandled,
        }
    }
}
