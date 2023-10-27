use std::collections::{HashMap, VecDeque};
use std::io::Write;

// Mediator has notification methods.
pub trait Mediator {
    fn notify_action(&mut self, object_name: &str) -> bool;
    fn notify_leave(&mut self, object_name: &str) -> bool;
}

// MediatorObject has methods called by Mediator.
pub trait MediatorObject {
    fn name(&self) -> &'static str;
    fn describe(&mut self, mediator: &mut dyn Mediator) -> bool;
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
        // if self.object_on_platform.is_some() {
        //     self.object_queue.push_back(object_name.into());
        //     false
        // } else {
        //     self.object_on_platform.replace(object_name.into());
        //     true
        // }
        false
    }

    fn notify_leave(&mut self, object_name: &str) -> bool {
        // if Some(object_name.into()) == self.object_on_platform {
        //     self.object_on_platform = None;

        //     if let Some(next_object_name) = self.object_queue.pop_front() {
        //         let mut next_object = self.objects.remove(&next_object_name).unwrap();
        //         next_object.arrive(self);
        //         self.objects.insert(next_object_name.clone(), next_object);

        //         self.object_on_platform = Some(next_object_name);
        //     }
        // }
        false
    }
}

impl Game {
    pub fn new() -> Self {
        Game {
            objects: HashMap::new(),
            object_queue: VecDeque::new(),
            here: None,
            prsa: None,
            prso: None,
            prsi: None,
        }
    }

    pub fn accept(&mut self, mut object: impl MediatorObject + 'static) {
        if self.objects.contains_key(object.name()) {
            println!("cannot accept duplicate object: '{}'", object.name());
            return;
        }
        object.accepted(self);
        self.objects
            .insert(object.name().to_string(), Box::new(object));
    }

    pub fn remove(&mut self, name: &'static str) {
        let object = self.objects.remove(name);
        if let Some(mut object) = object {
            object.removed(self);
        } else {
            println!("cannot remove unknown object: '{}'", name);
        }
    }

    pub fn get_input(&mut self) {
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");

        let mut words = input.split_whitespace();
        self.prsa = words.next().map(|x| x.to_string());
        self.prso = words.next().map(|x| x.to_string());
        self.prsi = words.next().map(|x| x.to_string());
    }

    pub fn print_help(&self) {
        println!("help");
    }

    //Parser reads vector of tokens from stdin and returns Tuple(PRSA, PRSO, PRSI).
    //PRSA is the action, PRSO direct object, and PRSI indirect object of the typed command.
    //The parser is responsible for converting the tokens into a PRSA, PRSO, PRSI tuple.
    //The parser is also responsible for checking the validity of the command.
    pub fn parse_command(&self) -> (String, Option<String>, Option<String>) {
        print!("(action) (direct object) (indirect object)> ");
        std::io::stdout().flush().ok();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        let mut tokens = Vec::new();
        for token in input.split_whitespace() {
            tokens.push(token);
        }
        if tokens.len() == 1 {
            return (tokens[0].to_string(), None, None);
        } else if tokens.len() == 2 {
            return (tokens[0].to_string(), Some(tokens[1].to_string()), None);
        } else if tokens.len() == 3 {
            return (
                tokens[0].to_string(),
                Some(tokens[1].to_string()),
                Some(tokens[2].to_string()),
            );
        } else {
            println!("I don't understand that.");
        }
        ("help".to_string(), None, None)
    }

    pub fn run(&mut self) {
        if self.here == None {
            println!("cannot run without a location. use 'move' first.");
            return;
        }

        println!("{}", self.here.as_mut().unwrap());

        loop {
            let _command = self.parse_command();
            // self.do_command();
        }
    }
}
