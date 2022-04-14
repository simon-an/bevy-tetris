use bevy::prelude::*;

use crate::{Board, Coordinates, CurrentTetromino, RotateEvent, Tetromino, Tile};

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
    mut rotate_event_rdr: EventReader<RotateEvent>,
    mut current_query: Query<(Entity, &mut Tetromino, &CurrentTetromino, &mut Transform)>,
) {
    for event in rotate_event_rdr.iter() {
        let all_clear = board.is_free(&event);
        if all_clear {
            let mut changes = vec![];
            for (entity, _, _current, mut transform) in current_query.iter_mut() {
                debug!("entity {:?}", entity);
                let target: Option<(Coordinates, Tile)> = board.rotate_block(&entity, &event);
                info!("rotation result {:?}", target);
                if let Some((coordinate, tile)) = target {
                    update_block_sprites(&mut transform, &coordinate, &board);
                    changes.push((coordinate, tile));
                }
            }

            // insert tile after ALL previous one have been set to empty
            changes.into_iter().for_each(|(coordinates, tile)| {
                let mustbe_empty_tile = board.map.insert(coordinates, tile).unwrap();
                assert_eq!(mustbe_empty_tile, Tile::Empty);
            });
        } else {
            info!("board is not free for rotation");
            return;
        }
        #[cfg(feature = "debug")]
        bevy::log::info!("{}", (*board).console_output());
    }
}
