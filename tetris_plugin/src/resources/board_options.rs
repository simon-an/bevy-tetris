use bevy::prelude::{Resource, Vec3};
use serde::{Deserialize, Serialize};

/// Tile size options
#[derive(Debug, Clone, Serialize, Deserialize, Resource)]
pub enum TileSize {
    /// Fixed tile size
    Fixed(f32),
    /// Window adaptative tile size
    Adaptive { min: f32, max: f32 },
}

/// Board position customization options
#[derive(Debug, Clone, Serialize, Deserialize, Resource)]
pub enum BoardPosition {
    /// Centered board
    Centered { offset: Vec3 },
    /// Custom position
    Custom(Vec3),
}

/// Board generation options. Must be used as a resource
// We use serde to allow saving option presets and loading them at runtime
#[derive(Debug, Clone, Serialize, Deserialize, Resource)]
pub struct BoardOptions {
    /// Tile map size
    pub map_size: (u16, u16),
    /// Board world position
    pub position: BoardPosition,
    /// Tile world size
    pub tile_size: TileSize,
    /// Padding between tiles
    pub tile_padding: f32,
}

impl Default for TileSize {
    fn default() -> Self {
        Self::Adaptive {
            min: 10.0,
            max: 50.0,
        }
    }
}

impl Default for BoardPosition {
    fn default() -> Self {
        Self::Centered {
            offset: Default::default(),
        }
    }
}

impl Default for BoardOptions {
    fn default() -> Self {
        Self {
            map_size: (15, 15),
            position: Default::default(),
            tile_size: Default::default(),
            tile_padding: 2.,
        }
    }
}
