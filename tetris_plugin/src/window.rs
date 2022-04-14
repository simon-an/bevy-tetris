use bevy::window::WindowDescriptor;
use bevy::ecs::system::Res;

/// Computes a tile size that matches the window according to the tile map size
pub fn adaptative_tile_size(
    window: Res<WindowDescriptor>,
    (min, max): (f32, f32),      // Tile size constraints
    (width, height): (u16, u16), // Tile map dimensions
) -> f32 {
    let max_width = window.width / width as f32;
    let max_heigth = window.height / height as f32;
    max_width.min(max_heigth).clamp(min, max)
}
