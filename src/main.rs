mod audio;
#[cfg(feature = "ui")]
mod egui_window;
#[cfg(feature = "menu")]
mod menu;

mod example_system;

use audio::Volume;
use bevy::color::palettes::css::DARK_GRAY;
use bevy::color::palettes::css::WHITE;
use bevy::prelude::*;

#[cfg(feature = "ui")]
use bevy::winit::WinitSettings;
#[cfg(feature = "ui")]
use bevy_egui::egui::Visuals;
#[cfg(feature = "ui")]
use bevy_egui::EguiContext;
#[cfg(feature = "ui")]
use bevy_egui::EguiPlugin;

use states::AppState;

use tetris_plugin::*;

#[cfg(feature = "debug")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    let mut app = App::new();

    app.init_resource::<Volume>();
    app.add_plugins(DefaultPlugins);
    app.add_plugins(TetrisPlugin);
    // Dont change order above this comment
    app.add_systems(Startup, (camera_setup, audio::setup_audio));
    app.add_systems(Startup, setup_board_assets);
    app.add_systems(Update, audio::volume);
    app.init_state::<AppState>();

    app.add_systems(
        Update,
        (
            example_system::change_title,
            example_system::toggle_override,
            example_system::change_scale_factor,
        ),
    );
    app.init_resource::<BoardOptions>();
    // Setup ui
    #[cfg(feature = "ui")]
    {
        app.insert_resource(WinitSettings::game()) // switching to desktop app saves energy, but breaks timers!
            .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))
            // .insert_resource(Msaa { samples: 4 })
            // .init_resource::<UiState>()
            .add_plugins(EguiPlugin)
            .add_systems(Startup, configure_visuals)
            .add_systems(Update, egui_window::ui_example);
    }

    // APPSTATE: MENU
    #[cfg(feature = "menu")]
    {
        app.add_systems(OnEnter(AppState::Menu), menu::setup_menu)
            .add_systems(Update, menu::menu)
            .add_systems(OnExit(AppState::Menu), menu::cleanup_menu);
    }
    // DEMO
    #[cfg(feature = "demo")]
    app.add_systems(OnEnter(AppState::InGame), menu::setup_game);
    #[cfg(feature = "demo")]
    app.add_systems(
        OnUpdate(AppState::InGame),
        (menu::movement, menu::change_color), // .with_system(tetris_plugin::game_command_handler::input)
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
    app.add_plugins(WorldInspectorPlugin::new());
    app.run();
}

#[cfg(feature = "ui")]
fn configure_visuals(mut egui_ctx: Query<&mut EguiContext>) {
    egui_ctx.single_mut().get_mut().set_visuals(Visuals {
        window_rounding: 5.0.into(),
        ..Default::default()
    });
}

fn camera_setup(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        // Window {
        //     present_mode: bevy::window::PresentMode::AutoVsync,
        //     title: "Snake!".to_string(),
        //     resizable: true,
        //     transparent: true,
        //     position: WindowPosition::Centered(MonitorSelection::Current),
        //     ..Default::default()
        // },
    ));
}

fn setup_board_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(BoardAssets {
        board_material: SpriteMaterial {
            color: WHITE.into(),
            texture: None,
        },
        tile_material: SpriteMaterial {
            color: DARK_GRAY.into(),
            texture: None,
        },
        font: asset_server.load("fonts/pixeled.ttf"),
    });
}
