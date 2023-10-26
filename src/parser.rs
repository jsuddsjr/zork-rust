//Parser reads vector of tokens from stdin and returns Tuple(PRSA, PRSO, PRSI).
//PRSA is the action, PRSO direct object, and PRSI indirect object of the typed command.
//The parser is responsible for converting the tokens into a PRSA, PRSO, PRSI tuple.
//The parser is also responsible for checking the validity of the command.
pub fn parse_command() -> (String, String, String) {
    let mut tokens = Vec::new();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    for token in input.split_whitespace() {
        tokens.push(token);
    }
    let mut prsa = String::new();
    let mut prso = String::new();
    let mut prsi = String::new();
    if tokens.len() == 1 {
        prsa = tokens[0].to_string();
    } else if tokens.len() == 2 {
        prsa = tokens[0].to_string();
        prso = tokens[1].to_string();
    } else if tokens.len() == 3 {
        prsa = tokens[0].to_string();
        prso = tokens[1].to_string();
        prsi = tokens[2].to_string();
    } else {
        println!("I don't understand that.");
    }
    (prsa, prso, prsi)
}