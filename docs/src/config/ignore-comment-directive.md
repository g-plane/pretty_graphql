# `ignoreCommentDirective`

Text directive for ignoring formatting specific statement.

Default is `"pretty-graphql-ignore"`, but if you're using as a plugin in dprint, it will be `"dprint-ignore"`.

## Example

```graphql
{
  # pretty-graphql-ignore
  hero {
       name
    height
  }
}
```
