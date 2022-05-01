use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

use crate::AppState;
use tetris_plugin::{BoardOptions, TileSize};

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
    mut egui_ctx: ResMut<EguiContext>,
    app_state: Res<State<AppState>>,
    mut options: ResMut<BoardOptions>,
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
        *rendered_texture_id = egui_ctx.add_image(images.bevy_icon.clone_weak());
        *adaptive_tile = true;
        *tile_size = 50.0;
    }

    egui::SidePanel::left("side_panel")
        .default_width(200.0)
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.heading("Status");

            ui.vertical(|ui| {
                ui.label(format!("App State: {}", app_state.current().to_string()));
                // ui.text_edit_singleline(&mut ui_state.label);
            });

            ui.allocate_space(egui::Vec2::new(200.0, 20.0));
            ui.vertical(|ui| {
                ui.heading("Options");
                ui.label("Position:");
                ui.label(format!("{:?}", options.position));
                ui.allocate_space(egui::Vec2::new(1.0, 10.0));
                ui.label("Map Size:");
                ui.add(egui::Slider::new(&mut options.map_size.0, 6..=15).text("width"));
                ui.add(egui::Slider::new(&mut options.map_size.1, 10..=25).text("height"));

                ui.allocate_space(egui::Vec2::new(1.0, 10.0));
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

            // ui.allocate_space(egui::Vec2::new(1.0, 100.0));
            // ui.horizontal(|ui| {
            //     load = ui.button("Load").clicked();
            //     invert = ui.button("Invert").clicked();
            //     remove = ui.button("Remove").clicked();
            // });

            // ui.add(egui::widgets::Image::new(
            //     // egui::TextureId::User(BEVY_TEXTURE_ID),
            //     *rendered_texture_id,
            //     [256.0, 256.0],
            // ));

            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                ui.add(egui::Hyperlink::from_label_and_url(
                    "powered by egui",
                    "https://github.com/emilk/egui/",
                ));
            });
        });
}
