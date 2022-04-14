mod egui_window;
mod input;

use bevy::log;
use bevy::prelude::*;
use tetris_plugin::*;

#[cfg(feature = "debug")]
use bevy_inspector_egui::WorldInspectorPlugin;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    Running,
    Paused,
    NewGame,
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
    app.insert_resource(SoftDropTimer(Timer::from_seconds(0.750, true)))
        .insert_resource(PrintInfoTimer(Timer::from_seconds(1.0, true)))
        .add_startup_system(setup_board)
        .add_state(AppState::Paused)
        .add_plugin(TetrisPlugin {
            running_state: AppState::Running,
            pause_state: AppState::Paused,
        })
        // .add_system(egui_window::ui_example)
        // .add_system(egui_window::load_assets)
        .add_system(state_handler)
        .add_system(input::input);
    app.add_startup_system(camera_setup).run();
}
struct SoftDropTimer(Timer);
struct PrintInfoTimer(Timer);

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

    state.set(AppState::Running).unwrap();
}

fn state_handler(
    mut state: ResMut<State<AppState>>,
    keys: Res<Input<KeyCode>>,
    mut game_command_event: EventWriter<GameCommand>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        log::debug!("Escape key detected");
        if state.current() == &AppState::Paused {
            log::info!("unpause game");
            state.pop().unwrap();
        } else {
            log::info!("pause game");
            state.push(AppState::Paused).unwrap();
        }
    }
    if keys.just_pressed(KeyCode::C) {
        log::debug!("clearing detected");
        if state.current() == &AppState::Running {
            log::info!("clearing game");
            state.set(AppState::Paused).unwrap();
        }
    }
    if keys.just_pressed(KeyCode::G) {
        log::debug!("generate detected");
        if state.current() == &AppState::Paused {
            log::info!("generate game");
            state.set(AppState::Running).unwrap();
        }
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
