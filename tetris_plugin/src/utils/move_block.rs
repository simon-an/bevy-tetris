use bevy::prelude::*;

use crate::*;

#[deprecated]
pub(crate) fn move_block(
    e: &Entity,
    event: &MoveEvent,
    // shape: &ShapeEntity,
    coordinates: &Coordinates,
    // pos: &ShapePosition,
    map: &mut Map,
) -> Result<Option<(Coordinates, Tile)>, String> {
    debug!("move entity: {:?}", e);
    // let mut coords = shape.position_on_board.clone() - shape.anker.clone() + pos.clone();
    let mut coords = coordinates.clone();
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
}
