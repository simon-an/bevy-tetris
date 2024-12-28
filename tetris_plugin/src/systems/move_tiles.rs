use bevy::prelude::*;

use crate::{Board, Coordinates, CurrentTetromino, Map, MoveEvent, ShapePosition, Tetromino, Tile};

pub(crate) fn update_block_sprites_translation(
    board: Res<Board>,
    mut current_query: Query<(&Coordinates, &mut Transform)>,
) {
    current_query
        .iter_mut()
        .for_each(|(coordinates, mut transform)| {
            let (new_x, new_y): (f32, f32) = board.calc_translation(&coordinates);

            let translation = &mut transform.translation;
            translation.x = new_x;
            translation.y = new_y;
        });
}

pub(crate) fn move_current(
    mut commands: Commands,
    mut map: ResMut<Map>,
    asset_server: Res<AssetServer>,
    // mut shape: Option<ResMut<ShapeEntity>>,
    mut move_event_rdr: EventReader<MoveEvent>,
    mut current_query: Query<(
        Entity,
        &mut Tetromino,
        &CurrentTetromino,
        &Coordinates,
        &mut ShapePosition,
    )>,
) {
    for event in move_event_rdr.read() {
        let collision = map.detect_move_collision(event);
        info!("move_tiles - Collision {:?}", collision);

        if let Some(_) = collision {
            commands.spawn((AudioPlayer::<AudioSource>(asset_server.load("error.mp3")),));
            return;
        }

        // if let Some(shape) = shape.as_deref_mut() {
        let vec: Vec<(_, _, _, _, _)> = current_query.iter_mut().collect();

        // Move the tetromino tiles
        let mut changes = vec![];

        for (entity, _, _current, coordinates, mut _pos) in vec.into_iter() {
            let removed_tile = map.insert(*coordinates, Tile::Empty);
            if removed_tile.is_none() {
                commands.spawn((AudioPlayer::<AudioSource>(asset_server.load("error.mp3")),));
                error!("tile not found");
                return;
            }
            let tile = removed_tile.unwrap();
            let new_coords = match event {
                &MoveEvent::Down => Coordinates {
                    x: coordinates.x,
                    y: coordinates.y + 1,
                },
                &MoveEvent::Left => Coordinates {
                    x: coordinates.x - 1,
                    y: coordinates.y,
                },
                &MoveEvent::Right => Coordinates {
                    x: coordinates.x + 1,
                    y: coordinates.y,
                },
            };
            info!("Moving ({}) -> ({})", coordinates, new_coords);
            // let target: Option<(Coordinates, Tile)> =
            //     match move_block(&entity, &event, &shape, &mut pos, &mut map) {
            //         Ok(target) => target,
            //         Err(e) => {
            //             commands.spawn((AudioPlayer::<AudioSource>(
            //                 asset_server.load("error.mp3"),
            //             ),));
            //             error!("moving block failed {}", e);
            //             None
            //         }
            //     };
            changes.push((new_coords, tile));
            commands.entity(entity).insert(new_coords);
        }

        // shape.move_shape(event);

        // insert tile after ALL previous one have been set to empty
        changes.into_iter().for_each(|(coordinates, tile)| {
            if let Some(mustbe_empty_tile) = map.insert(coordinates, tile) {
                assert_eq!(mustbe_empty_tile, Tile::Empty); // check if the tile has been empty before moving the new tile to this position
            } else {
                panic!("position {coordinates} not found in map");
            }
        });

        #[cfg(feature = "debug")]
        bevy::log::info!("{}", (*map));
        // }
    }
}
