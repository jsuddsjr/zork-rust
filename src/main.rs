use std::io::Error;
mod inv;
mod scene;
mod title;

fn main() -> Result<(), Error> {
    title::print();

    Ok(())
}
