use bevy_egui::{
    egui::{self, epaint::Shadow, Color32, Margin, Rounding, Stroke},
    EguiContexts,
};

/// System to init the EGUI UI.
pub fn init_egui_context(mut egui_contexts: EguiContexts) {
    let ctx = egui_contexts.ctx_mut();
    ctx.set_visuals(get_visuals());
}

pub fn get_visuals() -> egui::Visuals {
    let mut visuals = egui::Visuals::dark();
    visuals.override_text_color = Some(Color32::WHITE);
    visuals.extreme_bg_color = Color32::from_rgba_unmultiplied(0, 0, 0, 192);
    visuals
}

pub fn get_default_egui_frame() -> egui::Frame {
    let frame = egui::Frame {
        rounding: Rounding::none(),
        shadow: Shadow::NONE,
        fill: Color32::from_rgba_unmultiplied(0, 0, 0, 125),
        stroke: Stroke::NONE,
        inner_margin: Margin::symmetric(8.0, 8.0),
        outer_margin: Margin::symmetric(-8.0, -8.0),
    };
    frame
}
