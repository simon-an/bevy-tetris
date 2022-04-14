// board_assets.rs
use bevy::prelude::*;
use bevy::render::texture::DEFAULT_IMAGE_HANDLE;

/// Material of a `Sprite` with a texture and color
#[derive(Debug, Clone)]
pub struct SpriteMaterial {
    pub color: Color,
    pub texture: Handle<Image>,
}

impl Default for SpriteMaterial {
    fn default() -> Self {
        Self {
            color: Color::WHITE,
            texture: DEFAULT_IMAGE_HANDLE.typed(),
        }
    }
}

/// Assets for the board. Must be used as a resource.
///
/// Use the loader for partial setup
#[derive(Debug, Clone)]
pub struct BoardAssets {
    // /// Label
    // pub label: String,
    ///
    pub board_material: SpriteMaterial,
    ///
    pub tile_material: SpriteMaterial,
    // ///
    // pub covered_tile_material: SpriteMaterial,
    // ///
    pub font: Handle<Font>,
    // ///
    // pub bomb_counter_colors: Vec<Color>,
    // ///
    // pub flag_material: SpriteMaterial,
    // ///
    // pub bomb_material: SpriteMaterial,
}

impl BoardAssets {
    /// Default bomb counter color set
    pub fn default_colors() -> Vec<Color> {
        vec![
            Color::WHITE,
            Color::GREEN,
            Color::YELLOW,
            Color::ORANGE,
            Color::PURPLE,
        ]
    }
}
