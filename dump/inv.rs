
pub trait FunctionTarget<T: ItemFunction> {
    fn react(&self, source: &impl T) -> Result<String, &'static str> {
        Err("Nothing happens.")
    }
}

pub trait ItemFunction<T: FunctionTarget> {
    fn function_name() -> &'static str;
    fn apply(&self, target: &impl T) -> Result<String, &'static str> {
        target.react(&self)
    }
}

pub struct Item {
    name: String,
    description: String,
}

impl ItemFunction for Item {
    fn apply(&self, target: &mut dyn FunctionTarget) {
        // Apply the item's function to the target
        target.set_description(format!("{} is now shiny", target.describe()));
    }
}

impl Description for Item {
    fn describe(&self) -> &str {
        &self.description
    }
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_description(&self) -> &str {
        &self.description
    }

    fn set_description(&mut self, description: String) {
        self.description = description;
    }
}

impl FunctionTarget for Item {
}

struct Inventory {
    items: Vec<String>,
}

impl Inventory {
    fn new() -> Inventory {
        Inventory { items: Vec::new() }
    }

    fn add_item(&mut self, item: String) {
        self.items.push(item);
    }

    fn remove_item(&mut self, item: &str) -> Option<String> {
        if let Some(index) = self.items.iter().position(|x| x == item) {
            return Some(self.items.remove(index));
        }
        None
    }

    fn list_items(&self) -> String {
        if self.items.is_empty() {
            return String::from("Inventory is empty");
        }
        let items = self.items.join(", ");
        format!("Inventory: {}", items)
    }
}

impl Description for Inventory {
    fn get_name(&self) -> &str {
        "Inventory"
    }

    fn get_description(&self) -> &str {
        &self.list_items()
    }

    fn set_description(&mut self, _description: String) {
        // Do nothing, as the inventory's description cannot be set
    }
}
