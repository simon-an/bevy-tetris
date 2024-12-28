use bevy::{
    color::palettes::css::{BLACK, DARK_GRAY, PURPLE, RED},
    prelude::*,
};

use crate::{BoardAssets, PopupRef, PopupText};

pub fn hide_popup(mut commands: Commands, popup: Query<Entity, With<PopupRef>>) {
    println!("hide_popup");
    commands
        .entity(popup.get_single().unwrap())
        .despawn_recursive();
}
// pub fn hide_popup(mut commands: Commands, popup: Query<PopupRef>) {
//     commands.despawn(popup.0)
// }

pub fn show_popup(mut commands: Commands, text: Res<PopupText>, board_assets: Res<BoardAssets>) {
    println!("show_popup");
    commands
        .spawn((
            Node {
                display: Display::Flex,
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            PopupRef,
            // BackgroundColor(BLACK.into()),
            ZIndex(2000),
            Name::new("Popup"),
        ))
        .with_children(|parent| {
            parent.spawn((
                Node {
                    width: Val::Auto,
                    height: Val::Auto,
                    min_height: Val::Px(100.0),
                    min_width: Val::Px(200.0),
                    position_type: PositionType::Relative,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                Text::new(&text.0),
                TextLayout::new_with_justify(JustifyText::Center),
                TextFont {
                    font: board_assets.font.clone(),
                    font_size: 50.0,
                    ..Default::default()
                },
                Name::new("PopupText"),
                TextColor(RED.into()),
                // BackgroundColor(DARK_GRAY.into()),
            ));
        });
}
