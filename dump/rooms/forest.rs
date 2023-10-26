use lazy_static::lazy_static;

lazy_static! {
    pub static ref FOREST: Scene = Scene::new("You find yourself in a forest clearing. Trees surround you on all sides, and a small path leads to the north.");
    pub static ref CAVE: Scene = Scene::new("You find yourself in a dark cave. The air is damp and musty, and you can hear the sound of dripping water echoing through the cave.");
}

impl FOREST {
    fn new() -> Forest {
        let mut ground = Ground::new();
        ground.add_item(Item::new("key", "A small key"));
        Forest {
            ground,
            player_inventory: Inventory::new(),
        }
    }

    fn search_ground(&mut self) -> Option<Item> {
        self.ground.remove_item("key")
    }

    fn move_player(&mut self, direction: Direction) -> Result<(), &'static str> {
        match direction {
            Direction::North => {
                // Handle moving north
                Ok(())
            }
            Direction::South => {
                // Handle moving south
                Ok(())
            }
            Direction::East => {
                // Handle moving east
                Ok(())
            }
            Direction::West => {
                // Handle moving west
                Ok(())
            }
            Direction::Up => {
                // Handle moving up
                Ok(())
            }
            Direction::Down => {
                // Handle moving down
                Ok(())
            }
        }
    }
}

struct Ground {
    items: Vec<Item>,
}

impl Ground {
    fn new() -> Ground {
        Ground { items: Vec::new() }
    }

    fn add_item(&mut self, item: Item) {
        self.items.push(item);
    }

    fn remove_item(&mut self, item_name: &str) -> Option<Item> {
        if let Some(index) = self.items.iter().position(|x| x.name == item_name) {
            return Some(self.items.remove(index));
        }
        None
    }
}