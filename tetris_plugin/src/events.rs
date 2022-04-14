use bevy::core::Timer;

use crate::{resources::Color, Coordinates, ShapeType, Tetromino};

#[derive(Debug, Copy, Clone)]
pub struct BombExplosionEvent(i64); // Score

#[derive(Debug, Clone)]
pub struct TickEvent(pub Timer);
#[derive(Debug, Clone)]
pub struct PrintInfoTimer(pub Timer);

#[derive(Debug, Clone)]
pub struct SpawnEvent;

#[derive(Debug, Clone)]
pub struct GameOverEvent;

#[derive(Debug, Copy, Clone)]
pub enum MoveEvent {
    Left,
    Right,
    Down,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum RotateEvent {
    ClockWise,
    CounterClockWise,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum GameCommand {
    Load,
    Save,
    Reset,
    Pause,
    IncreaseSpeed,
    DecreaseSpeed,
}