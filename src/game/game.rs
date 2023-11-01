use super::parser::Parser;
use super::{Action, GameObject, Handled, Location, Notify};
use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;

// TODO: implement non-here context for the action.
// Can I take an object from the cupboard, if I'm not standing in the kicthen?
// Is the book in the bookshelf, or in the library itself?
// Currently, the bookshelf will move a book into the library when opened.
// I can look into a breadbox, but I can't interact with the bread until it is moved to the kitchen.

pub static INV: &str = "__inv";
pub static NOWHERE: &str = "__nowhere";
pub static _GLOBAL: &str = "__global";

/// The game atlas controls all objects in the game.
/// It is responsible for adding, removing, and moving objects.
/// It also provides a context for the parser.
///! The game atlas is the only object that can move objects.
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

    /// Get the current location.
    pub fn here(&self) -> String {
        self.here.clone()
    }

    /// Set the current location.
    pub fn set_here(&mut self, here: String) {
        self.here = here;
    }

    /// Add a list of objects to the game.
    pub fn add_all(&mut self, objects: Vec<Box<dyn GameObject>>) {
        for o in objects {
            self.add(o);
        }
    }

    /// Add an object to the game.
    /// If the object already exists, it will not be added.
    pub fn add(&mut self, object: Box<dyn GameObject>) {
        if self.atlas.contains_key(&object.name()) {
            println!("cannot add duplicate object: '{}'", object.name());
            return;
        }
        self.atlas.insert(object.name(), RefCell::new(object));
    }

    /// Get an immutable reference to the object. (Used primarily for testing.)
    #[allow(dead_code)]
    pub fn get(&self, name: String) -> Option<Ref<'_, Box<dyn GameObject>>> {
        self.atlas.get(&name).map_or(None, |o| Some(o.borrow()))
    }

    /// Get a mutable reference to the object. (Used primarily for testing.)
    #[allow(dead_code)]
    pub fn get_mut(&mut self, name: String) -> Option<RefMut<'_, Box<dyn GameObject>>> {
        self.atlas
            .get_mut(&name)
            .map_or(None, |o| Some(o.borrow_mut()))
    }

    /// Set the location of the object.
    pub fn set_loc(&mut self, name: String, location: String) -> bool {
        if let Some(o) = self.atlas.get(&name) {
            o.borrow_mut().set_loc(location);
            return true;
        }
        false
    }

    /// Get all objects in the given location, but not the location itself.
    pub fn get_locals(&self, here: String) -> Vec<Ref<'_, Box<dyn GameObject>>> {
        self.atlas
            .iter()
            .filter_map(|(_k, v)| {
                let v = v.borrow();
                if v.loc() == here {
                    Some(v)
                } else {
                    None
                }
            })
            .collect()
    }

    /// Shortcut for get_locals(self.here())
    pub fn get_locals_here(&self) -> Vec<Ref<'_, Box<dyn GameObject>>> {
        self.get_locals(self.here())
    }

    /// Get all objects in the inventory.
    pub fn get_inventory(&self) -> Vec<Ref<'_, Box<dyn GameObject>>> {
        self.atlas
            .iter()
            .filter_map(|(_k, v)| {
                let v = v.borrow();
                if v.loc().as_str() == INV {
                    Some(v)
                } else {
                    None
                }
            })
            .collect()
    }

    /// Objects, inventory, and here for the given location.
    pub fn _get_context_for(&self, here: String) -> GameContext {
        GameContext::new(
            here.clone(),
            self.get_locals(here.clone()),
            self.get_inventory(),
        )
    }

    /// Shortcut for get_context(self.here())
    pub fn get_context(&self) -> GameContext {
        GameContext::new(self.here(), self.get_locals_here(), self.get_inventory())
    }

    /// Move the object to the inventory.
    pub fn move_inventory(&mut self, object_name: String) -> bool {
        if let Some(rc) = self.atlas.get(&object_name) {
            let mut o = rc.borrow_mut();
            println!("** {} moves from {} to inventory", o.name(), o.loc());
            o.set_loc(INV.to_string());
            return true;
        }
        false
    }

    /// Move the object to the current location.
    pub fn move_local(&mut self, object_name: String) -> bool {
        if let Some(rc) = self.atlas.get(&object_name) {
            let mut o = rc.borrow_mut();
            println!("** {} appears here {}", o.name(), self.here());
            o.set_loc(self.here());
            return true;
        }
        false
    }

    /// Remove the object from the game. (Move it to nowhere.)
    pub fn _remove_object(&mut self, object_name: String) -> bool {
        if let Some(rc) = self.atlas.get(&object_name) {
            let mut o = rc.borrow_mut();
            println!("** {} from {} consumed", o.name(), o.loc());
            o.set_loc(NOWHERE.to_string());
            return true;
        }
        false
    }

    /// Replace the old object with a new object in the same location.
    /// Good for replacing a key with a loaf of bread, for example.
    /// NOTE: Method required two mutable borrows of the atlas, hence Rc.
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

    /// Invoke action on all objects. Returns true if the action was handled by any.
    pub fn invoke_all(&mut self, action: Action, object_names: Vec<Option<String>>) -> Handled {
        object_names
            .into_iter()
            .map(|o| match o {
                Some(name) => self.invoke(action.clone(), name.clone()),
                None => false,
            })
            .find(|b| *b)
            .unwrap_or(false)
    }

    /// Shortcut for Describe all objects in list.
    pub fn describe_all(&mut self, object_names: Vec<Option<String>>) -> Handled {
        self.invoke_all(Action::Describe(None), object_names)
    }

    /// Invoke action on objects until the action is handled. Stops at the first handled action.
    ///! This is useful for actions that should only be handled by one object.
    pub fn invoke_until(&mut self, action: Action, object_names: Vec<Option<String>>) -> Handled {
        object_names
            .into_iter()
            .find(|o| match o {
                Some(name) => self.invoke(action.clone(), name.clone()),
                None => false,
            })
            .is_some()
    }

    /// Shortcut to invoke the action on the current location only.
    pub fn invoke_here(&mut self, action: Action) -> Handled {
        self.invoke(action, self.here())
    }

    /// Invoke a specific action on the specified object. Returns true if the action was handled.
    pub fn invoke(&mut self, action: Action, object_name: String) -> Handled {
        if let Some(rc) = self.atlas.get(&object_name) {
            let notification: Notify = {
                let mut o = rc.borrow_mut();
                if o.can_do(&action) {
                    o.act(action)
                } else {
                    Notify::Unhandled
                }
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

/// The game context provides a list of object for the current location.
/// It's used primarily by the parser to determine which objects are available and what actions they support.
/// TODO: Remove lifetimes, if not needed.
///! I added lifetimes during one iteration to fix a borrow checker error.
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

/// The game is the entry point in the game.
///! The game is responsible for running the game loop and invoking the parser.
pub struct Game {
    last_here: String, // last location
    atlas: GameAtlas,  // all objects in game
}

impl Game {
    pub fn new(atlas: GameAtlas) -> Self {
        Self {
            last_here: String::new(),
            atlas,
        }
    }

    /// Inform the user of their impending doom. No one actually dies, though.
    pub fn print_death(&self) -> Handled {
        println!("**That would lead to your untimely demise.**\n\nTry again?");
        true
    }

    /// Print a list of actions.
    /// TODO: This should be a list of actions supported by the objects in view.
    pub fn print_help(&self) -> Handled {
        println!("Try these commands:\nLOOK\nMOVE\nTAKE\nDROP\nATTACK\nINV\nQUIT");
        true
    }

    /// Helper method to convert a list of objects to a list of object names.
    fn to_names(&self, objects: &Vec<Ref<'_, Box<dyn GameObject>>>) -> Vec<Option<String>> {
        objects.iter().map(|o| Some(o.name())).collect()
    }

    /// Print the current location and all objects in the location.
    /// Only print the location if it has changed since the last invocation.
    pub fn print_location(&mut self) -> Handled {
        let here = self.atlas.here();
        if here == self.last_here {
            return false;
        }

        self.last_here = here.clone();
        print!("\n{}\n", here.to_uppercase());

        let locals = self.to_names(&self.atlas.get_locals_here());
        if locals.is_empty() {
            println!("You see nothing of interest.");
        } else {
            self.atlas.describe_all(locals);
        }
        true
    }

    /// Print the inventory objects.
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

    /// Invoke the action on the specified objects in order of PRSI, PRSO, HERE.
    pub fn try_invoke(
        &mut self,
        action: Action,
        prso: Option<String>,
        prsi: Option<String>,
    ) -> Handled {
        let objects = vec![prsi, prso, Some(self.atlas.here())];
        self.atlas.invoke_until(action, objects)
    }

    /// Run the game loop.
    pub fn run(&mut self) {
        let parser = Parser::default();

        loop {
            self.print_location();

            let action = parser.input_action(&self.atlas.get_context());

            let handled: Handled = match action.clone() {
                Action::Die => self.print_death(),
                Action::Help => self.print_help(),
                Action::Inventory => self.print_inventory(),
                Action::Quit => break,
                Action::Go(_) | Action::Wait => self.atlas.invoke_here(action),
                Action::Describe(prso) => {
                    self.try_invoke(action, prso, None);
                    true // always handled
                }
                Action::Climb(prso)
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
