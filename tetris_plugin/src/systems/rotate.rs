use bevy::prelude::*;

use crate::{Board, Coordinates, CurrentTetromino, Map, RotateEvent, ShapeEntity, Tetromino, Tile};

pub(crate) fn update_block_sprites(
    transform: &mut Mut<Transform>,
    coordinates: &Coordinates,
    board: &ResMut<Board>,
) {
    let (new_x, new_y): (f32, f32) = board.calc_translation(coordinates);

    let translation = &mut transform.translation;
    // println!("translation: {translation}");
    translation.x = new_x;
    translation.y = new_y;
    // println!("new translation: {translation}");
}

pub(crate) fn rotate(
    _commands: Commands,
    mut board: ResMut<Board>,
    mut map: ResMut<Map>,
    mut shapeEntity: Option<ResMut<ShapeEntity>>,
    mut rotate_event_rdr: EventReader<RotateEvent>,
    mut current_query: Query<(Entity, &mut Tetromino, &CurrentTetromino, &mut Transform)>,
) {
    if let Some(mut shapeEntity) = shapeEntity {
        for event in rotate_event_rdr.iter() {
            let all_clear = board.is_free(&event); // TODO does not prevent panic in line 42
            if all_clear {
                let mut changes = vec![];
                for (entity, _, _current, mut transform) in current_query.iter_mut() {
                    debug!("entity {:?}", entity);
                    let target: Option<(Coordinates, Tile)> =
                        rotate_block(&entity, &event, &mut shapeEntity, &mut map);
                    info!("rotation result {:?}", target);
                    if let Some((coordinate, tile)) = target {
                        update_block_sprites(&mut transform, &coordinate, &board);
                        changes.push((coordinate, tile));
                    }
                }

                // insert tile after ALL previous one have been set to empty
                changes.into_iter().for_each(|(coordinates, tile)| {
                    let mustbe_empty_tile = map.insert(coordinates, tile).unwrap();
                    assert_eq!(mustbe_empty_tile, Tile::Empty); // TODO this can be a collision with an existing block
                });
            } else {
                info!("board is not free for rotation");
                return;
            }
            #[cfg(feature = "debug")]
            bevy::log::info!("{}", (*board).console_output());
        }
    }
}

pub(crate) fn rotate_block(
    e: &Entity,
    event: &RotateEvent,
    shape: &mut ShapeEntity,
    map: &mut Map,
) -> Option<(Coordinates, Tile)> {
    let mut coords = shape.shapepos_as_coords(shape.positions.get(e).unwrap());

    match shape.shape_type {
        crate::ShapeType::O => return None,
        crate::ShapeType::I => return None, // TODO
        _ => (),
    }

    println!("rotating entity: {:?}", e);
    let pos = shape.positions.get_mut(e).expect("Entity must be known");

    let orig_x = pos.x;
    let orig_y = pos.y;
    println!("index {:?}", pos);
    if orig_x == 1 || orig_y == 1 {
        if let &RotateEvent::ClockWise = event {
            pos.x = match orig_x {
                0 => 1,
                1 if orig_y == 0 => 2,
                1 if orig_y == 2 => 0,
                2 => 1,
                _ => 1,
            };
            pos.y = match orig_y {
                0 => 1,
                1 if orig_x == 0 => 0,
                1 if orig_x == 2 => 2,
                2 => 1,
                _ => 1,
            };
        } else {
            pos.x = match orig_x {
                0 => 1,
                1 if orig_y == 0 => 0,
                1 if orig_y == 2 => 2,
                2 => 1,
                _ => 1,
            };
            pos.y = match orig_y {
                0 => 1,
                1 if orig_x == 0 => 2,
                1 if orig_x == 2 => 0,
                2 => 1,
                _ => 1,
            };
        }
    } else {
        if let &RotateEvent::ClockWise = event {
            let (x, y) = match (orig_x, orig_y) {
                (0, 0) => (2, 0),
                (2, 0) => (2, 2),
                (2, 2) => (0, 2),
                (0, 2) => (0, 0),
                _ => (1, 1),
            };
            pos.x = x;
            pos.y = y;
        } else {
            let (x, y) = match (orig_x, orig_y) {
                (0, 0) => (0, 2),
                (0, 2) => (2, 2),
                (2, 2) => (2, 0),
                (2, 0) => (0, 0),
                _ => (1, 1),
            };
            pos.x = x;
            pos.y = y;
        }
    }

    println!("new index {:?}", pos);

    let delta_x: i16 = pos.x as i16 - orig_x as i16;
    let delta_y: i16 = pos.y as i16 - orig_y as i16;

    println!("new index {:?}. delta_x {delta_x}, delta_y {delta_y}", pos);

    if delta_x.is_negative() {
        coords.x -= (-delta_x) as u16;
    } else {
        coords.x += delta_x as u16;
    }
    if delta_y.is_negative() {
        coords.y -= (-delta_y) as u16;
    } else {
        coords.y += delta_y as u16;
    }

    // if delta_x == 0 && delta_y == 0 {
    //     None
    // } else {
    let tile = map
        .insert(
            shape.position_on_board.clone() + (orig_x, orig_y),
            Tile::Empty,
        )
        .unwrap();
    Some((coords, tile))
    // }
}
