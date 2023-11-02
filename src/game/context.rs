use super::GameObject;
use std::cell::Ref;

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
