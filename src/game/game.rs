use super::parser::Parser;
use super::{Action, GameObject, Handled, Location, Mediator, NotifyAction};
use std::collections::HashMap;

// TODO: do I need a context for the action?
// Can I take an object from the cupboard, if I'm not standing in the kicthen?
// Is the book in the bookshelf, or in the library itself?

static INV: &str = "__inv";
static NOWHERE: &str = "__nowhere";
static GLOBAL: &str = "__global";

#[derive(Default)]
pub struct GameAtlas {
    here: String,
    atlas: HashMap<String, Box<dyn GameObject>>,
}

impl GameAtlas {
    pub fn new(here: String) -> Self {
        Self {
            here,
            atlas: HashMap::new(),
        }
    }

    pub fn here(&self) -> String {
        self.here.clone()
    }

    pub fn set_here(&mut self, here: String) {
        self.here = here;
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
            o.set_loc(location);
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

    pub fn get_inventory(&self) -> Vec<&'_ Box<dyn GameObject>> {
        self.atlas
            .iter()
            .filter_map(|(_k, v)| {
                if v.loc().as_str() == INV {
                    Some(v)
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn get_context(&self) -> GameContext {
        GameContext::new(
            self.here(),
            self.get_locals(self.here()),
            self.get_inventory(),
        )
    }

    pub fn get_context_for(&self, here: String) -> GameContext {
        GameContext::new(
            here.clone(),
            self.get_locals(here.clone()),
            self.get_inventory(),
        )
    }

    pub fn move_inventory(&mut self, object_name: String) -> bool {
        if let Some(o) = self.atlas.get_mut(&object_name) {
            println!("** {} moves from {} to inventory", o.name(), o.loc());
            o.set_loc(INV.to_string());
            return true;
        }
        false
    }

    pub fn move_local(&mut self, object_name: String) -> bool {
        if let Some(o) = self.atlas.get_mut(&object_name) {
            println!("** {} moves from {} to inventory", o.name(), o.loc());
            o.set_loc(INV.to_string());
            return true;
        }
        false
    }

    pub fn _remove_object(&mut self, object_name: String) -> bool {
        if let Some(o) = self.atlas.get_mut(&object_name) {
            println!("** {} from {} consumed", o.name(), o.loc());
            o.set_loc(NOWHERE.to_string());
            return true;
        }
        false
    }

    pub fn invoke<'me, 'a>(
        &'me mut self,
        object_names: Vec<Option<String>>,
        action: Action,
    ) -> Handled
    where
        'me: 'a,
    {
        for name in object_names {
            if name.is_some() {
                if let Some(o) = self.atlas.get_mut(&name.unwrap()) {
                    if o.act(action.clone()) || o.act_react(self, action.clone()) {
                        return true;
                    }
                }
            }
        }
        false
    }
}

impl<'a> Mediator<'a> for GameAtlas {
    fn notify(&'a mut self, action: NotifyAction) -> Handled {
        match action {
            NotifyAction::Set(location) => match location {
                Location::To(name) => {
                    self.here = name;
                    return true;
                }
                _ => {
                    return false;
                }
            },

            NotifyAction::Move(object_name, location) => match location {
                Location::Local => self.move_local(object_name),
                Location::Inventory => self.move_inventory(object_name),
                Location::To(name) => self.set_loc(&object_name, name),
            },
        }
    }
}

#[derive(Default)]
pub struct GameContext<'a> {
    here: String,                         // current location
    locals: Vec<&'a Box<dyn GameObject>>, // objects in current location
    inv: Vec<&'a Box<dyn GameObject>>,    // objects carried to next location
}

impl<'a> GameContext<'a> {
    pub fn new(
        here: String,
        locals: Vec<&'a Box<dyn GameObject>>,
        inv: Vec<&'a Box<dyn GameObject>>,
    ) -> Self {
        Self { here, locals, inv }
    }

    pub fn here(&self) -> String {
        self.here.clone()
    }

    pub fn locals(&self) -> &Vec<&'a Box<dyn GameObject>> {
        &self.locals
    }

    pub fn inv(&self) -> &Vec<&'a Box<dyn GameObject>> {
        &self.inv
    }
}

pub struct Game {
    atlas: GameAtlas, // all objects in game
}

impl Game {
    pub fn new(atlas: GameAtlas) -> Self {
        Self { atlas }
    }

    pub fn print_death(&self) -> Handled {
        println!("**That would lead to your untimely demise.**\n\nTry again?");
        true
    }

    pub fn print_help(&self) -> Handled {
        println!("Try these commands:\nLOOK\nMOVE\nTAKE\nDROP\nINV\nQUIT");
        true
    }

    pub fn print_location(&self) -> Handled {
        let context = self.atlas.get_context();
        println!("You are in {}", context.here());
        true
    }

    pub fn print_explore(&self) -> Handled {
        let locals = self.atlas.get_locals(self.atlas.here());
        if locals.len() > 0 {
            println!("You see:");
            for o in locals {
                // TODO: Describe does not require callback.
                o.act(Action::Describe(Some(o.name())));
            }
        } else {
            println!("You see nothing of interest.");
        }
        true
    }

    pub fn print_inventory(&self) -> Handled {
        let inventory = self.atlas.get_inventory();
        println!("You are carrying:");
        for o in inventory {
            o.act(Action::Describe(Some(o.name())));
        }
        true
    }

    pub fn try_invoke(
        &mut self,
        action: Action,
        prso: Option<String>,
        prsi: Option<String>,
    ) -> Handled {
        let objects = vec![prsi, prso, Some(self.atlas.here())];
        self.atlas.invoke(objects, action)
    }

    pub fn run(&mut self) {
        // if self.context.here.is_none() {
        //     println!("No location set. Use 'move' first.");
        //     return;
        // }

        let parser = Parser::default();

        loop {
            let context = self.atlas.get_context();
            let action = parser.input_action(&context);
            let handled: Handled = match action.clone() {
                Action::Die => self.print_death(),
                Action::Help => self.print_help(),
                Action::Quit => break,
                Action::Go(_) => self.atlas.invoke(vec![], action),
                Action::Climb(prso)
                | Action::Describe(prso)
                | Action::Examine(prso)
                | Action::Listen(prso)
                | Action::Follow(prso)
                | Action::Take(prso) => self.try_invoke(action, prso, None),
                Action::Attack(prso, prsi)
                | Action::Drop(prso, prsi)
                | Action::Light(prso, prsi)
                | Action::Open(prso, prsi)
                | Action::Read(prso, prsi)
                | Action::Say(prso, prsi)
                | Action::Use(prso, prsi) => self.try_invoke(action, Some(prso), prsi),
                Action::UnknownAction(action) => {
                    println!("I don't know how to {}", action);
                    false
                }
                Action::AmbiguousObject(objects) => {
                    println!("Which {} do you mean?", objects.join(", "));
                    false
                }
                _ => false,
            };
            if !handled {
                println!("I didn't understand that. Have you tried 'help'?");
            }
        }
    }
}
