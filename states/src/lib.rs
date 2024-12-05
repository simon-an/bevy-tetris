use std::fmt::{Display, Formatter};

use bevy::prelude::{States, SubStates, StateSet};
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
