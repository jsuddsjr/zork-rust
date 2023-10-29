use crate::game::{Action, GameObject, Handled, Location, Mediator, NotifyAction};

#[derive(Clone, Default)]
pub struct Kitchen;

impl GameObject for Kitchen {
    fn name(&self) -> String {
        "kitchen".to_string()
    }

    fn act(&mut self, mediator: &mut dyn Mediator, action: Action) -> Handled {
        match action {
            Action::Arrive(_) => {
                println!("Eww... What's that smell?");
                true
            }
            Action::Leave(_) => {
                mediator.notify(NotifyAction::Set(Location::To("forest")));
                true
            }
            Action::Describe(_) => {
                println!("You are in a kitchen. By the looks of it, no one has been here for a very long time. Dishes piled in the sink have a thick layer of mold. The refrigerator is empty. The only thing that looks edible is a loaf of bread in a breadbox on the counter.");
                true
            }
            _ => false,
        }
    }
}

#[derive(Clone, Default)]
pub struct Knife {
    name: String,
    loc: String,
}

impl Knife {
    pub fn new() -> Self {
        Self {
            name: "knife".to_string(),
            loc: Kitchen::default().name(),
        }
    }
}

impl GameObject for Knife {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn loc(&self) -> String {
        self.loc.clone()
    }

    fn set_loc(&mut self, loc: String) {
        self.loc = loc;
    }

    fn can_do(&self, action: &Action) -> bool {
        match action {
            Action::Take(_) => true,
            Action::Attack(target, _) => *target == Bread::default().name(),
            _ => false,
        }
    }

    fn act(&mut self, _mediator: &mut dyn Mediator, action: Action) -> Handled {
        match action {
            Action::Describe(_) => {
                println!("A rusty knife.");
                true
            }
            Action::Examine(_) => {
                println!("It won't slay a dragon, but it might work on bread.");
                true
            }
            Action::Take(_) => true,
            Action::Attack(target, _) => {
                if target == Bread::default().name() {
                    println!("You slice the crusty loaf clean in two. Take that you vile loaf!!");
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }
}

#[derive(Clone, Default)]
pub struct BreadBox {
    name: String,
    loc: String,
    contains_bread: bool,
}

impl BreadBox {
    pub fn new() -> Self {
        Self {
            name: "breadbox".to_string(),
            loc: Kitchen::default().name(),
            contains_bread: true,
        }
    }
}

impl GameObject for BreadBox {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn loc(&self) -> String {
        self.loc.clone()
    }

    fn act(&mut self, _mediator: &mut dyn Mediator, action: Action) -> Handled {
        match action {
            Action::Describe(_) => {
                if self.contains_bread {
                    println!("A breadbox with a loaf of bread in it.");
                } else {
                    println!("An empty breadbox.");
                }
                true
            }
            Action::Examine(_) => {
                println!("It's a breadbox.");
                true
            }
            _ => false,
        }
    }
}

#[derive(Clone, Default)]
pub struct Bread {
    name: String,
    loc: String,
}

impl Bread {
    pub fn new() -> Self {
        Self {
            name: "bread".to_string(),
            loc: Kitchen::default().name(),
        }
    }
}

impl GameObject for Bread {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn loc(&self) -> String {
        self.loc.clone()
    }

    fn act(&mut self, _mediator: &mut dyn Mediator, action: Action) -> Handled {
        match action {
            Action::Describe(_) => {
                println!("It's a loaf of bread, very stale.");
                true
            }
            Action::Examine(_) => {
                println!("The crust is so dry and hard that you'd probably break a tooth trying to eat it.");
                true
            }
            Action::Attack(_, attacker) => {
                if attacker.is_none() {
                    println!("You punch the bread and scrape your knuckles badly. Ouch!");
                } else {
                    println!("The loaf withstands the {}.", attacker.unwrap());
                }
                false
            }
            _ => false,
        }
    }
}
