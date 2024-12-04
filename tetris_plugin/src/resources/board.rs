use crate::Bounds2;
use crate::Coordinates;
use bevy::prelude::*;

#[derive(Debug, Resource)]
pub(crate) struct Board {
    pub bounds: Bounds2,
    pub tile_size: f32,
    // pub current_tetromino_shape: Option<ShapeEntity>, // now a world resource
    pub entity: Entity,
}

impl Board {
    // TODO TEST THIS
    // pub fn set_map(&mut self, map: Map) {
    //     self.current_tetromino_shape = Some(map.try_parse_shape().expect("shape must be parsable"));
    //     self.map = map;
    // }

    pub fn calc_translation(&self, coordinates: &Coordinates) -> (f32, f32) {
        let new_x: f32 = (coordinates.x as f32 * self.tile_size) + (self.tile_size / 2.0);
        let new_y: f32 =
            self.bounds.size.y - (coordinates.y as f32 * self.tile_size) - 0.5 * self.tile_size;

        (new_x, new_y)
    }

    #[cfg(feature = "debug")]
    pub fn console_output(&self) -> String {
        let mut buffer = format!(
            "Board ({}, {}) with tile_size {}\n",
            self.bounds.size.x, self.bounds.size.y, self.tile_size
        );
        let map = self.map.as_tetris();
        buffer.push_str(&map);
        buffer
    }
}