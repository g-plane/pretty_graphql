#![doc = include_str!("../README.md")]

pub use crate::error::Error;
use crate::{
    config::FormatOptions,
    printer::{Ctx, DocGen},
};
use apollo_parser::{Parser, cst::Document};
use tiny_pretty::{IndentKind, PrintOptions, print};

pub mod config;
mod error;
mod printer;

#[inline]
/// Format the given source input.
pub fn format_text(input: &str, options: &FormatOptions) -> String {
    print_tree(&Parser::new(input).parse().document(), options)
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
