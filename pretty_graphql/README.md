`pretty_graphql` is a configurable GraphQL formatter.

## Basic Usage

You can format source code string by using [`format_text`] function.

```rust
use pretty_graphql::{config::FormatOptions, format_text};

let options = FormatOptions::default();
assert_eq!("{\n  field\n}\n", &format_text("{ field }", &options).unwrap());
```

For detailed documentation of configuration,
please read [configuration documentation](https://pretty-graphql.netlify.app/).

If there're syntax errors in source code, it will return `Err`:

```rust
use pretty_graphql::{config::FormatOptions, format_text};

let options = FormatOptions::default();
assert!(format_text("{", &options).is_err());
```

## Print Syntax Tree

If you have already parsed the syntax tree from [`apollo-parser`](https://docs.rs/apollo-parser),
you can use [`print_tree`] to print it.

```rust
use pretty_graphql::{config::FormatOptions, print_tree};
use apollo_parser::{cst::Document, Parser};

let input = "{ field }";
let parser = Parser::new(input);
let cst = parser.parse();

let options = FormatOptions::default();
assert_eq!("{\n  field\n}\n", &print_tree(&cst.document(), &options));
```
