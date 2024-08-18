use crate::{
    config::FormatOptions,
    printer::{Ctx, DocGen},
};
pub use apollo_parser::{cst::Document, Error};
use tiny_pretty::{print, IndentKind, PrintOptions};

pub mod config;
mod printer;

/// Format the given source input.
pub fn format_text(input: &str, options: &FormatOptions) -> Result<String, Vec<Error>> {
    let parser = apollo_parser::Parser::new(input);
    let cst = parser.parse();
    let errors = cst.errors().cloned().collect::<Vec<_>>();
    if errors.is_empty() {
        Ok(print_tree(&cst.document(), options))
    } else {
        Err(errors)
    }
}

/// Print the given concrete syntax tree.
/// You may use this when you already have the parsed CST.
pub fn print_tree(document: &Document, options: &FormatOptions) -> String {
    let ctx = Ctx {
        indent_width: options.layout.indent_width,
        options: &options.language,
    };
    print(
        &document.doc(&ctx),
        &PrintOptions {
            indent_kind: if options.layout.use_tabs {
                IndentKind::Tab
            } else {
                IndentKind::Space
            },
            line_break: options.layout.line_break.clone().into(),
            width: options.layout.print_width,
            tab_size: options.layout.indent_width,
        },
    )
}
