use super::game::GameContext;
use super::{Action, Direction, GameObject};
use std::cell::Ref;
use std::collections::VecDeque;
use std::fmt::Debug;
use std::io::{stdin, stdout, Write};

static SKIP_WORDS: [&str; 9] = ["a", "an", "at", "here", "of", "out", "the", "to", "with"];

static _POSITION_WORDS: [&str; 10] = [
    "above", "behind", "below", "beside", "beyond", "in", "inside", "near", "on", "under",
];

// static ROOM_WORDS: [&str; 10] = [
//     "attic",
//     "basement",
//     "bedroom",
//     "cellar",
//     "closet",
//     "dining",
//     "foyer",
//     "kitchen",
//     "library",
//     "study",
// ];

// static OBJECT_WORDS: [&str; 10] = [
//     "book",
//     "candle",
//     "door",
//     "key",
//     "lantern",
//     "letter",
//     "match",
//     "note",
//     "parchment",
//     "scroll",
// ];

// static DOOR_WORDS: [&str; 17] = [
//     "arbor",
//     "arch",
//     "door",
//     "entrance",
//     "exit",
//     "gate",
//     "hatch",
//     "hole",
//     "opening",
//     "passage",
//     "path",
//     "portal",
//     "stair",
//     "staircase",
//     "trapdoor",
//     "tunnel",
//     "way",
// ];

#[derive(Debug)]
pub struct Token {
    pub prsa: String,
    pub prso: Option<String>,
    pub prsi: Option<String>,
}

impl Token {
    #[allow(dead_code)]
    pub fn new(prsa: String, prso: Option<String>, prsi: Option<String>) -> Self {
        Self { prsa, prso, prsi }
    }

    pub fn from_action(prsa: &str) -> Self {
        Self {
            prsa: prsa.to_string(),
            prso: None,
            prsi: None,
        }
    }

    pub fn from_vec(vec: &mut VecDeque<String>) -> Self {
        Self {
            prsa: vec.pop_front().unwrap(),
            prso: vec.pop_front(),
            prsi: vec.pop_front(),
        }
    }
}

#[derive(Default)]
pub struct Parser;

impl Parser {
    //Parser reads vector of tokens from stdin and returns Tuple(PRSA, PRSO, PRSI).
    //PRSA is the action, PRSO direct object, and PRSI indirect object of the typed command.
    fn input_command(&self) -> Token {
        print!("\n>> ");
        stdout().flush().ok();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        // Filter words that are not useful.
        let mut tokens: VecDeque<String> = VecDeque::new();
        let lower = input.to_lowercase();
        for token in lower.split_whitespace() {
            if !SKIP_WORDS.contains(&token) {
                tokens.push_back(token.to_string());
            }
        }

        if tokens.len() == 1 {
            return Token::from_action(&tokens[0]);
        }

        if tokens.len() == 2 || tokens.len() == 3 {
            return Token::from_vec(&mut tokens);
        }

        if tokens.len() > 3 {
            println!("'{:?}' is too many words.", tokens);
        }

        Token::from_action("help")
    }

    fn get_targets(&self, action: &Action, map: &Vec<Ref<'_, Box<dyn GameObject>>>) -> Vec<String> {
        let mut matches: Vec<String> = Vec::new();
        // get all objects that can do action
        for obj in map.iter() {
            if obj.can_do(action) {
                matches.push(obj.name());
            }
        }
        matches
    }

    fn to_indirect_action(&self, token: Token, context: &GameContext) -> Action {
        match token.prso {
            None => Action::UnknownAction(token.prsa.to_string()),
            Some(prso) => {
                // For brevity.
                let (o, i) = (prso.clone(), token.prsi);

                match token.prsa.as_str() {
                    "attack" | "hit" | "kick" | "kill" | "throw" | "cut" | "slice" => {
                        Action::Attack(o, i)
                    }
                    "ignite" | "burn" | "light" | "switch" => Action::Light(o, i),
                    "drop" => Action::Drop(o, i),
                    "r" | "read" => Action::Read(o, i),
                    "unlock" | "open" => Action::Open(o, i),
                    "u" | "use" => {
                        // The symantic meaning of "use" is "use indirect on object".
                        // Examples: "Use key" or "Use key on door" or "use key with door"
                        // PRSA: use, PRSO: door, PRSI: key
                        // TODO: "use key to unlock door"
                        if i.is_some() {
                            // If indirect object is specified, use it.
                            Action::Use(i.unwrap(), Some(o))
                        } else {
                            // Otherwise, find a target that can be used.
                            let action = Action::Use("".to_string(), Some(o.clone()));
                            let targets = self.get_targets(&action, context.locals());
                            if targets.len() == 1 {
                                Action::Use(targets[0].clone(), Some(o))
                            } else if targets.len() > 1 {
                                Action::AmbiguousObject(targets)
                            } else {
                                Action::MissingTarget(o)
                            }
                        }
                    }
                    _ => Action::UnknownAction(token.prsa.to_string()),
                }
            }
        }
    }

    fn to_direct_action(&self, token: Token, context: &GameContext) -> Action {
        // For brevity.
        let o = token.prso.clone();

        return match token.prsa.as_str() {
            "climb" => Action::Climb(o),
            "desc" | "describe" | "look" => Action::Describe(o),
            "follow" | "stalk" => Action::Follow(o),
            "listen" | "play" => Action::Listen(o),
            "take" | "get" | "pick" => Action::Take(o),
            "x" | "examine" | "explore" | "inspect" => Action::Examine(o),
            &_ => self.to_indirect_action(token, context),
        };
    }

    fn to_direction(&self, direction: String) -> Option<Direction> {
        let dir = direction.as_str();
        match dir {
            "north" | "n" | "forward" | "f" => Some(Direction::North),
            "south" | "s" | "backward" | "b" => Some(Direction::South),
            "east" | "e" | "right" | "r" => Some(Direction::East),
            "west" | "w" | "left" | "l" => Some(Direction::West),
            "up" | "u" | "upstairs" => Some(Direction::Up),
            "down" | "d" => Some(Direction::Down),
            "in" | "inside" => Some(Direction::Enter),
            "out" | "outside" => Some(Direction::Exit),
            &_ => None,
        }
    }

    fn to_action(&self, token: Token, context: &GameContext) -> Action {
        let prsa = token.prsa.as_str();
        match prsa {
            "i" | "inv" | "inventory" => Action::Inventory,
            "q" | "quit" => Action::Quit,
            "?" | "help" | "hint" => Action::Help,
            "g" | "go" | "ascend" | "climb" | "crawl" | "descend" | "run" | "travel" | "turn"
            | "skip" | "walk" => {
                match token.prso {
                    // Room chooses direction, usually the only visible entrance or exit.
                    None => Action::Go(Direction::Exit),
                    Some(dir) => {
                        if let Some(direction) = self.to_direction(dir.clone()) {
                            Action::Go(direction)
                        } else {
                            Action::UnknownDirection(dir)
                        }
                    }
                }
            }
            "wait" => Action::Wait,
            "enter" => Action::Go(Direction::Enter),
            "leave" | "exit" => Action::Go(Direction::Exit),
            _ => self.to_direct_action(token, context),
        }
    }

    // Parser parses the PRSA of the command and returns an Action enum.
    pub fn input_action(&self, context: &GameContext) -> Action {
        let token = self.input_command();
        self.to_action(token, context)
    }
}

#[cfg(test)]
mod tests {
    use crate::game::{objects::kitchen, GameAtlas, Handled, Location, Notify};

    use super::*;

    #[derive(Default)]
    struct MockGame {
        atlas: GameAtlas,
        obj: String,
        loc: String,
    }

    impl MockGame {
        #[allow(dead_code)]
        fn new(atlas: GameAtlas) -> Self {
            Self {
                atlas,
                obj: String::from(""),
                loc: String::from(""),
            }
        }

        fn unpack_action(&self, action: Action) -> (String, Option<String>, Option<String>) {
            match action {
                Action::Go(d) => match d {
                    Direction::North => (String::from("go"), Some("north".to_string()), None),
                    Direction::South => (String::from("go"), Some("south".to_string()), None),
                    Direction::East => (String::from("go"), Some("east".to_string()), None),
                    Direction::West => (String::from("go"), Some("west".to_string()), None),
                    Direction::Up => (String::from("go"), Some("up".to_string()), None),
                    Direction::Down => (String::from("go"), Some("down".to_string()), None),
                    Direction::Exit => (String::from("go"), Some("exit".to_string()), None),
                    Direction::Enter => (String::from("go"), Some("enter".to_string()), None),
                },

                Action::Climb(o) => (String::from("climb"), o, None),
                Action::Describe(o) => (String::from("describe"), o, None),
                Action::Examine(o) => (String::from("examine"), o, None),
                Action::Follow(o) => (String::from("follow"), o, None),
                Action::Listen(o) => (String::from("listen"), o, None),
                Action::Take(o) => (String::from("take"), o, None),
                Action::Attack(o, i) => (String::from("attack"), Some(o), i),
                Action::Drop(o, i) => (String::from("drop"), Some(o), i),
                Action::Light(o, i) => (String::from("light"), Some(o), i),
                Action::Open(o, i) => (String::from("open"), Some(o), i),
                Action::Read(o, i) => (String::from("read"), Some(o), i),
                Action::Say(o, i) => (String::from("say"), Some(o), i),
                Action::Use(o, i) => (String::from("use"), Some(o), i),

                Action::Die => (String::from("die"), None, None),
                Action::Help => (String::from("help"), None, None),
                Action::Inventory => (String::from("inventory"), None, None),
                Action::Wait => (String::from("wait"), None, None),
                Action::Quit => (String::from("quit"), None, None),

                Action::Arrive(o) => (String::from("arrive"), Some(o), None),
                Action::Leave(o) => (String::from("leave"), Some(o), None),

                Action::AmbiguousObject(mut v) => (String::from("ambiguousObj"), v.pop(), v.pop()),
                Action::MissingTarget(a) => (String::from("missingTarget"), Some(a), None),
                Action::UnknownAction(a) => (String::from("unknownAction"), Some(a), None),
                Action::UnknownDirection(d) => (String::from("unknownDir"), Some(d), None),

                _ => (String::from("other"), None, None),
            }
        }

        pub fn invoke(&mut self, action: Action) -> Handled {
            let mut handled = false;
            let (prsa, prso, prsi) = self.unpack_action(action.clone());

            println!(
                "prsa: {} prso: {} prsi: {}",
                prsa,
                prso.as_ref().map_or_else(|| "".to_string(), |s| s.clone()),
                prsi.as_ref().map_or_else(|| "".to_string(), |s| s.clone())
            );

            if let Some(mut obj) = self.atlas.get_mut(prso.unwrap()) {
                let notification = obj.act(action.clone());
                match notification {
                    Notify::Handled => handled = true,
                    Notify::Unhandled => handled = false,

                    Notify::Set(location) => match location {
                        Location::To(name) => {
                            self.loc = name;
                            handled = true;
                        }
                        _ => handled = false,
                    },

                    Notify::Move(object_name, location) => match location {
                        Location::Inventory => {
                            self.obj = object_name;
                            self.loc = "inventory".to_string();
                            handled = true;
                        }
                        Location::Local => {
                            self.obj = object_name;
                            self.loc = "here".to_string();
                            handled = true;
                        }
                        Location::To(name) => {
                            self.obj = object_name;
                            self.loc = name;
                            handled = true;
                        }
                    },

                    Notify::Replace(_, object_name) => {
                        self.obj = object_name;
                        handled = true;
                    }
                }
            }
            handled
        }
    }

    fn setup_kitchen() -> GameAtlas {
        let mut vec = Vec::new() as Vec<Box<dyn GameObject>>;
        kitchen::create(&mut vec);

        let mut atlas = GameAtlas::new(vec[0].name());
        atlas.add_all(vec);
        atlas
    }

    #[test]
    fn test_parser() {
        let parser = Parser::default();
        let atlas = setup_kitchen();
        let context = atlas.get_context();
        let token: Token = Token::from_action("?");
        let action = parser.to_action(token, &context);
        assert_eq!(action, Action::Help);
    }

    #[test]
    fn test_parser_look() {
        let atlas = setup_kitchen();
        let parser = Parser::default();
        let sink = Some(String::from("sink"));
        let token = Token::new(String::from("look"), sink.clone(), None);
        let action = parser.to_action(token, &atlas.get_context());
        assert_eq!(action, Action::Examine(sink));
    }

    #[test]
    fn test_parser_move_knife() {
        let atlas = setup_kitchen();
        let parser = Parser::default();
        let sink = Some(String::from("sink"));
        let token = Token::new(String::from("x"), sink.clone(), None);
        let action = parser.to_action(token, &atlas.get_context());
        let mut game = MockGame::default();
        let handled = game.invoke(action);
        assert_eq!(handled, true);
    }

    // #[test]
    // fn test_parser_go_north_to() {
    //     let parser = Parser::new();
    //     let context = GameContext::new();
    //     let action = parser.input_action(&context);
    //     assert_eq!(action, Action::Help);
    // }

    // #[test]
    // fn test_parser_go_north_to_living_room() {
    //     let parser = Parser::new();
    //     let context = GameContext::new();
    //     let action = parser.input_action(&context);
    //     assert_eq!(action, Action::Help);
    // }

    // #[test]
    // fn test_parser_go_north_to_living_room_with() {
    //     let parser = Parser::new();
    //     let context = GameContext::new();
    //     let action = parser.input_action(&context);
    //     assert_eq!(action, Action::Help);
    // }

    // #[test]
    // fn test_parser_go_north_to_living_room_with_key() {
    //     let parser = Parser::new();
    //     let context = GameContext::new();
    //     let action = parser.input_action(&context);
    //     assert_eq!(action, Action::Help);
    // }
}
