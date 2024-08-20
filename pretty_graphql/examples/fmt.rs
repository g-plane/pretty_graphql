use pretty_graphql::{config::FormatOptions, format_text};
use std::{env, error::Error, fs, io};

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = env::args().nth(1).unwrap();
    let input = fs::read_to_string(&file_path)?;
    let options = match fs::read_to_string("config.json") {
        Ok(s) => serde_json::from_str(&s)?,
        Err(error) => {
            if error.kind() == io::ErrorKind::NotFound {
                FormatOptions::default()
            } else {
                return Err(Box::new(error));
            }
        }
    };

    let formatted = format_text(&input, &options)?;
    print!("{formatted}");
    Ok(())
}
