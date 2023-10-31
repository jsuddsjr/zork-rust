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

    pub fn get_inventory(&self) -> Vec<&'_ Box<dyn GameObject>> {
        self.atlas
            .iter()
            .filter_map(|(_k, v)| if v.loc() == "inv" { Some(v) } else { None })
            .collect()
    }

    pub fn invoke<'me, 'a>(
        &'me mut self,
        mediator: &'a mut dyn Mediator<'a>,
        object_name: Option<String>,
        action: Action,
    ) -> Handled
    where
        'me: 'a,
    {
        if object_name.is_some() {
            if let Some(o) = self.atlas.get_mut(&object_name.unwrap()) {
                return o.act(action.clone()) || o.act_react(mediator, action);
            }
        }
        false
    }
}

#[derive(Default)]
pub struct GameContext<'a> {
    here: String,                         // current location
    locals: Vec<&'a Box<dyn GameObject>>, // objects in current location
    inv: Vec<&'a Box<dyn GameObject>>,    // objects carried to next location
}

impl<'a> GameContext<'a> {
    pub fn new(here: String) -> Self {
        Self {
            here,
            vec: None,
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

    #[allow(dead_code)]
    pub fn set_locals(&mut self, locals: Box<Vec<Box<dyn GameObject>>>) {
        self.vec = Some(Box::leak(locals));
        self.locals.clear();
        self.vec.unwrap().into_iter().for_each(|o| {
            self.locals.insert(o.name(), &o);
        });
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

    pub fn _remove_inv(&mut self, object_name: String) {
        self.inv.remove(&object_name);
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

    pub fn print_death(&self) -> Handled {
        println!("**That would lead to your untimely demise.**\n\nTry again?");
        true
    }

    pub fn print_help(&self) -> Handled {
        println!("Try these commands:\nLOOK\nMOVE\nTAKE\nDROP\nINV\nQUIT");
        true
    }

    pub fn print_location(&self) -> Handled {
        println!("You are in {}", self.context.here);
        println!("You see:");
        for o in self.context.locals.values() {
            println!("  {}", o.name());
        }
        true
    }

    pub fn print_inventory(&self) -> Handled {
        println!("You are carrying:");
        for o in self.context.inv.values() {
            println!("  {}", o.name());
        }
        true
    }

    pub fn try_invoke_one(&'a mut self, object_name: Option<String>, action: Action) -> Handled {
        self.atlas.invoke(self, object_name, action)
    }

    pub fn try_invoke(
        &'a mut self,
        action: Action,
        prso: Option<String>,
        prsi: Option<String>,
    ) -> Handled {
        let here = Some(self.context.here());
        let mut atlas: &'a mut GameAtlas = &mut self.atlas;
        let mediator = self as &mut dyn Mediator;

        atlas.invoke(self, prsi, action.clone())
            || atlas.invoke(self, prso, action.clone())
            || atlas.invoke(self, here, action.clone())
    }

    pub fn run(&mut self) {
        // if self.context.here.is_none() {
        //     println!("No location set. Use 'move' first.");
        //     return;
        // }

        let parser = Parser::default();

        loop {
            let action = parser.input_action(&self.context);
            let handled: Handled = match action.clone() {
                Action::Die => self.print_death(),
                Action::Help => self.print_help(),
                Action::Quit => break,
                Action::Go(_) => self.atlas.invoke(self, Some(self.context.here()), action),
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
