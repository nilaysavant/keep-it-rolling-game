use bevy::prelude::*;
use bevy_egui::{
    egui::{self, RichText},
    EguiContexts,
};

use crate::constants::GAME_TITLE;

use super::egui::get_default_egui_frame;

pub fn display_credits(mut egui_contexts: EguiContexts) {
    let frame = get_default_egui_frame();
    egui::Window::new("Credits")
        .title_bar(false)
        .collapsible(true)
        .movable(false)
        .resizable(false)
        .frame(frame)
        .anchor(egui::Align2::LEFT_TOP, egui::vec2(0.0, 0.0))
        .show(egui_contexts.ctx_mut(), |ui| {
            ui.vertical(|ui| {
                ui.label(RichText::new(GAME_TITLE.to_string()).heading());
                ui.horizontal(|ui| {
                    ui.label("by");
                    ui.hyperlink_to("Nilay Savant", "https://nilaysavant.itch.io/");
                })
            });
        });
}
