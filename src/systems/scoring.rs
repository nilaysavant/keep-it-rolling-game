use bevy::{prelude::*, time::Stopwatch};

use crate::resources::ScoringResource;

pub fn setup_scoring(mut scoring_res: ResMut<ScoringResource>, time: Res<Time>) {
    scoring_res.stopwatch = Some(Stopwatch::new());
    scoring_res.grounds_passed = 0;
}

pub fn update_stopwatch(mut scoring_res: ResMut<ScoringResource>, time: Res<Time>) {
    let Some(stopwatch) = scoring_res.stopwatch.as_mut() else { return; };
    stopwatch.tick(time.delta());
}

pub fn display_score(mut scoring_res: ResMut<ScoringResource>) {
    let Some(stopwatch) = scoring_res.stopwatch.as_mut() else { return; };
    let watch_display = format!(
        "{:02.0}:{:02.0}",
        stopwatch.elapsed_secs() / 60.,
        stopwatch.elapsed_secs() % 60.
    );
    println!("Time: {}", watch_display);
}
