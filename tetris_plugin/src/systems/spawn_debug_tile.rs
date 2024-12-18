// use std::time::Duration;

// use bevy_tweening::{lens::TransformPositionLens, Animator, EaseFunction, TweeningType};

// use bevy::prelude::*;
// use bevy::css::Color;
// // use bevy_tweening::Tween;

// use crate::{components::Coordinates, Board, BoardOptions, TetrisPlugin, TileSize};

// pub(crate) fn spawn_debug_block(
//     mut commands: Commands,
//     options: ResMut<BoardOptions>,
//     window: Query<&Window>,
// ) {
//     // let tween = Tween::new(
//     //     // Use a quadratic easing on both endpoints.
//     //     EaseFunction::CircularInOut,
//     //     // Loop animation back and forth.
//     //     TweeningType::Once,
//     //     // Animation time (one way only; for ping-pong it takes 2 seconds
//     //     // to come back to start).
//     //     Duration::from_secs(1),
//     //     // The lens gives access to the Transform component of the Entity,
//     //     // for the Animator to animate it. It also contains the start and
//     //     // end values respectively associated with the progress ratios 0. and 1.
//     //     TransformPositionLens {
//     //         start: Vec3::ZERO,
//     //         end: Vec3::new(23., 23., 5.),
//     //     },
//     // );
//     let tile_size = match options.tile_size {
//         TileSize::Fixed(v) => v,
//         TileSize::Adaptive { min, max } => {
//             crate::window::adaptative_tile_size(window, (min, max), &options)
//         }
//     };

//     let entity = commands.spawn(
//         (
//             Sprite {
//                 color: RED,
//                 custom_size: Some(Vec2::splat(tile_size)),
//                 ..Default::default()
//             },
//             Transform::from_translation(Vec3::new(0.0, 0.0, 5.0)),
//             Name::new(format!("DEBUG BLOCK")),
//         ), // .insert(Animator::new(tween))
//            // .insert(Play)
//            // .insert(coordinates.clone())
//            // .insert(block.clone())
//     );
// }
