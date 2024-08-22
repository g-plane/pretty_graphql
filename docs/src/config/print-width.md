# `printWidth`

The line width limitation that Pretty GraphQL should *(but not must)* avoid exceeding. Pretty GraphQL will try its best to keep line width less than this value, but it may exceed for some cases, for example, a very very long single word.

Default option is `80`.

## Example for `80`

```graphql
query Query($veryVeryVeryVeryLong: VeryVeryVeryVeryLong) {
  field
}
```

## Example for `40`

```graphql
query Query(
  $veryVeryVeryVeryLong: VeryVeryVeryVeryLong
) {
  field
}
```
