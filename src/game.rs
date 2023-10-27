use std::collections::HashMap;
use std::io::Write;

pub type Handled = bool;

pub enum Direction {
    North,
    South,
    East,
    West,
    Up,
    Down,
}

pub enum Action {
    Move(Box<dyn Mediated>, Direction),

    Arrive(Box<dyn Mediated>),
    Examine(Box<dyn Mediated>),
    Follow(Box<dyn Mediated>),
    Leave(Box<dyn Mediated>),
    Look(Box<dyn Mediated>),
    Read(Box<dyn Mediated>),
    Take(Box<dyn Mediated>),

    Attack(Box<dyn Mediated>, Option<Box<dyn Mediated>>),
    Drop(Box<dyn Mediated>, Option<Box<dyn Mediated>>),
    Light(Box<dyn Mediated>, Option<Box<dyn Mediated>>),
    Open(Box<dyn Mediated>, Option<Box<dyn Mediated>>),
    Use(Box<dyn Mediated>, Option<Box<dyn Mediated>>),

    Die,
    Help,
    Inventory,
    Quit,
    Unknown,
}

pub enum NotifyAction {
    SetLocation(&'static str),
    MoveObject(&'static str, &'static str),
    RemoveObject(&'static str),
}

// Mediator has notification methods.
pub trait Mediator {
    fn notify_action(&mut self, action: NotifyAction) -> Handled;
}

// Mediated has methods called by Mediator.
pub trait Mediated {
    fn name(&self) -> String;
    fn loc(&self) -> String {
        "global".into()
    }
    fn set_loc(&mut self, _loc: &'static str) {}
    fn do_action(&mut self, mediator: &'static mut dyn Mediator, action: Action) -> Handled;
}

#[derive(Default)]
pub struct Game {
    objects: HashMap<String, Box<dyn Mediated>>, // all objects
    here_objects: Vec<Box<dyn Mediated>>,        // objects in current location
    here: Option<Box<dyn Mediated>>,             // current location
}

impl Mediator for Game {
    fn notify_action(&mut self, action: NotifyAction) -> Handled {
        match action {
            NotifyAction::SetLocation(name) => {
                if let Some(value) = self.validate_loc(name) {
                    return value;
                }

                self.here_objects.clear();
                self.here.replace(self.objects.get(name).unwrap().clone());
                for o in self.objects.values() {
                    if o.loc() == name {
                        self.here_objects.push(o);
                    }
                }
                return true;
            }

            NotifyAction::MoveObject(object_name, new_loc) => {
                if let Some(value) = self.validate_loc(new_loc) {
                    return value;
                }
                if self.objects.contains_key(object_name) {
                    let mut o = self.objects.get(object_name).unwrap().clone();
                    o.set_loc(new_loc);
                    if new_loc == self.here.unwrap().loc() {
                        self.here_objects.push(o);
                    }
                    return true;
                }
            }

            NotifyAction::RemoveObject(object_name) => {
                if let Some(_) = self.objects.remove(object_name) {
                    return true;
                }
            }
        }
        false
    }
}

impl Game {
    pub fn add(&mut self, object: impl Mediated + 'static) {
        if self.objects.contains_key(object.name().as_str()) {
            println!("cannot add duplicate object: '{}'", object.name());
            return;
        }
        self.objects.insert(object.name(), Box::new(object));
    }

    pub fn print_death(&self) {
        println!("**That would lead to your untimely demise.**\n\nTry again?");
    }

    pub fn print_help(&self) {
        println!("Try these commands:\nLOOK\nMOVE\nTAKE\nDROP\nINV\nQUIT");
    }

    pub fn print_location(&self) {
        println!("You are in {}", self.here.unwrap().name());
        println!("You see:");
        for o in &self.here_objects {
            println!("  {}", o.name());
        }
    }

    //Parser reads vector of tokens from stdin and returns Tuple(PRSA, PRSO, PRSI).
    //PRSA is the action, PRSO direct object, and PRSI indirect object of the typed command.
    pub fn input_command(&self) -> (String, Option<String>, Option<String>) {
        print!("\naction (object) (indirect)> ");
        std::io::stdout().flush().ok();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        let mut tokens = Vec::new();
        let lower = input.to_lowercase();
        for token in lower.split_whitespace() {
            if token == "the"
                || token == "a"
                || token == "an"
                || token == "with"
                || token == "on"
                || token == "from"
                || token == "into"
                || token == "to"
                || token == "at"
                || token == "of"
            {
                continue;
            }
            tokens.push(token);
        }
        if tokens.len() == 1 {
            return (tokens[0].to_string(), None, None);
        } else if tokens.len() == 2 {
            return (tokens[0].to_string(), Some(tokens[1].to_string()), None);
        } else if tokens.len() == 3 {
            return (
                tokens[0].to_string(),
                Some(tokens[1].to_string()),
                Some(tokens[2].to_string()),
            );
        } else {
            println!("I don't understand that.");
        }
        ("help".to_string(), None, None)
    }

    fn parse_command(&self, command: (String, Option<String>, Option<String>)) -> Action {
        let (prsa, _prso, _prsi) = command;
        match prsa.as_str() {
            "help" => return Action::Help,
            "quit" => return Action::Quit,
            &_ => return Action::Unknown,
        };
    }

    fn validate_loc(&self, name: &str) -> Option<bool> {
        if (self.here.is_some() && self.here.unwrap().name() == name)
            || !self.objects.contains_key(name)
        {
            return Some(false);
        }
        None
    }

    pub fn run(&self) {
        // if self.here.is_none() {
        //     println!("No location set. Use 'move' first.");
        //     return;
        // }

        loop {
            let action = self.parse_command(self.input_command());
            match action {
                Action::Die => self.print_death(),
                Action::Help => self.print_help(),
                Action::Quit => break,
                _ => println!("I didn't understand that. Have you tried 'help'?"),
            }
        }
    }
}
