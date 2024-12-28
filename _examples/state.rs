// use bevy::prelude::*;

// /// This example illustrates how to use [`States`] to control transitioning from a `Menu` state to
// /// an `InGame` state.
// fn main() {
//     App::new()
//         .add_plugins(DefaultPlugins)
//         .add_state(AppState::Menu)
//         .add_system_set(SystemSet::on_enter(AppState::Menu).with_system(setup_menu))
//         .add_system_set(SystemSet::on_update(AppState::Menu).with_system(menu))
//         .add_system_set(SystemSet::on_exit(AppState::Menu).with_system(cleanup_menu))
//         .add_system_set(SystemSet::on_enter(AppState::InGame).with_system(setup_game))
//         .add_system_set(
//             SystemSet::on_update(AppState::InGame)
//                 .with_system(movement)
//                 .with_system(change_color),
//         )
//         .run();
// }

// #[derive(Debug, Clone, Eq, PartialEq, Hash)]
// enum AppState {
//     Menu,
//     InGame,
// }

// struct MenuData {
//     button_entity: Entity,
// }

// const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
// const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
// const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

// fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
//     // ui camera
//     // commands.spawn(Camera2d::default());
//     let button_entity = commands
//         .spawn((
//             Button,
//             Node {
//                 // size: Size::new(Val::Px(150.0), Val::Px(65.0)),
//                 // center button
//                 margin: Rect::all(Val::Auto),
//                 // horizontally center child text
//                 justify_content: JustifyContent::Center,
//                 // vertically center child text
//                 align_items: AlignItems::Center,
//                 ..default()
//             },
//             // : NORMAL_BUTTON.into(),
//         ))
//         .with_children(|parent| {
//             parent.spawn((
//                 Text::new("Play"),
//                 TextStyle {
//                     font: asset_server.load("fonts/pixeled.ttf"),
//                     font_size: 40.0,
//                 },
//                 TextColor(Color::rgb(0.9, 0.9, 0.9)),
//             ));
//         })
//         .id();
//     commands.insert_resource(MenuData { button_entity });
// }

// fn menu(
//     mut state: ResMut<State<AppState>>,
//     mut interaction_query: Query<
//         (&Interaction, &mut UiColor),
//         (Changed<Interaction>, With<Button>),
//     >,
// ) {
//     for (interaction, mut color) in interaction_query.iter_mut() {
//         match *interaction {
//             Interaction::Clicked => {
//                 *color = PRESSED_BUTTON.into();
//                 state.set(AppState::InGame).unwrap();
//             }
//             Interaction::Hovered => {
//                 *color = HOVERED_BUTTON.into();
//             }
//             Interaction::None => {
//                 *color = NORMAL_BUTTON.into();
//             }
//         }
//     }
// }

// fn cleanup_menu(mut commands: Commands, menu_data: Res<MenuData>) {
//     commands.entity(menu_data.button_entity).despawn_recursive();
// }

// fn setup_game(mut commands: Commands, asset_server: Res<AssetServer>) {
//     commands.spawn(Camera2d);
//     commands.spawn(Sprite {
//         image: asset_server.load("icon.png"),
//         ..default()
//     });
// }

// const SPEED: f32 = 100.0;
// fn movement(
//     time: Res<Time>,
//     input: Res<ButtonInput<KeyCode>>,
//     mut query: Query<&mut Transform, With<Sprite>>,
// ) {
//     for mut transform in query.iter_mut() {
//         let mut direction = Vec3::ZERO;
//         if input.pressed(KeyCode::ArrowLeft) {
//             direction.x -= 1.0;
//         }
//         if input.pressed(KeyCode::ArrowRight) {
//             direction.x += 1.0;
//         }
//         if input.pressed(KeyCode::ArrowUp) {
//             direction.y += 1.0;
//         }
//         if input.pressed(KeyCode::ArrowDown) {
//             direction.y -= 1.0;
//         }

//         if direction != Vec3::ZERO {
//             transform.translation += direction.normalize() * SPEED * time.delta_secs();
//         }
//     }
// }

// fn change_color(time: Res<Time>, mut query: Query<&mut Sprite>) {
//     for mut sprite in query.iter_mut() {

//         let mut c = sprite.color.to_srgba();
//         c.blue = (time.elapsed_secs() * 0.5).sin() as f32 + 2.0;
//         sprite
//             .color =  c.into();
//     }
// }
