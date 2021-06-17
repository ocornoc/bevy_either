use super::{*, either_both::EitherBothState};

/// A type that contains either the [first](Either::Left) or [second](Either::Right) type.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum Either<T, U> {
    Left(T),
    Right(U),
}

pub struct EitherFetch<T, U> {
    left: T,
    right: U,
    matches: Matches,
}

enum Matches {
    Left,
    Right,
}

impl<'w, T: Fetch<'w>, U: Fetch<'w>> Fetch<'w> for EitherFetch<T, U> {
    type Item = Either<T::Item, U::Item>;
    type State = EitherBothState<T::State, U::State>;

    fn is_dense(&self) -> bool {
        self.left.is_dense() && self.right.is_dense()
    }

    unsafe fn init(
        world: &World,
        state: &Self::State,
        last_change_tick: u32,
        change_tick: u32,
    ) -> Self {
        EitherFetch {
            left: T::init(world, &state.left_state, last_change_tick, change_tick),
            right: U::init(world, &state.right_state, last_change_tick, change_tick),
            matches: Matches::Left,
        }
    }

    unsafe fn set_archetype(
        &mut self,
        state: &Self::State,
        archetype: &Archetype,
        tables: &Tables,
    ) {
        let left_match = state.left_state.matches_archetype(archetype);
        let right_match = state.right_state.matches_archetype(archetype);
        if left_match {
            self.left.set_archetype(&state.left_state, archetype, tables);
            self.matches = Matches::Left;
        } else if right_match {
            self.matches = Matches::Right;
            self.right.set_archetype(&state.right_state, archetype, tables);
        } else if cfg!(not(all(not(debug_assertions), unchecked))) {
            unreachable!("neither left nor right side matched. what?");
        }
    }

    unsafe fn set_table(&mut self, state: &Self::State, table: &Table) {
        let left_match = state.left_state.matches_table(table);
        let right_match = state.right_state.matches_table(table);
        if left_match {
            self.left.set_table(&state.left_state, table);
            self.matches = Matches::Left;
        } else if right_match {
            self.matches = Matches::Right;
            self.right.set_table(&state.right_state, table);
        } else if cfg!(not(all(not(debug_assertions), unchecked))) {
            unreachable!("neither left nor right side matched. what?");
        }
    }

    unsafe fn archetype_fetch(&mut self, archetype_index: usize) -> Self::Item {
        match self.matches {
            Matches::Left => Either::Left(self.left.archetype_fetch(archetype_index)),
            Matches::Right => Either::Right(self.right.archetype_fetch(archetype_index)),
        }
    }

    unsafe fn table_fetch(&mut self, table_row: usize) -> Self::Item {
        match self.matches {
            Matches::Left => Either::Left(self.left.table_fetch(table_row)),
            Matches::Right => Either::Right(self.right.table_fetch(table_row)),
        }
    }
}

unsafe impl<T: ReadOnlyFetch, U: ReadOnlyFetch> ReadOnlyFetch for EitherFetch<T, U> {}

impl<T: WorldQuery, U: WorldQuery> WorldQuery for Either<T, U> {
    type Fetch = EitherFetch<T::Fetch, U::Fetch>;
    type State = EitherBothState<T::State, U::State>;
}

#[cfg(test)]
mod tests {
    use bevy::prelude::*;
    use super::*;

    #[derive(Clone, Copy)]
    struct LeftElem;

    #[derive(Clone, Copy)]
    struct RightElem;

    fn push_entities(world: &mut World) -> (u32, u32, u32) {
        // 0 = None, 1 = Some(Left), 2 = Some(Right), 3 = Some(Both)
        static SUPERPERM: &[Option<EitherBoth<LeftElem, RightElem>>] = &[
            None,
            Some(EitherBoth::Left(LeftElem)),
            Some(EitherBoth::Right(RightElem)),
            Some(EitherBoth::Both(LeftElem, RightElem)),
            None,
            Some(EitherBoth::Left(LeftElem)),
            Some(EitherBoth::Right(RightElem)),
            None,
            Some(EitherBoth::Both(LeftElem, RightElem)),
            Some(EitherBoth::Left(LeftElem)),
            Some(EitherBoth::Right(RightElem)),
            None,
            Some(EitherBoth::Left(LeftElem)),
            Some(EitherBoth::Both(LeftElem, RightElem)),
            Some(EitherBoth::Right(RightElem)),
            None,
            Some(EitherBoth::Left(LeftElem)),
            None,
            Some(EitherBoth::Right(RightElem)),
            Some(EitherBoth::Both(LeftElem, RightElem)),
            Some(EitherBoth::Left(LeftElem)),
            None,
            Some(EitherBoth::Right(RightElem)),
            Some(EitherBoth::Left(LeftElem)),
            Some(EitherBoth::Both(LeftElem, RightElem)),
            None,
            Some(EitherBoth::Right(RightElem)),
            Some(EitherBoth::Left(LeftElem)),
            None,
            Some(EitherBoth::Both(LeftElem, RightElem)),
            Some(EitherBoth::Right(RightElem)),
            Some(EitherBoth::Left(LeftElem)),
            None,
        ];
        let mut left_count = 0;
        let mut right_count = 0;
        let mut both_count = 0;

        for &p in SUPERPERM {
            match p {
                Some(EitherBoth::Both(l, r)) => {
                    world.spawn().insert(l).insert(r);
                    both_count += 1;
                },
                Some(EitherBoth::Left(l)) => {
                    world.spawn().insert(l);
                    left_count += 1;
                },
                Some(EitherBoth::Right(r)) => {
                    world.spawn().insert(r);
                    right_count += 1;
                },
                None => {
                    world.spawn();
                },
            }
        }

        (left_count, right_count, both_count)
    }

    #[derive(Debug, PartialEq, Eq)]
    struct LeftCount(u32);

    #[derive(Debug, PartialEq, Eq)]
    struct RightCount(u32);

    #[test]
    fn test_eitherboth() {
        let mut world = World::default();
        world.insert_resource(LeftCount(0));
        world.insert_resource(RightCount(0));
        let (real_left_count, real_right_count, real_both_count) = push_entities(&mut world);
        let mut update_stage = SystemStage::single((|
                q: Query<Either<&LeftElem, &RightElem>>,
                mut l: ResMut<LeftCount>,
                mut r: ResMut<RightCount>,
            | {
                for eb in q.iter() {
                    match eb {
                        Either::Left(_) => {
                            l.0 += 1;
                        },
                        Either::Right(_) => {
                            r.0 += 1;
                        },
                    }
                }
            }
        ).system());
        update_stage.run(&mut world);
        assert_eq!(world.get_resource::<LeftCount>().unwrap().0, real_left_count + real_both_count);
        assert_eq!(world.get_resource::<RightCount>().unwrap().0, real_right_count);
    }
}
