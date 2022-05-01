use crate::GameCommand;
use bevy::log;
use bevy::prelude::*;

pub fn input(keys: Res<Input<KeyCode>>, mut game_command_event: EventWriter<GameCommand>) {
    if keys.just_pressed(KeyCode::Escape) {
        log::debug!("Escape key detected");
        game_command_event.send(GameCommand::TogglePause)
    }
    if keys.just_pressed(KeyCode::C) {
        log::debug!("clearing detected");
        game_command_event.send(GameCommand::Clear)
    }
    if keys.just_pressed(KeyCode::G) {
        log::debug!("generate detected");
        game_command_event.send(GameCommand::NewGame)
    }
    if keys.just_pressed(KeyCode::L) {
        log::info!("loading detected");
        game_command_event.send(GameCommand::Load)
    }
    if keys.just_pressed(KeyCode::S) {
        log::info!("save detected");
        game_command_event.send(GameCommand::Save)
    }
}
