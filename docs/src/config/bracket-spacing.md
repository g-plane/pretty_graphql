# `bracketSpacing`

Control whether whitespace should be inserted between brackets or not.

Default option is `false`.

Currently this option is only applied for `listValue` syntax node,
so there're no syntax-node-specific options.

## Example for `false`

```graphql
query Query @directive(key: [1, 2, 3]) {
  field
}
```

## Example for `true`

```graphql
query Query @directive(key: [ 1, 2, 3 ]) {
  field
}
```
