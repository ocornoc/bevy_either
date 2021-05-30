use bevy::prelude::*;
use bevy_either::EitherBoth;

#[derive(Clone, Copy)]
pub struct LeftElem;

#[derive(Clone, Copy)]
pub struct RightElem;

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

bevy_either::either_many!(readonly MyEither, Left(&'static LeftElem), Right(&'static RightElem));

#[test]
fn main() {
    let mut world = World::default();
    world.insert_resource(LeftCount(0));
    world.insert_resource(RightCount(0));
    let (real_left_count, real_right_count, real_both_count) = push_entities(&mut world);
    let mut update_stage = SystemStage::single((|
            q: Query<MyEither>,
            mut l: ResMut<LeftCount>,
            mut r: ResMut<RightCount>,
        | {
            for eb in q.iter() {
                match eb {
                    MyEither::Left(_) => {
                        l.0 += 1;
                    },
                    MyEither::Right(_) => {
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