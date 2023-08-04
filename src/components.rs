use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct RollingBall;

#[derive(Debug, Component)]
pub struct Ground;

#[derive(Debug, Component)]
pub struct GroundMesh;

#[derive(Debug, Component)]
pub struct BelongsToGround(pub Entity);

#[derive(Debug, Component)]
pub struct GroundSurfaceSensor;

#[derive(Debug, Component)]
pub struct GroundMidSensor;

#[derive(Debug, Component)]
pub struct Wall;

#[derive(Debug, Component)]
pub struct MyLight {
    pub init_transform: Transform,
}

#[derive(Debug, Component)]
pub struct MyCamera {
    pub init_transform: Transform,
}
