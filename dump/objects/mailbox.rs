struct Mailbox {
    contents: Vec<Item>,
}

impl Mailbox {
    fn new(&self) -> Mailbox {
        Mailbox { contents: Vec::new() }
    }
}

impl Item for Mailbox {
    fn get_name(&self) -> &str {
        "Mailbox"
    }

    fn get_description(&self) -> &str {
        "A mailbox with a flag up"
    }

    fn set_description(&mut self, description: String) {
        // Do nothing
    }
}

impl FunctionTarget for Mailbox {
    fn get_name(&self) -> &str {
        "Open Mailbox"
    }

    fn react(&mut self, _inventory: &mut Inventory) -> String {
        if self.contents.is_empty() {
            return String::from("The mailbox is empty");
        }
        let mut contents = String::new();
        for item in self.contents.iter() {
            contents.push_str(item.get_name());
            contents.push_str(", ");
        }
        contents.pop();
        contents.pop();
        format!("The mailbox contains: {}", contents)
    }
}