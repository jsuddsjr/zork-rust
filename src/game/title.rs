static TITLE: &'static str = include_str!("title.txt");

pub fn print() {
    println!("{}\n\n", &TITLE)
}
