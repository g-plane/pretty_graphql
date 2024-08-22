# `comma`

Control whether commas should be inserted inside a list of items.

Possible option values:

- `"always"`: Insert commas inside a list of items.
  For single line list, there won't be trailing comma; for multiple lines list, there will be trailing comma.
- `"never"`: Do not insert commas inside a list of items. All existed commas will be removed.
- `"noTrailing"`: Insert commas inside a list of items without trailing comma.
- `"onlySingleLine"`: Insert commas inside a list of items only for single line list. For multiple lines list, there won't be commas.

Default option value is `"onlySingleLine"`.

This global option can be overridden by different syntax nodes.
Some syntax-node-specific options will override by default:

- `arguments.comma`
- `argumentsDefinition.comma`
- `directives.comma` (default: `"never"`)
- `enumValuesDefinition.comma` (default: `"never"`)
- `fieldsDefinition.comma` (default: `"never"`)
- `inputFieldsDefinition.comma` (default: `"never"`)
- `listValue.comma`
- `objectValue.comma`
- `schemaDefinition.comma` (default: `"never"`)
- `schemaExtension.comma` (default: `"never"`)
- `selectionSet.comma` (default: `"never"`)
- `variableDefinitions.comma`

## Example for `"always"`

Single line:

```graphql
query Query($a: A, $b: B)  {
  field1
  field2
}
```

Multiple lines:

```graphql
query Query(
  $a: A,
  $b: B,
)  {
  field1
  field2
}
```

## Example for `"never"`

Single line:

```graphql
query Query($a: A $b: B)  {
  field1
  field2
}
```

Multiple lines:

```graphql
query Query(
  $a: A
  $b: B
)  {
  field1
  field2
}
```

## Example for `"noTrailing"`

Single line:

```graphql
query Query($a: A, $b: B)  {
  field1
  field2
}
```

Multiple lines:

```graphql
query Query(
  $a: A,
  $b: B
)  {
  field1
  field2
}
```

## Example for `"onlySingleLine"`

Single line:

```graphql
query Query($a: A, $b: B)  {
  field1
  field2
}
```

Multiple lines:

```graphql
query Query(
  $a: A
  $b: B
)  {
  field1
  field2
}
```
