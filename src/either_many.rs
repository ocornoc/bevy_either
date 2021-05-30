#[macro_export]
macro_rules! either_many {
    (@__first $first:tt $(, $($others:tt),+)?) => {
        $first
    };
    (readonly $(#[$($m:meta),*])? $name:ident, $($varn:ident($($vart:tt)+)),+ $(, )?) => {
        $crate::either_many!($(#[$($m),*])? $name, $($varn($($vart)+)),+);

        $crate::exports::paste!{
            unsafe impl ::bevy::ecs::query::ReadOnlyFetch for [<__ $name:lower>]::[<$name Fetch>] {}
        }
    };
    ($(#[$($m:meta),*])? $name:ident, $($varn:ident($($vart:tt)+)),+ $(, )?) => {
        $crate::exports::paste!{
            $(#[$($m),*])?
            #[doc =
                "A [world query](::bevy::ecs::query::WorldQuery) allowing you to match one of "
                "multiple other [world queries](::bevy::ecs::query::WorldQuery).\n"
                "There is exactly one variant per [world query](::bevy::ecs::query::WorldQuery) "
                "that [`" $name "`] can fulfill.\n\nIn priority order, [`" $name "`] matches:"
                $("\n * [`" $name "::" $varn "(_)`](" $name "::" $varn ")")+
            ]
            pub enum $name {
                $($varn(<
                    <$($vart)+ as ::bevy::ecs::query::WorldQuery>::Fetch
                    as ::bevy::ecs::query::Fetch<'static>
                >::Item)),+
            }
        }

        $crate::exports::paste!{mod [<__ $name:lower>] {
            use super::*;
            use ::bevy::prelude::*;
            use ::bevy::ecs::{storage::*, component::*, archetype::*, query::*};

            #[derive(Copy, Clone)]
            enum Matches {
                $($varn),+
            }

            impl Matches {
                const FIRST: Self = {
                    use Matches::*;
                    $crate::either_many!(@__first $($varn),+)
                };
            }

            #[allow(non_snake_case)]
            pub struct [<$name State>] {
                $($varn: <$($vart)+ as WorldQuery>::State),+
            }

            unsafe impl FetchState for [<$name State>] {
                fn init(world: &mut World) -> Self {
                    Self {
                        $($varn: <$($vart)+ as WorldQuery>::State::init(world)),+
                    }
                }

                fn update_component_access(&self, access: &mut FilteredAccess<ComponentId>) {
                    $(self.$varn.update_component_access(access);)+
                }
            
                fn update_archetype_component_access(
                    &self,
                    archetype: &Archetype,
                    access: &mut Access<ArchetypeComponentId>,
                ) {
                    $(self.$varn.update_archetype_component_access(archetype, access);)+
                }
            
                fn matches_archetype(&self, archetype: &Archetype) -> bool {
                    $(self.$varn.matches_archetype(archetype))||+
                }
            
                fn matches_table(&self, table: &Table) -> bool {
                    $(self.$varn.matches_table(table))||+
                }
            }

            #[allow(non_snake_case)]
            pub struct [<$name Fetch>] {
                matches: Matches,
                $($varn: <$($vart)+ as WorldQuery>::Fetch),+
            }

            impl Fetch<'_> for [<$name Fetch>] {
                type Item = $name;
                type State = [<$name State>];

                fn is_dense(&self) -> bool {
                    $(self.$varn.is_dense())||+
                }
            
                unsafe fn init(
                    world: &World,
                    state: &Self::State,
                    last_change_tick: u32,
                    change_tick: u32,
                ) -> Self {
                    Self {
                        matches: Matches::FIRST,
                        $($varn: <$($vart)+ as WorldQuery>::Fetch::init(
                            world,
                            &state.$varn,
                            last_change_tick,
                            change_tick,
                        ),)+
                    }
                }
            
                unsafe fn set_archetype(
                    &mut self,
                    state: &Self::State,
                    archetype: &Archetype,
                    tables: &Tables,
                ) {
                    $(if state.$varn.matches_archetype(archetype) {
                        self.$varn.set_archetype(&state.$varn, archetype, tables);
                        self.matches = {
                            use Matches::*;
                            $varn
                        };
                    })else+ else if cfg!(debug_assertions) {
                        unreachable!("None of the variants were matched. At least one should be.");
                    }
                }
            
                unsafe fn set_table(&mut self, state: &Self::State, table: &Table) {
                    $(if state.$varn.matches_table(table) {
                        self.$varn.set_table(&state.$varn, table);
                        self.matches = {
                            use Matches::*;
                            $varn
                        };
                    })else+ if cfg!(debug_assertions) {
                        unreachable!("None of the variants were matched. At least one should be.");
                    }
                }
            
                unsafe fn archetype_fetch(&mut self, archetype_index: usize) -> Self::Item {
                    use Matches::*;

                    match self.matches {
                        $($varn => $name::$varn(self.$varn.archetype_fetch(archetype_index)),)+
                    }
                }
            
                unsafe fn table_fetch(&mut self, table_row: usize) -> Self::Item {
                    use Matches::*;

                    match self.matches {
                        $($varn => $name::$varn(self.$varn.table_fetch(table_row)),)+
                    }
                }
            }

            impl WorldQuery for $name {
                type Fetch = [<$name Fetch>];
                type State = [<$name State>];
            }
        }}
    };
}
