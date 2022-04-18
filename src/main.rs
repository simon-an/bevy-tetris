mod egui_window;
mod input;

use bevy::log;
use bevy::prelude::*;
use tetris_plugin::*;

#[cfg(feature = "debug")]
use bevy_inspector_egui::WorldInspectorPlugin;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    MainInit,
    PluginInit,
    Ingame,
    Menu,
}
impl Default for AppState {
    fn default() -> Self {
        AppState::MainInit
    }
}

fn main() {
    let mut app = App::new();
    app.insert_resource(WindowDescriptor {
        title: "Tetris!".to_string(),
        width: 1000.,
        height: 1000.,
        ..Default::default()
    })
    .add_plugins(DefaultPlugins);
    #[cfg(feature = "debug")]
    // Debug hierarchy inspector
    app.add_plugin(WorldInspectorPlugin::new());
    app.add_startup_system(setup_board)
        .add_state(AppState::MainInit)
        .add_plugin(TetrisPlugin {
            plugin_init: AppState::PluginInit,
        })
        .add_system(egui_window::ui_example)
        // .add_system(egui_window::load_assets)
        .add_system(state_handler)
        .add_system(input::input);
    app.add_startup_system(camera_setup).run();
}

fn camera_setup(mut commands: Commands) {
    // 2D orthographic camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}

fn setup_board(
    mut commands: Commands,
    mut state: ResMut<State<AppState>>,
    asset_server: Res<AssetServer>,
) {
    commands.insert_resource(BoardOptions {
        map_size: (10, 22),
        tile_padding: 1.,
        ..Default::default()
    });

    commands.insert_resource(BoardAssets {
        board_material: SpriteMaterial {
            color: Color::WHITE,
            ..Default::default()
        },
        tile_material: SpriteMaterial {
            color: Color::DARK_GRAY,
            ..Default::default()
        },
        font: asset_server.load("fonts/pixeled.ttf"),
    });

    state.set(AppState::PluginInit).unwrap();
}

fn state_handler(
    // mut state: ResMut<State<AppState>>,
    keys: Res<Input<KeyCode>>,
    mut game_command_event: EventWriter<GameCommand>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        log::debug!("Escape key detected");
        // if state.current() == &AppState::Paused {
        //     log::info!("unpause game");
        //     state.pop().unwrap();
        // } else {
        //     log::info!("pause game");
        //     state.push(AppState::Paused).unwrap();
        // }
        game_command_event.send(GameCommand::TogglePause)
    }
    if keys.just_pressed(KeyCode::C) {
        log::debug!("clearing detected");
        game_command_event.send(GameCommand::Clear)
    }
    if keys.just_pressed(KeyCode::G) {
        log::debug!("generate detected");
        game_command_event.send(GameCommand::NewGame)
    }
    if keys.just_pressed(KeyCode::L) {
        log::info!("loading detected");
        game_command_event.send(GameCommand::Load)
    }
    if keys.just_pressed(KeyCode::S) {
        log::info!("save detected");
        game_command_event.send(GameCommand::Save)
    }
}
