use super::game::GameContext;
use super::{Action, Direction, GameObject};
use std::collections::{HashMap, VecDeque};
use std::io::{stdin, stdout, Write};

static SKIP_WORDS: [&str; 9] = ["a", "an", "at", "here", "of", "out", "the", "to", "with"];

static POSITION_WORDS: [&str; 10] = [
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

pub struct Token {
    pub prsa: String,
    pub prso: Option<String>,
    pub prsi: Option<String>,
}

impl Token {
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
    pub fn new() -> Self {
        Self {}
    }

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

    fn get_targets(
        &self,
        action: &Action,
        map: &HashMap<String, Box<dyn GameObject>>,
    ) -> Vec<String> {
        let mut matches: Vec<String> = Vec::new();
        // get all objects that can do action
        for (key, obj) in map.iter() {
            if obj.can_do(action) {
                matches.push(key.clone());
            }
        }
        matches
    }

    fn get_matches(&self, name: &str, map: &HashMap<String, Box<dyn GameObject>>) -> Vec<String> {
        let mut matches: Vec<String> = Vec::new();
        // get matching objects
        for (key, obj) in map.iter() {
            if key.contains(name) {
                matches.push(obj.name());
            }
        }
        matches
    }

    fn to_indirect_action(&self, token: Token, context: &GameContext) -> Action {
        if token.prso.is_none() {
            return Action::MissingTarget(token.prsa.to_string());
        }

        // For brevity.
        let (o, i) = (token.prso.unwrap().clone(), token.prsi);

        return match token.prsa.as_str() {
            "attack" | "hit" | "kick" | "kill" | "throw" | "cut" | "slice" => Action::Attack(o, i),
            "ignite" | "burn" | "light" | "switch" => Action::Light(o, i),
            "drop" => Action::Drop(o, i),
            "r" | "read" => Action::Read(o, i),
            "unlock" | "open" => Action::Open(o, i),
            "u" | "use" => {
                // The symantic meaning of "use" is "use indirect on object".
                // Examples: "Use key" or "Use key on door" or "use key with door"
                // TODO: "use key to unlock door"
                if i.is_some() {
                    // If indirect object is specified, use it.
                    Action::Use(i.unwrap(), Some(o))
                } else {
                    // Otherwise, find a target that can be used.
                    let action = Action::Use("".to_string(), Some(o.clone()));
                    let targets = self.get_targets(&action, &context.locals);
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
        };
    }

    fn to_direct_action(&self, token: Token, context: &GameContext) -> Action {
        // For brevity.
        let o = token.prso.clone();

        return match token.prsa.as_str() {
            "climb" => Action::Climb(o),
            "desc" | "describe" => Action::Describe(o),
            "follow" | "stalk" => Action::Follow(o),
            "listen" | "play" => Action::Listen(o),
            "take" | "get" | "pick" => Action::Take(o),
            "x" | "examine" | "explore" | "inspect" | "look" => Action::Examine(o),
            &_ => self.to_indirect_action(token, context),
        };
    }

    fn to_direction(&self, direction: &String) -> Option<Direction> {
        let dir = direction.as_str();
        match dir {
            "north" | "n" | "forward" | "f" => return Some(Direction::North),
            "south" | "s" | "backward" | "b" => return Some(Direction::South),
            "east" | "e" | "right" | "r" => return Some(Direction::East),
            "west" | "w" | "left" | "l" => return Some(Direction::West),
            "up" | "u" | "upstairs" => return Some(Direction::Up),
            "down" | "d" => return Some(Direction::Down),
            "in" | "inside" => return Some(Direction::Enter),
            "out" | "outside" => return Some(Direction::Exit),
            &_ => return None,
        }
    }

    fn to_action(&self, token: Token, context: &GameContext) -> Action {
        let prsa = token.prsa.as_str();
        match prsa {
            "i" | "inv" | "inventory" => return Action::Inventory,
            "q" | "quit" => return Action::Quit,
            "?" | "help" | "hint" => return Action::Help,
            "g" | "go" | "ascend" | "climb" | "crawl" | "descend" | "run" | "travel" | "turn"
            | "skip" | "walk" => {
                if token.prsi.is_some() {
                    let prsi = token.prsi.unwrap();
                    if let Some(direction) = self.to_direction(&prsi) {
                        return Action::Go(direction);
                    } else {
                        return Action::UnknownDirection(prsi);
                    }
                } else {
                    // Room chooses direction, usually the only visible entrance or exit.
                    return Action::Go(Direction::Exit);
                }
            }
            "wait" => return Action::Wait,
            "enter" => return Action::Go(Direction::Enter),
            "leave" | "exit" => return Action::Go(Direction::Exit),
            _ => return self.to_direct_action(token, context),
        }
    }

    // Parser parses the PRSA of the command and returns an Action enum.
    pub fn input_action(&self, context: &GameContext) -> Action {
        let token = self.input_command();
        return self.to_action(token, context);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::objects::kitchen::Kitchen;

    #[test]
    fn test_parser() {
        let parser = Parser::new();
        let context = GameContext::new(String::new());
        let token: Token = Token::from_action("?");
        let action = parser.to_action(token, &context);
        assert_eq!(action, Action::Help);
    }

    #[test]
    fn test_parser_go() {
        let parser = Parser::new();
        let context = GameContext::new("kitchen".to_string());
        let action = parser.input_action(&context);
        assert_eq!(action, Action::Help);
    }

    // #[test]
    // fn test_parser_go_north() {
    //     let parser = Parser::new();
    //     let context = GameContext::new();
    //     let action = parser.input_action(&context);
    //     assert_eq!(action, Action::Help);
    // }

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
