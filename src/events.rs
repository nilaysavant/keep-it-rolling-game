use bevy::prelude::*;

#[derive(Debug, Clone, Event)]
pub enum WallEvent {
    HoverUpdate {
        ground: Entity,
        transform: Transform,
    },
    HoverStop,
    Draw,
}

#[derive(Debug, Clone, Event)]
pub enum SceneEvent {
    Start,
    Restart,
}
