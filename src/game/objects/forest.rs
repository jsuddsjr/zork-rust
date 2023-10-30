use crate::game::{Action, Direction, GameObject, Handled, Location, Mediator, NotifyAction};

#[derive(Clone, Default)]
pub struct Forest;

impl GameObject for Forest {
    fn name(&self) -> String {
        "forest".to_string()
    }

    fn act(&mut self, _mediator: &mut dyn Mediator, action: Action) -> Handled {
        match action {
            Action::Go(Direction::North) => {
                println!("You follow the path north.");
                true
            }
            Action::Describe(_) => {
                println!("You find yourself standing in a forest clearing, surrounded by trees. There is a path to the north.");
                true
            }
            Action::Arrive(_) => {
                println!("The fog clears...");
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

#[derive(Clone, Default)]
pub struct Leaves;

impl GameObject for Leaves {
    fn name(&self) -> String {
        "leaves".to_string()
    }

    fn loc(&self) -> String {
        Forest::default().name()
    }

    fn act(&mut self, _mediator: &mut dyn Mediator, action: Action) -> Handled {
        match action {
            Action::Describe(_) => {
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

#[derive(Clone, Default)]
pub struct Key {
    loc: String,
}

impl Key {
    pub fn new() -> Self {
        Self {
            loc: Leaves::default().name(),
        }
    }
}

impl GameObject for Key {
    fn name(&self) -> String {
        "key".to_string()
    }

    fn loc(&self) -> String {
        Leaves::default().name()
    }

    fn set_loc(&mut self, loc: String) {
        self.loc = loc;
    }

    fn act(&mut self, _mediator: &mut dyn Mediator, action: Action) -> Handled {
        match action {
            Action::Describe(_) => {
                println!("A shiny key glints in the grass.");
                // mediator.notify(NotifyAction::Move("key", Location::Local));
                true
            }
            Action::Take(_) => {
                println!("You take the key.");
                // mediator.notify(NotifyAction::Move("key", Location::Inventory));
                true
            }
            _ => false,
        }
    }
}
