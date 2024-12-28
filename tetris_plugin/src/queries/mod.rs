use bevy::{ecs::query::QueryData, prelude::*};

use crate::{Coordinates, CurrentTetromino, PreviewRef, ShapePosition, Tetromino};

#[derive(QueryData)]
pub struct CurrentTetrominoQuery {
    pub entity: Entity,
    pub tetromino: &'static Tetromino,
    pub coordinates: &'static Coordinates,
    pub shape_position: &'static ShapePosition,
    // pub shape_type: &'static ShapeType,
    pub _marker: &'static CurrentTetromino,
}
#[derive(QueryData)]
pub struct PreviewQuery {
    pub entity: Entity,
    pub _marker: &'static PreviewRef,
}
