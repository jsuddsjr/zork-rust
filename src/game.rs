use std::collections::{HashMap, VecDeque};

// Mediator has notification methods.
pub trait Mediator {
    fn notify_action(&mut self, object_name: &str) -> bool;
    fn notify_leave(&mut self, object_name: &str) -> bool;
}

// MediatorObject has methods called by Mediator.
pub trait MediatorObject {
    fn name(&self) -> &'static str;
    fn accepted(&mut self, mediator: &mut dyn Mediator);
    fn removed(&mut self, mediator: &mut dyn Mediator);
    fn leaving(&mut self, mediator: &mut dyn Mediator);
    fn arrive(&mut self, mediator: &mut dyn Mediator);
}

#[derive(Default)]
pub struct Game {
    objects: HashMap<String, Box<dyn MediatorObject>>,
    object_queue: VecDeque<String>,
    here: Option<String>, // current location
    prsa: Option<String>, // action
    prso: Option<String>, // direct object
    prsi: Option<String>, // indirect object
}

impl Mediator for Game {
    fn notify_action(&mut self, object_name: &str) -> bool {
        if self.object_on_platform.is_some() {
            self.object_queue.push_back(object_name.into());
            false
        } else {
            self.object_on_platform.replace(object_name.into());
            true
        }
    }

    fn notify_leave(&mut self, object_name: &str) {
        if Some(object_name.into()) == self.object_on_platform {
            self.object_on_platform = None;

            if let Some(next_object_name) = self.object_queue.pop_front() {
                let mut next_object = self.objects.remove(&next_object_name).unwrap();
                next_object.arrive(self);
                self.objects.insert(next_object_name.clone(), next_object);

                self.object_on_platform = Some(next_object_name);
            }
        }
    }
}

impl Game {
    pub fn accept(&mut self, mut object: impl MediatorObject + 'static) {
        if self.objects.contains_key(object.name()) {
            println!("cannot accept duplicate object: '{}'", object.name());
            return;
        }
        self.objects.insert(object.name().clone(), Box::new(object));
        object.accepted(self);
    }

    pub fn remove(&mut self, name: &'static str) {
        let object = self.objects.remove(name);
        if let Some(mut object) = object {
            object.removed(self);
        } else {
            println!("cannot remove unknown object: '{}'", name);
        }
    }

    pub fn unlock(&mut self) {
        if let Some(object_name) = self.object_on_platform.take() {
            self.remove(&object_name);
        }
    }
}
