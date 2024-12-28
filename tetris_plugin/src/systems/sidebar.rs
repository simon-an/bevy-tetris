use bevy::{
    color::palettes::css::{BLACK, DARK_GRAY, DARK_GREEN, GOLD},
    prelude::*,
};

use crate::{BoardAssets, Score, SidebarRef};

#[derive(Component)]
pub struct ScoreRef;

pub fn update_score(score: Res<Score>, mut text: Query<&mut Text, With<ScoreRef>>) {
    if score.is_changed() {
        info!("score changed");
        if let Ok(mut text) = text.get_single_mut() {
            text.0 = format!("Score: {}", score.0);
        }
    }
}

pub fn sidebar(mut commands: Commands, score: Res<Score>, board_assets: Res<BoardAssets>) {
    commands
        .spawn((
            Node {
                display: Display::Flex,
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Start,
                align_items: AlignItems::Center,
                width: Val::Percent(20.0),
                left: Val::Percent(80.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            SidebarRef,
            BackgroundColor(DARK_GRAY.into()),
            ZIndex(1000),
            Name::new("Sidebar Right"),
        ))
        .with_children(|parent| {
            parent.spawn((
                Node {
                    width: Val::Auto,
                    height: Val::Auto,
                    min_height: Val::Px(100.0),
                    min_width: Val::Px(200.0),
                    position_type: PositionType::Relative,
                    justify_content: JustifyContent::Start,
                    align_items: AlignItems::Start,
                    ..default()
                },
                Text::new(format!("Score: {}", score.0)),
                TextLayout::new_with_justify(JustifyText::Left),
                TextFont {
                    font: board_assets.font.clone(),
                    font_size: 15.0,
                    ..Default::default()
                },
                Name::new("Score"),
                ScoreRef,
                TextColor(GOLD.into()),
                // BackgroundColor(DARK_GRAY.into()),
            ));
        });
}
pub fn sidebar_left(mut commands: Commands, board_assets: Res<BoardAssets>) {
    commands
        .spawn((
            Node {
                display: Display::Grid,
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Start,
                align_items: AlignItems::Start,
                width: Val::Percent(20.0),
                min_width: Val::Px(200.0),
                height: Val::Percent(100.0),
                grid_template_columns: vec![GridTrack::px(50.0), GridTrack::auto()],
                grid_auto_rows: vec![GridTrack::px(25.0)],
                ..default()
            },
            BackgroundColor(BLACK.into()),
            ZIndex(1000),
            Name::new("Sidebar Left"),
        ))
        .with_children(|parent| {
            for (key, command) in vec![
                ("+", "Increase Volume"),
                ("-", "Decrease Volume"),
                ("Pause", "Pause"),
                ("Esc", "Pause"),
                ("C", "Clear (disabled)"),
                ("L", "Load"),
                ("S", "Save"),
                ("Arrow Up", "Rotate"),
                ("Arrow Down", "Rotate"),
                ("Arrow Left", "Move Left"),
                ("Arrow Right", "Move Right"),
            ]
            .iter()
            {
                parent.spawn((
                    Node {
                        // width: Val::Auto,
                        // height: Val::Auto,
                        min_height: Val::Px(20.0),
                        min_width: Val::Px(50.0),
                        position_type: PositionType::Relative,
                        justify_content: JustifyContent::Start,
                        align_items: AlignItems::Start,
                        ..default()
                    },
                    Text::new(key.to_string()),
                    TextLayout::new_with_justify(JustifyText::Left),
                    TextFont {
                        font: board_assets.font.clone(),
                        font_size: 10.0,
                        ..Default::default()
                    },
                    Name::new("Key"),
                    TextColor(GOLD.into()),
                    // BackgroundColor(DARK_GRAY.into()),
                ));
                parent.spawn((
                    Node {
                        // width: Val::Auto,
                        // height: Val::Auto,
                        min_height: Val::Px(20.0),
                        min_width: Val::Px(150.0),
                        position_type: PositionType::Relative,
                        justify_content: JustifyContent::Start,
                        align_items: AlignItems::Start,
                        margin: UiRect {
                            left: Val::Px(10.0),
                            ..Default::default()
                        },
                        ..default()
                    },
                    Text::new(command.to_string()),
                    TextLayout::new_with_justify(JustifyText::Left),
                    TextFont {
                        font: board_assets.font.clone(),
                        font_size: 10.0,
                        ..Default::default()
                    },
                    Name::new("Command"),
                    TextColor(GOLD.into()),
                    // BackgroundColor(DARK_GREEN.into()),
                ));
            }
        });
}
