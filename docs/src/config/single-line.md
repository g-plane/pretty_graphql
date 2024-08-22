# `singleLine`

Control whether items should be placed on single line as possible, even they're originally on multiple lines,
or force them to be on multiple lines.

Possible option values:

- `"prefer"`: Place items on single line as possible.
- `"smart"`: Whether items should be placed on single line will be determined by original layout.
- `"never"`: Force items to be on multiple lines.

Default option value is `"smart"`.

This global option can be overridden by different syntax nodes.
Some syntax-node-specific options will override by default:

- `arguments.singleLine`
- `argumentsDefinition.singleLine`
- `directiveLocations.singleLine`
- `directives.singleLine`
- `enumValuesDefinition.singleLine` (default: `"never"`)
- `fieldsDefinition.singleLine` (default: `"never"`)
- `implementsInterfaces.singleLine`
- `inputFieldsDefinition.singleLine` (default: `"never"`)
- `listValue.singleLine`
- `objectValue.singleLine`
- `schemaDefinition.singleLine` (default: `"never"`)
- `schemaExtension.singleLine` (default: `"never"`)
- `selectionSet.singleLine` (default: `"never"`)
- `unionMemberTypes.singleLine`
- `variableDefinitions.singleLine`

## Example for `"prefer"`

```graphql
query Query(
  $a: A
  $b: B
) {
  field1
  field2
}
```

will be formatted as:

```graphql
query Query($a: A, $b: B) {
  field1
  field2
}
```

## Example for `"smart"`

```graphql
query Query(
  $a: A
  $b: B
) {
  field1
  field2
}
```

will be formatted as:

```graphql
query Query(
  $a: A
  $b: B
) {
  field1
  field2
}
```

which is the same as the original layout.

But,

```graphql
query Query($a: A
  $b: B
) {
  field1
  field2
}
```

will be formatted as:

```graphql
query Query($a: A, $b: B) {
  field1
  field2
}
```

## Example for `"never"`

```graphql
query Query($a: A, $b: B) {
  field1
  field2
}
```

will be formatted as:

```graphql
query Query(
  $a: A
  $b: B
) {
  field1
  field2
}
```
