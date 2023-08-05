use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Slider},
    EguiContexts,
};

use crate::resources::SettingsResource;

use super::egui::get_default_egui_frame;

pub fn display_settings(
    mut settings_res: ResMut<SettingsResource>,
    mut egui_contexts: EguiContexts,
) {
    let frame = get_default_egui_frame();
    egui::Window::new("Settings")
        .title_bar(true)
        .default_open(false)
        .collapsible(true)
        .movable(false)
        .resizable(false)
        .frame(frame)
        .anchor(egui::Align2::LEFT_BOTTOM, egui::vec2(0.0, 0.0))
        .show(egui_contexts.ctx_mut(), |ui| {
            ui.vertical(|ui| {
                ui.add(
                    Slider::new(&mut settings_res.wall_rotation_sensitivity, 0.0..=1.)
                        .clamp_to_range(false)
                        .text("Wall rotation sensitivity"),
                )
            });
        });
}
