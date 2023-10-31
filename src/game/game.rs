use super::parser::Parser;
use super::{Action, GameObject, Handled, Location, Mediator, NotifyAction};
use std::collections::HashMap;

// TODO: do I need a context for the action?
// Can I take an object from the cupboard, if I'm not standing in the kicthen?
// Is the book in the bookshelf, or in the library?

#[derive(Default)]
pub struct GameAtlas {
    atlas: HashMap<String, Box<dyn GameObject>>,
}

impl GameAtlas {
    pub fn new() -> Self {
        Self {
            atlas: HashMap::new(),
        }
    }

    pub fn add_all(&mut self, objects: Vec<Box<dyn GameObject>>) {
        for o in objects {
            self.add(o);
        }
    }

    pub fn add(&mut self, object: Box<dyn GameObject>) {
        if self.atlas.contains_key(&object.name()) {
            println!("cannot add duplicate object: '{}'", object.name());
            return;
        }
        self.atlas.insert(object.name(), object);
    }

    pub fn get(&self, name: &String) -> Option<&'_ Box<dyn GameObject>> {
        self.atlas.get(name)
    }

    pub fn set_loc(&mut self, name: &String, location: String) -> bool {
        if let Some(o) = self.atlas.get_mut(name) {
            o.set_loc(location.to_string());
            return true;
        }
        false
    }

    pub fn get_locals(&self, here: String) -> Vec<&'_ Box<dyn GameObject>> {
        self.atlas
            .iter()
            .filter_map(|(k, v)| {
                if v.loc() == here || *k == here {
                    Some(v)
                } else {
                    None
                }
            })
            .collect()
    }
}

#[derive(Default)]
pub struct GameContext<'a> {
    here: String,                                     // current location
    locals: HashMap<String, &'a Box<dyn GameObject>>, // objects in current location
    inv: HashMap<String, &'a Box<dyn GameObject>>,    // objects carried to next location
}

impl<'a> GameContext<'a> {
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

    pub fn locals(&self) -> &HashMap<String, &'a Box<dyn GameObject>> {
        &self.locals
    }

    pub fn add_local(&mut self, object: &'a Box<dyn GameObject>) {
        if let Some(o) = self.inv.remove(&object.name()) {
            println!("** {} removed from {}", o.name(), o.loc());
        }
        self.locals.insert(object.name(), object);
    }

    pub fn inv(&self) -> &HashMap<String, &'a Box<dyn GameObject>> {
        &self.inv
    }

    pub fn add_inv(&mut self, object: &'a Box<dyn GameObject>) {
        if let Some(o) = self.locals.remove(&object.name()) {
            println!("** {} removed from {}", o.name(), o.loc());
        }
        self.inv.insert(object.name(), object);
    }
}

pub struct Game<'a> {
    atlas: GameAtlas, // all objects in game
    context: Box<GameContext<'a>>,
}

impl<'a> Mediator<'a> for Game<'a> {
    fn notify(&'a mut self, action: NotifyAction) -> Handled {
        match action {
            NotifyAction::Set(location) => match location {
                Location::To(name) => {
                    self.next_context(name);
                    return true;
                }
                _ => {
                    return false;
                }
            },

            NotifyAction::Move(object_name, location) => {
                self.move_object(object_name, location);
                return true;
            }
        }
    }
}

impl<'a> Game<'a> {
    pub fn new(here: String, atlas: GameAtlas) -> Self {
        Self {
            atlas,
            context: Box::new(GameContext::new(here)),
        }
    }

    pub fn next_context<'me>(&'me mut self, name: String)
    where
        'me: 'a,
    {
        let mut context = GameContext::new(name.clone());
        self.context.inv().into_iter().for_each(|(_k, v)| {
            context.add_inv(*v);
        });
        self.atlas.get_locals(name).into_iter().for_each(|o| {
            context.add_local(o);
        });
        self.context = Box::new(context);
    }

    fn move_object<'me>(&'me mut self, object_name: String, location: Location) -> bool
    where
        'me: 'a,
    {
        match location {
            Location::Local => {
                if self.atlas.set_loc(&object_name, self.context.here()) {
                    if let Some(o) = self.atlas.get(&object_name) {
                        self.context.add_local(o);
                        return true;
                    }
                }
            }
            Location::Inventory => {
                if self.atlas.set_loc(&object_name, "inv".to_string()) {
                    if let Some(o) = self.atlas.get(&object_name) {
                        self.context.add_inv(o);
                        return true;
                    }
                }
            }
            Location::To(name) => {
                return self.atlas.set_loc(&object_name, name.to_string());
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
