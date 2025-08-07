# Pretty GraphQL

Pretty GraphQL is a configurable GraphQL formatter.

![GitHub Downloads](https://img.shields.io/github/downloads/g-plane/pretty_graphql/latest/plugin.wasm?style=flat-square)

## Getting Started

### dprint

We've provided [dprint](https://dprint.dev/) integration.

Run the command below to add plugin:

```shell
dprint config add g-plane/pretty_graphql
```

After adding the dprint plugin, update your `dprint.json` and add configuration:

```jsonc
{
  // ...
  "graphql": { // <-- the key name here is "graphql", not "pretty_graphql"
    // Pretty GraphQL config comes here
  },
  "plugins": [
    "https://plugins.dprint.dev/g-plane/pretty_graphql-v0.2.3.wasm"
  ]
}
```

You can also read [dprint CLI documentation](https://dprint.dev/cli/) for using dprint to format files.

## Configuration

Please read [configuration documentation](https://pretty-graphql.netlify.app/).

## Using in Rust

To use Pretty GraphQL in Rust, please read the [documentation](https://docs.rs/pretty_graphql).

## Credit

Tests come from [Prettier](https://github.com/prettier/prettier/tree/main/tests/format/graphql).

## License

MIT License

Copyright (c) 2024-present Pig Fang
