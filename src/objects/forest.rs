use crate::game::{Action, Handled, Mediated, Mediator, NotifyAction};

#[derive(Default)]
pub struct Forest;

impl Mediated for Forest {
    fn name(&self) -> String {
        "forest".into()
    }

    fn do_action(&mut self, _mediator: &mut dyn Mediator, action: Action) -> Handled {
        match action {
            Action::Look(_) => {
                println!("You find yourself standing in a forest clearing, surrounded by trees.");
                true
            }
            Action::Arrive(_) => {
                println!("You are standing in a forest clearing.");
                true
            }
            Action::Examine(_) => {
                println!("One of the trees nearby has been carved with the inscription: O+5.");
                true
            }
            _ => false,
        }
    }
}

#[derive(Default)]
pub struct Leaves;

impl Mediated for Leaves {
    fn name(&self) -> String {
        String::from("leaves")
    }

    fn loc(&self) -> String {
        String::from("forest")
    }

    fn do_action(&mut self, _mediator: &mut dyn Mediator, action: Action) -> Handled {
        match action {
            Action::Look(_) => {
                println!("You see a pile of leaves.");
                true
            }
            Action::Examine(_) => {
                println!("The leaves flutter and fly as you kick through them.");
                true
            }
            _ => false,
        }
    }
}

pub struct Key {
    loc: String,
}

impl Key {
    pub fn new() -> Self {
        Self {
            loc: String::from("leaves"),
        }
    }
}

impl Mediated for Key {
    fn name(&self) -> String {
        "key".to_string()
    }

    fn loc(&self) -> String {
        self.loc.clone()
    }

    fn set_loc(&mut self, loc: &'static str) {
        self.loc = String::from(loc);
    }

    fn do_action(&mut self, mediator: &'static mut dyn Mediator, action: Action) -> Handled {
        match action {
            Action::Look(_) => {
                println!("A shiny key glints in the grass.");
                mediator.notify_action(NotifyAction::MoveObject("key", "forest"));
                true
            }
            Action::Take(_) => {
                println!("You take the key.");
                mediator.notify_action(NotifyAction::MoveObject("key", "player"));
                true
            }
            _ => false,
        }
    }
}
