use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

pub struct Images {
    bevy_icon: Handle<Image>,
    bevy_icon_inverted: Handle<Image>,
}

impl FromWorld for Images {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource_mut::<AssetServer>().unwrap();
        Self {
            bevy_icon: asset_server.load("icon.png"),
            bevy_icon_inverted: asset_server.load("icon_inverted.png"),
        }
    }
}

pub fn ui_example(
    mut egui_ctx: ResMut<EguiContext>,
    // mut ui_state: ResMut<UiState>,
    // You are not required to store Egui texture ids in systems. We store this one here just to
    // demonstrate that rendering by using a texture id of a removed image is handled without
    // making bevy_egui panic.
    mut rendered_texture_id: Local<egui::TextureId>,
    mut is_initialized: Local<bool>,
    // If you need to access the ids from multiple systems, you can also initialize the `Images`
    // resource while building the app and use `Res<Images>` instead.
    images: Local<Images>,
) {
    if !*is_initialized {
        *is_initialized = true;
        *rendered_texture_id = egui_ctx.add_image(images.bevy_icon.clone_weak());
    }

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
                // egui::TextureId::User(BEVY_TEXTURE_ID),
                *rendered_texture_id,
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
