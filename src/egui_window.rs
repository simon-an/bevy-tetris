use bevy::{prelude::*, window::PrimaryWindow};
use bevy_egui::{egui, EguiContext};
use states::{GameLogicState, GameStatus};

use crate::{audio::Volume, AppState};
use tetris_plugin::{BoardOptions, Score, TickCounter, TileSize};

pub struct Images {
    bevy_icon: Handle<Image>,
}

impl FromWorld for Images {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource_mut::<AssetServer>().unwrap();
        Self {
            bevy_icon: asset_server.load("icon.png"),
        }
    }
}

pub fn ui_example(
    mut egui_ctx: Query<&mut EguiContext, With<PrimaryWindow>>,
    app_state: Res<State<AppState>>,
    game_state: Option<Res<State<GameStatus>>>,
    logic_state: Option<Res<State<GameLogicState>>>,
    mut options: ResMut<BoardOptions>,
    mut volume: ResMut<Volume>,
    score: Option<Res<Score>>,
    ticks: Option<Res<TickCounter>>,
    // board_assets: Res<BoardAsset>

    // mut ui_state: ResMut<UiState>,
    // You are not required to store Egui texture ids in systems. We store this one here just to
    // demonstrate that rendering by using a texture id of a removed image is handled without
    // making bevy_egui panic.
    mut rendered_texture_id: Local<egui::TextureId>,
    mut is_initialized: Local<bool>,
    mut tile_size: Local<f32>,
    mut adaptive_tile: Local<bool>,
    // If you need to access the ids from multiple systems, you can also initialize the `Images`
    // resource while building the app and use `Res<Images>` instead.
    images: Local<Images>,
) {
    if !*is_initialized {
        *is_initialized = true;
        // *rendered_texture_id = egui_ctx.add_image(images.bevy_icon.clone_weak());
        *adaptive_tile = true;
        *tile_size = 50.0;
    }

    if let Ok(mut ctx) = egui_ctx.get_single_mut() {
        let ctx = ctx.get_mut();
        egui::SidePanel::left("side_panel")
            .default_width(200.0)
            .show(ctx, |ui| {
                ui.heading("Score");
                if let Some(score) = score {
                    ui.heading(score.0.to_string());
                } else {
                    ui.heading("no score");
                }
                ui.heading("Tick");
                if let Some(ticks) = ticks {
                    ui.heading(ticks.0.to_string());
                } else {
                    ui.heading("no ticks");
                }

                ui.separator();
                ui.heading("Audio");
                ui.add(egui::Slider::new(&mut volume.0, 0.0..=2.0).text("volume"));

                ui.separator();
                ui.heading("Status");

                ui.vertical(|ui| {
                    ui.label(format!("App State: {}", app_state.get().to_string()));
                    // ui.text_edit_singleline(&mut ui_state.label);
                });
                if let Some(logic_state) = logic_state {
                    ui.vertical(|ui| {
                        ui.label(format!("Logic State: {}", logic_state.get().to_string()));
                        // ui.text_edit_singleline(&mut ui_state.label);
                    });
                }
                if let Some(game_ste) = game_state {
                    ui.vertical(|ui| {
                        ui.label(format!("Game State: {}", game_ste.get().to_string()));
                        // ui.text_edit_singleline(&mut ui_state.label);
                    });
                }

                ui.separator();
                ui.vertical(|ui| {
                    ui.heading("Options");
                    ui.label("Position:");
                    ui.label(format!("{:?}", options.position));
                    ui.separator();
                    ui.label("Map Size:");
                    ui.add(egui::Slider::new(&mut options.map_size.0, 6..=15).text("width"));
                    ui.add(egui::Slider::new(&mut options.map_size.1, 10..=25).text("height"));

                    ui.separator();
                    ui.label("Tile Padding");
                    ui.label(format!("{:?}", options.tile_padding));
                    ui.allocate_space(egui::Vec2::new(1.0, 10.0));
                    ui.label("Tile Size");
                    ui.label(format!("{:?}", options.tile_size));
                    ui.checkbox(&mut *adaptive_tile, "Adaptive");
                    options.tile_size = if *adaptive_tile == true {
                        TileSize::Adaptive {
                            min: 10.0,
                            max: *tile_size,
                        }
                    } else {
                        TileSize::Fixed(*tile_size)
                    };
                    ui.add(egui::Slider::new(&mut *tile_size, 10.0..=50.0).text("tile_size_max"));
                });

                ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                    ui.add(egui::Hyperlink::from_label_and_url(
                        "powered by egui",
                        "https://github.com/emilk/egui/",
                    ));
                });
            });
    }
}
