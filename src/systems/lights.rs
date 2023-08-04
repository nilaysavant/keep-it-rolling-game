use bevy::prelude::*;
use bevy_rapier3d::prelude::Velocity;

use crate::{
    components::{Ground, MyLight, RollingBall},
    resources::GroundsResource,
};

pub fn move_lighting_with_grounds(
    mut query_lights: Query<(&mut Transform, &MyLight)>,
    query_grounds: Query<&Transform, (With<Ground>, Without<MyLight>)>,
    query_ball: Query<&Velocity, With<RollingBall>>,
    ground_res: Res<GroundsResource>,
    time: Res<Time>,
) {
    let Some(current_ground) = ground_res.current_ground else { return; };
    let Ok(current_ground_transform) = query_grounds.get(current_ground) else { return; };
    let Ok((mut light_transform, MyLight { init_transform })) = query_lights.get_single_mut() else { return; };
    let Ok(ball_vel) = query_ball.get_single() else { return; };
    let light_transform_lerp_fact = 1. * ball_vel.linvel.z;
    light_transform.translation = light_transform.translation.lerp(
        current_ground_transform.translation + init_transform.translation,
        time.delta_seconds() * light_transform_lerp_fact,
    );
}
