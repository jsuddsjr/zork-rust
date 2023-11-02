use super::{Action, GameContext, GameObject, Handled, Location, Notify};
use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;

// TODO: implement non-here context for the action.
// Can I take an object from the cupboard, if I'm not standing in the kicthen?
// Is the book in the bookshelf, or in the library itself?
// Currently, the bookshelf will move a book into the library when opened.
// I can look into a breadbox, but I can't interact with the bread until it is moved to the kitchen.

pub static INVENTORY: &str = "__inv";
pub static NOWHERE: &str = "__nowhere";
pub static _GLOBAL: &str = "__global";

/// The game atlas controls all objects in the game.
/// It is responsible for adding, removing, and moving objects.
/// It also provides a context for the parser.
/// ! The game atlas is the only object that can move objects.
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
                if v.loc().as_str() == INVENTORY {
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
            o.set_loc(INVENTORY.to_string());
            return true;
        }
        false
    }

    /// Move the object to the current location.
    pub fn move_local(&mut self, object_name: String) -> bool {
        if let Some(rc) = self.atlas.get(&object_name) {
            let mut o = rc.borrow_mut();
            println!("** {} appears in the {}", o.name(), self.here());
            o.set_loc(self.here());
            return true;
        }
        false
    }

    /// Remove the object from the game. (Move it to nowhere.)
    pub fn _remove_object(&mut self, object_name: String) -> bool {
        if let Some(rc) = self.atlas.get(&object_name) {
            let mut o = rc.borrow_mut();
            println!("** {} disappears from the {}", o.name(), o.loc());
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
                println!("** {} is replaced by the {}", o1.name(), o2.name());
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
            .fold(false, |acc, x| acc || x) // Returns true if any are true.
    }

    /// Shortcut for Describe all objects in list.
    pub fn describe_all(&mut self, object_names: Vec<Option<String>>) -> Handled {
        self.invoke_all(Action::Describe(None), object_names)
    }

    /// Invoke action on objects until the action is handled. Stops at the first handled action.
    /// ! This is useful for actions that should only be handled by one object.
    pub fn invoke_until(&mut self, action: Action, object_names: Vec<Option<String>>) -> Handled {
        object_names
            .into_iter()
            .map(|o| match o {
                Some(name) => self.invoke(action.clone(), name.clone()),
                None => false,
            })
            .find(|x| *x) // Stops when true is found.
            .unwrap_or(false)
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
