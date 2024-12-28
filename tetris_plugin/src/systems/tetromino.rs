use bevy::prelude::*;

use crate::Tetromino;

pub(crate) fn update_block_sprites_colors(mut query: Query<(&Tetromino, &mut Sprite)>) {
    query.iter_mut().for_each(|(t, mut s)| {
        s.color = t.color;
    });
}
