use colored::Colorize;
use core::fmt;
use std::fmt::{Display, Formatter};

/// Enum describing a Minesweeper tile
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Color(pub u8, pub u8, pub u8);

impl From<bevy::prelude::Color> for Color {
    fn from(c: bevy::prelude::Color) -> Self {
        Self(
            (c.r() * 255.0) as u8,
            (c.g() * 255.0) as u8,
            (c.b() * 255.0) as u8,
        )
    }
}
impl From<Color> for bevy::prelude::Color {
    fn from(c: Color) -> Self {
        bevy::prelude::Color::rgb_u8(c.0, c.1, c.2)
    }
}

// #[derive(Debug, Copy, Clone, Eq, PartialEq)]
// pub enum Tile {
//     /// Is movable
//     CurrentTetromino(Color, Entity),
//     // not movable
//     Block(Color, Entity),
//     /// Empty tile
//     Empty,
// }

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub enum Tile {
    /// Is movable
    CurrentTetromino(char),
    // not movable
    Block(char),
    /// Empty tile
    Empty,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub enum TileBlueprint {
    CurrentTetromino,
    Empty,
}
impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                // Tile::Block(Color(r, g, b), e) => format!("[{:?}]", e).truecolor(*r, *g, *b),
                // Tile::CurrentTetromino(Color(r, g, b), e) =>
                Tile::Block(c) => format!("{c}").red(),
                Tile::CurrentTetromino(c) => format!("{c}").to_ascii_uppercase().green(),
                Tile::Empty => " ".normal(),
            }
        )
    }
}

impl Tile {
    /// Is the tile a bomb?
    pub const fn is_block(&self) -> bool {
        // matches!(self, Self::Block(_, _))
        matches!(self, Self::Block(_))
    }
    pub const fn is_moveable(&self) -> bool {
        // matches!(self, Self::CurrentTetromino(_, _))
        matches!(self, Self::CurrentTetromino(_))
    }

    // #[cfg(feature = "debug")]
    // pub fn console_output(&self) -> String {
    //     format!(
    //         "{}",
    //         match self {
    //             Tile::Block(Color(r, g, b), e) => format!("[{:?}]", e).truecolor(*r, *g, *b),
    //             Tile::CurrentTetromino(Color(r, g, b), e) =>
    //                 format!("<{:?}>", e).truecolor(*r, *g, *b),
    //             Tile::Empty => "       ".normal(),
    //         }
    //     )
    // }
}
