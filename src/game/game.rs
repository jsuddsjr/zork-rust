use super::parser::Parser;
use super::{Action, GameObject, Handled, Location, Mediator, NotifyAction};
use std::collections::HashMap;

// TODO: do I need a context for the action?
// Can I take an object from the cupboard, if I'm not standing in the kicthen?
// Is the book in the bookshelf, or in the library?

#[derive(Default)]
pub struct GameAtlas {
    atlas: HashMap<String, Box<dyn GameObject + 'static>>,
}

impl GameAtlas {
    pub fn new() -> Self {
        Self {
            atlas: HashMap::new(),
        }
    }

    pub fn add(&mut self, object: impl GameObject + 'static) {
        if self.atlas.contains_key(&object.name()) {
            println!("cannot add duplicate object: '{}'", object.name());
            return;
        }
        self.atlas.insert(object.name(), Box::new(object));
    }

    pub fn get(&self, name: &str) -> Option<&Box<dyn GameObject>> {
        self.atlas.get(name).map(|o| o)
    }

    pub fn values(&self) -> Vec<&Box<dyn GameObject>> {
        self.atlas.values().map(|o| o).collect()
    }
}

#[derive(Default)]
pub struct GameContext {
    here: String,                                     // current location
    pub locals: HashMap<String, Box<dyn GameObject>>, // objects in current location
    pub inv: HashMap<String, Box<dyn GameObject>>,    // objects carried to next location
}

impl GameContext {
    pub fn new(here: String) -> Self {
        Self {
            here,
            locals: HashMap::new(),
            inv: HashMap::new(),
        }
    }

    pub fn here(&self) -> String {
        self.here.clone()
    }
}

pub struct Game {
    atlas: Box<GameAtlas>, // all objects in game
    context: Box<GameContext>,
}

impl Mediator for Game {
    fn notify(&'static mut self, action: NotifyAction) -> Handled {
        match action {
            NotifyAction::Set(location) => match location {
                Location::To(name) => {
                    self.next_context(name.to_string());
                    return true;
                }
                _ => {
                    return false;
                }
            },

            NotifyAction::Move(object_name, location) => {
                return self.move_object(object_name.to_string(), location);
            }
        }
    }
}

impl Game {
    pub fn new(here: String, atlas: GameAtlas) -> Self {
        Self {
            atlas: Box::new(atlas),
            context: Box::new(GameContext::new(here)),
        }
    }

    pub fn next_context(&mut self, name: String) {
        let mut context = Box::new(GameContext::new(name.clone()));
        if let Some(o) = self.atlas.get(&name) {
            // Bring inventory into "here" scope.
            context.locals.clone_from(&self.context.inv);
            // Bring objects into "here" scope.
            context
                .locals
                .extend(self.atlas.values().into_iter().filter_map(|o| {
                    if o.loc() == name {
                        Some((o.name(), *o))
                    } else {
                        None
                    }
                }));
            // Alias.
            context.inv = self.context.inv;
        }
        self.context = context;
    }

    fn move_object(&'static mut self, object_name: String, location: Location) -> bool {
        if let Some(o) = self.atlas.get(&object_name) {
            match location {
                Location::Local => {
                    o.set_loc(self.context.here());
                    self.context.locals.insert(o.name(), *o);
                    return true;
                }
                Location::Inventory => {
                    o.set_loc("inv".to_string());
                    self.context.inv.insert(o.name(), *o);
                    return true;
                }
                Location::To(name) => {
                    o.set_loc(name.to_string());
                    return true;
                }
            }
        }
        false
    }

    pub fn print_death(&self) {
        println!("**That would lead to your untimely demise.**\n\nTry again?");
    }

    pub fn print_help(&self) {
        println!("Try these commands:\nLOOK\nMOVE\nTAKE\nDROP\nINV\nQUIT");
    }

    pub fn print_location(&self) {
        println!("You are in {}", self.context.here);
        println!("You see:");
        for o in self.context.locals.values() {
            println!("  {}", o.name());
        }
    }

    pub fn run(&self) {
        // if self.context.here.is_none() {
        //     println!("No location set. Use 'move' first.");
        //     return;
        // }

        let parser = Parser::default();

        loop {
            let action = parser.input_action(&self.context);
            match action {
                Action::Die => self.print_death(),
                Action::Help => self.print_help(),
                Action::Quit => break,
                _ => println!("I didn't understand that. Have you tried 'help'?"),
            }
        }
    }
}
