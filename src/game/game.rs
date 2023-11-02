use super::parser::Parser;
use super::{Action, GameAtlas, GameObject, Handled, NOWHERE};
use std::cell::Ref;

/// The game is the entry point in the game.
///! The game is responsible for running the game loop and invoking the parser.
pub struct Game {
    last_here: String, // last location
    atlas: GameAtlas,  // all objects in game
}

impl Game {
    pub fn new(atlas: GameAtlas) -> Self {
        Self {
            last_here: String::from(NOWHERE),
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

    pub fn print_locals(&mut self, location: String) -> Handled {
        print!("\n{}\n", location.clone().to_uppercase());

        self.atlas.invoke_here(Action::Describe(None));
        let locals = self.to_names(&self.atlas.get_locals(location));
        if locals.is_empty() {
            println!("You see nothing of interest.");
        } else {
            self.atlas.describe_all(locals);
        }
        true
    }

    /// Print the current location and all objects in the location.
    /// Only print the location if it has changed since the last invocation.
    pub fn print_location(&mut self) -> Handled {
        let here = self.atlas.here();
        if here == self.last_here {
            return false;
        }

        self.atlas.invoke(
            Action::Leave(self.last_here.clone()),
            self.last_here.clone(),
        );

        self.last_here = here.clone();

        self.atlas.invoke_here(Action::Arrive(here.clone()));

        self.print_locals(here.clone())
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
                Action::Go(_) => {
                    self.atlas.invoke_here(action) || {
                        println!("You can't go that way.");
                        true
                    }
                }
                Action::Wait => self.atlas.invoke_here(action),
                Action::Describe(prso) | Action::Examine(prso) => match prso {
                    None => self.print_locals(self.atlas.here()),
                    Some(name) => self.try_invoke(action, Some(name), None),
                },
                Action::Climb(prso)
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
                    println!("I don't know how to {}. Have you tried 'HELP'?", action);
                    true
                }
                Action::AmbiguousObject(objects) => {
                    println!("That action could apply to: {}.", objects.join(", "));
                    true
                }
                _ => false,
            };
            if !handled {
                println!("Nothing happens.");
            }
        }
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::game::{objects::forest, objects::kitchen, GameAtlas, Handled, Location, Notify};

    use super::*;

    #[derive(Default)]
    struct MockGame {
        atlas: GameAtlas,
        prso: String,
        prsi: String,
        loc: String,
    }

    impl MockGame {
        #[allow(dead_code)]
        fn new(atlas: GameAtlas) -> Self {
            Self {
                atlas,
                prso: String::from(""),
                prsi: String::from(""),
                loc: String::from(""),
            }
        }

        pub fn invoke(&mut self, action: Action) -> Handled {
            let mut handled = false;
            action.print();

            if let Some(prso) = action.get_object() {
                println!("prso: {}", prso.clone());
                self.prso = prso;
            }

            if let Some(prsi) = action.get_indirect_object() {
                println!("prsi: {}", prsi.clone());
                self.prsi = prsi;
            }

            if let Some(mut rc) = self.atlas.get_mut(self.prso.clone()) {
                let notification = rc.act(action.clone());
                match notification {
                    Notify::Handled => handled = true,
                    Notify::Unhandled => handled = false,

                    Notify::Set(location) => match location {
                        Location::To(name) => {
                            self.loc = name;
                            handled = true;
                        }
                        _ => handled = false,
                    },

                    Notify::Move(object_name, location) => match location {
                        Location::Inventory => {
                            self.prso = object_name;
                            self.loc = "inventory".to_string();
                            handled = true;
                        }
                        Location::Local => {
                            self.prso = object_name;
                            self.loc = "here".to_string();
                            handled = true;
                        }
                        Location::To(name) => {
                            self.prso = object_name;
                            self.loc = name;
                            handled = true;
                        }
                    },

                    Notify::Replace(_, object_name) => {
                        self.prso = object_name;
                        handled = true;
                    }
                }
            }
            handled
        }
    }

    fn setup_atlas() -> GameAtlas {
        let mut vec = Vec::new() as Vec<Box<dyn GameObject>>;
        kitchen::create(&mut vec);
        forest::create(&mut vec);

        let mut atlas = GameAtlas::new(vec[0].name());
        atlas.add_all(vec);
        atlas
    }
}
