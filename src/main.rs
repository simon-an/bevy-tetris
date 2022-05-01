#[cfg(feature = "ui")]
mod egui_window;
#[cfg(feature = "menu")]
mod menu;

mod example_system;

use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use bevy::prelude::*;
use bevy::window::PresentMode;

#[cfg(feature = "ui")]
use bevy::winit::WinitSettings;
#[cfg(feature = "ui")]
use bevy_egui::egui::Visuals;
#[cfg(feature = "ui")]
use bevy_egui::EguiContext;
#[cfg(feature = "ui")]
use bevy_egui::EguiPlugin;

use colored::Colorize;

use tetris_plugin::*;

#[cfg(feature = "debug")]
use bevy_inspector_egui::{RegisterInspectable, WorldInspectorPlugin};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    Menu,
    // PluginInit,
    InGame,
}
impl Default for AppState {
    fn default() -> Self {
        AppState::Menu
    }
}

impl AppState {
    fn to_color_string(&self) -> String {
        match self {
            AppState::InGame => "Game is running".green().to_string(),
            AppState::Menu => "Main Menu".blue().to_string(),
        }
    }
}

impl Display for AppState {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                AppState::InGame => "Game is running",
                AppState::Menu => "Main Menu",
            }
        )
    }
}

fn main() {
    // App::new()
    //     .add_startup_system(camera_setup)
    //     .add_state(AppState::Menu)
    //     .insert_resource(WindowDescriptor {
    //         present_mode: PresentMode::Mailbox,
    //         title: "Tetris!".to_string(),
    //         resizable: true,
    //         transparent: true,
    //         width: 5000.,
    //         height: 1400.,
    //         ..Default::default()
    //     })
    //     .insert_resource(BoardOptions {
    //         map_size: (10, 22),
    //         tile_size: TileSize::Fixed(15.0),
    //         tile_padding: 1.,
    //         ..Default::default()
    //     })
    //     .insert_resource(WinitSettings::game()) // switching to desktop app saves energy, but breaks timers!
    //     .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
    //     .insert_resource(Msaa { samples: 4 })
    //     // .init_resource::<UiState>()
    //     .add_system(example_system::change_title)
    //     .add_system(example_system::toggle_override)
    //     .add_system(example_system::change_scale_factor)
    //     .add_plugins(DefaultPlugins)
    //     .add_plugin(EguiPlugin)
    //     .add_startup_system(configure_visuals)
    //     .add_system(egui_window::ui_example)
    //     .add_plugin(TetrisPlugin {
    //         plugin_init: AppState::InGame,
    //     })
    //     .add_system_set(SystemSet::on_enter(AppState::Menu).with_system(menu::setup_menu))
    //     .add_system_set(SystemSet::on_update(AppState::Menu).with_system(menu::menu))
    //     .add_system_set(
    //         SystemSet::on_exit(AppState::Menu)
    //             .with_system(menu::cleanup_menu)
    //             .with_system(setup_board),
    //     )
    //     .run();

    let mut app = App::new();
    app.insert_resource(WindowDescriptor {
        present_mode: PresentMode::Mailbox,
        title: "Tetris!".to_string(),
        resizable: true,
        transparent: true,
        width: 1000.,
        height: 1000.,
        ..Default::default()
    });
    app.add_plugins(DefaultPlugins);
    // Dont change order above this comment
    app.add_startup_system(camera_setup);
    app.add_state(AppState::Menu);
    app.add_system(example_system::change_title);
    app.add_system(example_system::toggle_override);
    app.add_system(example_system::change_scale_factor);
    app.insert_resource(BoardOptions {
        map_size: (10, 22),
        tile_size: TileSize::Fixed(15.0),
        tile_padding: 1.,
        ..Default::default()
    });

    // Setup ui
    #[cfg(feature = "ui")]
    {
        app.insert_resource(WinitSettings::game()) // switching to desktop app saves energy, but breaks timers!
            .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
            .insert_resource(Msaa { samples: 4 })
            // .init_resource::<UiState>()
            .add_plugin(EguiPlugin)
            .add_startup_system(configure_visuals)
            .add_system(egui_window::ui_example);
    }

    // TETRIS
    #[cfg(not(feature = "demo"))]
    app.add_plugin(TetrisPlugin {
        plugin_init: AppState::InGame,
    });

    // APPSTATE: MENU
    #[cfg(feature = "menu")]
    {
        app.add_system_set(SystemSet::on_enter(AppState::Menu).with_system(menu::setup_menu))
            .add_system_set(SystemSet::on_update(AppState::Menu).with_system(menu::menu));
        app.add_system_set(
            SystemSet::on_exit(AppState::Menu)
                .with_system(menu::cleanup_menu)
                .with_system(setup_board),
        );
    }
    #[cfg(not(feature = "menu"))]
    app.add_startup_system(setup_board);
    // DEMO
    #[cfg(feature = "demo")]
    app.add_system_set(SystemSet::on_enter(AppState::InGame).with_system(menu::setup_game));
    #[cfg(feature = "demo")]
    app.add_system_set(
        SystemSet::on_update(AppState::InGame)
            .with_system(menu::movement)
            .with_system(menu::change_color), // .with_system(tetris_plugin::game_command_handler::input)
                                              // .with_system(tetris_plugin::input::input),
    );
    // app.add_event::<GameCommand>();
    // app.add_event::<MoveEvent>();
    // app.add_event::<RotateEvent>();

    // Debug hierarchy inspector. put it add the end pf main, so the z-index is higher than rest of ui
    #[cfg(feature = "debug")]
    {
        // TYPE must derive(bevy_inspector_egui::Inspectable)
        // app.register_inspectable::<menu::MenuComponent>(); // TODO find out why this does not work
        app.register_type::<bevy::ui::Interaction>();
        app.register_type::<menu::MenuComponent>();
    }
    #[cfg(feature = "debug")]
    app.add_plugin(WorldInspectorPlugin::new());
    app.run();
}

#[cfg(feature = "ui")]
fn configure_visuals(mut egui_ctx: ResMut<EguiContext>) {
    egui_ctx.ctx_mut().set_visuals(Visuals {
        window_rounding: 5.0.into(),
        ..Default::default()
    });
}

fn camera_setup(mut commands: Commands) {
    // 2D orthographic camera
    commands.spawn_bundle(UiCameraBundle::default());
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn setup_board(mut commands: Commands, asset_server: Res<AssetServer>) {
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
}
