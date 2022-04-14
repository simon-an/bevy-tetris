use bevy::prelude::*;

use crate::{Shape, ShapePosition, ShapeType, TileBlueprint};

// A block can be part of a tetromino. Stores the block's index within that
// tetromino for the purpose of rotation.
#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Debug, Component, Clone)]
pub struct Tetromino {
    pub color: Color,
    pub tetromino_type: ShapeType,
    pub index: ShapePosition,
}

impl Tetromino {
    #[deprecated]
    pub fn blocks_from_type(shape_type: ShapeType) -> Vec<Tetromino> {
        let blueprint = Shape::blueprint(shape_type);
        blueprint
            .positions
            .into_iter()
            .filter(|(_, b)| b == &TileBlueprint::CurrentTetromino)
            .map(|(pos, _)| Tetromino {
                color: shape_type.get_color(),
                index: pos,
                tetromino_type: shape_type,
            })
            .collect()
    }
}
