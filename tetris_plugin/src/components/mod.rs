pub use block::*;
pub use coordinates::*;
pub use tetromino::*;
mod block;
mod coordinates;
mod tetromino;

use bevy::prelude::*;

#[derive(Debug, Clone, Copy)]

pub struct Matrix {
    pub width: u16,
    pub height: u16,
}

// A block can be part of the currently controlled tetromino.
#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Component)]
pub struct CurrentTetromino;