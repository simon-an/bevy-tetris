use bevy::prelude::*;
use states::GameStatus;

use crate::{GameCommand, PopupText};

pub(crate) fn events_to_state(
    mut commands: Commands,
    mut game_command: EventReader<GameCommand>,
    mut state: ResMut<State<GameStatus>>,
    mut next_state: ResMut<NextState<GameStatus>>,
) {
    for event in game_command.read() {
        match event {
            GameCommand::TogglePause => {
                if state.get() == &GameStatus::Paused {
                    next_state
                        .set(GameStatus::Running);
                    commands.remove_resource::<PopupText>();
                } else {
                    next_state
                        .set(GameStatus::Paused);
                    commands.insert_resource(PopupText("PAUSE".to_string()));
                }
            }

            GameCommand::Pause => {
                next_state
                    .set(GameStatus::Paused);
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
