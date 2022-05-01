mod spawn;
pub(crate) use spawn::spawn_tetromino;
mod tick;
pub(crate) use tick::tock;
mod gameover;
pub(crate) use gameover::gameover;
mod move_tiles;
pub(crate) use move_tiles::gogo;
mod rotate;
pub(crate) use rotate::*;
mod line;
pub(crate) use line::*;
mod load_and_save;
pub(crate) use load_and_save::*;
mod engine;
pub(crate) use engine::*;

#[cfg(not(feature = "debug"))]
pub(crate) fn spawn_debug_block() {}
#[cfg(feature = "debug")]
mod spawn_debug_tile;
#[cfg(feature = "debug")]
pub(crate) use spawn_debug_tile::spawn_debug_block;

mod board;
pub(crate) use board::*;
mod popup;
pub(crate) use popup::*;
mod game_command;
pub(crate) use game_command::events_to_state;
