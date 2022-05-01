use std::{collections::BTreeMap, time::Duration};

use crate::*;
use bevy::prelude::*;
use bevy_tweening::{lens::TransformPositionLens, *};

pub(crate) fn animate(
    mut commands: Commands,
    board: Res<Board>,
    // mut map: ResMut<Map>,
    mut transitions: Option<ResMut<Transitions>>,
    // new_on_the_block: Query<(Entity, &Coordinates), Added<Block>>,
    all: Query<(Entity, &Coordinates, &mut Transform)>,
) {
    let c_e_map: BTreeMap<&Coordinates, (&Transform, Entity)> =
        all.iter().map(|(e, c, t)| (c, (t, e))).collect();
    if let Some(ts) = transitions.take() {
        commands.remove_resource::<Transitions>();
        for (from, to) in ts.0.iter() {
            // let gui_pos = self.calc_translation(&from);
            if let Some((t, e)) = c_e_map.get(&from) {
                let (new_x, new_y): (f32, f32) = board.calc_translation(&(to.x, to.y).into());
                // println!(
                //     "current translation: {:?}, new pos {:?}",
                //     gui_pos,
                //     (new_x, new_y)
                // );

                let tween = Tween::new(
                    // Use a quadratic easing on both endpoints.
                    EaseFunction::QuadraticInOut,
                    // Loop animation back and forth.
                    TweeningType::Once,
                    // Animation time (one way only; for ping-pong it takes 2 seconds
                    // to come back to start).
                    Duration::from_secs(1),
                    // The lens gives access to the Transform component of the Entity,
                    // for the Animator to animate it. It also contains the start and
                    // end values respectively associated with the progress ratios 0. and 1.
                    TransformPositionLens {
                        start: t.translation,
                        end: Vec3::new(new_x, new_y, 5.),
                    },
                );

                commands
                    .entity(*e)
                    // Add an Animator component to control and execute the animation.
                    .insert(Animator::new(tween))
                    .insert(*to);
            } else {
                panic!("from coordinates not found {}", from)
            }
        }
    }
}
