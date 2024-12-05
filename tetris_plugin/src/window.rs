use bevy::window::Window;

use crate::BoardOptions;

/// Computes a tile size that matches the window according to the tile map size
pub fn adaptative_tile_size(
    window: &Window,
    (min, max): (f32, f32), // Tile size constraints
    options: &BoardOptions,
) -> f32 {
    let max_window_x = window.width() - 200.0 - options.tile_padding * options.map_size.0 as f32;
    let max_window_y = window.height() - options.tile_padding * options.map_size.1 as f32;
    let max_width = max_window_x / options.map_size.0 as f32;
    let max_heigth = max_window_y / options.map_size.1 as f32;
    max_width.min(max_heigth).clamp(min, max)
}
