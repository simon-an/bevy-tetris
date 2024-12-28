use bevy::prelude::*;

use crate::{
    queries::{self, CurrentTetrominoQueryItem},
    Block, CurrentTetromino, Map, Tetromino, Tile,
};

pub(crate) fn convert_to_block(
    mut commands: Commands,
    mut map: ResMut<Map>,
    mut query: Query<queries::CurrentTetrominoQuery>,
) {
    for CurrentTetrominoQueryItem {
        entity,
        coordinates,
        tetromino,
        ..
    } in query.iter_mut()
    {
        commands.entity(entity).remove::<CurrentTetromino>();
        commands.entity(entity).remove::<Tetromino>(); // why we remove this?
        let _tile = map
            .insert(
                *coordinates,
                Tile::Block(tetromino.tetromino_type.as_char()),
            )
            .expect("tile must exist");
        commands.entity(entity).insert(Block {});
    }
}
