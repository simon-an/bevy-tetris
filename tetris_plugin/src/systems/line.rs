use std::collections::BTreeMap;

use bevy::prelude::*;

use crate::{Block, Board, Coordinates};

pub(crate) fn line(
    mut commands: Commands,
    mut board: ResMut<Board>,
    new_on_the_block: Query<(Entity, &Coordinates), Added<Block>>,
    all: Query<(Entity, &Coordinates, &mut Transform)>,
) {
    let mut lines: Vec<u16> = vec![];
    let c_e_map: BTreeMap<&Coordinates, Entity> = all.iter().map(|(e, c, _)| (c, e)).collect();

    assert_eq!(c_e_map.len(), board.map.occupied());

    let c_t_map: BTreeMap<_, _> = all.iter().map(|(e, c, t)| (c, (t, e))).collect();
    assert_eq!(c_t_map.len(), board.map.occupied());

    for (entity, coordinates) in new_on_the_block.iter() {
        trace!("BLOCK {:?} {coordinates}", entity);
        if !lines.contains(&coordinates.y) {
            if board.map.is_line_full(coordinates.y) {
                lines.push(coordinates.y);
            }
        }
    }

    if lines.is_empty() {
        return;
    }
    info!("Deleting lines: {:?}", lines);

    let coordinates: Vec<Coordinates> = board.map.despawn_lines(lines);
    for c in coordinates {
        if let Some(e) = c_e_map.get(&c) {
            commands.entity(*e).despawn_recursive();
        } else {
            println!("{:?}", c_e_map);
            println!("{:?}", c_t_map);
            panic!(
                "coordinate not found in query for entity and transform {}",
                c
            )
        }
    }
    let _lines = board.map.move_blocks_above_empty_lines();

    board.animate(c_t_map, commands);
}
