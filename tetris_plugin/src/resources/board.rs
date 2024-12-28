use crate::Bounds2;
use crate::Coordinates;
use bevy::prelude::*;

#[derive(Debug, Resource)]
pub(crate) struct Board {
    pub bounds: Bounds2,
    pub tile_size: f32,
    // pub current_tetromino_shape: Option<ShapeEntity>, // now a world resource
    #[deprecated] // use Query<Entity, With<TileMapRoot>> instead
    pub entity: Entity,
}

impl Board {
    pub fn calc_translation(&self, coordinates: &Coordinates) -> (f32, f32) {
        let new_x: f32 = self.bounds.position.x
            + (coordinates.x as f32 * self.tile_size)
            + (self.tile_size / 2.0);
        let new_y: f32 = -self.bounds.position.y
            - (coordinates.y as f32 * self.tile_size)
            - 0.5 * self.tile_size;
        (new_x, new_y)
    }

    pub fn calc_transform(&self, coordinates: &Coordinates) -> Transform {
        let new_x: f32 = self.bounds.position.x
            + (coordinates.x as f32 * self.tile_size)
            + (self.tile_size / 2.0);
        let new_y: f32 =
            self.bounds.position.y + (coordinates.y as f32 * self.tile_size) + 0.5 * self.tile_size;

        Transform::from_translation(Vec3::new(new_x, new_y, 50.0))
    }
}
