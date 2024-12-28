use bevy::prelude::*;

use crate::AppState;

#[cfg(feature = "demo")]
pub use demo::change_color;
#[cfg(feature = "demo")]
pub use demo::movement;
#[cfg(feature = "demo")]
pub use demo::setup_game;

#[cfg_attr(feature = "debug", derive(Reflect))]
#[derive(Component, Debug, Default, Clone, Copy)]
#[cfg_attr(feature = "debug", reflect(Component))]
pub struct MenuComponent;

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

pub fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    // ui camera
    // commands.spawn_bundle(UiCameraBundle::default());
    let _button_entity = commands
        .spawn((
            Button,
            Node {
                width: Val::Px(150.0),
                height: Val::Px(65.0),
                // center button
                margin: UiRect::all(Val::Auto),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            BorderColor(Color::BLACK),
            BorderRadius::MAX,
            BackgroundColor(NORMAL_BUTTON),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Play"),
                TextFont {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
            ));
        })
        .insert(MenuComponent)
        .id();
    // println!("setup menu {:?}", button_entity);
    // commands.insert_resource(MenuData { button_entity });
}

pub fn menu(
    mut state: ResMut<NextState<AppState>>,
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut background, mut _border, _children) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background = PRESSED_BUTTON.into();
                println!("SET INGAME STATE");
                state.set(AppState::InGame);
            }
            Interaction::Hovered => {
                *background = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *background = NORMAL_BUTTON.into();
            }
        }
    }
}

// pub fn cleanup_menu(mut commands: Commands, menu_data: Res<MenuData>) {
//     commands.entity(menu_data.button_entity).despawn_recursive();
// }
pub fn cleanup_menu(mut commands: Commands, menu_data: Query<Entity, With<MenuComponent>>) {
    for e in menu_data.iter() {
        commands.entity(e).despawn_recursive();
    }
}

#[cfg(feature = "demo")]
mod demo {

    use bevy::prelude::*;

    const SPEED: f32 = 100.0;
    pub fn movement(
        time: Res<Time>,
        input: Res<Input<KeyCode>>,
        mut query: Query<&mut Transform, With<Sprite>>,
    ) {
        for mut transform in query.iter_mut() {
            let mut direction = Vec3::ZERO;
            if input.pressed(KeyCode::Left) {
                direction.x -= 1.0;
            }
            if input.pressed(KeyCode::Right) {
                direction.x += 1.0;
            }
            if input.pressed(KeyCode::Up) {
                direction.y += 1.0;
            }
            if input.pressed(KeyCode::Down) {
                direction.y -= 1.0;
            }

            if direction != Vec3::ZERO {
                transform.translation += direction.normalize() * SPEED * time.delta_seconds();
            }
        }
    }

    pub fn change_color(time: Res<Time>, mut query: Query<&mut Sprite>) {
        for mut sprite in query.iter_mut() {
            sprite
                .color
                .set_b((time.seconds_since_startup() * 0.5).sin() as f32 + 2.0);
        }
    }

    pub fn setup_game(mut commands: Commands, asset_server: Res<AssetServer>) {
        commands.spawn_bundle(SpriteBundle {
            texture: asset_server.load("icon.png"),
            ..default()
        });
    }
}
