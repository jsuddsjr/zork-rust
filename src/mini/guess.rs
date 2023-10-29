static WORD: &'static str = include_str!("guess.txt");


pub fn run() {
    let letters = WORD.chars().collect::<Vec<char>>();

    loop {

    }
    let mut guessed = vec![' '; letters.len()];
    let mut wrong = 0;
    let mut correct = 0;
    let mut guesses = Vec::new();
    let mut input = String::new();
    let mut rng = rand::thread_rng();
    let mut word_list = Vec::new();
    let mut last_word = Vec::new();
    let file = std::fs::read("corncob_lowercase.txt").unwrap();

}


