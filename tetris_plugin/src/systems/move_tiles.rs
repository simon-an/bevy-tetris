use bevy::prelude::*;

use crate::{
    components::Block, update_block_sprites, utils::move_block, Board, CollisionDetection,
    Coordinates, CurrentTetromino, Map, MoveEvent, ShapeEntity, ShapePosition, SpawnEvent,
    Tetromino, Tile,
};

pub(crate) fn gogo(
    mut commands: Commands,
    mut board: ResMut<Board>,
    mut map: ResMut<Map>,
    mut shape: Option<ResMut<ShapeEntity>>,
    mut move_event_rdr: EventReader<MoveEvent>,
    mut current_query: Query<(
        Entity,
        &mut Tetromino,
        &CurrentTetromino,
        &Coordinates,
        &mut ShapePosition,
        &mut Transform,
    )>, // TODO: Remove coodinates as component. only part of board
    mut spawn_ewr: EventWriter<SpawnEvent>,
) {
    for event in move_event_rdr.read() {
        if let Some(shape) = shape.as_deref_mut() {
            let vec: Vec<(_, _, _, _, _, _)> = current_query.iter_mut().collect();

            let collision = map.detect_move_collision(event);
            debug!("Collition {:?}", collision);

            if let Some(CollisionDetection::Bottom | CollisionDetection::Block) = collision {
                let shape_as_char = shape.shape_type.as_char();
                for (entity, _, _current, coordinates, _, _) in vec.into_iter() {
                    commands.entity(entity).remove::<CurrentTetromino>();
                    commands.entity(entity).remove::<Tetromino>();
                    let _tile = map
                        .insert(*coordinates, Tile::Block(shape_as_char))
                        .expect("tile must exist");
                    commands.entity(entity).insert(Block {});
                    // *tile = Tile::Block(tetromino.color.into(), entity)
                }
                commands.remove_resource::<ShapeEntity>();
                spawn_ewr.send(SpawnEvent);
                return;
            } else if let Some(CollisionDetection::OutOfBounds) = collision {
                return;
            }

            let mut changes = vec![];
            for (entity, _, _current, _coordinates, mut pos, mut transform) in vec.into_iter() {
                debug!("entity {:?}", entity);
                let target: Option<(Coordinates, Tile)> =
                    match move_block(&entity, &event, &shape, &mut pos, &mut map) {
                        Ok(target) => target,
                        Err(e) => {
                            error!("moving block failed {}", e);
                            None
                        }
                    };
                debug!("move result {:?}", target);
                if let Some((coordinate, tile)) = target {
                    update_block_sprites(&mut transform, &coordinate, &board);
                    changes.push((coordinate, tile));
                    commands.entity(entity).remove::<Coordinates>();
                    commands.entity(entity).insert(coordinate);
                }
            }

            shape.move_shape(event);

            // insert tile after ALL previous one have been set to empty
            changes.into_iter().for_each(|(coordinates, tile)| {
                let mustbe_empty_tile = map.insert(coordinates, tile).unwrap();
                assert_eq!(mustbe_empty_tile, Tile::Empty);
            });

            // #[cfg(feature = "debug")]
            // bevy::log::info!("{}", (*board).console_output());
        }
    }
}
