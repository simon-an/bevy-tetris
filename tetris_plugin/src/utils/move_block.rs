use bevy::prelude::*;

use crate::*;

pub(crate) fn move_block(
    e: &Entity,
    event: &MoveEvent,
    shape: &ShapeEntity,
    map: &mut Map,
) -> Result<Option<(Coordinates, Tile)>, String> {
    // if let ShapeEntity {
    //     anker,
    //     position_on_board,
    //     positions,
    //     ..
    // } = &mut shape
    // {
    debug!("move entity: {:?}", e);
    let pos = shape.positions.get(e);
    if pos.is_none() {
        return Err("enitity not found".to_string());
    }
    let pos = pos.unwrap();
    let mut coords = shape.position_on_board.clone() - shape.anker.clone() + pos.clone();
    let tile = map.insert(coords, Tile::Empty);
    if tile.is_none() {
        return Err("tile not found".to_string());
    }
    let tile = tile.unwrap();

    match event {
        &MoveEvent::Down => {
            coords.y += 1;
        }
        &MoveEvent::Left => {
            coords.x -= 1;
        }
        &MoveEvent::Right => {
            coords.x += 1;
        }
    };
    Ok(Some((coords, tile)))
    // } else {
    //     Ok(None)
    // }
}
