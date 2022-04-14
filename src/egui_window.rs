use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

const BEVY_TEXTURE_ID: u64 = 0;

// pub fn ui_example(mut egui_context: ResMut<EguiContext>) {
//     egui::Window::new("Hello").show(egui_context.ctx_mut(), |ui| {
//         ui.label("world");
//     });
// }

pub fn load_assets(mut egui_context: ResMut<EguiContext>, assets: Res<AssetServer>) {
    let texture_handle = assets.load("icon.png");
    egui_context.set_egui_texture(BEVY_TEXTURE_ID, texture_handle);
}

pub fn ui_example(
    mut egui_ctx: ResMut<EguiContext>,
    // mut ui_state: ResMut<UiState>,
    assets: Res<AssetServer>,
) {
    egui::SidePanel::left("side_panel")
        .default_width(200.0)
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.heading("Side Panel");

            // ui.horizontal(|ui| {
            //     ui.label("Write something: ");
            //     ui.text_edit_singleline(&mut ui_state.label);
            // });

            // ui.add(egui::Slider::new(&mut ui_state.value, 0.0..=10.0).text("value"));
            // if ui.button("Increment").clicked() {
            //     ui_state.value += 1.0;
            // }

            // ui.allocate_space(egui::Vec2::new(1.0, 100.0));
            // ui.horizontal(|ui| {
            //     load = ui.button("Load").clicked();
            //     invert = ui.button("Invert").clicked();
            //     remove = ui.button("Remove").clicked();
            // });

            ui.add(egui::widgets::Image::new(
                egui::TextureId::User(BEVY_TEXTURE_ID),
                [256.0, 256.0],
            ));

            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                ui.add(egui::Hyperlink::from_label_and_url(
                    "powered by egui",
                    "https://github.com/emilk/egui/",
                ));
            });
        });
}
