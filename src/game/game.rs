use super::parser::Parser;
use super::{Action, GameObject, Handled, Location, Notify};
use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;

// TODO: do I need a context for the action?
// Can I take an object from the cupboard, if I'm not standing in the kicthen?
// Is the book in the bookshelf, or in the library itself?

pub static INV: &str = "__inv";
pub static NOWHERE: &str = "__nowhere";
pub static _GLOBAL: &str = "__global";

#[derive(Default)]
pub struct GameAtlas {
    here: String,
    atlas: HashMap<String, RefCell<Box<dyn GameObject>>>,
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
        self.atlas.insert(object.name(), RefCell::new(object));
    }

    #[allow(dead_code)]
    pub fn get(&self, name: String) -> Option<Ref<'_, Box<dyn GameObject>>> {
        self.atlas.get(&name).map_or(None, |o| Some(o.borrow()))
    }

    #[allow(dead_code)]
    pub fn get_mut(&mut self, name: String) -> Option<RefMut<'_, Box<dyn GameObject>>> {
        self.atlas
            .get_mut(&name)
            .map_or(None, |o| Some(o.borrow_mut()))
    }

    pub fn set_loc(&mut self, name: String, location: String) -> bool {
        if let Some(o) = self.atlas.get(&name) {
            o.borrow_mut().set_loc(location);
            return true;
        }
        false
    }

    pub fn get_locals(&self, here: String) -> Vec<Ref<'_, Box<dyn GameObject>>> {
        self.atlas
            .iter()
            .filter_map(|(k, v)| {
                if v.borrow().loc() == here || *k == here {
                    Some(v.borrow())
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn get_locals_here(&self) -> Vec<Ref<'_, Box<dyn GameObject>>> {
        self.get_locals(self.here())
    }

    pub fn get_inventory(&self) -> Vec<Ref<'_, Box<dyn GameObject>>> {
        self.atlas
            .iter()
            .filter_map(|(_k, v)| {
                if v.borrow().loc().as_str() == INV {
                    Some(v.borrow())
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn get_context(&self) -> GameContext {
        GameContext::new(self.here(), self.get_locals_here(), self.get_inventory())
    }

    pub fn _get_context_for(&self, here: String) -> GameContext {
        GameContext::new(
            here.clone(),
            self.get_locals(here.clone()),
            self.get_inventory(),
        )
    }

    pub fn move_inventory(&mut self, object_name: String) -> bool {
        if let Some(rc) = self.atlas.get(&object_name) {
            let mut o = rc.borrow_mut();
            println!("** {} moves from {} to inventory", o.name(), o.loc());
            o.set_loc(INV.to_string());
            return true;
        }
        false
    }

    pub fn move_local(&mut self, object_name: String) -> bool {
        if let Some(rc) = self.atlas.get(&object_name) {
            let mut o = rc.borrow_mut();
            println!("** {} appears here {}", o.name(), self.here());
            o.set_loc(self.here());
            return true;
        }
        false
    }

    pub fn _remove_object(&mut self, object_name: String) -> bool {
        if let Some(rc) = self.atlas.get(&object_name) {
            let mut o = rc.borrow_mut();
            println!("** {} from {} consumed", o.name(), o.loc());
            o.set_loc(NOWHERE.to_string());
            return true;
        }
        false
    }

    pub fn replace_object(&mut self, old_name: String, new_name: String) -> bool {
        if let Some(rc1) = self.atlas.get(&old_name) {
            if let Some(rc2) = self.atlas.get(&new_name) {
                let mut o1 = rc1.borrow_mut();
                let mut o2 = rc2.borrow_mut();
                println!("** {} replaced by {}", o1.name(), o2.name());
                o2.set_loc(o1.loc());
                o1.set_loc(NOWHERE.to_string());
                return true;
            }
        }
        return false;
    }

    pub fn invoke_all(&mut self, action: Action, object_names: Vec<Option<String>>) -> Handled {
        for name in object_names {
            if name.is_some() {
                let handled = self.invoke(action.clone(), name.unwrap());
                if handled {
                    return true;
                }
            }
        }
        false
    }

    pub fn invoke_here(&mut self, action: Action) -> Handled {
        self.invoke(action, self.here())
    }

    pub fn invoke(&mut self, action: Action, object_name: String) -> Handled {
        if let Some(rc) = self.atlas.get(&object_name) {
            let notification: Notify = {
                let mut o = rc.borrow_mut();
                o.act(action)
            };
            match notification {
                Notify::Handled => true,
                Notify::Unhandled => false,

                Notify::Set(location) => match location {
                    Location::To(name) => {
                        self.set_here(name);
                        true
                    }
                    _ => false,
                },

                Notify::Move(object_name, location) => match location {
                    Location::Local => self.move_local(object_name),
                    Location::Inventory => self.move_inventory(object_name),
                    Location::To(name) => self.set_loc(object_name, name),
                },

                Notify::Replace(old_obj, new_obj) => self.replace_object(old_obj, new_obj),
            }
        } else {
            false
        }
    }
}

#[derive(Default)]
#[allow(dead_code)]
pub struct GameContext<'a> {
    here: String,                              // current location
    locals: Vec<Ref<'a, Box<dyn GameObject>>>, // objects in current location
    inv: Vec<Ref<'a, Box<dyn GameObject>>>,    // objects carried to next location
}

#[allow(dead_code)]
impl<'a> GameContext<'a> {
    pub fn new(
        here: String,
        locals: Vec<Ref<'a, Box<dyn GameObject>>>,
        inv: Vec<Ref<'a, Box<dyn GameObject>>>,
    ) -> Self {
        Self { here, locals, inv }
    }

    pub fn here(&self) -> String {
        self.here.clone()
    }

    pub fn locals(&self) -> &Vec<Ref<'a, Box<dyn GameObject>>> {
        &self.locals
    }

    pub fn inv(&self) -> &Vec<Ref<'a, Box<dyn GameObject>>> {
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

    fn to_names(&self, objects: &Vec<Ref<'_, Box<dyn GameObject>>>) -> Vec<Option<String>> {
        objects.iter().map(|o| Some(o.name())).collect()
    }

    pub fn print_location(&mut self) -> Handled {
        print!("\n{}\n", self.atlas.here().to_uppercase());
        let locals = self.to_names(&self.atlas.get_locals_here());
        if locals.is_empty() {
            println!("You see nothing of interest.");
        } else {
            self.atlas.invoke_all(Action::Describe(None), locals);
        }
        true
    }

    pub fn print_inventory(&mut self) -> Handled {
        let inventory = self.to_names(&self.atlas.get_inventory());
        if inventory.is_empty() {
            println!("You are not carrying anything.");
            return true;
        } else {
            println!("You are carrying:");
            self.atlas.invoke_all(Action::Describe(None), inventory);
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
        self.atlas.invoke_all(action, objects)
    }

    pub fn run(&mut self) {
        // if self.context.here.is_none() {
        //     println!("No location set. Use 'move' first.");
        //     return;
        // }

        let parser = Parser::default();
        let mut last_here = String::new();

        loop {
            let here = self.atlas.here();
            if here != last_here {
                last_here = here.clone();
                self.print_location();
            }

            let action = {
                let context = self.atlas.get_context();
                parser.input_action(&context)
            };

            let handled: Handled = match action.clone() {
                Action::Die => self.print_death(),
                Action::Help => self.print_help(),
                Action::Inventory => self.print_inventory(),
                Action::Quit => break,
                Action::Go(_) | Action::Wait => self.atlas.invoke_here(action),
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
                    println!("That action could apply to: {}.", objects.join(", "));
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
