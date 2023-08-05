use bevy::{prelude::*, time::Stopwatch};
use bevy_egui::{
    egui::{self, FontId, RichText},
    EguiContexts,
};

use crate::resources::{GroundsResource, PreviousScoresRes, ScoresResource};

use super::egui::get_default_egui_frame;

pub fn setup_scoring(mut scoring_res: ResMut<ScoresResource>, time: Res<Time>) {
    scoring_res.stopwatch = Some(Stopwatch::new());
    scoring_res.grounds_passed = 0;
}

pub fn update_stopwatch(mut scoring_res: ResMut<ScoresResource>, time: Res<Time>) {
    let Some(stopwatch) = scoring_res.stopwatch.as_mut() else { return; };
    stopwatch.tick(time.delta());
}

pub fn update_grounds_passed(
    grounds_res: Res<GroundsResource>,
    mut scoring_res: ResMut<ScoresResource>,
    mut prev_ground: Local<Option<Entity>>,
) {
    if !grounds_res.is_changed() {
        return;
    }
    let GroundsResource { previous_ground: Some(previous_ground), .. } = *grounds_res else { return; };
    if *prev_ground != Some(previous_ground) {
        scoring_res.grounds_passed += 1;
        *prev_ground = Some(previous_ground);
    }
}

pub fn display_scoreboard(
    scoring_res: Res<ScoresResource>,
    prev_scoring_res: Res<PreviousScoresRes>,
    mut egui_contexts: EguiContexts,
) {
    let ScoresResource { stopwatch: Some(stopwatch), grounds_passed } = scoring_res.as_ref() else { return; };
    let watch_display = format!(
        "{:02.0}:{:02.0}",
        stopwatch.elapsed_secs() / 60.,
        stopwatch.elapsed_secs() % 60.
    );
    let score_display = format!("Time: {}  Panels: {}", watch_display, grounds_passed);
    let prev_scores_display = prev_scoring_res.0.iter().filter_map(|scoring_res| {
        let ScoresResource { stopwatch: Some(stopwatch), grounds_passed } = scoring_res
        else { return None; };
        let watch_display = format!(
            "{:02.0}:{:02.0}",
            stopwatch.elapsed_secs() / 60.,
            stopwatch.elapsed_secs() % 60.
        );
        let score_display = format!("Time: {}  Panels: {}", watch_display, grounds_passed);
        Some(score_display)
    });

    let frame = get_default_egui_frame();
    // println!(
    //     "Time: {}, grounds_passed: {}",
    //     watch_display, grounds_passed
    // );
    egui::Window::new("Scoreboard")
        .title_bar(false)
        .collapsible(false)
        .movable(false)
        .resizable(false)
        .frame(frame)
        .anchor(egui::Align2::RIGHT_TOP, egui::vec2(0.0, 0.0))
        .show(egui_contexts.ctx_mut(), |ui| {
            ui.vertical(|ui| {
                ui.label(RichText::new("Score").heading());
                ui.label(RichText::new(score_display).size(17.));
                ui.separator();
                if prev_scores_display.clone().count() > 0 {
                    ui.label(RichText::new("Previous").heading());
                    for score_display in prev_scores_display {
                        ui.label(score_display);
                    }
                    ui.separator();
                }
            });
        });
}
