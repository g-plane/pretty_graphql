# `braceSpacing`

Control whether whitespace should be inserted between braces or not.

Default option is `true`.

This global option can be overridden by different syntax nodes:

- `enumValuesDefinition.braceSpacing`
- `fieldsDefinition.braceSpacing`
- `inputFieldsDefinition.braceSpacing`
- `objectValue.braceSpacing`
- `schemaDefinition.braceSpacing`
- `schemaExtension.braceSpacing`
- `selectionSet.braceSpacing`

## Example for `false`

```graphql
query Query($object: Any = {key: "value"}) {
  field
}
```

## Example for `true`

```graphql
query Query($object: Any = { key: "value" }) {
  field
}
```
