use std::{env, error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = env::args().nth(1).unwrap();
    let input = fs::read_to_string(&file_path)?;
    let parser = apollo_parser::Parser::new(&input);
    println!("{:?}", parser.parse());
    Ok(())
}
