use crate::inv::Item;

pub enum Direction {
    North,
    South,
    East,
    West,
    Up,
    Down,
}

pub struct Scene {
    description: String,
    transitions: std::collections::HashMap<Direction, Scene>,
    scene_objects: std::collections::HashMap<String, Item>,
}

impl Scene {
    pub fn new(description: &str) -> Scene {
        Scene {
            description: String::from(description),
            transitions: std::collections::HashMap::new(),
            scene_objects: std::collections::HashMap::new(),
        }
    }

    pub fn add_transition(&mut self, direction: Direction, scene: Scene) {
        self.transitions.insert(direction, scene);
    }

    pub fn get_transition(&self, direction: &Direction) -> Option<&Scene> {
        self.transitions.get(direction)
    }

    pub fn get_available_transitions(&self) -> Vec<Direction> {
        let mut available_transitions = Vec::new();
        for direction in self.transitions.keys() {
            available_transitions.push(*direction);
        }
        available_transitions
    }

    pub fn add_object(&mut self, object: Item) {
        self.scene_objects
            .insert(object.get_name().to_string(), object);
    }
}
