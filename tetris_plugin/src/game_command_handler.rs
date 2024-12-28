use crate::GameCommand;
use bevy::log;
use bevy::prelude::*;

pub fn input(keys: Res<ButtonInput<KeyCode>>, mut game_command_event: EventWriter<GameCommand>) {
    // println!("input");
    if keys.just_pressed(KeyCode::Pause) {
        log::debug!("Pause key detected");
        game_command_event.send(GameCommand::TogglePause);
    }
    if keys.just_pressed(KeyCode::Escape) {
        log::debug!("Escape key detected");
        game_command_event.send(GameCommand::TogglePause);
    }
    if keys.just_pressed(KeyCode::KeyC) {
        log::debug!("clearing detected");
        game_command_event.send(GameCommand::Clear);
    }
    if keys.just_pressed(KeyCode::KeyG) {
        log::debug!("generate detected");
        game_command_event.send(GameCommand::NewGame);
    }
    if keys.just_pressed(KeyCode::KeyL) {
        log::info!("loading detected");
        game_command_event.send(GameCommand::Load);
    }
    if keys.just_pressed(KeyCode::KeyS) {
        log::info!("save detected");
        game_command_event.send(GameCommand::Save);
    }
}
