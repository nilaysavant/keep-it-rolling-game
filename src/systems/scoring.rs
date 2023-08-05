use bevy::{prelude::*, time::Stopwatch};

use crate::resources::{GroundsResource, ScoresResource};

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

pub fn display_score(scoring_res: Res<ScoresResource>) {
    let ScoresResource { stopwatch: Some(stopwatch), grounds_passed } = scoring_res.as_ref() else { return; };
    let watch_display = format!(
        "{:02.0}:{:02.0}",
        stopwatch.elapsed_secs() / 60.,
        stopwatch.elapsed_secs() % 60.
    );
    println!(
        "Time: {}, grounds_passed: {}",
        watch_display, grounds_passed
    );
}
