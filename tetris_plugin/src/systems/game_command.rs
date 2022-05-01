use bevy::prelude::*;

use crate::{GameCommand, GameStatus, PopupText};

pub(crate) fn events_to_state(
    mut commands: Commands,
    mut game_command: EventReader<GameCommand>,
    mut state: ResMut<State<GameStatus>>,
) {
    for event in game_command.iter() {
        match event {
            GameCommand::TogglePause => {
                if state.current() == &GameStatus::Paused {
                    state
                        .push(GameStatus::Running)
                        .expect("pushing state must work");
                    commands.remove_resource::<PopupText>();
                } else {
                    state
                        .push(GameStatus::Paused)
                        .expect("pushing state must work");
                    commands.insert_resource(PopupText("PAUSE".to_string()));
                }
            }

            GameCommand::Pause => {
                state
                    .push(GameStatus::Paused)
                    .expect("pushing state must work");
                commands.insert_resource(PopupText("PAUSE".to_string()));
            }
            GameCommand::Save => {
                // Handled in load_and_save system
            }
            GameCommand::Load => {
                // Handled in load_and_save system
            }
            _ => info!("do nothing with command {:?}", event),
        }
    }
}
