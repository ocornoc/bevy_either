# Optional matching for Bevy `WorldQuery`s!

This library provides world queries over other world queries, allowing only one of multiple to be
satisfied and returning the query items.

## `Either<T, U>`

Given two world queries `T` and `U`, `Either<T, U>` provides a world query that contains either the
`T`'s item or the `U`'s item. If both `T` and `U` successfully match an entity, then only `T`'s item
is given, e.g. there isn't a "both" variant.

## `EitherBoth<T, U>`

Similarly to `Either<T, U>`, `EitherBoth<T, U>` does allow one to match over `T`'s item or `U`'s
item. What sets it apart is the `Both(t, u)` variant, allowing both `T`'s and `U`'s items to be
provided, given that they do both match.

## `either_many!`

This macro creates a new world query enum with a new variant for each of its possible matched
world queries. There isn't a "both"/"multiple" variant and the priority is always given to the first
declared variant when multiple matches occur. This lets you create world queries similar to
`Either`, matching over one of the variant world queries with some priority order.

### `readonly`

When using `either_many!`, you can put `readonly` before the name of the new query. This will make
the resulting type's fetcher read only. The type is read only if and only if all of its variants are
read only, and this is an invariant *you* must uphold.

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
