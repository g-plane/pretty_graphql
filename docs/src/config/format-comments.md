# `formatComments`

Control whether whitespace should be inserted at the beginning of comments or not.

When this option is set to `false`, comments contain leading whitespace will still be kept as-is.

Default option is `false`.

## Example for `true`

```graphql
#comment
```

will be formatted as:

```graphql
# comment
```