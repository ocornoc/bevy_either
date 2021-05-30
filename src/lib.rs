//! # `bevy-either`
//!
//! This library provides [world queries] over other [world queries], allowing only one of multiple
//! to be satisfied and returning the query items.
//!
//! ## [`Either<T, U>`](Either)
//!
//! Given two [world queries] `T` and `U`, `Either<T, U>` provides a [world query] that contains
//! either the [`T`'s item](Either::Left) or the [`U`'s item](Either::Right). If both `T` and `U`
//! successfully match an entity, then only [`T`'s item](Either::Left) is given, e.g. there isn't a
//! "both" variant.
//!
//! ## [`EitherBoth<T, U>`](EitherBoth)
//!
//! Similarly to [`Either<T, U>`](Either), [`EitherBoth<T, U>`](EitherBoth) does allow one to match
//! over [`T`'s item](EitherBoth::Left) or [`U`'s item](EitherBoth::Right). What sets it apart is
//! the [`Both(t, u)`](EitherBoth::Both) variant, allowing both `T`'s and `U`'s items to be
//! provided, given that they do both match.
//!
//! ## [`either_many!`](either_many)
//!
//! This macro creates a new [world query] enum with a new variant for each of its possible matched
//! [world queries]. There isn't a "both"/"multiple" variant and the priority is always given to the
//! first declared variant when multiple matches occur. This lets you create [world queries] similar
//! to [`Either`], matching over one of the variant [world queries] with some priority order.
//!
//! ### `readonly`
//!
//! When using [`either_many!`](either_many), you can put `readonly` before the name of the new
//! [query](WorldQuery). This will make the resulting type's [fetcher](Fetch)
//! [read only](ReadOnlyFetch). The type is [read only](ReadOnlyFetch) if and only if all of its
//! variants are [read only](ReadOnlyFetch), and this is an invariant *you* must uphold.
//!
//! [world query]: WorldQuery
//! [world queries]: WorldQuery

#![no_std]

use bevy::prelude::*;
use bevy::ecs::{storage::*, component::*, archetype::*, query::*};

mod either_both;
mod either;
mod either_many;

pub use either_both::EitherBoth;
pub use either::Either;

pub mod exports {
    pub use paste::paste;
}
