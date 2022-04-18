use bevy::prelude::*;

use crate::{BoardAssets, GameOverEvent, GameStatus};

pub fn gameover(
    mut commands: Commands,
    mut game_over_event_rdr: EventReader<GameOverEvent>,
    mut state: ResMut<State<GameStatus>>,
    board_assets: Res<BoardAssets>,
) {
    let event = game_over_event_rdr.iter().next();
    if event.is_some() {
        state.set(GameStatus::Gameover).unwrap();

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
