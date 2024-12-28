use std::fmt::{Display, Formatter};

use bevy::prelude::{ComputedStates, StateSet, States, SubStates};
// use colored::Colorize;

#[derive(Debug, Clone, Eq, PartialEq, Hash, SubStates, Default)]
#[source(AppState = AppState::InGame)]
pub enum GameStatus {
    #[default]
    Init,
    Running,
    Paused,
    Gameover,
    Loading,
}

impl Display for GameStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                GameStatus::Init => "Game is initializing",
                GameStatus::Running => "Game is running",
                GameStatus::Paused => "Game is paused",
                GameStatus::Gameover => "Game is over",
                GameStatus::Loading => "Game is loading",
            }
        )
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct InGame;
impl ComputedStates for InGame {
    type SourceStates = Option<GameStatus>;
    fn compute(sources: Option<GameStatus>) -> Option<Self> {
        match sources {
            Some(GameStatus::Running) => Some(InGame),
            Some(GameStatus::Paused) => Some(InGame),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, SubStates, Default)]
#[source(InGame = InGame)]
pub enum GameLogicState {
    #[default]
    Spawning,
    Ticking,
    Cleaning,
}

impl Display for GameLogicState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                GameLogicState::Spawning => "Spawning",
                GameLogicState::Ticking => "Ticking",
                GameLogicState::Cleaning => "Cleaning",
            }
        )
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, States)]
pub enum AppState {
    Menu,
    // PluginInit,
    InGame,
}
impl Default for AppState {
    fn default() -> Self {
        AppState::Menu
    }
}

// impl AppState {
//     fn to_color_string(&self) -> String {
//         match self {
//             AppState::InGame => "Game is running".green().to_string(),
//             AppState::Menu => "Main Menu".blue().to_string(),
//         }
//     }
// }

impl Display for AppState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                AppState::InGame => "Game is running",
                AppState::Menu => "Main Menu",
            }
        )
    }
}
