use bevy::prelude::*;
use states::GameStatus;

use crate::{GameCommand, PopupText};

pub(crate) fn events_to_state(
    mut commands: Commands,
    mut game_command: EventReader<GameCommand>,
    state: Res<State<GameStatus>>,
    mut next_state: ResMut<NextState<GameStatus>>,
) {
    for event in game_command.read() {
        match event {
            GameCommand::TogglePause => {
                if state.get() == &GameStatus::Paused {
                    println!("setting game to Running");
                    println!("State {:?}", state.get());
                    next_state.set(GameStatus::Running);
                    commands.remove_resource::<PopupText>();
                } else {
                    println!("setting game to paused");
                    next_state.set(GameStatus::Paused);
                    commands.insert_resource(PopupText("PAUSE".to_string()));
                }
            }

            GameCommand::Pause => {
                panic!("Pause command is deprecated");
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

#[cfg(test)]
mod unit_tests {
    use super::*;
    use bevy::state::app::StatesPlugin;
    use states::{AppState, InGame};

    #[test]
    fn events_to_state_test() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, StatesPlugin));
        app.add_systems(Update, super::events_to_state);
        app.add_event::<GameCommand>();
        app.init_state::<AppState>();
        app.add_computed_state::<states::InGame>();
        app.add_sub_state::<states::GameStatus>();
        app.add_sub_state::<states::GameLogicState>();

        let actual = app.world().resource::<State<AppState>>();
        assert_eq!(actual.get(), &AppState::Menu);
        let actual = app.world().get_resource::<State<GameStatus>>();
        assert!(actual.is_none());

        app.update();

        app.insert_state(AppState::InGame);

        app.update();

        let actual = app.world().get_resource::<State<GameStatus>>();
        assert!(actual.is_some());
        let actual = app.world().resource::<State<GameStatus>>();
        assert_eq!(actual.get(), &GameStatus::Init);
        let actual = app.world().get_resource::<State<GameLogicState>>();
        assert!(actual.is_none());

        app.insert_state(GameStatus::Running);
        app.update();

        let actual = app.world().get_resource::<State<InGame>>();
        assert!(actual.is_some());
        let actual = app.world().resource::<State<GameLogicState>>();
        assert_eq!(actual.get(), &GameLogicState::Spawning);

        app.insert_state(GameLogicState::Ticking);
        app.update();

        let actual = app.world().resource::<State<GameStatus>>();
        assert_eq!(actual.get(), &GameStatus::Running);

        app.world_mut().send_event(GameCommand::TogglePause);

        app.update();
        app.update();

        let actual = app.world().resource::<State<GameStatus>>();
        assert_eq!(actual.get(), &GameStatus::Paused);

        app.world_mut().send_event(GameCommand::TogglePause);

        app.update();
        app.update();

        let actual = app.world().resource::<State<GameStatus>>();
        assert_eq!(actual.get(), &GameStatus::Running);
    }
}
