use bevy::{prelude::*, time::Stopwatch};
use bevy_inspector_egui::InspectorOptions;

#[derive(Clone, Resource, Default, Debug, Reflect, InspectorOptions)]
#[reflect(Resource)]
pub struct GroundsResource {
    pub previous_ground: Option<Entity>,
    pub current_ground: Option<Entity>,
    pub next_ground: Option<Entity>,
}

#[derive(Clone, Resource, Default, Debug, Reflect, InspectorOptions)]
#[reflect(Resource)]
pub struct ScoresResource {
    pub stopwatch: Option<Stopwatch>,
    pub grounds_passed: u64,
}

#[derive(Clone, Resource, Default, Debug, Reflect, InspectorOptions)]
#[reflect(Resource)]
pub struct PreviousScoresRes(pub Vec<ScoresResource>);

#[derive(Clone, Resource, Debug, Reflect, InspectorOptions)]
#[reflect(Resource)]
pub struct SettingsResource {
    pub rotation_sensitivity: f32,
}

impl Default for SettingsResource {
    fn default() -> Self {
        Self {
            rotation_sensitivity: 0.05,
        }
    }
}
