use crate::{CurrentTetromino, PrintInfoTimer, Tetromino};
use bevy::prelude::*;

pub(crate) fn print_info(
    time: Res<Time>,
    mut timer: ResMut<PrintInfoTimer>,
    mut current_query: Query<(Entity, &Tetromino, &CurrentTetromino)>,
) {
    timer.0.tick(time.delta());
    if timer.0.just_finished() {
        for (entity, tetromino, current) in current_query.iter_mut() {
            println!("entiry {:?}", entity);
            println!(
                "{:?} Current tetromino: {:?} part of shape: {:?}",
                entity, tetromino, tetromino.tetromino_type
            );
        }
    }
}
