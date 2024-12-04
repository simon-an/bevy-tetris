use bevy::{
    prelude::{Event, Resource},
    time::Timer,
};

#[derive(Debug, Copy, Clone, Event)]
pub struct ScoreEvent(pub u64);

#[derive(Debug, Clone, Resource)]
pub struct Tick(pub Timer);
impl Default for Tick {
    fn default() -> Self {
        Self(Timer::from_seconds(1.0, bevy::time::TimerMode::Repeating))
    }
}

#[derive(Debug, Clone)]
pub struct PrintInfoTimer(pub Timer);

#[derive(Debug, Clone, Event)]
pub struct SpawnEvent;

#[derive(Debug, Clone, Event)]
pub struct GameOverEvent;

#[derive(Debug, Copy, Clone, Event)]
pub enum MoveEvent {
    Left,
    Right,
    Down,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Event)]
pub enum RotateEvent {
    ClockWise,
    CounterClockWise,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Event)]
pub enum GameCommand {
    Load,
    Save,
    NewGame,
    Pause,
    Clear,
    IncreaseSpeed,
    DecreaseSpeed,
    TogglePause,
}
