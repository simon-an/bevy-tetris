use bevy::{color::palettes::css::RED, prelude::*};

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
    commands.spawn((
        Text::new(&text.0),
        TextFont {
            font: board_assets.font.clone(),
            font_size: 50.0,
            ..Default::default()
        },
        TextColor(RED.into()),
        Transform::from_xyz(0., 0., 10.),
        PopupRef,
    ));
}
