use bevy::prelude::*;

use crate::{
    components::Block, update_block_sprites, Board, CollisionDetection, Coordinates,
    CurrentTetromino, SpawnEvent, Tetromino, TickEvent, Tile,
};

pub(crate) fn tock(
    mut commands: Commands,
    mut board: ResMut<Board>,
    time: Res<Time>,
    mut timer: ResMut<TickEvent>,
    mut current_query: Query<(
        Entity,
        &mut Tetromino,
        &CurrentTetromino,
        &Coordinates,
        &mut Transform,
    )>,
    mut spawn_ewr: EventWriter<SpawnEvent>,
) {
    timer.0.tick(time.delta());
    // for event in move_event_rdr.iter() {
    //     println!("moved into {:?}", event);
    // }

    if timer.0.just_finished() {
        #[cfg(feature = "debug")]
        bevy::log::info!("{}", (*board).console_output());
        let vec: Vec<(Entity, Mut<Tetromino>, &CurrentTetromino, &Coordinates, _)> =
            current_query.iter_mut().collect();

        let collisions = board.map.detect_collision();
        debug!("Collition {:?}", collisions);
        debug!("{}", board.map);

        if let Some(
            CollisionDetection::Bottom
            | CollisionDetection::Block
            | CollisionDetection::OutOfBounds,
        ) = collisions
        {
            let c = board
                .current_tetromino_shape
                .as_ref()
                .unwrap()
                .shape_type
                .as_char();
            for (entity, _, _current, coordinates, _) in vec.into_iter() {
                commands.entity(entity).remove::<CurrentTetromino>();
                commands.entity(entity).remove::<Tetromino>();
                let _tile = board
                    .map
                    .insert(*coordinates, Tile::Block(c))
                    .expect("tile must exist");
                commands.entity(entity).insert(Block {});
                // *tile = Tile::Block(tetromino.color.into(), entity)
            }
            board.current_tetromino_shape = None;
            spawn_ewr.send(SpawnEvent);
            return;
        }

        let mut changes = vec![];
        for (entity, _, _current, _coordinates, mut transform) in vec.into_iter() {
            let target: Option<(Coordinates, Tile)> = board
                .move_block(&entity, &crate::MoveEvent::Down)
                .expect("block must move");
            debug!("move result {:?}", target);
            if let Some((coordinate, tile)) = target {
                update_block_sprites(&mut transform, &coordinate, &board);
                changes.push((coordinate, tile));
                commands.entity(entity).remove::<Coordinates>();
                commands.entity(entity).insert(coordinate);
            }
        }
        board.move_shape(&crate::MoveEvent::Down);

        // insert tile after ALL previous one have been set to empty
        changes.into_iter().for_each(|(coordinates, tile)| {
            let mustbe_empty_tile = board.map.insert(coordinates, tile).unwrap();
            assert_eq!(mustbe_empty_tile, Tile::Empty);
        });

        #[cfg(feature = "debug")]
        bevy::log::info!("{}", (*board).console_output());
    }
}
