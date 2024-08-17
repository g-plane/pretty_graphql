use crate::config::FormatOptions;
pub use apollo_parser::Error;

pub mod config;
mod printer;

/// Format the given source input.
pub fn format_text(input: &str, options: &FormatOptions) -> Result<String, Vec<Error>> {
    let parser = apollo_parser::Parser::new(input);
    let cst = parser.parse();
    let errors = cst.errors().cloned().collect::<Vec<_>>();
    if errors.is_empty() {
        Ok(String::new())
    } else {
        Err(errors)
    }
}
