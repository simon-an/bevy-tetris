use bevy::prelude::*;

use crate::{BoardAssets, PopupRef, PopupText};

pub fn hide_popup(mut commands: Commands, popup: Res<PopupRef>) {
    println!("hide_popup");
    commands.entity(popup.0).despawn_recursive();
    commands.remove_resource::<PopupRef>()
}
// pub fn hide_popup(mut commands: Commands, popup: Query<PopupRef>) {
//     commands.despawn(popup.0)
// }

pub fn show_popup(mut commands: Commands, text: Res<PopupText>, board_assets: Res<BoardAssets>) {
    let id = commands
        .spawn_bundle(Text2dBundle {
            text: Text {
                sections: vec![TextSection {
                    value: text.0.clone(),
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
            transform: Transform::from_xyz(0., 0., 10.),
            ..Default::default()
        })
        .id();
    commands.insert_resource(PopupRef(id));
}
