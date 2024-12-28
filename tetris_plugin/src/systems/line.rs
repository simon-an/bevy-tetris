use std::collections::BTreeMap;

use bevy::prelude::*;

use crate::{Block, Coordinates, Map, ScoreEvent};

pub(crate) fn delete_line(
    mut commands: Commands,
    mut map: ResMut<Map>,
    new_on_the_block: Query<(Entity, &Coordinates), Added<Block>>,
    all: Query<(Entity, &Coordinates, &mut Transform)>,
    mut score: EventWriter<ScoreEvent>,
) {
    let mut lines: Vec<u16> = vec![];

    let entities: BTreeMap<&Coordinates, Entity> = all.iter().map(|(e, c, _)| (c, e)).collect();
    // assert_eq!(c_e_map.len(), board.map.occupied());

    if entities.len() != map.occupied() {
        // TODO this can happen when game is over so lets clean up map
        error!(
            "entity query does not match map state. entities in world is not equal map.occupied: {} {}",
            entities.len(),
            map.occupied()
        );
        error!("query does not match map state {}", *map);
        return;
    }

    // let c_t_map: BTreeMap<_, _> = all.iter().map(|(e, c, t)| (c, (t, e))).collect();
    // // assert_eq!(c_t_map.len(), board.map.occupied());
    // if c_t_map.len() != map.occupied() {
    //     error!(
    //         "entity query does not match map state. entities in world is not equal map.occupied: {} {}",
    //         c_t_map.len(),
    //         map.occupied()
    //     );
    //     error!("query does not match map state {}", *map);
    //     return;
    // }

    for (entity, coordinates) in new_on_the_block.iter() {
        trace!("BLOCK {:?} {coordinates}", entity);
        if !lines.contains(&coordinates.y) {
            if map.is_line_full(coordinates.y) {
                info!("Line is full: {}", coordinates.y);
                lines.push(coordinates.y);
            }
        }
    }

    if lines.is_empty() {
        return;
    }
    info!("Deleting lines: {:?}", lines);

    let coordinates: Vec<Coordinates> = map.set_lines_to_empty(lines);
    for c in coordinates {
        if let Some(e) = entities.get(&c) {
            commands.entity(*e).despawn_recursive();
            score.send(ScoreEvent(100));
        } else {
            println!("{:?}", entities);
            // println!("{:?}", c_t_map);
            panic!(
                "coordinate {} not found in query for entity and transform",
                c
            )
        }
    }
    if let Some((_lines, transitions)) = map.move_blocks_above_empty_lines() {
        commands.insert_resource(transitions); // TODO it might be better to use components add attach them to the entities being moved instead of using a single resource. Depends on animation implementation
    }
}
