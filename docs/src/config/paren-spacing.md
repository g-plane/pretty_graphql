# `parenSpacing`

Control whether whitespace should be inserted between parentheses or not.

Default option is `false`.

This global option can be overridden by different syntax nodes:

- `arguments.parenSpacing`
- `argumentsDefinition.parenSpacing`
- `variableDefinitions.parenSpacing`

## Example for `false`

```graphql
query Query($a: A) @directive(key: "value") {
  field
}
```

## Example for `true`

```graphql
query Query( $a: A ) @directive( key: "value" ) {
  field
}
```
