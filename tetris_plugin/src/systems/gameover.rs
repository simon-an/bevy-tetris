use bevy::{color::palettes::css::RED, prelude::*};

use states::GameStatus;

use crate::{BoardAssets, GameOverEvent};

pub fn gameover(
    mut commands: Commands,
    mut game_over_event_rdr: EventReader<GameOverEvent>,
    mut state: ResMut<NextState<GameStatus>>,
    board_assets: Res<BoardAssets>,
) {
    let event = game_over_event_rdr.read().next();
    if event.is_some() {
        state.set(GameStatus::Gameover);

        commands.spawn((
            Text2d("GAME OVER".to_string()),
            TextColor(RED.into()),
            TextLayout {
                justify: JustifyText::Center,
                linebreak: LineBreak::WordBoundary,
            },
            TextFont {
                font: board_assets.font.clone(),
                font_size: 50.0,
                ..Default::default()
            },
            Transform::from_xyz(0., 0., 1000.),
        ));

        error!("gameover");
    }
}
