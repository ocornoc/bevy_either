use super::*;

/// A type that contains either the [first](EitherBoth::Left) type, [second](EitherBoth::Right)
/// type, or [both](EitherBoth::Both).
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum EitherBoth<T, U> {
    Left(T),
    Right(U),
    Both(T, U),
}

enum Matches {
    Left,
    Right,
    Both,
}

pub struct EitherBothState<T: FetchState, U: FetchState> {
    pub(super) left_state: T,
    pub(super) right_state: U,
}

unsafe impl<T: FetchState, U: FetchState> FetchState for EitherBothState<T, U> {
    fn init(world: &mut World) -> Self {
        EitherBothState {
            left_state: T::init(world),
            right_state: U::init(world),
        }
    }

    fn update_component_access(&self, access: &mut FilteredAccess<ComponentId>) {
        self.left_state.update_component_access(access);
        self.right_state.update_component_access(access);
    }

    fn update_archetype_component_access(
        &self,
        archetype: &Archetype,
        access: &mut Access<ArchetypeComponentId>,
    ) {
        self.left_state.update_archetype_component_access(archetype, access);
        self.right_state.update_archetype_component_access(archetype, access);
    }

    fn matches_archetype(&self, archetype: &Archetype) -> bool {
        self.left_state.matches_archetype(archetype) || self.right_state.matches_archetype(archetype)
    }

    fn matches_table(&self, table: &Table) -> bool {
        self.left_state.matches_table(table) || self.right_state.matches_table(table)
    }
}

pub struct EitherBothFetch<T, U> {
    left: T,
    right: U,
    matches: Matches,
}

unsafe impl<T: ReadOnlyFetch, U: ReadOnlyFetch> ReadOnlyFetch for EitherBothFetch<T, U> {}

impl<'w, T: Fetch<'w>, U: Fetch<'w>> Fetch<'w> for EitherBothFetch<T, U> {
    type Item = EitherBoth<T::Item, U::Item>;
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
        EitherBothFetch {
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
            if right_match {
                self.matches = Matches::Both;
                self.right.set_archetype(&state.right_state, archetype, tables);
            } else {
                self.matches = Matches::Left;
            }
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
            if right_match {
                self.matches = Matches::Both;
                self.right.set_table(&state.right_state, table);
            } else {
                self.matches = Matches::Left;
            }
        } else if right_match {
            self.matches = Matches::Right;
            self.right.set_table(&state.right_state, table);
        } else if cfg!(not(all(not(debug_assertions), unchecked))) {
            unreachable!("neither left nor right side matched. what?");
        }
    }

    unsafe fn archetype_fetch(&mut self, archetype_index: usize) -> Self::Item {
        match self.matches {
            Matches::Both => EitherBoth::Both(
                self.left.archetype_fetch(archetype_index),
                self.right.archetype_fetch(archetype_index),
            ),
            Matches::Left => EitherBoth::Left(self.left.archetype_fetch(archetype_index)),
            Matches::Right => EitherBoth::Right(self.right.archetype_fetch(archetype_index)),
        }
    }

    unsafe fn table_fetch(&mut self, table_row: usize) -> Self::Item {
        match self.matches {
            Matches::Both => EitherBoth::Both(
                self.left.table_fetch(table_row),
                self.right.table_fetch(table_row),
            ),
            Matches::Left => EitherBoth::Left(self.left.table_fetch(table_row)),
            Matches::Right => EitherBoth::Right(self.right.table_fetch(table_row)),
        }
    }
}

impl<T: WorldQuery, U: WorldQuery> WorldQuery for EitherBoth<T, U> {
    type Fetch = EitherBothFetch<T::Fetch, U::Fetch>;
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

    #[derive(Debug, PartialEq, Eq)]
    struct BothCount(u32);

    #[test]
    fn test_eitherboth() {
        let mut world = World::default();
        world.insert_resource(LeftCount(0));
        world.insert_resource(RightCount(0));
        world.insert_resource(BothCount(0));
        let (real_left_count, real_right_count, real_both_count) = push_entities(&mut world);
        let mut update_stage = SystemStage::single((|
                q: Query<EitherBoth<&LeftElem, &RightElem>>,
                mut l: ResMut<LeftCount>,
                mut r: ResMut<RightCount>,
                mut b: ResMut<BothCount>,
            | {
                for eb in q.iter() {
                    match eb {
                        EitherBoth::Left(_) => {
                            l.0 += 1;
                        },
                        EitherBoth::Right(_) => {
                            r.0 += 1;
                        },
                        EitherBoth::Both(_, _) => {
                            b.0 += 1;
                        },
                    }
                }
            }
        ).system());
        update_stage.run(&mut world);
        assert_eq!(world.get_resource::<LeftCount>().unwrap().0, real_left_count);
        assert_eq!(world.get_resource::<RightCount>().unwrap().0, real_right_count);
        assert_eq!(world.get_resource::<BothCount>().unwrap().0, real_both_count);
    }
}
