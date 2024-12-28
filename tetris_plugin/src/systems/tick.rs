use bevy::prelude::*;

use crate::{Coordinates, CurrentTetromino, Map, ShapePosition, Tetromino, TickCounter, Tile};

pub(crate) fn tick_counter(mut timer: ResMut<TickCounter>) {
    (*timer).0 += 1;
    debug!("tick_counter: {}", (*timer).0);
}

pub(crate) fn tick(
    mut commands: Commands,
    mut map: ResMut<Map>,
    mut current_query: Query<(
        Entity,
        &Tetromino,
        &CurrentTetromino,
        &Coordinates,
        &mut ShapePosition,
    )>,
) {
    if map.detect_collision().is_some() {
        info!("Skipping tick due to collision. This means the tetromino is moved by user to the last available position");
        return;
    }
    #[cfg(feature = "debug")]
    bevy::log::info!("{}", (*map));
    let vec: Vec<(Entity, _, _, _, _)> = current_query.iter_mut().collect();

    let mut changes = vec![];
    for (entity, _, _current, coordinates, mut _pos) in vec.into_iter() {
        let target: Option<(Coordinates, Tile)> =
            crate::utils::move_block(&entity, &crate::MoveEvent::Down, coordinates, &mut map)
                .expect("block must move");
        debug!("move result {:?}", target);
        if let Some((coordinate, tile)) = target {
            changes.push((coordinate, tile));
            // commands.entity(entity).remove::<Coordinates>();insert overwrites components
            commands.entity(entity).insert(coordinate);
            // update_block_sprites(&mut transform, &coordinate); this is a proper system now
        }
    }

    // insert tile after ALL previous one have been set to empty
    changes.into_iter().for_each(|(coordinates, tile)| {
        let mustbe_empty_tile = map.insert(coordinates, tile).unwrap();
        assert_eq!(mustbe_empty_tile, Tile::Empty);
    });

    #[cfg(feature = "debug")]
    bevy::log::info!("{}", (*map));
}
