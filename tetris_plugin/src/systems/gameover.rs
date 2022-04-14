use bevy::{ecs::schedule::StateData, prelude::*};

use crate::{BoardAssets, GameOverEvent};

pub fn stop<T>(
    mut commands: Commands,
    mut game_over_event_rdr: EventReader<GameOverEvent>,
    mut state: ResMut<State<T>>,
    pause_state: ResMut<T>,
    board_assets: Res<BoardAssets>,
) where
    T: StateData + Send + Sync,
{
    let event = game_over_event_rdr.iter().next();
    if event.is_some() {
        state.set(pause_state.clone()).unwrap();

        commands.spawn_bundle(Text2dBundle {
            text: Text {
                sections: vec![TextSection {
                    value: "GAME OVER".to_string(),
                    style: TextStyle {
                        color: Color::RED,
                        font: board_assets.font.clone(),
                        font_size: 50.0,
                    },
                }],
                alignment: TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center,
                },
            },
            transform: Transform::from_xyz(0., 0., 1.),
            ..Default::default()
        });

        error!("gameover");
    }
}
