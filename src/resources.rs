use bevy::prelude::*;
use bevy_inspector_egui::InspectorOptions;

#[derive(Clone, Resource, Default, Debug, Reflect, InspectorOptions)]
#[reflect(Resource)]
pub struct GroundsResource {
    pub previous_ground: Option<Entity>,
    pub current_ground: Option<Entity>,
    pub next_ground: Option<Entity>,
}
