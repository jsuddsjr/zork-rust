use super::{Action, Direction, GameContext, GameObject};
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

#[derive(Debug, Default, PartialEq)]
pub struct Token {
    pub prsa: String,
    pub prso: Option<String>,
    pub prsi: Option<String>,
}

/// Token is a tuple of PRSA, PRSO, PRSI. (Refer to the ZIL design document.)
/// PRSA is the action, PRSO direct object, and PRSI indirect object of the typed command.
#[allow(dead_code)]
impl Token {
    pub fn from_action(prsa: &str) -> Self {
        Self {
            prsa: prsa.to_string(),
            prso: None,
            prsi: None,
        }
    }

    pub fn from_object(prsa: &str, prso: &str) -> Self {
        Self {
            prsa: prsa.to_string(),
            prso: Some(prso.to_string()),
            prsi: None,
        }
    }

    pub fn from_indirect(prsa: &str, prso: &str, prsi: &str) -> Self {
        Self {
            prsa: prsa.to_string(),
            prso: Some(prso.to_string()),
            prsi: Some(prsi.to_string()),
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
    fn read_line(&self) -> String {
        print!("\n>> ");
        stdout().flush().ok();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    }

    /// Parser reads vector of tokens from stdin and returns Tuple(PRSA, PRSO, PRSI).
    /// Value are just as they were entered, no normalization is performed yet.
    fn parse_token(&self, input: String) -> Token {
        // Filter words that are not useful.
        let mut tokens: VecDeque<String> = VecDeque::new();

        let lower = input.to_lowercase();
        for token in lower.split_whitespace() {
            if !SKIP_WORDS.contains(&token) {
                tokens.push_back(token.to_string());
            }
        }

        if tokens.is_empty() {
            return Token::from_action("help");
        } else if tokens.len() > 3 {
            println!("'{:?}' is too many words.", tokens);
            Token::from_action("help")
        } else {
            Token::from_vec(&mut tokens)
        }
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

                let action = match token.prsa.as_str() {
                    "attack" | "hit" | "kick" | "kill" | "throw" | "cut" | "slice" | "stab"
                    | "skewer" | "slash" | "strike" | "chop" | "swing" | "beat" | "poke" => {
                        Action::Attack(o, i)
                    }
                    "ignite" | "burn" | "light" | "switch" => Action::Light(o, i),
                    "d" | "drop" => Action::Drop(o, i),
                    "r" | "read" => Action::Read(o, i),
                    "unlock" | "open" => Action::Open(o, i),

                    // The symantic meaning of "use" is "use indirect on object".
                    // Examples: "Use key" or "Use key on door" or "use key with door"
                    // PRSA: use, PRSO: door, PRSI: key
                    // TODO: "use key to unlock door" or "use key to open door"
                    "u" | "use" => Action::Use(i.unwrap_or("".to_string()), Some(o)),

                    _ => Action::UnknownAction(token.prsa.to_string()),
                };

                // These actions run best with two objects, so try to find a match.
                if action.is_error() {
                    action
                } else if action.get_object().is_none() {
                    let targets = self.get_targets(&action, context.locals());
                    if targets.len() == 1 {
                        action.set_object(targets.first().unwrap().to_string())
                    } else if targets.len() > 1 {
                        Action::AmbiguousObject(targets)
                    } else {
                        Action::MissingTarget(token.prsa)
                    }
                } else if action.get_indirect_object().is_none() {
                    let targets = self.get_targets(&action, context.inv());
                    if targets.len() == 1 {
                        action.set_indirect_object(targets.first().unwrap().to_string())
                    } else {
                        action
                    }
                } else {
                    action
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
        let token = self.parse_token(self.read_line());
        self.to_action(token, context)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::atlas::GameAtlas;
    use crate::game::objects::{forest, kitchen};

    fn setup_atlas() -> GameAtlas {
        let mut vec = Vec::new() as Vec<Box<dyn GameObject>>;
        kitchen::create(&mut vec);
        forest::create(&mut vec);

        let mut atlas = GameAtlas::new(vec[0].name());
        atlas.add_all(vec);
        atlas
    }

    #[test]
    fn test_parser() {
        let atlas = setup_atlas();
        let token: Token = Token::from_action("?");

        let parser = Parser::default();
        let action = parser.to_action(token, &atlas.get_context());
        assert_eq!(action, Action::Help);
    }

    #[test]
    fn test_parser_inventory() {
        let atlas = setup_atlas();
        let token: Token = Token::from_action("i");

        let parser = Parser::default();
        let action = parser.to_action(token, &atlas.get_context());
        assert_eq!(action, Action::Inventory);
    }

    #[test]
    fn test_parser_quit() {
        let atlas = setup_atlas();
        let token: Token = Token::from_action("q");

        let parser = Parser::default();
        let action = parser.to_action(token, &atlas.get_context());
        assert_eq!(action, Action::Quit);
    }

    #[test]
    fn test_parser_go_north() {
        let atlas = setup_atlas();
        let token: Token = Token::from_object("g", "n");

        let parser = Parser::default();
        let action = parser.to_action(token, &atlas.get_context());
        assert_eq!(action, Action::Go(Direction::North));
    }

    #[test]
    fn test_parser_go_exit() {
        let atlas = setup_atlas();
        let token: Token = Token::from_action("go");

        let parser = Parser::default();
        let action = parser.to_action(token, &atlas.get_context());
        assert_eq!(action, Action::Go(Direction::Exit));
    }

    #[test]
    fn test_parser_look() {
        let atlas = setup_atlas();
        let token = Token::from_object("look", "sink");
        let expected = Action::Describe(token.prso.clone());

        let parser = Parser::default();
        let action = parser.to_action(token, &atlas.get_context());
        assert_eq!(action, expected);
    }

    #[test]
    fn test_parser_find_target() {
        let mut atlas = setup_atlas();
        atlas.set_here(String::from("kitchen"));
        atlas.move_local(String::from("bread"));

        let token = Token::from_object("use", "knife");
        let expected = Action::Attack(String::from("bread"), token.prso.clone());

        let parser = Parser::default();
        let action = parser.to_action(token, &atlas.get_context());
        assert_eq!(action, expected);
    }

    #[test]
    fn test_parser_ignore_stop_words() {
        let input = String::from("go to the north");
        let expected = Token::from_object("go", "north");

        let parser = Parser::default();
        let token = parser.parse_token(input);
        assert_eq!(token, expected);
    }
}
