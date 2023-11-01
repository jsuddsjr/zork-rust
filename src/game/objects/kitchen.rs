use crate::game::{Action, Direction, GameObject, Location, Notify};

pub fn create(vec: &mut Vec<Box<dyn GameObject>>) {
    vec.push(Box::new(Kitchen::new()));
    vec.push(Box::new(Sink::new()));
    vec.push(Box::new(Knife::new()));
    vec.push(Box::new(BreadBox::new()));
    vec.push(Box::new(Bread::new()));
    vec.push(Box::new(GoldRing::new()));
}

pub static KITCHEN: &str = "kitchen";
pub static SINK: &str = "sink";
pub static KNIFE: &str = "knife";
pub static BREADBOX: &str = "breadbox";
pub static BREAD: &str = "bread";
pub static GOLDRING: &str = "gold ring";

#[derive(Default)]
pub struct Kitchen {
    name: String,
    seen: bool,
}

impl Kitchen {
    pub fn new() -> Self {
        Self {
            name: KITCHEN.to_string(),
            seen: false,
        }
    }
}

impl GameObject for Kitchen {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn act(&mut self, action: Action) -> Notify {
        return match action {
            Action::Arrive(_) => {
                if !self.seen {
                    println!("You are in a kitchen. The dishes are piled in the sink. The refrigerator is empty. There is a breadbox on the counter.");
                    self.seen = true;
                } else {
                    println!("You are in a kitchen. The dishes are still piled in the sink. The refrigerator is still empty. The breadbox is still on the counter.");
                }
                Notify::Handled
            }
            Action::Listen(_) => {
                println!("You hear the faint buzzing of flies and a slow drip into the sink.");
                Notify::Handled
            }
            Action::Describe(_) => {
                println!("You are in a kitchen. It's a mess.");
                Notify::Handled
            }
            Action::Leave(_) | Action::Go(Direction::Exit) => {
                println!("You head toward fresher air.");
                Notify::Set(Location::To("forest".to_string()))
            }
            _ => Notify::Unhandled,
        };
    }
}

#[derive(Default)]
pub struct Sink {
    name: String,
    loc: String,
    holds_knife: bool,
}

impl Sink {
    pub fn new() -> Self {
        Self {
            name: SINK.to_string(),
            loc: KITCHEN.to_string(),
            holds_knife: true,
        }
    }
}

impl GameObject for Sink {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn loc(&self) -> String {
        self.loc.clone()
    }

    fn act(&mut self, action: Action) -> Notify {
        return match action {
            Action::Describe(_) => {
                println!("A sink full of dirty dishes.");
                Notify::Handled
            }
            Action::Examine(_) => {
                if self.holds_knife {
                    println!("The dishes are covered in mold and a milky slime. Wait... is that a knife?");
                    Notify::Move("knife".to_string(), Location::Local)
                } else {
                    println!("The dishes are covered in mold and a milky slime. Gross.");
                    Notify::Handled
                }
            }
            _ => Notify::Unhandled,
        };
    }
}

#[derive(Default)]
pub struct Knife {
    name: String,
    loc: String,
}

impl Knife {
    pub fn new() -> Self {
        Self {
            name: KNIFE.to_string(),
            loc: SINK.to_string(),
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
            Action::Describe(_) => true,
            Action::Examine(_) => true,
            Action::Take(_) => true,
            Action::Use(_, _) => true,
            Action::Attack(_, _) => true,
            _ => false,
        }
    }

    fn act(&mut self, action: Action) -> Notify {
        match action {
            Action::Describe(_) => {
                println!("A rusty knife.");
                Notify::Handled
            }
            Action::Examine(_) => {
                println!("This blade won't slay a dragon, but it might work on bread.");
                Notify::Handled
            }
            Action::Take(_) => {
                println!(
                    "You reach in gingerly and take the knife, barely resisting the urge to vomit."
                );
                Notify::Move(self.name(), Location::Inventory)
            }
            Action::Use(target, _) | Action::Attack(target, _) => {
                if target.as_str() == BREAD {
                    println!("You hack the crusty loaf clean in two. Take that you vile loaf!!");
                    Notify::Move(GOLDRING.to_string(), Location::Local)
                } else {
                    println!("Are you serious? You can't use a knife on that.");
                    Notify::Handled
                }
            }
            _ => Notify::Unhandled,
        }
    }
}

#[derive(Default)]
pub struct BreadBox {
    name: String,
    loc: String,
    unlocked: bool,
}

impl BreadBox {
    pub fn new() -> Self {
        Self {
            name: BREADBOX.to_string(),
            loc: KITCHEN.to_string(),
            unlocked: false,
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

    fn can_do(&self, action: &Action) -> bool {
        match action {
            Action::Describe(_) => true,
            Action::Examine(_) => true,
            Action::Open(_, _) => true,
            _ => false,
        }
    }

    fn act(&mut self, action: Action) -> Notify {
        match action {
            Action::Describe(_) => {
                if self.unlocked {
                    println!("An empty breadbox.");
                } else {
                    println!("A breadbox.");
                }
                Notify::Handled
            }
            Action::Examine(_) => {
                if self.unlocked {
                    println!("It's an empty breadbox.");
                } else {
                    println!("You give the breadbox a shake and something heavy and hard rattles inside.\nUnfortunately, you can't see what it is because the breadbox is locked.");
                }
                println!("It's a breadbox.");
                Notify::Handled
            }
            Action::Open(_, with) => match with {
                None => {
                    if self.unlocked {
                        println!("It's empty.");
                        return Notify::Handled;
                    } else {
                        println!("You try to open the breadbox, but it's locked.\nWhat kind of person locks a breadbox?");
                        return Notify::Handled;
                    }
                }
                Some(item) => {
                    if item.as_str() != "key" {
                        println!("You can't open the breadbox with that.");
                        return Notify::Handled;
                    } else {
                        println!("You open the breadbox and take the loaf of bread.");
                        Notify::Replace("key".to_string(), "bread".to_string())
                    }
                }
            },
            _ => Notify::Unhandled,
        }
    }
}

#[derive(Default)]
pub struct Bread {
    name: String,
    loc: String,
}

impl Bread {
    pub fn new() -> Self {
        Self {
            name: BREAD.to_string(),
            loc: KITCHEN.to_string(),
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

    fn can_do(&self, action: &Action) -> bool {
        match action {
            Action::Describe(_) => true,
            Action::Examine(_) => true,
            Action::Take(_) => true,
            Action::Attack(_, _) => true,
            _ => false,
        }
    }

    fn act(&mut self, action: Action) -> Notify {
        match action {
            Action::Describe(_) => {
                println!("A crusty loaf of bread.");
                Notify::Handled
            }
            Action::Examine(_) => {
                println!("The crust is so dry and hard that you'd break a tooth trying to eat it.");
                Notify::Handled
            }
            Action::Take(_) => {
                println!("You take the bread.");
                Notify::Move(self.name(), Location::Inventory)
            }
            Action::Attack(_, attacker) => {
                match attacker {
                    None => println!("You punch the bread and scrape your knuckles badly. Ouch!"),
                    Some(attacker) => println!("The loaf resists the {}.", attacker),
                }
                Notify::Handled
            }
            _ => Notify::Unhandled,
        }
    }
}

#[derive(Clone, Default)]
struct GoldRing {
    name: String,
    loc: String,
    seen: bool,
}

impl GoldRing {
    pub fn new() -> Self {
        Self {
            name: GOLDRING.to_string(),
            loc: BREAD.to_string(),
            seen: false,
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }
}

impl GameObject for GoldRing {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn loc(&self) -> String {
        self.loc.clone()
    }

    fn can_do(&self, action: &Action) -> bool {
        match action {
            Action::Describe(_) => true,
            Action::Examine(_) => true,
            Action::Take(_) => true,
            _ => false,
        }
    }

    fn act(&mut self, action: Action) -> Notify {
        match action {
            Action::Describe(_) => {
                if !self.seen {
                    println!("A gold ring, barely big enough for your pinky finger, falls onto the counter with clear tinkling sound.");
                    self.seen = true;
                } else {
                    println!("A gold ring, barely big enough for your pinky finger.");
                }
                Notify::Handled
            }
            Action::Examine(_) => {
                println!("It's a pretty, albeit small, gold ring.");
                Notify::Handled
            }
            Action::Take(_) => {
                println!("You slip the ring into your pocket.");
                Notify::Move(self.name(), Location::Inventory)
            }
            _ => Notify::Unhandled,
        }
    }
}
